//! <https://docs.swift.org/swift-book/ReferenceManual/Attributes.html#ID583>
use std::fmt;

use clang::{Entity, PlatformAvailability, Version};

use crate::context::Context;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Unavailable {
    pub(crate) ios: bool,
    pub(crate) ios_app_extension: bool,
    pub(crate) macos: bool,
    pub(crate) macos_app_extension: bool,
    pub(crate) maccatalyst: bool,
    pub(crate) watchos: bool,
    pub(crate) tvos: bool,
    pub(crate) library_unavailablility: Option<Box<Unavailable>>,
}

impl Unavailable {
    pub fn list_oses(&self) -> Vec<&str> {
        let mut unavailable_oses = Vec::new();
        if self.ios
            && !self
                .library_unavailablility
                .as_ref()
                .map(|u| u.ios)
                .unwrap_or_else(|| false)
        {
            unavailable_oses.push("target_os = \"ios\"");
        }
        if self.macos
            && !self
                .library_unavailablility
                .as_ref()
                .map(|u| u.macos)
                .unwrap_or_else(|| false)
        {
            unavailable_oses.push("target_os = \"macos\"");
        }
        if self.tvos
            && !self
                .library_unavailablility
                .as_ref()
                .map(|u| u.tvos)
                .unwrap_or_else(|| false)
        {
            unavailable_oses.push("target_os = \"tvos\"");
        }
        if self.watchos
            && !self
                .library_unavailablility
                .as_ref()
                .map(|u| u.watchos)
                .unwrap_or_else(|| false)
        {
            unavailable_oses.push("target_os = \"watchos\"");
        }
        unavailable_oses
    }
}
impl fmt::Display for Unavailable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let unavailable_oses = self.list_oses();
        if !unavailable_oses.is_empty() {
            let unavailable_oses = unavailable_oses.join(",");
            write!(f, "#[cfg(not(any({unavailable_oses})))]")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
struct Versions {
    ios: Option<Version>,
    ios_app_extension: Option<Version>,
    macos: Option<Version>,
    macos_app_extension: Option<Version>,
    maccatalyst: Option<Version>,
    watchos: Option<Version>,
    tvos: Option<Version>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Availability {
    pub(crate) unavailable: Unavailable,
    introduced: Versions,
    deprecated: Versions,
    message: Option<String>,
    _swift: Option<PlatformAvailability>,
}

impl Availability {
    pub fn parse(
        entity: &Entity<'_>,
        _context: &Context<'_>,
        library_unavailablility: &Unavailable,
    ) -> Self {
        let availabilities = entity
            .get_platform_availability()
            .expect("platform availability");

        let mut unavailable = Unavailable::default();
        unavailable.library_unavailablility = Some(Box::new(library_unavailablility.clone()));
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
                "ios" => set(
                    availability,
                    &mut unavailable.ios,
                    &mut introduced.ios,
                    &mut deprecated.ios,
                ),
                "ios_app_extension" => set(
                    availability,
                    &mut unavailable.ios_app_extension,
                    &mut introduced.ios_app_extension,
                    &mut deprecated.ios_app_extension,
                ),
                "macos" => set(
                    availability,
                    &mut unavailable.macos,
                    &mut introduced.macos,
                    &mut deprecated.macos,
                ),
                "macos_app_extension" => set(
                    availability,
                    &mut unavailable.macos_app_extension,
                    &mut introduced.macos_app_extension,
                    &mut deprecated.macos_app_extension,
                ),
                "maccatalyst" => set(
                    availability,
                    &mut unavailable.maccatalyst,
                    &mut introduced.maccatalyst,
                    &mut deprecated.maccatalyst,
                ),
                "watchos" => set(
                    availability,
                    &mut unavailable.watchos,
                    &mut introduced.watchos,
                    &mut deprecated.watchos,
                ),
                "tvos" => set(
                    availability,
                    &mut unavailable.tvos,
                    &mut introduced.tvos,
                    &mut deprecated.tvos,
                ),
                "swift" => {
                    _swift = Some(availability);
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
}

impl fmt::Display for Availability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.deprecated {
            Versions {
                ios: None,
                ios_app_extension: None,
                macos: None,
                macos_app_extension: None,
                maccatalyst: None,
                watchos: None,
                tvos: None,
            } => {
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
        write!(f, "{}", self.unavailable)?;

        // TODO: Emit availability checks based on `self.introduced`
        Ok(())
    }
}
