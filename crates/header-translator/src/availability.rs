use std::fmt;

use clang::{Entity, PlatformAvailability, Version};

use crate::context::Context;

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

/// <https://docs.swift.org/swift-book/ReferenceManual/Attributes.html#ID583>
#[derive(Debug, Clone, PartialEq)]
pub struct Availability {
    unavailable: Unavailable,
    introduced: Versions,
    deprecated: Versions,
    message: Option<String>,
    _swift: Option<PlatformAvailability>,
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

                // TODO: Unsure how we would handle these if they exist
                if availability.obsoleted.is_some() {
                    error!("availability attribute containd `obsoleted`");
                }

                if let Some(m) = availability.message {
                    if let Some(message) = message.as_deref() {
                        if m != message {
                            error!(m, message, "message avalability attributes were not equal");
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
