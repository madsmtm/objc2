//! Utility for testing frameworks' features.
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

const POPULAR_FOUNDATION_FEATURES: &[&str] = &[
    "NSString",
    "NSArray",
    "NSDictionary",
    "NSSet",
    "NSEnumerator",
    "NSError",
    "NSException",
    "NSValue",
    "NSThread",
];

fn get_pairs<'a>(items: &'a [&'a str]) -> impl Iterator<Item = (&'a str, &'a str)> + 'a {
    items
        .iter()
        .enumerate()
        .flat_map(|(i, &item1)| items[i..].iter().map(move |&item2| (item1, item2)))
}

fn get_features(cargo_toml: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    let cargo_toml = fs::read_to_string(cargo_toml)?;
    let CargoToml { features } = basic_toml::from_str(&cargo_toml)?;

    // Skip GNUStep-related and "all" features
    Ok(features
        .into_keys()
        .filter(|feature| !feature.contains("gnustep") && feature != "all")
        .collect())
}

fn test_feature_sets<'a>(
    success: &mut bool,
    workspace_dir: &Path,
    feature_sets: impl IntoIterator<Item = Vec<&'a str>>,
    package: &str,
) -> Result<(), Box<dyn Error>> {
    for features in feature_sets {
        let mut cmd = Command::new("cargo");
        cmd.arg("check");
        cmd.arg("--package");
        cmd.arg(package);
        cmd.arg("--features");
        cmd.arg(features.join(","));

        println!("running: {cmd:?}");

        cmd.current_dir(workspace_dir);
        cmd.arg("--quiet");
        let status = cmd.status()?;

        if !status.success() {
            *success = false;
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir.parent().unwrap().parent().unwrap();

    let mut success = true;

    println!("Testing all Foundation features");
    let features = get_features(
        &workspace_dir
            .join("framework-crates")
            .join("objc2-foundation")
            .join("Cargo.toml"),
    )?;
    let feature_sets = get_pairs(POPULAR_FOUNDATION_FEATURES)
        .map(|(feature1, feature2)| vec![feature1, feature2])
        .chain(features.iter().map(|feature| vec![&**feature]));
    test_feature_sets(
        &mut success,
        workspace_dir,
        feature_sets,
        "objc2-foundation",
    )?;

    println!("Testing all AppKit features");
    let features = get_features(
        &workspace_dir
            .join("framework-crates")
            .join("objc2-app-kit")
            .join("Cargo.toml"),
    )?;
    let feature_sets = features.iter().map(|feature| vec![&**feature]);
    test_feature_sets(&mut success, workspace_dir, feature_sets, "objc2-app-kit")?;

    println!("Testing all Metal features");
    let features = get_features(
        &workspace_dir
            .join("framework-crates")
            .join("objc2-metal")
            .join("Cargo.toml"),
    )?;
    let feature_sets = features.iter().map(|feature| vec![&**feature]);
    test_feature_sets(&mut success, workspace_dir, feature_sets, "objc2-metal")?;

    println!("Testing building each framework with `--features=all` and only their own features");
    for dir in workspace_dir.join("framework-crates").read_dir().unwrap() {
        let dir = dir.unwrap();
        if dir.file_type().unwrap().is_dir() {
            test_feature_sets(
                &mut success,
                workspace_dir,
                [vec!["all"]],
                dir.file_name().to_str().unwrap(),
            )?;
        }
    }

    if !success {
        panic!("one or more checks failed");
    }

    Ok(())
}
