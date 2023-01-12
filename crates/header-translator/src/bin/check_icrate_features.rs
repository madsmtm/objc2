//! # Utility for testing `icrate`'s feature set
//!
//! Run using:
//! ```sh
//! cargo run --bin=check_icrate_features --features=run
//! ```
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

const POPULAR_FEATURES: &[&str] = &[
    "Foundation_NSString",
    "Foundation_NSMutableString",
    "Foundation_NSArray",
    "Foundation_NSMutableArray",
    "Foundation_NSDictionary",
    "Foundation_NSMutableDictionary",
    "Foundation_NSSet",
    "Foundation_NSMutableSet",
    "Foundation_NSEnumerator",
    "Foundation_NSError",
    "Foundation_NSException",
    "Foundation_NSNumber",
    "Foundation_NSValue",
    "Foundation_NSThread",
];

fn get_pairs<'a>(items: &'a [&'a str]) -> impl Iterator<Item = (&'a str, &'a str)> + 'a {
    items
        .into_iter()
        .enumerate()
        .map(|(i, &item1)| items[i..].into_iter().map(move |&item2| (item1, item2)))
        .flatten()
}

fn main() -> Result<(), Box<dyn Error>> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let crates_dir = manifest_dir.parent().unwrap();
    let cargo_toml = fs::read_to_string(crates_dir.join("icrate").join("Cargo.toml"))?;

    let CargoToml { features } = toml::from_str(&cargo_toml)?;

    println!("Testing all Foundation features in `icrate`");

    let feature_sets = get_pairs(POPULAR_FEATURES)
        .map(|(feature1, feature2)| vec![feature1, feature2])
        .chain(features.keys().filter_map(|feature| {
            if feature.contains("gnustep") {
                // Skip GNUStep-related features
                None
            } else if feature.contains("_all") || feature.contains("unstable-frameworks-") {
                // Skip "_all" features for now
                None
            } else if !feature.contains("Foundation") {
                // Skip all other than "Foundation" features for now
                None
            } else {
                Some(vec![&**feature])
            }
        }));

    let mut success = true;

    for features in feature_sets {
        println!(
            "running: cargo check --features=Foundation,{}",
            features.join(",")
        );

        let status = Command::new("cargo")
            .args([
                "check",
                "--quiet",
                "--package=icrate",
                "--features=Foundation",
                "--features",
                &features.join(","),
            ])
            .current_dir(crates_dir)
            .status()?;

        if !status.success() {
            success = false;
        }
    }

    if !success {
        panic!("one or more checks failed");
    }

    Ok(())
}
