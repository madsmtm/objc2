//! Listen for low-memory warnings.
//!
//! There's four ways to listen for this signal:
//! - Using an app delegate callback.
//! - Using a view controller callback.
//! - Using a notification observer.
//! - Using a dispatch queue.
//!
//! See also Apple's documentation:
//! <https://developer.apple.com/documentation/uikit/responding-to-memory-warnings>
//!
//! This example showcases all four - you probably want to pick either the
//! notification observer or the dispatch queue, but it's useful to see what
//! would be required for the other two (in case you do need it, or need
//! access to some of the other functionality that is only exposed through
//! delegate / view controller callbacks).
//!
//!
//! ## Testing
//!
//! Run this example on the iOS simulator with:
//! ```console
//! $ cargo install cargo-apple-runner
//! $ CARGO_TARGET_AARCH64_APPLE_IOS_SIM_RUNNER=cargo-apple-runner cargo run --example memory_pressure
//! ```
//!
//! And press `Debug > Simulate Memory Warning` (Shift+CMD+M) to see the
//! event get triggered.
//!
//! I'm unsure how to trigger this outside the simulator, other than filling
//! up all the available memory with something.

use block2::RcBlock;
use dispatch2::{
    DispatchObject, DispatchRetained, DispatchSource, DispatchSourceMemoryPressureFlags,
    DispatchSourceType,
};

fn main() {
    // Register via notification observer.
    #[cfg(any(target_os = "ios", target_os = "tvos", target_os = "visionos"))]
    let _notification = uikit::notification_observer(|| println!("memory pressure notification"));

    // Register via libdispatch memory pressure sources.
    //
    // We could combine the flags here, but we register each separately to
    // better see each event individually.
    let _dispatch_normal = dispatch_source(DispatchSourceMemoryPressureFlags::Normal, || {
        println!("dispatch pressure normal")
    });
    let _dispatch_warn = dispatch_source(DispatchSourceMemoryPressureFlags::Warn, || {
        println!("dispatch pressure warning")
    });
    let _dispatch_critical = dispatch_source(DispatchSourceMemoryPressureFlags::Critical, || {
        println!("dispatch pressure critical")
    });

    // Run the runloop. This is necessary for the event to fire.
    //
    // See docs/run_loop.md for details.
    #[cfg(any(target_os = "ios", target_os = "tvos", target_os = "visionos"))]
    {
        use objc2::{ClassType, MainThreadMarker};
        use objc2_foundation::NSString;

        let mtm = MainThreadMarker::new().unwrap();
        let delegate_class = NSString::from_class(uikit::AppDelegate::class());
        objc2_ui_kit::UIApplication::main(None, Some(&delegate_class), mtm);
    }
    #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "visionos")))]
    {
        // SAFETY: We don't rely on queued blocks executing on the main thread.
        unsafe { dispatch2::dispatch_main() };
    }
}

fn dispatch_source(
    flags: DispatchSourceMemoryPressureFlags,
    callback: impl Fn() + Send + Sync + 'static,
) -> DispatchRetained<DispatchSource> {
    // SAFETY: The callback is thread-safe, so the queue this runs on doesn't
    // matter.
    let source = unsafe {
        DispatchSource::new(
            DispatchSourceType::memory_pressure(),
            0,
            flags.0 as usize,
            None,
        )
    };

    // SAFETY: Prooobably fine, the callback is Send + Sync + 'static.
    unsafe { source.set_event_handler_with_block(Some(&RcBlock::new(callback))) };

    source.activate();

    source
}

#[cfg(any(target_os = "ios", target_os = "tvos", target_os = "visionos"))]
mod uikit {
    use std::cell::RefCell;

    use block2::RcBlock;
    use objc2::rc::{Allocated, Retained};
    use objc2::runtime::{NSObject, ProtocolObject};
    use objc2::{
        define_class, msg_send, ClassType, Ivars, MainThreadMarker, MainThreadOnly, Message,
    };
    use objc2_foundation::{ns_string, NSNotificationCenter, NSObjectProtocol, NSOperationQueue};
    use objc2_ui_kit::{
        UIApplication, UIApplicationDelegate, UIApplicationDidReceiveMemoryWarningNotification,
        UIScene, UISceneConfiguration, UISceneConnectionOptions, UISceneDelegate, UISceneSession,
        UIViewController, UIWindow, UIWindowScene, UIWindowSceneDelegate,
    };

