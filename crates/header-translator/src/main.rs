use std::collections::BTreeMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use clang::source::File;
use clang::{Clang, Entity, EntityKind, EntityVisitResult, Index};

use header_translator::{create_rust_file, Config};

fn main() {
    // let sysroot = Path::new("/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk");
    let sysroot = Path::new("./ideas/MacOSX-SDK-changes/MacOSXA.B.C.sdk");
    let framework_path = sysroot.join("System/Library/Frameworks");

    let clang = Clang::new().unwrap();

    let index = Index::new(&clang, true, true);

    let config = dbg!(Config::from_file(Path::new("icrate/src/AppKit.toml")).unwrap());
    let config =
        dbg!(Config::from_file(Path::new("objc2/src/foundation/Foundation.toml")).unwrap());

    let _module_path = framework_path.join("module.map");

    // for entry in framework_path.read_dir().unwrap() {
    //     let dir = entry.unwrap();
    //     println!("{:?}", dir.file_name());
    //     if dir.file_type().unwrap().is_dir() {

    let dir = framework_path.join("Foundation.framework").join("Headers");
    let header = dir.join("Foundation.h");

    let tu = index
        .parser(&header)
        .detailed_preprocessing_record(true)
        // .single_file_parse(true)
        .skip_function_bodies(true)
        .include_attributed_types(true)
        .visit_implicit_attributes(true)
        .retain_excluded_conditional_blocks(true)
        .arguments(&[
            "-x",
            "objective-c",
            "-fobjc-arc",
            "-fobjc-abi-version=2", // 3??
            // "-mmacosx-version-min="
            "-fparse-all-comments",
            // "-fapinotes",
            "-isysroot",
            sysroot.to_str().unwrap(),
        ])
        .parse()
        .unwrap();

    dbg!(&tu);
    dbg!(tu.get_target());
    dbg!(tu.get_memory_usage());
    dbg!(tu.get_diagnostics());

    // let dbg_file = |file: File<'_>| {
    //     dbg!(
    //         &file,
    //         file.get_module(),
    //         file.get_skipped_ranges(),
    //         file.is_include_guarded(),
    //         // file.get_includes(),
    //         // file.get_references(),
    //     );
    // };
    //
    // dbg_file(tu.get_file(&header).unwrap());
    // dbg_file(tu.get_file(&dir.join("NSAccessibility.h")).unwrap());
    // let cursor_file = tu.get_file(&dir.join("NSCursor.h")).unwrap();
    // dbg_file(cursor_file);

    let entity = tu.get_entity();

    dbg!(&entity);
    dbg!(entity.get_availability());

    let mut entities_left = usize::MAX;

    let mut result: BTreeMap<PathBuf, Vec<Entity<'_>>> = BTreeMap::new();

    entity.visit_children(|entity, _parent| {
        // EntityKind::InclusionDirective
        // if let Some(file) = entity.get_file() {
        //     let path = file.get_path();
        //     if path.starts_with(&dir) {
        //         result.entry(path).or_default().push(entity);
        //     }
        // }
        if let Some(location) = entity.get_location() {
            if let Some(file) = location.get_file_location().file {
                let path = file.get_path();
                if path.starts_with(&dir) {
                    result.entry(path).or_default().push(entity);
                }
                // entities_left = 20;
                // println!("{:?}: {}", entity.get_kind(), entity.get_display_name().unwrap_or_else(|| "`None`".to_string()));
                // if entity.get_display_name().as_deref() == Some("TARGET_OS_IPHONE") {
                // dbg!(&entity);
                // dbg!(&entity.get_range());
                // dbg!(&entity.get_children());
                // }
            }
        }

        // if entities_left < 100 {
        //     dbg!(&entity);
        // }

        // if let Some(left) = entities_left.checked_sub(1) {
        //     entities_left = left;
        //     EntityVisitResult::Recurse
        // } else {
        //     EntityVisitResult::Break
        // }
        // if e.get_kind() == EntityKind::StructDecl {
        //     // EntityVisitResult::Recurse
        // } else {
        //     EntityVisitResult::Break
        // }
        EntityVisitResult::Continue
    });

    // for entity in &result[&dir.join("NSCursor.h")] {
    //     println!("{:?}: {}", entity.get_kind(), entity.get_display_name().unwrap_or_else(|| "`None`".to_string()));
    //     if let Some(comment) = entity.get_comment() {
    //         println!("{}", comment);
    //     }
    // }

    for res in result.values() {
        let res = format!("{}", create_rust_file(&res));

        // println!("{}\n\n\n\n", res);

        let mut child = Command::new("rustfmt")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed running rustfmt");

        let mut stdin = child.stdin.take().expect("failed to open stdin");
        stdin.write_all(res.as_bytes()).expect("failed writing");
        drop(stdin);

        let output = child.wait_with_output().expect("failed formatting");
        // println!("{}", String::from_utf8(output.stdout).unwrap());
    }

    //     }
    // }
}
