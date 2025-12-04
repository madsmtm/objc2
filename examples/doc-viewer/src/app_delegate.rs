use std::cell::OnceCell;

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{define_class, msg_send, sel, Ivars, MainThreadMarker, MainThreadOnly};
use objc2_app_kit::{
    NSApplication, NSApplicationActivationPolicy, NSApplicationDelegate, NSMenu, NSMenuItem,
    NSWindow, NSWindowController, NSWindowStyleMask,
};
use objc2_foundation::{ns_string, NSNotification, NSObject, NSObjectProtocol};

use crate::split_view_controller::SplitViewController;

define_class!(
    // SAFETY:
    // - The superclass NSObject does not have any subclassing requirements.
    // - `AppDelegate` does not implement `Drop`.
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    pub struct AppDelegate {
        window: OnceCell<Retained<NSWindow>>,
        window_controller: OnceCell<Retained<NSWindowController>>,
    }

    // SAFETY: No problematic methods on `NSObjectProtocol` are implemented.
    unsafe impl NSObjectProtocol for AppDelegate {}

    // SAFETY: `NSApplicationDelegate` has no safety requirements.
    unsafe impl NSApplicationDelegate for AppDelegate {
        // SAFETY: The signature is correct.
        #[unsafe(method(applicationDidFinishLaunching:))]
        fn did_finish_launching(&self, notification: &NSNotification) {
            let mtm = self.mtm();
            let app = notification
                .object()
                .unwrap()
                .downcast::<NSApplication>()
                .unwrap();

            add_menubar(&app);

            let view_controller = SplitViewController::new(mtm);

            let window = NSWindow::windowWithContentViewController(&view_controller);

            // The view controller is a split view, and we'd like it to take
            // up the entire left side of the window.
            window.setStyleMask(window.styleMask() | NSWindowStyleMask::FullSizeContentView);

            let window_controller =
                NSWindowController::initWithWindow(NSWindowController::alloc(mtm), Some(&window));

            unsafe { window_controller.showWindow(None) };
            // TODO: Enable once we've figured out the layout.
            // window_controller.setWindowFrameAutosaveName(ns_string!("MainWindow"));

            // Since we're compiling with Cargo, and not bundling the binary
            // into an `.app`, we need to change the activation policy and
            // activate the application such that our window will appear.
            app.setActivationPolicy(NSApplicationActivationPolicy::Regular);
            #[allow(deprecated)]
            app.activateIgnoringOtherApps(false);

            // Store for later use.
            self.window().set(window).unwrap();
            self.window_controller().set(window_controller).unwrap();
        }

        // SAFETY: The signature is correct.
        #[unsafe(method(applicationWillTerminate:))]
        fn will_terminate(&self, _notification: &NSNotification) {}
    }
);

impl AppDelegate {
    // FIXME: Make it possible to avoid this boilerplate.
    fn new(mtm: MainThreadMarker) -> Retained<Self> {
        let this = Self::alloc(mtm);
        let this = this.set_ivars(Ivars::<Self> {
            window: Default::default(),
            window_controller: Default::default(),
        });
        // SAFETY: `AppDelegate` is safe to initialize.
        unsafe { msg_send![super(this), init] }
    }
}

pub fn set_application_delegate(app: &NSApplication) {
    let delegate = AppDelegate::new(app.mtm());
    let object = ProtocolObject::from_ref(&*delegate);
    app.setDelegate(Some(object));
}

/// Create a minimal menubar with a "Quit" entry.
fn add_menubar(app: &NSApplication) {
    let mtm = app.mtm();

    let menu = NSMenu::initWithTitle(NSMenu::alloc(mtm), ns_string!(""));
    let menu_app_item = unsafe {
        NSMenuItem::initWithTitle_action_keyEquivalent(
            NSMenuItem::alloc(mtm),
            ns_string!(""),
            None,
            ns_string!(""),
        )
    };
    let menu_app_menu = NSMenu::initWithTitle(NSMenu::alloc(mtm), ns_string!(""));
    unsafe {
        menu_app_menu.addItemWithTitle_action_keyEquivalent(
            ns_string!("Quit"),
            Some(sel!(terminate:)),
            ns_string!("q"),
        )
    };
    menu_app_item.setSubmenu(Some(&menu_app_menu));
    menu.addItem(&menu_app_item);

    app.setMainMenu(Some(&menu));
}
