#![deny(unsafe_op_in_unsafe_fn)]

use icrate::{
    ns_string,
    AppKit::{
        NSApplication, NSApplicationActivationPolicyRegular, NSBackingStoreBuffered,
        NSBezelStyleShadowlessSquare, NSButton, NSColor, NSControl, NSControlTextEditingDelegate,
        NSLayoutAttributeHeight, NSLayoutAttributeWidth, NSMenu, NSMenuItem, NSStackView,
        NSStackViewDistributionFill, NSStackViewDistributionFillEqually, NSTextField,
        NSTextFieldDelegate, NSTextView, NSUserInterfaceLayoutOrientationHorizontal,
        NSUserInterfaceLayoutOrientationVertical, NSWindow, NSWindowStyleMaskClosable,
        NSWindowStyleMaskResizable, NSWindowStyleMaskTitled,
    },
    Foundation::{NSObject, NSObjectProtocol, NSPoint, NSRect, NSSize, NSURLRequest, NSURL},
    WebKit::{WKNavigation, WKNavigationDelegate, WKWebView},
};
use objc2::{
    declare::{Ivar, IvarDrop},
    declare_class, extern_methods, msg_send,
    mutability::InteriorMutable,
    rc::{Allocated, Id},
    runtime::{AnyObject, ProtocolObject, Sel},
    sel, ClassType,
};

declare_class!(
    struct Delegate {
        text_field: IvarDrop<Id<NSTextField>, "_text_field">,
        web_view: IvarDrop<Id<WKWebView>, "_web_view">,
    }
    mod ivars;

    unsafe impl ClassType for Delegate {
        type Super = NSObject;
        type Mutability = InteriorMutable;
        const NAME: &'static str = "Delegate";
    }

    unsafe impl Delegate {
        #[method(initWithTextField:andWebView:)]
        #[allow(non_snake_case)]
        unsafe fn __init_withTextField_andWebView(
            self: &mut Self,
            text_field: &NSTextField,
            web_view: &WKWebView,
        ) -> Option<&mut Self> {
            let this: Option<&mut Self> = msg_send![super(self), init];
            let this = this?;
            Ivar::write(&mut this.text_field, text_field.retain());
            Ivar::write(&mut this.web_view, web_view.retain());
            Some(this)
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
            if command_selector == sel!(insertNewline:) {
                if let Some(url) = unsafe { NSURL::URLWithString(&text_view.string()) } {
                    unsafe {
                        self.web_view
                            .loadRequest(&NSURLRequest::requestWithURL(&url))
                    };
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
            unsafe {
                if let Some(url) = web_view.URL().and_then(|url| url.absoluteString()) {
                    self.text_field.setStringValue(&url);
                }
            }
        }
    }
);

extern_methods!(
    unsafe impl Delegate {
        #[method_id(initWithTextField:andWebView:)]
        #[allow(non_snake_case)]
        pub fn initWithTextField_andWebView(
            this: Option<Allocated<Self>>,
            text_field: &NSTextField,
            web_view: &WKWebView,
        ) -> Id<Self>;
    }
);

unsafe impl NSObjectProtocol for Delegate {}

fn main() {
    let app = unsafe { NSApplication::sharedApplication() };
    unsafe { app.setActivationPolicy(NSApplicationActivationPolicyRegular) };

    // create the app window
    let window = {
        let this = NSWindow::alloc();
        let content_rect = NSRect::new(NSPoint::new(0., 0.), NSSize::new(1024., 768.));
        let style =
            NSWindowStyleMaskClosable | NSWindowStyleMaskResizable | NSWindowStyleMaskTitled;
        let backing_store_type = NSBackingStoreBuffered;
        let flag = false;
        unsafe {
            NSWindow::initWithContentRect_styleMask_backing_defer(
                this,
                content_rect,
                style,
                backing_store_type,
                flag,
            )
        }
    };

    // create the web view
    let web_view = {
        let this = WKWebView::alloc();
        let frame_rect = NSRect::ZERO;
        unsafe { WKWebView::initWithFrame(this, frame_rect) }
    };

    // create the nav bar view
    let nav_bar = {
        let frame_rect = NSRect::ZERO;
        let this = NSStackView::alloc();
        let this = unsafe { NSStackView::initWithFrame(this, frame_rect) };
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
        let this = NSStackView::alloc();
        let this = unsafe { NSStackView::initWithFrame(this, frame_rect) };
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
        let this = unsafe { NSButton::buttonWithTitle_target_action(title, target, action) };
        unsafe { this.setBezelStyle(NSBezelStyleShadowlessSquare) };
        this
    };

    // create the forward button
    let forward_button = {
        // configure the button to navigate the web view forward
        let title = ns_string!("forward");
        let target = Some::<&AnyObject>(&web_view);
        let action = Some(sel!(goForward));
        let this = unsafe { NSButton::buttonWithTitle_target_action(title, target, action) };
        unsafe { this.setBezelStyle(NSBezelStyleShadowlessSquare) };
        this
    };

    unsafe {
        nav_buttons.addArrangedSubview(&back_button);
        nav_buttons.addArrangedSubview(&forward_button);
    }

    // create the url text field
    let nav_url = {
        let frame_rect = NSRect::ZERO;
        let this = NSTextField::alloc();
        let this = unsafe { NSTextField::initWithFrame(this, frame_rect) };
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
        let frame_rect = unsafe { window.frame() };
        let this = NSStackView::alloc();
        let this = unsafe { NSStackView::initWithFrame(this, frame_rect) };
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

    // create the app delegate
    let delegate = {
        let this = Delegate::alloc();
        Delegate::initWithTextField_andWebView(this, &nav_url, &web_view)
    };

    unsafe {
        // handle input from text field (on <ENTER>, load URL from text field in web view)
        let object = ProtocolObject::from_ref(&*delegate);
        nav_url.setDelegate(Some(object));

        // handle nav events from web view (on finished navigating, update text area with current URL)
        let object = ProtocolObject::from_ref(&*delegate);
        web_view.setNavigationDelegate(Some(object));
    }

    // create the menu with a "quit" entry
    unsafe {
        let menu = NSMenu::initWithTitle(NSMenu::alloc(), ns_string!(""));
        let menu_app_item = NSMenuItem::initWithTitle_action_keyEquivalent(
            NSMenuItem::alloc(),
            ns_string!(""),
            None,
            ns_string!(""),
        );
        let menu_app_menu = NSMenu::initWithTitle(NSMenu::alloc(), ns_string!(""));
        menu_app_menu.addItemWithTitle_action_keyEquivalent(
            ns_string!("Quit"),
            Some(sel!(terminate:)),
            ns_string!("q"),
        );
        menu_app_item.setSubmenu(Some(&menu_app_menu));
        menu.addItem(&menu_app_item);
        app.setMainMenu(Some(&menu));
    }

    // configure the window
    unsafe {
        window.setContentView(Some(&content_view));
        window.center();
        window.setTitle(ns_string!("browser example"));
        window.makeKeyAndOrderFront(None);
    }

    // request the web view navigate to a page
    unsafe {
        let request = {
            let url_string = ns_string!("https://google.com");
            let url = NSURL::URLWithString(url_string).expect("URL should parse");
            NSURLRequest::requestWithURL(&url)
        };
        web_view.loadRequest(&request);
    }

    // run the app
    unsafe { app.run() };
}
