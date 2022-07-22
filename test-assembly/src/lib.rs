use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

fn strip_lines(data: &str, starts_with: &str) -> String {
    data.lines()
        .filter(|line| !line.trim_start().starts_with(starts_with))
        .collect::<Vec<_>>()
        .join("\n")
}

fn strip_section(data: &str, section: &str) -> String {
    let mut res = String::with_capacity(data.len());
    let mut in_removed_section = false;
    for line in data.lines() {
        // This only works for the __LLVM sections we're interested in
        if line.trim().starts_with(".section") {
            if line.contains(section) {
                in_removed_section = true;
                println!("Stripped {section} section");
            } else {
                in_removed_section = false;
            }
        }
        if !in_removed_section {
            res.push_str(line);
            res.push('\n');
        }
        if line.is_empty() {
            in_removed_section = false;
        }
    }
    res
}

pub fn read_assembly<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let s = fs::read_to_string(path)?;
    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .as_os_str()
        .to_str()
        .unwrap();
    let s = s.replace(workspace_dir, "$WORKSPACE");
    // HACK: Replace Objective-C image info for simulator targets
    let s = s.replace(
        ".asciz\t\"\\000\\000\\000\\000`\\000\\000\"",
        ".asciz\t\"\\000\\000\\000\\000@\\000\\000\"",
    );
    // Strip various uninteresting directives
    let s = strip_lines(&s, ".cfi_");
    let s = strip_lines(&s, ".macosx_version_");
    let s = strip_lines(&s, ".ios_version_");
    let s = strip_lines(&s, ".build_version");
    // We remove the __LLVM,__bitcode and __LLVM,__cmdline sections because
    // they're uninteresting for out use-case.
    //
    // See https://github.com/rust-lang/rust/blob/1.59.0/compiler/rustc_codegen_llvm/src/back/write.rs#L978-L1074
    Ok(strip_section(&s, "__LLVM"))
}

#[cfg(feature = "run")]
pub fn get_artifact(result_stream: &[u8], package: &str) -> PathBuf {
    use cargo_metadata::Message;
    Message::parse_stream(result_stream)
        .find_map(|message| {
            if let Message::CompilerArtifact(artifact) = message.unwrap() {
                // Brittle!
                if artifact.target.name == package && artifact.filenames.len() == 2 {
                    let path = artifact.filenames[1].clone();
                    let stem = path.file_stem().unwrap().strip_prefix("lib").unwrap();
                    return Some(path.with_file_name(format!("{stem}.s")));
                }
            }
            None
        })
        .unwrap_or_else(|| {
            panic!(
                "Could not find package data:\n{}",
                String::from_utf8_lossy(result_stream)
            )
        })
        .into_std_path_buf()
}

#[cfg(not(feature = "run"))]
pub fn get_artifact(_result_stream: &[u8], _package: &str) -> PathBuf {
    panic!("`run` feature must be enabled")
}