    define_class!(
        #[unsafe(super(NSObject))]
        #[thread_kind = MainThreadOnly]
        #[derive(Debug)]
        pub(crate) struct AppDelegate;

        /// Dummy doc comment to get rustfmt to work.
        impl AppDelegate {
            // Called by `UIApplication::main`.
            #[unsafe(method(init))]
            fn init(this: Allocated<Self>) -> Retained<Self> {
                let this = this.set_ivars(Ivars::<Self> {});
                unsafe { msg_send![super(this), init] }
            }
        }

        unsafe impl NSObjectProtocol for AppDelegate {}

        unsafe impl UIApplicationDelegate for AppDelegate {
            #[unsafe(method(application:configurationForConnectingSceneSession:options:))]
            fn application_configuration_for_connecting_scene_session_options(
                &self,
                _application: &UIApplication,
                connecting_scene_session: &UISceneSession,
                _options: &UISceneConnectionOptions,
            ) -> Retained<UISceneConfiguration> {
                let config = UISceneConfiguration::initWithName_sessionRole(
                    UISceneConfiguration::alloc(self.mtm()),
                    Some(ns_string!("Default")),
                    &connecting_scene_session.role(),
                );
                unsafe { config.setDelegateClass(Some(SceneDelegate::class())) };
                config
            }

            #[unsafe(method(applicationDidReceiveMemoryWarning:))]
            fn did_receive_memory_warning(&self, _app: &UIApplication) {
                println!("application memory warning");
            }
        }
    );

    // Create a UISceneDelegate object to comply with the scene-based
    // application lifecycle. Yes, this is very verbose.
    define_class!(
        #[unsafe(super(NSObject))]
        #[thread_kind = MainThreadOnly]
        #[derive(Debug)]
        pub(crate) struct SceneDelegate {
            window_: RefCell<Option<Retained<UIWindow>>>,
        }

        /// Dummy doc comment to get rustfmt to work.
        impl SceneDelegate {
            // Called to initialize the scene.
            #[unsafe(method(init))]
            fn init(this: Allocated<Self>) -> Retained<Self> {
                let this = this.set_ivars(Ivars::<Self> {
                    window_: RefCell::new(None),
                });
                unsafe { msg_send![super(this), init] }
            }
        }

        unsafe impl NSObjectProtocol for SceneDelegate {}

        unsafe impl UISceneDelegate for SceneDelegate {
            #[unsafe(method(scene:willConnectToSession:options:))]
            fn scene_will_connect_to_session_options(
                &self,
                scene: &UIScene,
                _session: &UISceneSession,
                _connection_options: &UISceneConnectionOptions,
            ) {
                let window_scene = scene.downcast_ref::<UIWindowScene>().unwrap();
                let window =
                    UIWindow::initWithWindowScene(UIWindow::alloc(self.mtm()), window_scene);
                window.setRootViewController(Some(&ViewController::new(self.mtm())));
                window.makeKeyAndVisible();
                *self.window_().borrow_mut() = Some(window);
            }
        }

        unsafe impl UIWindowSceneDelegate for SceneDelegate {
            #[unsafe(method(window))]
            fn window(&self) -> Option<Retained<UIWindow>> {
                self.window_().borrow().clone()
            }

            #[unsafe(method(setWindow:))]
            fn set_window(&self, window: Option<&UIWindow>) {
                *self.window_().borrow_mut() = window.map(|w| w.retain());
            }
        }
    );

    define_class!(
        #[unsafe(super(UIViewController))]
        #[derive(Debug)]
        struct ViewController;

        unsafe impl NSObjectProtocol for ViewController {}

        impl ViewController {
            #[unsafe(method(didReceiveMemoryWarning))]
            fn did_receive_memory_warning(&self) {
                println!("view controller memory warning");
            }
        }
    );

    impl ViewController {
        // Called by `UIApplication::main`.
        fn new(mtm: MainThreadMarker) -> Retained<Self> {
            let this = Self::alloc(mtm).set_ivars(Ivars::<Self> {});
            unsafe { msg_send![super(this), init] }
        }
    }

    /// Register a notification observer.
    ///
    /// The returned object will remove the observer on `Drop`.
    pub(crate) fn notification_observer(
        callback: impl Fn() + Send + Sync + 'static,
    ) -> Retained<ProtocolObject<dyn NSObjectProtocol>> {
        let center = NSNotificationCenter::defaultCenter();

        // SAFETY: `callback` is `Send + Sync`, so we can safely pass it to the main thread.
        unsafe {
            center.addObserverForName_object_queue_usingBlock(
                Some(UIApplicationDidReceiveMemoryWarningNotification),
                // No sender filter
                None,
                // Run on the main thread.
                Some(&NSOperationQueue::mainQueue()),
                &RcBlock::new(move |_| callback()),
            )
        }
    }
}
