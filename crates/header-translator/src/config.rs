use std::collections::HashMap;
use std::fs;
use std::io::Result;
use std::path::{Path, PathBuf};

use serde::Deserialize;

type ClassName = String;
type Selector = String;

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize)]
pub struct Unsafe {
    #[serde(rename = "safe-methods")]
    pub safe_methods: HashMap<ClassName, Vec<Selector>>,
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub name: String,
    pub headers: Vec<String>,
    /// The output path, relative to the toml file.
    pub output: PathBuf,
    pub unsafe_: Unsafe,
    pub mutating_methods: HashMap<ClassName, Vec<Selector>>,
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

        Ok(Self {
            name,
            headers,
            output,
            unsafe_: config.unsafe_,
            mutating_methods: config.mutating_methods,
        })
    }
}
