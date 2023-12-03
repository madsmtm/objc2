#![deny(unsafe_op_in_unsafe_fn)]
use core::cell::OnceCell;

#[allow(deprecated)]
use icrate::{
    AppKit::{
        NSApplication, NSApplicationActivationPolicyRegular, NSApplicationDelegate,
        NSBackingStoreBuffered, NSBezelStyleShadowlessSquare, NSButton, NSColor, NSControl,
        NSControlTextEditingDelegate, NSLayoutAttributeHeight, NSLayoutAttributeWidth, NSMenu,
        NSMenuItem, NSStackView, NSStackViewDistributionFill, NSStackViewDistributionFillEqually,
        NSTextField, NSTextFieldDelegate, NSTextView, NSUserInterfaceLayoutOrientationHorizontal,
        NSUserInterfaceLayoutOrientationVertical, NSWindow, NSWindowStyleMaskClosable,
        NSWindowStyleMaskResizable, NSWindowStyleMaskTitled,
    },
    Foundation::{
        ns_string, MainThreadMarker, NSNotification, NSObject, NSObjectProtocol, NSPoint, NSRect,
        NSSize, NSURLRequest, NSURL,
    },
    WebKit::{WKNavigation, WKNavigationDelegate, WKWebView},
};
use objc2::{
    declare_class, msg_send_id,
    mutability::MainThreadOnly,
    rc::Id,
    runtime::{AnyObject, ProtocolObject, Sel},
    sel, ClassType, DeclaredClass,
};

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
    nav_url: OnceCell<Id<NSTextField>>,
    web_view: OnceCell<Id<WKWebView>>,
    window: OnceCell<Id<NSWindow>>,
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
                let style = NSWindowStyleMaskClosable
                    | NSWindowStyleMaskResizable
                    | NSWindowStyleMaskTitled;
                let backing_store_type = NSBackingStoreBuffered;
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
                    this.setOrientation(NSUserInterfaceLayoutOrientationHorizontal);
                    this.setAlignment(NSLayoutAttributeHeight);
                    this.setDistribution(NSStackViewDistributionFill);
                    this.setSpacing(0.);
                }
                this
            };

            // create the nav buttons view
            let nav_buttons = {
                let frame_rect = NSRect::ZERO;
                let this = unsafe { NSStackView::initWithFrame(mtm.alloc(), frame_rect) };
                unsafe {
                    this.setOrientation(NSUserInterfaceLayoutOrientationHorizontal);
                    this.setAlignment(NSLayoutAttributeHeight);
                    this.setDistribution(NSStackViewDistributionFillEqually);
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
                    this.setBezelStyle(NSBezelStyleShadowlessSquare)
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
                    this.setBezelStyle(NSBezelStyleShadowlessSquare)
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
                    this.setOrientation(NSUserInterfaceLayoutOrientationVertical);
                    this.setAlignment(NSLayoutAttributeWidth);
                    this.setDistribution(NSStackViewDistributionFill);
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
    pub fn new(mtm: MainThreadMarker) -> Id<Self> {
        let this = mtm.alloc();
        let this = this.set_ivars(Ivars::default());
        unsafe { msg_send_id![super(this), init] }
    }
}

fn main() {
    let mtm = MainThreadMarker::new().unwrap();
    let app = NSApplication::sharedApplication(mtm);
    app.setActivationPolicy(NSApplicationActivationPolicyRegular);

    // configure the application delegate
    let delegate = Delegate::new(mtm);
    let object = ProtocolObject::from_ref(&*delegate);
    app.setDelegate(Some(object));

    // run the app
    unsafe { app.run() };
}
