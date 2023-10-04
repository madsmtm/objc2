#![deny(unsafe_op_in_unsafe_fn)]
use core::cell::RefCell;
use core::ffi::c_void;
use core::ptr::NonNull;

use icrate::AppKit::{
    NSApplication, NSApplicationActivationPolicyRegular, NSApplicationDelegate,
    NSBackingStoreBuffered, NSButton, NSColor, NSControl, NSLayoutAttributeCenterX,
    NSLayoutConstraint, NSSlider, NSStackView, NSTextField,
    NSUserInterfaceLayoutOrientationVertical, NSView, NSViewController, NSWindow,
    NSWindowController, NSWindowStyleMaskClosable, NSWindowStyleMaskMiniaturizable,
    NSWindowStyleMaskResizable, NSWindowStyleMaskTitled,
};
use icrate::Foundation::{
    ns_string, CGFloat, MainThreadMarker, NSArray, NSDictionary, NSNotification, NSNumber,
    NSNumberFormatter, NSNumberFormatterNoStyle, NSObject, NSObjectProtocol, NSPoint, NSRect,
    NSSize,
};
use objc2::declare::{Ivar, IvarDrop};
use objc2::encode::{Encode, Encoding};
use objc2::rc::{Allocated, Id, WeakId};
use objc2::runtime::ProtocolObject;
use objc2::{declare_class, extern_methods, msg_send, msg_send_id, mutability, sel, ClassType};

declare_class!(
    pub struct ViewController {
        app_delegate: IvarDrop<Box<WeakId<AppDelegate>>, "_app_delegate">,
        text_field: IvarDrop<Id<NSTextField>, "_text_field">,
        slider: IvarDrop<Id<NSSlider>, "_slider">,
    }

    mod controller_ivars;

    unsafe impl ClassType for ViewController {
        type Super = NSViewController;
        type Mutability = mutability::MainThreadOnly;
        const NAME: &'static str = "ViewController";
    }

    unsafe impl ViewController {
        #[method(initWithAppDelegate:)]
        unsafe fn __init(this: *mut Self, app_delegate: &AppDelegate) -> Option<NonNull<Self>> {
            let this: Option<&mut Self> = unsafe { msg_send![super(this), init] };

            this.map(|this| {
                Ivar::write(&mut this.app_delegate, Box::new(WeakId::from(app_delegate)));
                Ivar::write(&mut this.text_field, unsafe {
                    let view = NSTextField::new();
                    view.setFormatter(Some(
                        {
                            let formatter = NSNumberFormatter::new();
                            formatter.setNumberStyle(NSNumberFormatterNoStyle);
                            formatter.setMinimum(Some(&NSNumber::new_f32(0.0)));
                            formatter.setMaximum(Some(&NSNumber::new_f32(10.0)));
                            formatter
                        }
                        .as_super(),
                    ));
                    view.widthAnchor()
                        .constraintGreaterThanOrEqualToConstant(96.0)
                        .setActive(true);
                    view.setTarget(Some(&app_delegate));
                    view.setAction(Some(sel!(takeFloatValueForVolumeFrom:)));
                    view
                });
                Ivar::write(&mut this.slider, unsafe {
                    let view = NSSlider::new();
                    view.setVertical(true);
                    view.setMinValue(0.0);
                    view.setMaxValue(10.0);
                    view.setAllowsTickMarkValuesOnly(true);
                    view.setNumberOfTickMarks(11);
                    view.setContinuous(true);
                    view.heightAnchor()
                        .constraintGreaterThanOrEqualToConstant(80.0)
                        .setActive(true);
                    view.setTarget(Some(&app_delegate));
                    view.setAction(Some(sel!(takeFloatValueForVolumeFrom:)));
                    view
                });
                NonNull::from(this)
            })
        }

        #[method(loadView)]
        fn load_view(&self) {
            let app_delegate = self.app_delegate.load().unwrap();

            // Unused for now
            let view = unsafe {
                NSView::initWithFrame(
                    MainThreadMarker::from(self).alloc(),
                    NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(100.0, 100.0)),
                )
            };
            unsafe { view.setWantsLayer(true) };
            let background = unsafe { NSColor::redColor() };
            unsafe {
                // TODO:
                // let layer = view.layer();
                // layer.setBorderWidth(2.0);
                // layer.setBorderColor(background.cgColor());
                #[repr(transparent)]
                struct CGColorRef(*mut c_void);

                unsafe impl Encode for CGColorRef {
                    const ENCODING: Encoding = Encoding::Pointer(&Encoding::Struct("CGColor", &[]));
                }

                let background: CGColorRef = msg_send![&background, CGColor];

                // CALayer
                let layer: Id<NSObject> = msg_send_id![&view, layer];
                let _: () = msg_send![&layer, setBorderWidth: 2.0 as CGFloat];
                let _: () = msg_send![&layer, setBorderColor: background];
            }
            // unsafe { self.setView(&view) };

            let views = [
                Id::into_super(Id::into_super(self.text_field.clone())),
                Id::into_super(Id::into_super(self.slider.clone())),
                Id::into_super(Id::into_super(unsafe {
                    let view = NSButton::buttonWithTitle_target_action(
                        ns_string!("Mute"),
                        Some(&app_delegate),
                        Some(sel!(mute:)),
                    );
                    view
                })),
            ];

            unsafe {
                let stack = NSStackView::stackViewWithViews(&NSArray::from_id_slice(&views));
                stack.setOrientation(NSUserInterfaceLayoutOrientationVertical);
                stack.setAlignment(NSLayoutAttributeCenterX);
                stack.setSpacing(8.0);
                stack.setTranslatesAutoresizingMaskIntoConstraints(false);

                let view = NSView::new();
                view.addSubview(&stack);

                NSLayoutConstraint::activateConstraints(
                    &NSLayoutConstraint::constraintsWithVisualFormat_options_metrics_views(
                        ns_string!("|-[stack]-|"),
                        0,
                        None,
                        &NSDictionary::from_keys_and_objects(
                            &[ns_string!("stack")],
                            vec![Id::into_super(Id::into_super(Id::into_super(
                                Id::into_super(stack.clone()),
                            )))],
                        ),
                    ),
                );
                NSLayoutConstraint::activateConstraints(
                    &NSLayoutConstraint::constraintsWithVisualFormat_options_metrics_views(
                        ns_string!("V:|-[stack]-|"),
                        0,
                        None,
                        &NSDictionary::from_keys_and_objects(
                            &[ns_string!("stack")],
                            vec![Id::into_super(Id::into_super(Id::into_super(
                                Id::into_super(stack.clone()),
                            )))],
                        ),
                    ),
                );

                self.setView(&view);
            }
        }
    }
);

