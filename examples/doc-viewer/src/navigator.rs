use std::cell::Cell;
use std::ptr::NonNull;

use apple_doc::{external_dir, Kind, NavigatorItem, NavigatorTree};
use block2::RcBlock;
use objc2::rc::Retained;
use objc2::runtime::{AnyObject, ProtocolObject};
use objc2::{define_class, msg_send, AnyThread, Ivars, MainThreadMarker, MainThreadOnly, Message};
use objc2_app_kit::{
    NSColor, NSControlTextEditingDelegate, NSFont, NSImage, NSImageScaling, NSImageView,
    NSLineBreakMode, NSOutlineView, NSOutlineViewDataSource, NSOutlineViewDelegate,
    NSOutlineViewSelectionDidChangeNotification, NSResponder, NSScrollView, NSTableCellView,
    NSTableColumn, NSTableViewStyle, NSTextField, NSView, NSViewController,
};
use objc2_foundation::{
    ns_string, NSIndexPath, NSInteger, NSNotFound, NSNotification, NSNotificationCenter, NSObject,
    NSObjectProtocol, NSPoint, NSRect, NSSize, NSString,
};

use crate::detail_view_controller::DetailViewController;

impl Navigator {
    // FIXME: Make it possible to avoid this boilerplate.
    pub fn new(
        mtm: MainThreadMarker,
        detail_view_controller: &DetailViewController,
    ) -> Retained<Self> {
        // TODO: Show an error here if the user doesn't have Xcode installed.
        let tree = NavigatorTree::from_external_dir(&external_dir()).unwrap();

        let image_module = NSImage::initWithContentsOfFile(
            NSImage::alloc(),
            ns_string!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/module.svg")),
        )
        .expect("failed loading module.svg");
        image_module.setTemplate(true);

        let this = Self::alloc(mtm);
        let this = this.set_ivars(Ivars::<Self> {
            tree,
            _observer: Cell::new(None),
            detail_view_controller: detail_view_controller.retain(),
            image_module,
        });
        // SAFETY: The controller is safe to initialize.
        unsafe { msg_send![super(this), init] }
    }

    pub fn outline_view(&self) -> Retained<NSOutlineView> {
        self.view()
            .downcast::<NSScrollView>()
            .unwrap()
            .documentView()
            .unwrap()
            .downcast::<NSOutlineView>()
            .unwrap()
    }

    fn load(&self) {
        let mtm = self.mtm();
        let outline = NSOutlineView::new(mtm);

        // Set various style preferences.
        outline.setAllowsColumnReordering(false);
        outline.setAllowsColumnResizing(true);
        outline.setStyle(NSTableViewStyle::SourceList);
        outline.setHeaderView(None);

        // When creating NSOutlineView outside interface builder, we must
        // create the table column that the view will use. Otherwise, our
        // delegate methods won't get called.
        let col = NSTableColumn::initWithIdentifier(mtm.alloc(), ns_string!("MainColumn"));
        outline.addTableColumn(&col);
        unsafe { outline.setOutlineTableColumn(Some(&col)) };

        // These are a lot of items in Apple's documentation bundle. The
        // data is static, so we could create and insert the nested view
        // structure up-front.
        //
        // That would probably be a lot of overhead, and most views aren't
        // visible to being with anyhow (they're collapsed inside the
        // outline), so instead, we make the outline view create the
        // required views on-the-fly using a data source and a delegate.
        unsafe { outline.setDataSource(Some(ProtocolObject::from_ref(self))) };
        unsafe { outline.setDelegate(Some(ProtocolObject::from_ref(self))) };

        // TODO: Implement autosaving
        // outline.setAutosaveName(Some(ns_string!("todo")));
        // outline.setAutosaveExpandedItems(true);

        // Wrap the outline view in a vertical scroll view.
        let scroll = NSScrollView::initWithFrame(
            mtm.alloc(),
            NSRect::new(NSPoint::ZERO, NSSize::new(250.0, 600.0)),
        );
        scroll.setDrawsBackground(false);
        scroll.setHasHorizontalScroller(false);
        scroll.setHorizontalScroller(None);
        scroll.setDocumentView(Some(&outline));

        self.setView(&scroll);

        let this = self.retain();
        let block = RcBlock::new(move |notification: NonNull<NSNotification>| {
            let notification = unsafe { notification.as_ref() };
            let outline_view = notification
                .object()
                .unwrap()
                .downcast::<NSOutlineView>()
                .unwrap();
            let row_indexes = outline_view.selectedRowIndexes();
            debug_assert!(row_indexes.count() <= 1);
            let idx = row_indexes.firstIndex() as isize;
            if idx == NSNotFound {
                return;
            }
            let item = outline_view.itemAtRow(idx);
            let index_path = index_iter_for_item(item.as_deref());
            if let Some(item_id) = this.tree().id_at(index_path.clone()) {
                let item = this.tree().item(item_id).unwrap();
                this.detail_view_controller().switch_to(item_id, &item);
            } else {
                debug_assert!(false, "failed finding item for index: {idx}, {item:?}");
            }
        });
        let center = NSNotificationCenter::defaultCenter();
        let observer = unsafe {
            center.addObserverForName_object_queue_usingBlock(
                Some(NSOutlineViewSelectionDidChangeNotification),
                None, // Some(outline_view),
                None, // No queue, run on posting thread (i.e. main thread)
                &block,
            )
        };
        self._observer().set(Some(observer));
    }
}

