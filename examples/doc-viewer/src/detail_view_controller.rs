use std::cell::RefCell;

use apple_doc::{external_dir, url_to_uuid, BlobStore, Lmdb, NavigatorItem, SqliteDb};
use objc2::rc::Retained;
use objc2::{define_class, msg_send, Ivars, MainThreadMarker, MainThreadOnly};
use objc2_app_kit::{NSResponder, NSViewController};
use objc2_foundation::NSObject;

define_class!(
    // SAFETY:
    // - We correctly override `NSViewController` methods.
    // - `SplitViewController` does not implement `Drop`.
    #[unsafe(super(NSViewController, NSResponder, NSObject))]
    pub struct DetailViewController {
        sqlite_db: SqliteDb,
        blobs: RefCell<BlobStore>,
        lmdb: Lmdb,
    }

    impl DetailViewController {
        // SAFETY: The signature is correct.
        #[unsafe(method(viewDidLoad))]
        fn view_did_load(&self) {
            let _: () = unsafe { msg_send![super(self), viewDidLoad] };
            let _mtm = self.mtm();

            // TODO
        }
    }
);

impl DetailViewController {
    pub fn new(mtm: MainThreadMarker) -> Retained<Self> {
        let external_dir = external_dir();

        let sqlite_db = apple_doc::SqliteDb::from_external_dir(&external_dir).unwrap();
        let blobs = RefCell::new(
            apple_doc::BlobStore::from_external_dir(&sqlite_db, &external_dir).unwrap(),
        );

        let lmdb = Lmdb::from_external_dir(&external_dir).unwrap();

        let this = Self::alloc(mtm).set_ivars(Ivars::<Self> {
            sqlite_db,
            blobs,
            lmdb,
        });
        // SAFETY: `DetailViewController` is safe to initialize.
        unsafe { msg_send![super(this), init] }
    }

    pub fn switch_to(&self, navigator_id: u32, item: &NavigatorItem<'_>) {
        let url = self.lmdb().url_from_navigator_id(navigator_id).unwrap();

        // The type of this depends on the kind of item.
        //
        // - `Kind::Module`: The underlying framework/module name
        //   (Core Animation -> QuartzCore).
        // - `Kind::Article`/SampleCode: Name is unique?
        //
        // - `Kind::Class`: `c:objc(cs)CALayer`.
        // - `Kind::Protocol`: `c:objc(pl)CALayerDelegate`.
        // - `Kind::Method`: `c:objc(pl)CAAction(im)runActionForKey:object:arguments:`.
        // - `Kind::Method`: `c:objc(pl)CAAction(im)runActionForKey:object:arguments:`.

        dbg!((url, item, item.kind));

        if let Some(url) = url {
            let uuid = url_to_uuid(url);
            if let Some(r) = self.sqlite_db().get_ref(&uuid).unwrap() {
                let doc = self.blobs().borrow_mut().parse_doc(&r).unwrap();
                dbg!(doc);
            } else {
                eprintln!("invalid ID: {uuid:?} in {url:?}");
            }
        }
    }
}
