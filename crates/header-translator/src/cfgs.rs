use std::fmt::Display;

use crate::config::LibraryConfig;
use crate::display_helper::FormatterFn;

#[derive(Debug, Copy, Clone, Default, PartialEq)]
enum CfgState {
    ShouldGate,
    /// Whether we emit a `cfg` or not is irrelevant, because it is already
    /// gated at a higher level.
    #[default]
    AlreadyGated,
    Omit,
}

impl CfgState {
    fn new(enabled: bool) -> Self {
        if enabled {
            Self::ShouldGate
        } else {
            Self::AlreadyGated
        }
    }

    fn explicit(should_gate: bool) -> Self {
        if should_gate {
            Self::ShouldGate
        } else {
            Self::Omit
        }
    }

    fn dependency(&mut self, dependency: bool) {
        *self = match (*self, dependency) {
            (Self::ShouldGate, true) => Self::ShouldGate,
            (Self::ShouldGate, false) => Self::Omit,
            (Self::AlreadyGated, _) => Self::AlreadyGated,
            (Self::Omit, _) => Self::Omit,
        };
    }

    fn implied(&mut self, implied: bool) {
        *self = match (*self, implied) {
            (Self::ShouldGate, false) => Self::ShouldGate,
            (_, false) => Self::AlreadyGated,
            (state, true) => state,
        };
    }

    fn active(&self) -> bool {
        matches!(self, Self::ShouldGate)
    }

