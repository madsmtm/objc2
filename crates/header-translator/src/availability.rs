use std::{
    cmp::Ordering,
    fmt::{self, Display},
};

use clang::{Entity, PlatformAvailability, Version};

use crate::{context::Context, display_helper::FormatterFn};

#[derive(Debug, Clone, PartialEq, Default)]
struct Unavailable {
    ios: bool,
    macos: bool,
    maccatalyst: bool,
    watchos: bool,
    tvos: bool,
    visionos: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Versions {
    pub(crate) macos: Option<Version>,
    pub(crate) maccatalyst: Option<Version>,
    pub(crate) ios: Option<Version>,
    pub(crate) tvos: Option<Version>,
    pub(crate) watchos: Option<Version>,
    pub(crate) visionos: Option<Version>,
}

impl Versions {
    const NONE: Self = Self {
        macos: None,
        maccatalyst: None,
        ios: None,
        tvos: None,
        watchos: None,
        visionos: None,
    };

    const RUST_OS_MIN: Self = Self {
        macos: Some(Version {
            x: 10,
            y: Some(12),
            z: None,
        }),
        maccatalyst: Some(Version {
            x: 13,
            y: Some(1),
            z: None,
        }),
        ios: Some(Version {
            x: 10,
            y: Some(0),
            z: None,
        }),
        tvos: Some(Version {
            x: 10,
            y: Some(0),
            z: None,
        }),
        watchos: Some(Version {
            x: 5,
            y: Some(0),
            z: None,
        }),
        visionos: Some(Version {
            x: 1,
            y: Some(0),
            z: None,
        }),
    };

    fn emit_if(&self, bound: &Self, condition: impl Fn(Version, Version) -> bool) -> Self {
        let filter = |this, bound| {
            if let Some(this) = this {
                if let Some(bound) = bound {
                    if (condition)(this, bound) {
                        Some(this)
                    } else {
                        None
                    }
                } else {
                    Some(this)
                }
            } else {
                None
            }
        };

        Self {
            macos: filter(self.macos, bound.macos),
            maccatalyst: filter(self.maccatalyst, bound.maccatalyst),
            ios: filter(self.ios, bound.ios),
            tvos: filter(self.tvos, bound.tvos),
            watchos: filter(self.watchos, bound.watchos),
            visionos: filter(self.visionos, bound.visionos),
        }
    }
}

/// <https://docs.swift.org/swift-book/ReferenceManual/Attributes.html#ID583>
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Availability {
    unavailable: Unavailable,
    introduced: Versions,
    deprecated: Versions,
    message: Option<String>,
    _swift: Option<PlatformAvailability>,
}

fn format_version(version: Version) -> impl Display {
    FormatterFn(move |f| {
        write!(f, "{}", version.x)?;

        if let Some(y) = version.y {
            write!(f, ".{}", y)?;

            if let Some(z) = version.z {
                write!(f, ".{}", z)?;
            }
        } else if let Some(z) = version.z {
            // Probably never gonna happen, but just to make sure
            write!(f, ".0.{}", z)?;
        }

        Ok(())
    })
}

fn version_cmp(left: Version, right: Version) -> Ordering {
    left.x
        .cmp(&right.x)
        .then_with(|| left.y.unwrap_or(0).cmp(&right.y.unwrap_or(0)))
        .then_with(|| left.z.unwrap_or(0).cmp(&right.z.unwrap_or(0)))
}

impl Availability {
    pub fn parse(entity: &Entity<'_>, _context: &Context<'_>) -> Self {
        let availabilities = entity
            .get_platform_availability()
            .expect("platform availability");

        let mut unavailable = Unavailable::default();
        let mut introduced = Versions::default();
        let mut deprecated = Versions::default();
        let mut message = None;
        let mut _swift = None;

        for availability in availabilities {
            let mut set = |availability: PlatformAvailability,
                           unavailable: &mut bool,
                           introduced: &mut Option<Version>,
                           deprecated: &mut Option<Version>| {
                *unavailable = availability.unavailable;
                *introduced = availability.introduced;
                *deprecated = availability.deprecated;

                if availability.obsoleted.is_some() {
                    // TODO: Handle obsoletions somehow, maybe by cfg-ing
                    // obsoleted things out?
                }

                if let Some(m) = availability.message {
                    if let Some(message) = message.as_deref() {
                        if m != message {
                            // TODO: Either use `cfg_attr` on the `deprecated`
                            // attributes, or merge it into a single string.
                            warn!(m, message, "message availability attributes were not equal");
                        }
                    }
                    message = Some(m);
                }
            };

            // TODO: Ensure that a specific platform only appears once
            match &*availability.platform {
                "macos" => set(
                    availability,
                    &mut unavailable.macos,
                    &mut introduced.macos,
                    &mut deprecated.macos,
                ),
                "maccatalyst" => set(
                    availability,
                    &mut unavailable.maccatalyst,
                    &mut introduced.maccatalyst,
                    &mut deprecated.maccatalyst,
                ),
                "ios" => set(
                    availability,
                    &mut unavailable.ios,
                    &mut introduced.ios,
                    &mut deprecated.ios,
                ),
                "tvos" => set(
                    availability,
                    &mut unavailable.tvos,
                    &mut introduced.tvos,
                    &mut deprecated.tvos,
                ),
                "watchos" => set(
                    availability,
                    &mut unavailable.watchos,
                    &mut introduced.watchos,
                    &mut deprecated.watchos,
                ),
                "xros" => set(
                    availability,
                    &mut unavailable.visionos,
                    &mut introduced.visionos,
                    &mut deprecated.visionos,
                ),
                "swift" => {
                    _swift = Some(availability);
                }
                "driverkit" | "bridgeos" => {
                    // Ignore
                }
                platform if platform.ends_with("_app_extension") => {
                    // Ignore availability attributes for app extensions
                }
                platform => error!(?platform, "unknown availability platform"),
            }
        }

        Self {
            unavailable,
            introduced,
            deprecated,
            message,
            _swift,
        }
    }

