use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

#[cfg(feature = "run")]
pub fn get_runtime() -> String {
    use regex::Regex;
    let re = Regex::new(r"--features[= ]+(([a-z0-9_-]+,?)+)").unwrap();

    let combined = std::env::args().collect::<Vec<_>>().join(" ");

    let result = re
        .captures_iter(&combined)
        .find_map(|c| {
            c.get(1).unwrap().as_str().split(',').find_map(|f| match f {
                "apple" => Some("apple"),
                "gnustep-1-7" | "gnustep-1-8" | "gnustep-1-9" => Some("gnustep-old"),
                "gnustep-2-0" | "gnustep-2-1" => Some("gnustep"),
                _ => None,
            })
        })
        .unwrap_or("apple")
        .to_string();
    result
}

#[cfg(not(feature = "run"))]
pub fn get_runtime() -> String {
    panic!("`run` feature must be enabled")
}

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

pub fn read_assembly<P: AsRef<Path>>(path: P, package_path: &Path) -> io::Result<String> {
    let s = fs::read_to_string(path.as_ref())?;
    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .as_os_str()
        .to_str()
        .unwrap();
    let s = s.replace(workspace_dir, "$WORKSPACE");
    let s = s.replace(
        package_path
            .as_os_str()
            .to_str()
            .unwrap()
            .strip_prefix(workspace_dir)
            .unwrap()
            .strip_prefix('/')
            .unwrap(),
        "$DIR",
    );
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
    let s = strip_lines(&s, ".file");
    // Added in nightly-2022-07-21
    let s = strip_lines(&s, ".no_dead_strip");
    // We remove the __LLVM,__bitcode and __LLVM,__cmdline sections because
    // they're uninteresting for out use-case.
    //
    // See https://github.com/rust-lang/rust/blob/1.59.0/compiler/rustc_codegen_llvm/src/back/write.rs#L978-L1074
    let s = strip_section(&s, "__LLVM");
    let s = demangle_assembly(&s);
    Ok(s)
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

/// VERY BRITTLE!
#[cfg(feature = "run")]
fn demangle_assembly(assembly: &str) -> String {
    use std::collections::HashMap;

    use lazy_static::lazy_static;
    use regex::Captures;
    use regex::Regex;

    lazy_static! {
        // All identifiers
        static ref RE_IDENT: Regex = Regex::new(r"[a-zA-Z_][a-zA-Z_0-9]*").unwrap();

        // Replace crate ID that the compiler inserts
        //
        // Example: test_msg_send_static_sel[f7bb0e08e35403f3]::handle_with_sel::NAME_DATA
        // Becomes: test_msg_send_static_sel[CRATE_ID]::handle_with_sel::NAME_DATA
        static ref RE_CRATE_ID: Regex = Regex::new(r"\[.*\]").unwrap();

        // Replace last part of symbol if it looks to be autogenerated
        //
        // Example: objc2_foundation::__string_macro::CFStringUtf16::as_ptr::hbadb49a829a98ec7
        // Becomes: objc2_foundation::__string_macro::CFStringUtf16::as_ptr::GENERATED_ID
        static ref RE_LAST: Regex = Regex::new(r"[a-z0-9]{17}$").unwrap();
    }

    // Demangled name -> List of seen mangled names for this demangled name
    let mut demangle_unique: HashMap<String, Vec<String>> = HashMap::new();

    // Find all identifiers, and attempt to demangle them
    RE_IDENT
        .replace_all(assembly, |caps: &Captures| {
            let ident = caps.get(0).unwrap().as_str();
            match rustc_demangle::try_demangle(ident) {
                Ok(s) => {
                    let s = s.to_string();
                    let s = RE_CRATE_ID.replace_all(&s, "[CRATE_ID]");
                    let s = RE_LAST.replace(&s, "GENERATED_ID");
                    let list_for_this_symbol = demangle_unique
                        .entry(s.to_string())
                        .or_insert_with(|| vec![ident.to_string()]);
                    let unique_identifier = list_for_this_symbol
                        .iter()
                        .position(|x| x == ident)
                        .unwrap_or_else(|| {
                            list_for_this_symbol.push(ident.to_string());
                            list_for_this_symbol.len() - 1
                        });

                    format!("SYM({}, {})", s, unique_identifier)
                }
                Err(_) => ident.to_string(),
            }
        })
        .to_string()
}

#[cfg(not(feature = "run"))]
fn demangle_assembly(_s: &str) -> String {
    panic!("`run` feature must be enabled")
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "run")]
    #[test]
    fn test_demangle() {
        use super::*;

        let before = r#"
    .section    __TEXT,__text,regular,pure_instructions
    .intel_syntax noprefix
    .globl  _handle_with_sel
    .p2align    4, 0x90
_handle_with_sel:
    push    rbp
    mov rbp, rsp
    lea rsi, [rip + __RNvNvCslgFcLFxF7mp_24test_msg_send_static_sel15handle_with_sel9NAME_DATA]
    pop rbp
    jmp _objc_msgSend

    .section    __TEXT,__objc_methname,cstring_literals
__RNvNvCslgFcLFxF7mp_24test_msg_send_static_sel15handle_with_sel9NAME_DATA:
    .asciz  "someSelector"
        "#;

        let after = r#"
    .section    __TEXT,__text,regular,pure_instructions
    .intel_syntax noprefix
    .globl  _handle_with_sel
    .p2align    4, 0x90
_handle_with_sel:
    push    rbp
    mov rbp, rsp
    lea rsi, [rip + SYM(test_msg_send_static_sel[CRATE_ID]::handle_with_sel::NAME_DATA, 0)]
    pop rbp
    jmp _objc_msgSend

    .section    __TEXT,__objc_methname,cstring_literals
SYM(test_msg_send_static_sel[CRATE_ID]::handle_with_sel::NAME_DATA, 0):
    .asciz  "someSelector"
        "#;
        let output = demangle_assembly(before);
        assert_eq!(output, after, "Got {}", output);

        let before = r#"
_get_ascii:
    stp x29, x30, [sp, #-16]!
    mov x29, sp
Lloh0:
    adrp    x0, l___unnamed_1@PAGE
Lloh1:
    add x0, x0, l___unnamed_1@PAGEOFF
    mov w1, #3
    bl  __ZN16objc2_foundation14__string_macro8is_ascii17h6ed9b17e599aba93E
    tbz w0, #0, LBB0_2
Lloh2:
    adrp    x0, __RNvNvCs9IkGjU4WDwV_14test_ns_string9get_ascii8CFSTRING@PAGE
Lloh3:
    add x0, x0, __RNvNvCs9IkGjU4WDwV_14test_ns_string9get_ascii8CFSTRING@PAGEOFF
    ldp x29, x30, [sp], #16
    b   __ZN16objc2_foundation14__string_macro13CFStringAscii6as_ptr17hb04bc801907abfefE
LBB0_2:
Lloh4:
    adrp    x0, __RNvNvCs9IkGjU4WDwV_14test_ns_string9get_asciis_8CFSTRING@PAGE
Lloh5:
    add x0, x0, __RNvNvCs9IkGjU4WDwV_14test_ns_string9get_asciis_8CFSTRING@PAGEOFF
    ldp x29, x30, [sp], #16
    b   __ZN16objc2_foundation14__string_macro13CFStringUtf166as_ptr17h2d998f5fc92d4caaE
    .loh AdrpAdd    Lloh0, Lloh1
    .loh AdrpAdd    Lloh2, Lloh3
    .loh AdrpAdd    Lloh4, Lloh5
        "#;

        let after = r#"
_get_ascii:
    stp x29, x30, [sp, #-16]!
    mov x29, sp
Lloh0:
    adrp    x0, l___unnamed_1@PAGE
Lloh1:
    add x0, x0, l___unnamed_1@PAGEOFF
    mov w1, #3
    bl  SYM(objc2_foundation::__string_macro::is_ascii::GENERATED_ID, 0)
    tbz w0, #0, LBB0_2
Lloh2:
    adrp    x0, SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 0)@PAGE
Lloh3:
    add x0, x0, SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 0)@PAGEOFF
    ldp x29, x30, [sp], #16
    b   SYM(objc2_foundation::__string_macro::CFStringAscii::as_ptr::GENERATED_ID, 0)
LBB0_2:
Lloh4:
    adrp    x0, SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 1)@PAGE
Lloh5:
    add x0, x0, SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 1)@PAGEOFF
    ldp x29, x30, [sp], #16
    b   SYM(objc2_foundation::__string_macro::CFStringUtf16::as_ptr::GENERATED_ID, 0)
    .loh AdrpAdd    Lloh0, Lloh1
    .loh AdrpAdd    Lloh2, Lloh3
    .loh AdrpAdd    Lloh4, Lloh5
        "#;
        let output = demangle_assembly(before);
        assert_eq!(output, after, "Got {}", output);
    }
}