extern_methods!(
    unsafe impl ViewController {
        #[method_id(initWithAppDelegate:)]
        fn init(this: Option<Allocated<Self>>, app_delegate: &AppDelegate) -> Id<Self>;
    }
);

impl ViewController {
    fn update_user_interface(&self) {
        let volume = self.app_delegate.load().unwrap().track.borrow().volume;
        unsafe { self.text_field.setFloatValue(volume as _) };
        unsafe { self.slider.setFloatValue(volume as _) };
    }
}

declare_class!(
    pub struct AppDelegate {
        track: IvarDrop<Box<RefCell<Track>>, "_track">,
        window_controller:
            IvarDrop<Box<RefCell<Option<Id<NSWindowController>>>>, "_window_controller">,
        view_controller: IvarDrop<Box<RefCell<Option<Id<ViewController>>>>, "_view_controller">,
    }

    mod ivars;

    unsafe impl ClassType for AppDelegate {
        type Super = NSObject;
        type Mutability = mutability::MainThreadOnly;
        const NAME: &'static str = "AppDelegate";
    }

    unsafe impl AppDelegate {
        #[method(init)]
        unsafe fn __init(this: *mut Self) -> Option<NonNull<Self>> {
            let this: Option<&mut Self> = unsafe { msg_send![super(this), init] };

            this.map(|this| {
                Ivar::write(&mut this.track, Box::new(RefCell::new(Track::default())));
                Ivar::write(&mut this.window_controller, Box::new(RefCell::new(None)));
                Ivar::write(&mut this.view_controller, Box::new(RefCell::new(None)));
                NonNull::from(this)
            })
        }

        #[method(mute:)]
        fn mute(&self, _sender: &NSButton) {
            self.track.borrow_mut().volume = 0.0;

            self.view_controller
                .borrow()
                .as_ref()
                .unwrap()
                .update_user_interface();
        }

        #[method(takeFloatValueForVolumeFrom:)]
        fn take_float_value_for_volume_from(&self, sender: &NSControl) {
            let new_value = unsafe { sender.floatValue() } as _;

            self.track.borrow_mut().volume = new_value;

            self.view_controller
                .borrow()
                .as_ref()
                .unwrap()
                .update_user_interface();
        }
    }

    unsafe impl NSApplicationDelegate for AppDelegate {
        #[method(applicationDidFinishLaunching:)]
        fn did_finish_launching(&self, notification: &NSNotification) {
            let app: Id<NSApplication> = unsafe { Id::cast(notification.object().unwrap()) };
            let mtm = MainThreadMarker::from(self);
            // Or just let app = NSApplication::sharedApplication();
            unsafe {
                let window = NSWindow::initWithContentRect_styleMask_backing_defer(
                    mtm.alloc(),
                    NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(480.0, 270.0)),
                    NSWindowStyleMaskTitled
                        | NSWindowStyleMaskClosable
                        | NSWindowStyleMaskMiniaturizable
                        | NSWindowStyleMaskResizable,
                    NSBackingStoreBuffered,
                    false,
                );
                // TODO: Make this the default when creating new windows, to
                // preserve memory safety!
                window.setReleasedWhenClosed(false);

                window.center();
                window.setTitle(ns_string!("A window"));

                let view_controller = ViewController::init(mtm.alloc(), self);
                window.setContentView(Some(&view_controller.view()));

                let window_controller =
                    NSWindowController::initWithWindow(mtm.alloc(), Some(&window));
                window_controller.setWindowFrameAutosaveName(ns_string!("main"));

                window.makeKeyAndOrderFront(None);

                app.setActivationPolicy(NSApplicationActivationPolicyRegular);
                app.activateIgnoringOtherApps(true);

                *self.window_controller.borrow_mut() = Some(window_controller);
                *self.view_controller.borrow_mut() = Some(view_controller);

                self.view_controller
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .update_user_interface();
            }
        }
    }
);

unsafe impl NSObjectProtocol for AppDelegate {}

extern_methods!(
    unsafe impl AppDelegate {
        #[method_id(init)]
        fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

#[derive(Debug)]
pub struct Track {
    volume: f32,
}

impl Default for Track {
    fn default() -> Self {
        Self { volume: 5.0 }
    }
}

fn main() {
    let mtm = MainThreadMarker::new().unwrap();

    unsafe {
        let app = NSApplication::sharedApplication();
        let delegate = AppDelegate::init(mtm.alloc());
        app.setDelegate(Some(ProtocolObject::from_ref(&*delegate)));

        app.run();
    }
}
