use std::collections::hash_map::DefaultHasher;
use std::env;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::process::Command;

fn main() {
    generate_rust_types_from_shader_types();
    compile_shaders();
}

// xcrun -sdk macosx metal -c shaders.metal -o shaders.air
// xcrun -sdk macosx metallib shaders.air -o shaders.metallib
fn compile_shaders() {
    println!("cargo:rerun-if-changed=shaders.metal");
    println!("cargo:rerun-if-changed=shader_types.h");

    let output = Command::new("xcrun")
        .arg("-sdk")
        .arg("macosx")
        .arg("metal")
        .args(&["-c", "shaders.metal"])
        .args(&["-o", "shaders.air"])
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();
    if !output.status.success() {
        panic!(
            r#"
stdout: {}
stderr: {}
"#,
            String::from_utf8(output.stdout).unwrap(),
            String::from_utf8(output.stderr).unwrap()
        );
    }

    Command::new("xcrun")
        .arg("-sdk")
        .arg("macosx")
        .arg("metallib")
        .arg("shaders.air")
        .args(&["-o", "shaders.metallib"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

fn generate_rust_types_from_shader_types() {
    println!("cargo:rerun-if-changed=shader_types");

    let current_hash = hash_shader_types();

    if let Some(old_hash) = read_cached_shader_types_hash() {
        if old_hash == current_hash {
            return;
        }
    }

    let bindings = bindgen::Builder::default()
        .header("shader_types/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out = out.join("shader_bindings.rs");

    bindings.write_to_file(out).unwrap();

    save_shader_types_hash(current_hash);
}

fn hash_shader_types() -> u64 {
    let mut hasher = DefaultHasher::new();

    PathBuf::from("shader_types")
        .read_dir()
        .unwrap()
        .for_each(|entry| {
            let entry = entry.unwrap();
            let file = std::fs::read(entry.path()).unwrap();

            file.hash(&mut hasher);
        });

    hasher.finish()
}

fn read_cached_shader_types_hash() -> Option<u64> {
    let hash = shader_types_hash_file();
    let hash = match std::fs::read(hash) {
        Ok(hash) => Some(hash),
        _ => None,
    }?;

    let hash = String::from_utf8(hash).unwrap().parse::<u64>().unwrap();

    Some(hash)
}

fn save_shader_types_hash(hash: u64) {
    std::fs::write(shader_types_hash_file(), format!("{}", hash)).unwrap();
}

fn shader_types_hash_file() -> PathBuf {
    PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("shader_types_hash")
}