define_class!(
    // SAFETY:
    // - We correctly override `NSViewController` methods.
    // - The type does not implement `Drop`.
    #[unsafe(super(NSViewController, NSResponder, NSObject))]
    pub struct Navigator {
        tree: NavigatorTree,
        _observer: Cell<Option<Retained<ProtocolObject<dyn NSObjectProtocol>>>>,
        detail_view_controller: Retained<DetailViewController>,
        image_module: Retained<NSImage>,
    }

    unsafe impl NSObjectProtocol for Navigator {}

    // We use `NSIndexPath` as the "item" in the data source.
    unsafe impl NSOutlineViewDataSource for Navigator {
        #[unsafe(method(outlineView:numberOfChildrenOfItem:))]
        fn outlineView_numberOfChildrenOfItem(
            &self,
            _outline_view: &NSOutlineView,
            item: Option<&AnyObject>,
        ) -> NSInteger {
            // TODO: Optimize this with caching the amount of children inside
            // `NavigatorTree`?
            self.tree()
                .children_at(index_iter_for_item(item))
                .unwrap()
                .count() as isize
        }

        #[unsafe(method_id(outlineView:child:ofItem:))]
        fn outlineView_child_ofItem(
            &self,
            _outline_view: &NSOutlineView,
            index: NSInteger,
            item: Option<&AnyObject>,
        ) -> Retained<AnyObject> {
            if let Some(item) = item {
                let index_path = item.downcast_ref::<NSIndexPath>().unwrap();
                index_path.indexPathByAddingIndex(index as usize)
            } else {
                NSIndexPath::indexPathWithIndex(index as usize)
            }
            .into_super()
            .into_super()
        }

        #[unsafe(method(outlineView:isItemExpandable:))]
        fn outlineView_isItemExpandable(
            &self,
            _outline_view: &NSOutlineView,
            item: &AnyObject,
        ) -> bool {
            // Expandable if it has any children.
            self.tree()
                .children_at(index_iter_for_item(Some(item)))
                .unwrap()
                .next()
                .is_some()
        }

        // Only used for cell-based views, not actually needed any longer.
        #[unsafe(method_id(outlineView:objectValueForTableColumn:byItem:))]
        fn outlineView_objectValueForTableColumn_byItem(
            &self,
            _outline_view: &NSOutlineView,
            _table_column: Option<&NSTableColumn>,
            item: Option<&AnyObject>,
        ) -> Option<Retained<AnyObject>> {
            // If this was used, we should probably add `kind` here as well.
            self.tree()
                .item_at(index_iter_for_item(item))
                .map(|item| NSString::from_str(item.name).into_super().into_super())
        }

        // TODO: Persist open menu state.
        // #[unsafe(method_id(outlineView:itemForPersistentObject:))]
        // unsafe fn outlineView_itemForPersistentObject(
        //     &self,
        //     outline_view: &NSOutlineView,
        //     object: &AnyObject,
        // ) -> Option<Retained<AnyObject>> { todo!() }
        //
        // #[unsafe(method_id(outlineView:persistentObjectForItem:))]
        // unsafe fn outlineView_persistentObjectForItem(
        //     &self,
        //     outline_view: &NSOutlineView,
        //     item: Option<&AnyObject>,
        // ) -> Option<Retained<AnyObject>> { todo!() }

        // The user cannot edit the documentation, so we don't implement
        // `outlineView:setObjectValue:forTableColumn:byItem:`, nor do we
        // touch the drag-n-drop methods.
    }

    unsafe impl NSControlTextEditingDelegate for Navigator {}

    unsafe impl NSOutlineViewDelegate for Navigator {
        #[unsafe(method(outlineView:isGroupItem:))]
        fn outlineView_isGroupItem(&self, _outline_view: &NSOutlineView, _item: &AnyObject) -> bool {
            false
        }

        #[unsafe(method_id(outlineView:viewForTableColumn:item:))]
        fn outlineView_viewForTableColumn_item(
            &self,
            outline_view: &NSOutlineView,
            table_column: Option<&NSTableColumn>,
            item: &AnyObject,
        ) -> Option<Retained<NSView>> {
            let table_column = table_column.unwrap();
            self.tree()
                .item_at(index_iter_for_item(Some(item)))
                .map(|item| new_view(self, outline_view, table_column, item))
        }

        #[unsafe(method(outlineView:shouldSelectItem:))]
        fn outlineView_shouldSelectItem(
            &self,
            _outline_view: &NSOutlineView,
            item: &AnyObject,
        ) -> bool {
            self.tree()
                .item_at(index_iter_for_item(Some(item)))
                .map(|item| item.kind != Kind::GroupMarker)
                .unwrap_or(false)
        }
    }

    impl Navigator {
        // SAFETY: The signature is correct.
        #[unsafe(method(loadView))]
        fn load_view(&self) {
            self.load();
        }

        #[unsafe(method(viewWillLayout))]
        fn viewWillLayout(&self) {
            // Workaround: There isn't (that I've found) a way to size the
            // `NSTableColumn` correctly in `loadView` - so instead, we do it
            // later, once the view has had time to figure out which size it
            // actually needs to be.
            self.outline_view().sizeLastColumnToFit();
        }
    }
);

