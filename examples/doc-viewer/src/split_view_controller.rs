use objc2::rc::Retained;
use objc2::{define_class, msg_send, Ivars, MainThreadMarker, MainThreadOnly};
use objc2_app_kit::{
    NSLayoutPriorityDefaultLow, NSResponder, NSSplitViewController, NSSplitViewItem,
    NSViewController,
};
use objc2_foundation::NSObject;

use crate::detail_view_controller::DetailViewController;
use crate::navigator::Navigator;

define_class!(
    // SAFETY:
    // - We correctly override `NSSplitViewController` methods.
    // - `SplitViewController` does not implement `Drop`.
    #[unsafe(super(NSSplitViewController, NSViewController, NSResponder, NSObject))]
    pub struct SplitViewController;

    impl SplitViewController {
        // SAFETY: The signature is correct.
        #[unsafe(method(viewDidLoad))]
        fn view_did_load(&self) {
            let _: () = unsafe { msg_send![super(self), viewDidLoad] };

            // TODO: Enable once we've figured out the layout.
            // self.splitView().setAutosaveName(Some(ns_string!("SplitView")));
        }
    }
);

impl SplitViewController {
    pub fn new(mtm: MainThreadMarker) -> Retained<Self> {
        let this = Self::alloc(mtm);

        let this = this.set_ivars(Ivars::<Self> {});

        // SAFETY: `SplitViewController` is safe to initialize.
        let this: Retained<Self> = unsafe { msg_send![super(this), init] };

        let detail_view_controller = DetailViewController::new(mtm);
        let navigator = Navigator::new(mtm, &detail_view_controller);

        let outline = NSSplitViewItem::sidebarWithViewController(&navigator);
        this.addSplitViewItem(&outline);

        let detail = NSSplitViewItem::splitViewItemWithViewController(&detail_view_controller);
        this.addSplitViewItem(&detail);

        // Magical values set by default in Interface Builder.
        outline.setHoldingPriority(NSLayoutPriorityDefaultLow + 10.0);
        outline.setMinimumThickness(140.0);
        detail.setHoldingPriority(NSLayoutPriorityDefaultLow - 51.0);

        this
    }

    #[allow(dead_code)]
    pub fn outline(&self) -> Retained<Navigator> {
        self.splitViewItems().objectAtIndex(0).downcast().unwrap()
    }

    #[allow(dead_code)]
    pub fn detail(&self) -> Retained<DetailViewController> {
        self.splitViewItems().objectAtIndex(1).downcast().unwrap()
    }
}