    fn allowed_active(&self) -> bool {
        matches!(self, Self::ShouldGate | Self::AlreadyGated)
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlatformCfg {
    macos: CfgState,
    maccatalyst: CfgState,
    ios: CfgState,
    tvos: CfgState,
    watchos: CfgState,
    visionos: CfgState,
    gnustep: CfgState,
}

impl PlatformCfg {
    pub fn from_config(lib: &LibraryConfig) -> Self {
        Self {
            macos: CfgState::new(lib.macos.is_some()),
            // FIXME: Temporarily disable any form of Mac Catalyst handling,
            // as `cfg(target_abi = ...)` is only stable since Rust 1.78, and
            // that is higher than our MSRV.
            maccatalyst: CfgState::AlreadyGated,
            ios: CfgState::new(lib.ios.is_some()),
            tvos: CfgState::new(lib.tvos.is_some()),
            watchos: CfgState::new(lib.watchos.is_some()),
            visionos: CfgState::new(lib.visionos.is_some()),
            gnustep: CfgState::new(lib.gnustep),
        }
    }

    pub fn from_config_explicit(lib: &LibraryConfig) -> Self {
        Self {
            macos: CfgState::explicit(lib.macos.is_some()),
            // FIXME: Temporarily disable Mac Catalyst, see above
            maccatalyst: CfgState::AlreadyGated,
            ios: CfgState::explicit(lib.ios.is_some()),
            tvos: CfgState::explicit(lib.tvos.is_some()),
            watchos: CfgState::explicit(lib.watchos.is_some()),
            visionos: CfgState::explicit(lib.visionos.is_some()),
            // FIXME: Support this better?
            gnustep: CfgState::new(false),
        }
    }

    pub fn dependency(&mut self, dependency: &LibraryConfig) {
        self.macos.dependency(dependency.macos.is_some());
        // FIXME: Temporarily disable Mac Catalyst, see above
        // self.maccatalyst.dependency(dependency.maccatalyst.is_some());
        self.ios.dependency(dependency.ios.is_some());
        self.tvos.dependency(dependency.tvos.is_some());
        self.watchos.dependency(dependency.watchos.is_some());
        self.visionos.dependency(dependency.visionos.is_some());
        self.gnustep.dependency(dependency.gnustep);
    }

    pub fn implied(&mut self, implied: &LibraryConfig) {
        self.macos.implied(implied.macos.is_some());
        // FIXME: Temporarily disable Mac Catalyst, see above
        // self.maccatalyst.implied(implied.maccatalyst.is_some());
        self.ios.implied(implied.ios.is_some());
        self.tvos.implied(implied.tvos.is_some());
        self.watchos.implied(implied.watchos.is_some());
        self.visionos.implied(implied.visionos.is_some());
        self.gnustep.implied(implied.gnustep);
    }

    pub fn cfgs(&self) -> Option<impl Display + '_> {
        // Don't emit a cfg if we can avoid it
        if self.macos.allowed_active()
            && self.maccatalyst.allowed_active()
            && self.ios.allowed_active()
            && self.tvos.allowed_active()
            && self.watchos.allowed_active()
            && self.visionos.allowed_active()
            && self.gnustep.allowed_active()
        {
            return None;
        }

        Some(FormatterFn(|f| {
            match (
                self.macos.allowed_active(),
                self.maccatalyst.allowed_active(),
                self.ios.allowed_active(),
                self.tvos.allowed_active(),
                self.watchos.allowed_active(),
                self.visionos.allowed_active(),
                self.gnustep.allowed_active(),
            ) {
                // Emit `target_vendor = "apple"` if at all possible, since
                // it's more general.
                (true, true, true, true, true, true, false) => {
                    return write!(f, "target_vendor = \"apple\"");
                }
                // Emit negative `cfg`s when only a niche platform is
                // excluded, since it's more likely to be the "true" config
                // for this item (e.g. if something is excluded on iOS and
                // macOS, it'll likely be on other targets in the future. But
                // if something is excluded just for tvOS, that exclusion
                // probably only applies to tvOS).
                (true, false, true, true, true, true, true) => {
                    return write!(f, "not(target_abi = \"macabi\")");
                }
                (true, true, true, false, true, true, true) => {
                    return write!(f, "not(target_os = \"tvos\")");
                }
                (true, true, true, true, false, true, true) => {
                    return write!(f, "not(target_os = \"watchos\")");
                }
                (true, true, true, false, false, true, true) => {
                    // tvOS and watchOS are more bare-bones that the others
                    return write!(f, "not(any(target_os = \"tvos\", target_os = \"watchos\"))");
                }
                (true, true, true, true, true, false, true) => {
                    return write!(f, "not(target_os = \"visionos\")");
                }
                _ => {}
            }

            let mut cfgs: Vec<&str> = Vec::new();

            if self.macos.active() {
                cfgs.push("target_os = \"macos\"");
            }
            match (self.ios, self.maccatalyst) {
                (CfgState::ShouldGate, CfgState::ShouldGate | CfgState::AlreadyGated) => {
                    cfgs.push("target_os = \"ios\"")
                }
                (CfgState::ShouldGate, CfgState::Omit) => {
                    cfgs.push("all(target_os = \"ios\", not(target_abi = \"macabi\"))")
                }
                (CfgState::AlreadyGated, CfgState::ShouldGate) => cfgs.push("target_os = \"ios\""),
                (CfgState::AlreadyGated, _) => {}
                (CfgState::Omit, CfgState::ShouldGate) => cfgs.push("target_abi = \"macabi\""),
                (CfgState::Omit, _) => {}
            }
            if self.tvos.active() {
                cfgs.push("target_os = \"tvos\"");
            }
            if self.watchos.active() {
                cfgs.push("target_os = \"watchos\"");
            }
            if self.visionos.active() {
                cfgs.push("target_os = \"visionos\"");
            }
            if self.gnustep.active() {
                // FIXME: This will fail if it got emitted in `Cargo.toml`
                cfgs.push("feature = \"gnustep-1-7\"");
            }

            match &*cfgs {
                [] => write!(f, "any()"), // Should be unreachable in reality
                [cfg] => write!(f, "{cfg}"),
                cfgs => write!(f, "any({})", cfgs.join(", ")),
            }
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[track_caller]
    fn assert_cfgs(platform: &PlatformCfg, expected: &str) {
        let actual = platform
            .cfgs()
            .map(|cfg| cfg.to_string())
            .unwrap_or("all()".to_string());
        assert_eq!(expected, actual, "platform: {platform:?}");
    }

    #[test]
    fn basic() {
        let mut platform = PlatformCfg::default();
        assert_cfgs(&platform, r#"all()"#);

        platform.gnustep = CfgState::Omit;
        assert_cfgs(&platform, r#"target_vendor = "apple""#);

        platform.tvos = CfgState::Omit;
        assert_cfgs(&platform, r#"any()"#);

        platform.visionos = CfgState::ShouldGate;
        assert_cfgs(&platform, r#"target_os = "visionos""#);

        platform.watchos = CfgState::ShouldGate;
        assert_cfgs(
            &platform,
            r#"any(target_os = "watchos", target_os = "visionos")"#,
        );

        platform.watchos = CfgState::AlreadyGated;
    }

    #[test]
    #[allow(clippy::field_reassign_with_default)]
    fn maccatalyst() {
        let mut platform = PlatformCfg::default();
        platform.macos = CfgState::Omit;
        platform.tvos = CfgState::Omit;

        // iOS Gated
        platform.ios = CfgState::ShouldGate;
        platform.maccatalyst = CfgState::ShouldGate;
        assert_cfgs(&platform, r#"target_os = "ios""#);
        platform.maccatalyst = CfgState::AlreadyGated;
        assert_cfgs(&platform, r#"target_os = "ios""#);
        platform.maccatalyst = CfgState::Omit;
        assert_cfgs(
            &platform,
            r#"all(target_os = "ios", not(target_abi = "macabi"))"#,
        );

        // iOS Irrelevant
        platform.ios = CfgState::AlreadyGated;
        platform.maccatalyst = CfgState::ShouldGate;
        assert_cfgs(&platform, r#"target_os = "ios""#);
        platform.maccatalyst = CfgState::AlreadyGated;
        assert_cfgs(&platform, r#"any()"#);
        platform.maccatalyst = CfgState::Omit;
        assert_cfgs(&platform, r#"any()"#);

        // iOS Omit
        platform.ios = CfgState::Omit;
        platform.maccatalyst = CfgState::ShouldGate;
        assert_cfgs(&platform, r#"target_abi = "macabi""#);
        platform.maccatalyst = CfgState::AlreadyGated;
        assert_cfgs(&platform, r#"any()"#);
        platform.maccatalyst = CfgState::Omit;
        assert_cfgs(&platform, r#"any()"#);
    }

    #[test]
    #[allow(clippy::field_reassign_with_default)]
    fn systematic() {
        #[rustfmt::skip]
        let tests = &[
            ((true,  true),  (true,  true),  "all()"),                 // any(macos, ios)
            ((true,  false), (true,  true),  "target_os = \"ios\""),   // ios
            ((false, true),  (true,  true),  "all()"),                 // any(macos, ios) | ios
            ((false, false), (true,  true),  "all()"),                 // any(macos, ios) | ios
            ((true,  true),  (true,  false), "target_os = \"macos\""), // macos
            ((true,  false), (true,  false), "any()"),                 // any()
            ((false, true),  (true,  false), "any()"),                 // macos | any()
            ((false, false), (true,  false), "any()"),                 // macos | any()
            ((true,  true),  (false, true),  "all()"),                 // any(macos, ios) | macos
            ((true,  false), (false, true),  "any()"),                 // ios | any()
            ((false, true),  (false, true),  "all()"),                 // any(macos, ios) | macos | ios | any()
            ((false, false), (false, true),  "all()"),                 // any(macos, ios) | macos | ios | any()
            ((true,  true),  (false, false), "all()"),                 // any(macos, ios) | macos
            ((true,  false), (false, false), "any()"),                 // ios | any()
            ((false, true),  (false, false), "all()"),                 // any(macos, ios) | macos | ios | any()
            ((false, false), (false, false), "all()"),                 // any(macos, ios) | macos | ios | any()
        ];

        for ((macos_used, macos_avail), (ios_used, ios_avail), expected) in tests {
            let mut platform = PlatformCfg::default();
            platform.macos = CfgState::new(*macos_used);
            platform.macos.dependency(*macos_avail);
            platform.ios = CfgState::new(*ios_used);
            platform.ios.dependency(*ios_avail);
            platform.maccatalyst = platform.ios;

            assert_cfgs(&platform, expected);
        }
    }
}
