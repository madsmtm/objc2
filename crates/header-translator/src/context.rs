use std::borrow::Cow;
use std::collections::HashMap;
use std::ops;
use std::path::{Path, PathBuf};

use apple_sdk::SdkPath;
use clang::Entity;

use crate::config::Config;
use crate::id::Location;

pub struct Context<'a> {
    config: &'a Config,
    pub macro_invocations: HashMap<clang::source::Location<'a>, Entity<'a>>,
    framework_dir: PathBuf,
    include_dir: PathBuf,
}

impl<'a> Context<'a> {
    pub fn new(config: &'a Config, sdk: &SdkPath) -> Self {
        Self {
            config,
            macro_invocations: Default::default(),
            framework_dir: sdk.path.join("System/Library/Frameworks"),
            include_dir: sdk.path.join("usr/include"),
        }
    }

    pub fn get_location(&self, entity: &Entity<'_>) -> Option<Location> {
        if let Some(location) = entity.get_location() {
            if let Some(file) = location.get_file_location().file {
                let path = file.get_path();
                if let Ok(path) = path.strip_prefix(&self.framework_dir) {
                    let mut components: Vec<Cow<'_, str>> = path
                        .components()
                        .filter(|component| {
                            component.as_os_str() != "Headers"
                                && component.as_os_str() != "Frameworks"
                        })
                        .map(|component| component.as_os_str().to_str().expect("component to_str"))
                        .map(|component| component.strip_suffix(".framework").unwrap_or(component))
                        .map(|component| component.strip_suffix(".h").unwrap_or(component))
                        .map(|s| s.to_string().into())
                        .collect();

                    // Put items in umbrella header in `mod.rs`
                    if let [.., innermost_framework_name, file_name] = &*components {
                        let umbrella_header = self
                            .libraries
                            .get(&**innermost_framework_name)
                            .and_then(|lib| lib.umbrella_header.as_deref())
                            .unwrap_or(innermost_framework_name);

                        if file_name == umbrella_header {
                            let _ = components.pop();
                        }
                    }

                    return Some(Location::from_components(components));
                } else if let Ok(path) = path.strip_prefix(&self.include_dir) {
                    if path.starts_with("objc") {
                        return Some(Location::from_components(vec!["objc2".into()]));
                    }
                    if path == Path::new("MacTypes.h") {
                        return Some(Location::from_components(vec!["System".into()]));
                    }
                    if path.starts_with("sys") {
                        return Some(Location::from_components(vec!["libc".into()]));
                    }
                }
            }
        }
        None
    }
}

impl ops::Deref for Context<'_> {
    type Target = Config;

    fn deref(&self) -> &Self::Target {
        self.config
    }
}
