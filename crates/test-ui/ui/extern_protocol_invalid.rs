use objc2::extern_protocol;

extern_protocol!(
    trait IsSafe {}
);

extern_protocol!(
    #[super = objc2::runtime::NSObject]
    unsafe trait HasSuper {}
);

extern_protocol!(
    #[unsafe(super(objc2::runtime::NSObject))]
    unsafe trait HasUnsafeSuper {}
);

extern_protocol!(
    #[name = "A"]
    #[name = "B"]
    unsafe trait DuplicateName {}
);

extern_protocol!(
    #[ivars = i32]
    unsafe trait HasIvars {}
);

extern_protocol!(
    #[thread_kind = MainThreadOnly]
    unsafe trait HasThreadKind {}
);

extern_protocol!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    unsafe trait HasDerive {}
);

fn main() {}