    pub fn new_deprecated(msg: impl Into<String>) -> Self {
        Self {
            deprecated: Versions::RUST_OS_MIN,
            message: Some(msg.into()),
            ..Default::default()
        }
    }

    /// Available and non-deprecated enum cases.
    pub fn is_available_non_deprecated(&self) -> bool {
        !matches!(
            self.unavailable,
            Unavailable {
                ios: true,
                macos: true,
                maccatalyst: true,
                watchos: true,
                tvos: true,
                visionos: true,
            }
        ) && !self.is_deprecated()
    }

    pub fn is_deprecated(&self) -> bool {
        !matches!(
            self.deprecated,
            Versions {
                ios: None,
                macos: None,
                maccatalyst: None,
                watchos: None,
                tvos: None,
                visionos: None,
            }
        )
    }

    pub fn check_is_available(&self) -> Option<impl Display + '_> {
        let mut introduced = self.introduced.emit_if(&Versions::RUST_OS_MIN, |v, rust| {
            version_cmp(v, rust).is_gt()
        });

        let unavailable = &self.unavailable;

        let max = Some(Version {
            x: 9999,
            y: None,
            z: None,
        });
        if unavailable.macos {
            introduced.macos = max;
        }
        if unavailable.maccatalyst {
            introduced.maccatalyst = max;
        }
        if unavailable.ios {
            introduced.ios = max;
        }
        if unavailable.tvos {
            introduced.tvos = max;
        }
        if unavailable.watchos {
            introduced.watchos = max;
        }
        if unavailable.visionos {
            introduced.visionos = max;
        }

        if introduced == Versions::NONE {
            return None;
        }

        Some(FormatterFn(move |f| {
            write!(f, "available!(")?;

            if let Some(version) = introduced.macos {
                write!(f, "macos = {}, ", format_version(version))?;
            }
            if let Some(version) = introduced.ios {
                write!(f, "ios = {}, ", format_version(version))?;
            }
            if let Some(version) = introduced.tvos {
                write!(f, "tvos = {}, ", format_version(version))?;
            }
            if let Some(version) = introduced.watchos {
                write!(f, "watchos = {}, ", format_version(version))?;
            }
            if let Some(version) = introduced.visionos {
                write!(f, "visionos = {}, ", format_version(version))?;
            }

            write!(f, "..)")?;

            // TODO: Add cfg!(not(...)) based on self.unavailable

            Ok(())
        }))
    }

    // Used when testing
    pub fn is_available_host(&self) -> bool {
        if self.unavailable.macos {
            return false;
        }
        if let Some(macos) = self.introduced.macos {
            // Disable test if introduced later than my current OS.
            // TODO: Use `available!` macro here.
            if 14 < macos.x {
                return false;
            }
        }
        // Disable test if deprecated.
        // Fixes `MLModelCollectionDidChangeNotification` not linking (it is
        // only marked as unavailable if the deployment target is macOS 15.0).
        if self.deprecated.macos.is_some() {
            return false;
        }
        true
    }
}

impl fmt::Display for Availability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.deprecated {
            _ if !self.is_deprecated() => {
                // Not deprecated
            }
            Versions { .. } => {
                // Deprecated
                // TODO: Use version data to output a more detailed message
                if let Some(message) = &self.message {
                    writeln!(f, "#[deprecated = {message:?}]")?;
                } else {
                    writeln!(f, "#[deprecated]")?;
                }
            }
        }
        // TODO: Emit `cfg` attributes based on `self.unavailable`
        // TODO: Emit availability checks based on `self.introduced`
        Ok(())
    }
}
