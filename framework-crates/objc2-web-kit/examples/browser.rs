#![deny(unsafe_op_in_unsafe_fn)]
#![allow(clippy::incompatible_msrv)]
use core::cell::OnceCell;

use objc2::{
    declare_class, msg_send_id,
    mutability::MainThreadOnly,
    rc::Retained,
    runtime::{AnyObject, ProtocolObject, Sel},
    sel, ClassType, DeclaredClass,
};
#[allow(deprecated)]
use objc2_app_kit::{
    NSApplication, NSApplicationActivationPolicy, NSApplicationDelegate, NSBackingStoreType,
    NSBezelStyle, NSButton, NSColor, NSControl, NSControlTextEditingDelegate, NSLayoutAttribute,
    NSMenu, NSMenuItem, NSStackView, NSStackViewDistribution, NSTextField, NSTextFieldDelegate,
    NSTextView, NSUserInterfaceLayoutOrientation, NSWindow, NSWindowStyleMask,
};
use objc2_foundation::{
    ns_string, MainThreadMarker, NSNotification, NSObject, NSObjectProtocol, NSPoint, NSRect,
    NSSize, NSURLRequest, NSURL,
};
use objc2_web_kit::{WKNavigation, WKNavigationDelegate, WKWebView};

macro_rules! idcell {
    ($name:ident => $this:expr) => {
        $this.ivars().$name.set($name).expect(&format!(
            "ivar should not already be initialized: `{}`",
            stringify!($name)
        ));
    };
    ($name:ident <= $this:expr) => {
        #[rustfmt::skip]
        let Some($name) = $this.ivars().$name.get() else {
            unreachable!(
                "ivar should be initialized: `{}`",
                stringify!($name)
            )
        };
    };
}

#[derive(Default)]
struct Ivars {
    nav_url: OnceCell<Retained<NSTextField>>,
    web_view: OnceCell<Retained<WKWebView>>,
    window: OnceCell<Retained<NSWindow>>,
}

