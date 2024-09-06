use crate::{AllocAnyThread, ClassType, MainThreadOnly, ThreadKind};

/// Helper for ensuring that `ClassType::ThreadKind`, if specified, is set
/// correctly.
pub trait ValidThreadKind<Requested: ?Sized + ThreadKind>
where
    Self: ClassType<ThreadKind = Requested>,
    // Ensure the user did not attempt to declare or define a root class.
    Self::Super: ClassType,
{
    // Required to reference the trait.
    fn check() {}
}

/// Always allow setting `MainThreadOnly`.
impl<'a, Cls> ValidThreadKind<dyn MainThreadOnly + 'a> for Cls
where
    Self: ClassType<ThreadKind = dyn MainThreadOnly + 'a>,
    Self::Super: ClassType,
{
}

/// But restrict `AllocAnyThread` to only if the superclass also sets it.
impl<'a, 'b, Cls> ValidThreadKind<dyn AllocAnyThread + 'a> for Cls
where
    Self: ClassType<ThreadKind = dyn AllocAnyThread + 'a>,
    Self::Super: ClassType<ThreadKind = dyn AllocAnyThread + 'b>,
{
}

/// Check that `MainThreadOnly` types do not implement `Send` and `Sync`.
///
/// Check implemented using type inference:
/// let _ = <MyType as MainThreadOnlyDoesNotImplSendSync<_>>::check
pub trait MainThreadOnlyDoesNotImplSendSync<Inferred> {
    // Required to reference the trait.
    fn check() {}
}

// Type inference will find this blanket impl...
impl<Cls: ?Sized> MainThreadOnlyDoesNotImplSendSync<()> for Cls {}

// ... unless one of these impls also apply, then type inference fails.
struct ImplsSend;
impl<Cls: ?Sized + MainThreadOnly + Send> MainThreadOnlyDoesNotImplSendSync<ImplsSend> for Cls {}

struct ImplsSync;
impl<Cls: ?Sized + MainThreadOnly + Sync> MainThreadOnlyDoesNotImplSendSync<ImplsSync> for Cls {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extern_class;
    use crate::runtime::NSObject;

    extern_class!(
        struct SetAnyThread;

        unsafe impl ClassType for SetAnyThread {
            type Super = NSObject;
            type ThreadKind = dyn AllocAnyThread;
            const NAME: &'static str = "NSObject";
        }
    );

    extern_class!(
        struct SendSync;

        unsafe impl ClassType for SendSync {
            type Super = NSObject;
            type ThreadKind = dyn AllocAnyThread;
            const NAME: &'static str = "NSObject";
        }
    );

    unsafe impl Send for SendSync {}
    unsafe impl Sync for SendSync {}

    extern_class!(
        struct OnlyMain;

        unsafe impl ClassType for OnlyMain {
            type Super = NSObject;
            type ThreadKind = dyn MainThreadOnly;
            const NAME: &'static str = "NSObject";
        }
    );

    extern_class!(
        struct OnlyMainSubDefault;

        unsafe impl ClassType for OnlyMainSubDefault {
            type Super = OnlyMain;
            const NAME: &'static str = "NSObject";
        }
    );

    extern_class!(
        struct OnlyMainSubExplicit;

        unsafe impl ClassType for OnlyMainSubExplicit {
            type Super = OnlyMain;
            type ThreadKind = dyn MainThreadOnly;
            const NAME: &'static str = "NSObject";
        }
    );
}