fn new_view(
    navigator: &Navigator,
    outline_view: &NSOutlineView,
    table_column: &NSTableColumn,
    item: NavigatorItem<'_>,
) -> Retained<NSView> {
    let mtm = outline_view.mtm();
    let cell = NSTableCellView::initWithFrame(
        mtm.alloc(),
        NSRect::new(
            NSPoint::ZERO,
            NSSize::new(table_column.width(), outline_view.rowHeight()),
        ),
    );

    let text_field = NSTextField::labelWithString(&NSString::from_str(item.name), mtm);
    if item.kind == Kind::GroupMarker {
        // TODO: Normal label colors are automatically dimmed when the
        // window loses focus, but secondary label colors aren't for some
        // reason? This makes the view look kinda odd, so we should fix
        // this by making a custom `NSTextField` implementation that
        // listens for `NSWindowDid[Become|Resign]MainNotification`.
        text_field.setTextColor(Some(&NSColor::secondaryLabelColor()));
        text_field.setFont(Some(&NSFont::boldSystemFontOfSize(0.0)));
    } else {
        text_field.setTextColor(Some(&NSColor::labelColor()));
    }

    enum Image {
        Named(&'static NSString, Retained<NSColor>),
        Custom(Retained<NSImage>, Retained<NSColor>),
        None,
    }

    let data = match item.kind {
        Kind::Root | Kind::GroupMarker | Kind::LangHeader => Image::None,
        Kind::Module => Image::Custom(navigator.image_module().clone(), NSColor::labelColor()),
        Kind::SampleCode => Image::Named(ns_string!("curlybraces"), NSColor::systemBlueColor()), // light blue
        Kind::Article => Image::Named(ns_string!("text.document"), NSColor::labelColor()),
        Kind::Overview => Image::Named(ns_string!("list.bullet"), NSColor::labelColor()),
        Kind::PropertyListKey => Image::Named(ns_string!("key.fill"), NSColor::systemBlueColor()), // light blue
        // TODO: Box it in instead
        Kind::Function => Image::Named(ns_string!("f.cursive"), NSColor::greenColor()),
        Kind::GlobalVariable => Image::Named(ns_string!("v.square.fill"), NSColor::greenColor()),
        Kind::Enum => Image::Named(ns_string!("e.square.fill"), NSColor::orangeColor()),
        Kind::EnumCase => Image::Named(ns_string!("e.square.fill"), NSColor::orangeColor()),
        Kind::Class => Image::Named(ns_string!("c.square.fill"), NSColor::purpleColor()),
        // TODO: [Pr]
        Kind::Protocol => Image::Named(ns_string!("p.square.fill"), NSColor::purpleColor()),
        Kind::Macro => Image::Named(ns_string!("number.square.fill"), NSColor::redColor()),
        Kind::Method | Kind::InitMethod | Kind::TypeMethod => {
            Image::Named(ns_string!("m.square.fill"), NSColor::blueColor())
        }
        Kind::Property => Image::Named(ns_string!("p.square.fill"), NSColor::systemBlueColor()), // light blue
        Kind::Typedef => Image::Named(ns_string!("t.square.fill"), NSColor::orangeColor()),
        Kind::Struct => Image::Named(ns_string!("s.square.fill"), NSColor::purpleColor()),
        Kind::Union => Image::Named(ns_string!("u.square.fill"), NSColor::purpleColor()),
        Kind::AssociatedType => Image::None, // TODO
        Kind::SwiftOp | Kind::SwiftSubscriptOp => Image::None, // TODO
        Kind::Const => Image::None,          // TODO
        Kind::Endpoint => Image::None,       // TODO
        Kind::Object => Image::None,         // TODO
        Kind::Namespace => Image::None,      // TODO
        kind => {
            eprintln!("unknown item kind: {kind:?}");
            Image::None
        }
    };

    let data = match data {
        // TODO: Add accessibility description? Can't that be gotten from
        // the image itself?
        Image::Named(symbol, color) => Some((
            NSImage::imageWithSystemSymbolName_accessibilityDescription(symbol, None).unwrap(),
            color,
        )),
        Image::Custom(image, color) => Some((image, color)),
        Image::None => None,
    };

    let image_view = if let Some((image, color)) = data {
        let image_view = NSImageView::imageViewWithImage(&image, mtm);
        image_view.setContentTintColor(Some(&color));
        image_view.setImageScaling(NSImageScaling::ScaleProportionallyUpOrDown);
        unsafe { cell.setImageView(Some(&image_view)) };
        cell.addSubview(&image_view);
        Some(image_view)
    } else {
        None
    };

    unsafe { cell.setTextField(Some(&text_field)) };
    cell.addSubview(&text_field);

    // Set up Auto Layout at the very end
    text_field.setLineBreakMode(NSLineBreakMode::ByTruncatingTail);
    text_field.setTranslatesAutoresizingMaskIntoConstraints(false);
    text_field
        .centerYAnchor()
        .constraintEqualToAnchor(&cell.centerYAnchor())
        .setActive(true);
    if let Some(image_view) = image_view {
        image_view.setTranslatesAutoresizingMaskIntoConstraints(false);
        image_view
            .topAnchor()
            .constraintEqualToAnchor_constant(&cell.topAnchor(), 4.0)
            .setActive(true);
        image_view
            .bottomAnchor()
            .constraintEqualToAnchor_constant(&cell.bottomAnchor(), -4.0)
            .setActive(true);
        image_view
            .widthAnchor()
            .constraintEqualToAnchor(&image_view.heightAnchor())
            .setActive(true);

        image_view
            .leadingAnchor()
            .constraintEqualToAnchor_constant(&cell.leadingAnchor(), 3.0)
            .setActive(true);
        text_field
            .leadingAnchor()
            .constraintEqualToAnchor_constant(&image_view.trailingAnchor(), 5.0)
            .setActive(true);
    } else {
        text_field
            .leadingAnchor()
            .constraintEqualToAnchor_constant(&cell.leadingAnchor(), 3.0)
            .setActive(true);
    }
    cell.trailingAnchor()
        .constraintGreaterThanOrEqualToAnchor_constant(&text_field.trailingAnchor(), 5.0)
        .setActive(true);

    cell.into_super()
}

#[derive(Clone, Debug)]
struct IndexPathIter<'a> {
    current_position: usize,
    index_path: &'a NSIndexPath,
}

impl Iterator for IndexPathIter<'_> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index_path.indexAtPosition(self.current_position);
        if index as isize == NSNotFound {
            return None;
        }
        self.current_position += 1;
        Some(index as u32)
    }
}

fn index_iter_for_item(item: Option<&AnyObject>) -> impl Iterator<Item = u32> + Clone + '_ {
    item.into_iter().flat_map(|item| {
        let index_path = item.downcast_ref::<NSIndexPath>().unwrap();
        IndexPathIter {
            current_position: 0,
            index_path,
        }
    })
}
