use std::env;
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
        .args(["-c", "shaders.metal"])
        .args(["-o", "shaders.air"])
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
        .args(["-o", "shaders.metallib"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

fn generate_rust_types_from_shader_types() {
    println!("cargo:rerun-if-changed=shader_types");

    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out = out.join("shader_bindings.rs");

    let bindings = bindgen::Builder::default()
        .header("shader_types/wrapper.h")
        .allowlist_type("TexturedVertex")
        .allowlist_type("TextureIndex")
        .allowlist_type("VertexInputIndex")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings.write_to_file(out).unwrap();
}