declare_class!(
    struct Delegate;

    // SAFETY:
    // - The superclass NSObject does not have any subclassing requirements.
    // - Main thread only mutability is correct, since this is an application delegate.
    // - `Delegate` does not implement `Drop`.
    unsafe impl ClassType for Delegate {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
        const NAME: &'static str = "Delegate";
    }

    impl DeclaredClass for Delegate {
        type Ivars = Ivars;
    }

    unsafe impl NSObjectProtocol for Delegate {}

    unsafe impl NSApplicationDelegate for Delegate {
        #[method(applicationDidFinishLaunching:)]
        #[allow(non_snake_case)]
        unsafe fn applicationDidFinishLaunching(&self, _notification: &NSNotification) {
            let mtm = MainThreadMarker::from(self);
            // create the app window
            let window = {
                let content_rect = NSRect::new(NSPoint::new(0., 0.), NSSize::new(1024., 768.));
                let style = NSWindowStyleMask::Closable
                    | NSWindowStyleMask::Resizable
                    | NSWindowStyleMask::Titled;
                let backing_store_type = NSBackingStoreType::NSBackingStoreBuffered;
                let flag = false;
                unsafe {
                    NSWindow::initWithContentRect_styleMask_backing_defer(
                        mtm.alloc(),
                        content_rect,
                        style,
                        backing_store_type,
                        flag,
                    )
                }
            };

            // create the web view
            let web_view = {
                let frame_rect = NSRect::ZERO;
                unsafe { WKWebView::initWithFrame(mtm.alloc(), frame_rect) }
            };

            // create the nav bar view
            let nav_bar = {
                let frame_rect = NSRect::ZERO;
                let this = unsafe { NSStackView::initWithFrame(mtm.alloc(), frame_rect) };
                unsafe {
                    this.setOrientation(NSUserInterfaceLayoutOrientation::Horizontal);
                    this.setAlignment(NSLayoutAttribute::Height);
                    this.setDistribution(NSStackViewDistribution::Fill);
                    this.setSpacing(0.);
                }
                this
            };

            // create the nav buttons view
            let nav_buttons = {
                let frame_rect = NSRect::ZERO;
                let this = unsafe { NSStackView::initWithFrame(mtm.alloc(), frame_rect) };
                unsafe {
                    this.setOrientation(NSUserInterfaceLayoutOrientation::Horizontal);
                    this.setAlignment(NSLayoutAttribute::Height);
                    this.setDistribution(NSStackViewDistribution::FillEqually);
                    this.setSpacing(0.);
                }
                this
            };

            // create the back button
            let back_button = {
                // configure the button to navigate the webview backward
                let title = ns_string!("back");
                let target = Some::<&AnyObject>(&web_view);
                let action = Some(sel!(goBack));
                let this =
                    unsafe { NSButton::buttonWithTitle_target_action(title, target, action, mtm) };
                #[allow(deprecated)]
                unsafe {
                    this.setBezelStyle(NSBezelStyle::ShadowlessSquare)
                };
                this
            };

            // create the forward button
            let forward_button = {
                // configure the button to navigate the web view forward
                let title = ns_string!("forward");
                let target = Some::<&AnyObject>(&web_view);
                let action = Some(sel!(goForward));
                let this =
                    unsafe { NSButton::buttonWithTitle_target_action(title, target, action, mtm) };
                #[allow(deprecated)]
                unsafe {
                    this.setBezelStyle(NSBezelStyle::ShadowlessSquare)
                };
                this
            };

            unsafe {
                nav_buttons.addArrangedSubview(&back_button);
                nav_buttons.addArrangedSubview(&forward_button);
            }

            // create the url text field
            let nav_url = {
                let frame_rect = NSRect::ZERO;
                let this = unsafe { NSTextField::initWithFrame(mtm.alloc(), frame_rect) };
                unsafe {
                    this.setDrawsBackground(true);
                    this.setBackgroundColor(Some(&NSColor::lightGrayColor()));
                    this.setTextColor(Some(&NSColor::blackColor()));
                }
                this
            };

            unsafe {
                nav_bar.addArrangedSubview(&nav_buttons);
                nav_bar.addArrangedSubview(&nav_url);
            }

            // create the window content view
            let content_view = {
                let frame_rect = window.frame();
                let this = unsafe { NSStackView::initWithFrame(mtm.alloc(), frame_rect) };
                unsafe {
                    this.setOrientation(NSUserInterfaceLayoutOrientation::Vertical);
                    this.setAlignment(NSLayoutAttribute::Width);
                    this.setDistribution(NSStackViewDistribution::Fill);
                    this.setSpacing(0.);
                }
                this
            };

            unsafe {
                content_view.addArrangedSubview(&nav_bar);
                content_view.addArrangedSubview(&web_view);
            }

            unsafe {
                // handle input from text field (on <ENTER>, load URL from text field in web view)
                let object = ProtocolObject::from_ref(self);
                nav_url.setDelegate(Some(object));

                // handle nav events from web view (on finished navigating, update text area with current URL)
                let object = ProtocolObject::from_ref(self);
                web_view.setNavigationDelegate(Some(object));
            }

            // create the menu with a "quit" entry
            unsafe {
                let menu = NSMenu::initWithTitle(mtm.alloc(), ns_string!(""));
                let menu_app_item = NSMenuItem::initWithTitle_action_keyEquivalent(
                    mtm.alloc(),
                    ns_string!(""),
                    None,
                    ns_string!(""),
                );
                let menu_app_menu = NSMenu::initWithTitle(mtm.alloc(), ns_string!(""));
                menu_app_menu.addItemWithTitle_action_keyEquivalent(
                    ns_string!("Quit"),
                    Some(sel!(terminate:)),
                    ns_string!("q"),
                );
                menu_app_item.setSubmenu(Some(&menu_app_menu));
                menu.addItem(&menu_app_item);

                let app = NSApplication::sharedApplication(mtm);
                app.setMainMenu(Some(&menu));
            }

            // configure the window
            window.setContentView(Some(&content_view));
            window.center();
            window.setTitle(ns_string!("browser example"));
            window.makeKeyAndOrderFront(None);

            // request the web view navigate to a page
            unsafe {
                let request = {
                    let url_string = ns_string!("https://google.com");
                    let url = NSURL::URLWithString(url_string).expect("URL should parse");
                    NSURLRequest::requestWithURL(&url)
                };
                web_view.loadRequest(&request);
            }

            idcell!(nav_url => self);
            idcell!(web_view => self);
            idcell!(window => self);
        }
    }

    unsafe impl NSControlTextEditingDelegate for Delegate {
        #[method(control:textView:doCommandBySelector:)]
        #[allow(non_snake_case)]
        unsafe fn control_textView_doCommandBySelector(
            &self,
            _control: &NSControl,
            text_view: &NSTextView,
            command_selector: Sel,
        ) -> bool {
            idcell!(web_view <= self);
            if command_selector == sel!(insertNewline:) {
                if let Some(url) = unsafe { NSURL::URLWithString(&text_view.string()) } {
                    unsafe { web_view.loadRequest(&NSURLRequest::requestWithURL(&url)) };
                    return true.into();
                }
            }
            false
        }
    }

    unsafe impl NSTextFieldDelegate for Delegate {}

    unsafe impl WKNavigationDelegate for Delegate {
        #[method(webView:didFinishNavigation:)]
        #[allow(non_snake_case)]
        unsafe fn webView_didFinishNavigation(
            &self,
            web_view: &WKWebView,
            _navigation: Option<&WKNavigation>,
        ) {
            idcell!(nav_url <= self);
            unsafe {
                if let Some(url) = web_view.URL().and_then(|url| url.absoluteString()) {
                    nav_url.setStringValue(&url);
                }
            }
        }
    }
);

impl Delegate {
    fn new(mtm: MainThreadMarker) -> Retained<Self> {
        let this = mtm.alloc();
        let this = this.set_ivars(Ivars::default());
        unsafe { msg_send_id![super(this), init] }
    }
}

fn main() {
    let mtm = MainThreadMarker::new().unwrap();
    let app = NSApplication::sharedApplication(mtm);
    app.setActivationPolicy(NSApplicationActivationPolicy::Regular);

    // configure the application delegate
    let delegate = Delegate::new(mtm);
    let object = ProtocolObject::from_ref(&*delegate);
    app.setDelegate(Some(object));

    // run the app
    unsafe { app.run() };
}
