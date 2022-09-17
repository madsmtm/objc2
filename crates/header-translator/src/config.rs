use std::collections::HashMap;
use std::fs;
use std::io::Result;
use std::path::{Path, PathBuf};

use serde::Deserialize;

type ClassName = String;
type Selector = String;

#[derive(Default, Deserialize)]
struct Unsafe {
    #[serde(rename = "safe-methods")]
    safe_methods: HashMap<ClassName, Vec<Selector>>,
}

#[derive(Default, Deserialize)]
struct Skipped {
    methods: HashMap<ClassName, Vec<Selector>>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct InnerConfig {
    name: Option<String>,
    headers: Option<Vec<String>>,
    output: Option<PathBuf>,
    #[serde(rename = "unsafe")]
    #[serde(default)]
    unsafe_: Unsafe,
    #[serde(rename = "mutating-methods")]
    #[serde(default)]
    mutating_methods: HashMap<ClassName, Vec<Selector>>,
    #[serde(default)]
    skipped: Skipped,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ClassData {
    pub selector_data: HashMap<Selector, MethodData>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct MethodData {
    pub safe: bool,
    pub skipped: bool,
    // TODO: mutating
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub name: String,
    pub headers: Vec<String>,
    /// The output path, relative to the toml file.
    pub output: PathBuf,
    pub class_data: HashMap<ClassName, ClassData>,
}

impl Config {
    pub fn from_file(file: &Path) -> Result<Self> {
        let s = fs::read_to_string(file)?;

        let config: InnerConfig = toml::from_str(&s)?;

        let name = config.name.unwrap_or_else(|| {
            file.file_stem()
                .expect("file stem")
                .to_string_lossy()
                .into_owned()
        });
        let headers = config.headers.unwrap_or_else(|| vec![format!("{name}.h")]);
        let parent = file.parent().expect("parent");
        let output = parent.join(
            config
                .output
                .unwrap_or_else(|| Path::new("generated").join(&name)),
        );

        let mut class_data: HashMap<ClassName, ClassData> = HashMap::new();

        for (class_name, selectors) in config.unsafe_.safe_methods {
            let data = class_data.entry(class_name).or_default();
            for sel in selectors {
                data.selector_data.entry(sel).or_default().safe = true;
            }
        }

        for (class_name, selectors) in config.skipped.methods {
            let data = class_data.entry(class_name).or_default();
            for sel in selectors {
                data.selector_data.entry(sel).or_default().skipped = true;
            }
        }

        Ok(Self {
            name,
            headers,
            output,
            class_data,
        })
    }
}
