use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
struct CargoToml {
    features: BTreeMap<String, Vec<String>>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let crates_dir = manifest_dir.parent().unwrap();
    let cargo_toml = fs::read_to_string(crates_dir.join("icrate").join("Cargo.toml"))?;

    let CargoToml { features } = toml::from_str(&cargo_toml)?;

    println!("Testing all Foundation features in `icrate`");

    for feature in features.keys() {
        if feature.contains("gnustep") {
            // Skip GNUStep-related features
            continue;
        }
        if feature.contains("_all") || feature.contains("unstable-frameworks-") {
            // Skip "_all" features for now
            continue;
        }
        if !feature.contains("Foundation") {
            // Skip all other than "Foundation" features for now
            continue;
        }

        println!("Testing {feature:?}");

        let status = Command::new("cargo")
            .args([
                "check",
                "--package=icrate",
                "--features=Foundation",
                "--features",
                feature,
            ])
            .current_dir(crates_dir)
            .status()?;

        assert!(status.success(), "failed running cargo check");
    }

    Ok(())
}
