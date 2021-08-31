// Always available in Objective-C
// See https://clang.llvm.org/docs/AutomaticReferenceCounting.html#arc-runtime-objc-retain
id objc_retain(id value);

int RustObjCExceptionTryCatch(void (*try)(void *), void *context, id *error) {
    @try {
        try(context);
        if (error) {
            *error = (id)0; // nil
        }
        return 0;
    } @catch (id exception) {
        if (error) {
            *error = objc_retain(exception);
        }
        return 1;
    }
}
