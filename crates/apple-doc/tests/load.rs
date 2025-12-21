use std::{
    fs,
    sync::{LazyLock, Mutex},
};

use apple_doc::{
    external_dir, AvailabilityLookup, BlobStore, Content, DocKind, IndexId, Lmdb, NavigatorItem,
    SqliteDb, TxtItem,
};

#[test]
fn sqlite() {
    let sqlite_db = SqliteDb::from_external_dir(&external_dir()).unwrap();
    for _ in sqlite_db.all_data().unwrap() {}
    for _ in sqlite_db.all_refs().unwrap() {}
}

#[test]
fn fs() {
    let sqlite_db = SqliteDb::from_external_dir(&external_dir()).unwrap();
    let mut blobs = BlobStore::from_external_dir(&sqlite_db, &external_dir()).unwrap();

    for r in sqlite_db.all_refs().unwrap() {
        if r.uuid.starts_with("io/published") {
            // PNG or SVG data.
            let _data = blobs.data(&r).unwrap();
        } else {
            // Pick out ~50 to test by default, to avoid loading and
            // decompressing the entire 2GB dataset into memory.
            if cfg!(feature = "full-test") || (r.row_id % 10000 == 0) {
                let _doc = blobs.parse_doc(&r).unwrap();
            }
        }
    }
}

static LMDB: LazyLock<Mutex<Lmdb>> =
    LazyLock::new(|| Mutex::new(Lmdb::from_external_dir(&external_dir()).unwrap()));

#[test]
fn lmdb_load() {
    let lmdb = LMDB.lock().unwrap();

    for entry in lmdb.index.iter(&lmdb.rtxn).unwrap() {
        let (key, _) = entry.unwrap();

        let _value = lmdb.lookup_by_id(key).unwrap().unwrap();
    }
}

#[test]
fn lmdb_two_keys() {
    let lmdb = LMDB.lock().unwrap();

    // Test a two keys that map to the same identifier.
    assert_eq!(
        lmdb.lookup_by_id(IndexId::Hex("2b09780c2c97")).unwrap(),
        Some("objective-c/documentation/coregraphics/cgcontext/converttouserspace(_:)-3mtg3")
    );
    assert_eq!(
        lmdb.lookup_by_id(IndexId::Lang("Objective-C-1jvd7bu"))
            .unwrap(),
        Some("objective-c/documentation/coregraphics/cgcontext/converttouserspace(_:)-3mtg3")
    );
}

#[test]
fn availability() {
    let lookup = AvailabilityLookup::from_external_dir(&external_dir()).unwrap();

    let lmdb = LMDB.lock().unwrap();

    let data = fs::read(external_dir().join("index/navigator.index")).unwrap();
    for (_section_id, entry) in NavigatorItem::iter_all(&data) {
        let platform_availability_ids = lmdb
            .availability
            .get(&lmdb.rtxn, &entry.availability_id)
            .unwrap()
            .unwrap();

        for id in platform_availability_ids {
            let _platform_availability = lookup.data.get(id).unwrap();
        }

        assert!(
            !format!("{:?}", entry.kind).contains("("),
            "had unknown kind: {:?}",
            entry.kind
        );
    }
}

#[test]
fn txt() {
    // let sqlite_db = SqliteDb::from_external_dir(&external_dir()).unwrap();

    for filename in ["0.txt", "1.txt", "3.txt"] {
        let txt = fs::read_to_string(external_dir().join(filename)).unwrap();
        for item in TxtItem::iter(&txt) {
            assert!(
                !format!("{:?}", item.kind).contains("("),
                "txt item (from {filename}) had unknown kind: {:?}",
                item
            );

            // TODO: Some items are not here?
            // if !sqlite_db.get_ref(&item.id).unwrap().is_some() {
            //     panic!("failed looking up txt item (from {filename}) in cache: {item:?}");
            // }
        }
    }
}

/// Compare data about `applicationDidFinishLaunching:` gotten from txt->cache->fs.
#[test]
fn get_and_compare_result() {
    let external_dir = external_dir();
    let sqlite_db = SqliteDb::from_external_dir(&external_dir).unwrap();
    let mut blobs = BlobStore::from_external_dir(&sqlite_db, &external_dir).unwrap();

    let txt = fs::read_to_string(external_dir.join("1.txt")).unwrap();
    for item in TxtItem::iter(&txt).filter(|item| item.name == "applicationDidFinishLaunching:") {
        let r = sqlite_db.get_ref(item.uuid).unwrap().unwrap();
        let doc = blobs.parse_doc(&r).unwrap();
        let DocKind::Symbol(page) = &doc.kind else {
            panic!("unexpected doc kind for symbol: {doc:?}");
        };

        match doc.metadata.external_id.as_deref() {
            Some("c:objc(pl)NSApplicationDelegate(im)applicationDidFinishLaunching:") => {
                assert_eq!(
                page.abstract_,
                vec![Content::Text {
                    text: "Tells the delegate that the app’s initialization is complete but it hasn’t received its first event.".to_string()
                }]
            );
            }
            Some("c:objc(pl)UIApplicationDelegate(im)applicationDidFinishLaunching:") => {
                assert_eq!(
                    page.abstract_,
                    vec![Content::Text {
                        text: "Tells the delegate when the app has finished launching.".to_string()
                    }]
                );
            }
            _ => panic!("unexpected doc page: {doc:?}"),
        }
    }
}
