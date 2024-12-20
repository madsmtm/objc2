use std::sync::LazyLock;

use regex::Regex;

/// Algorithm described in:
/// <https://clang.llvm.org/docs/AutomaticReferenceCounting.html#auditing-of-c-retainable-pointer-interfaces>
///
/// > A function obeys the create/copy naming convention if its name
/// > contains as a substring:
/// > - either “Create” or “Copy” not followed by a lowercase letter, or
/// > - either “create” or “copy” not followed by a lowercase letter and
/// >   not preceded by any letter, whether uppercase or lowercase.
///
/// See also Clang's implementation:
/// <https://github.com/llvm/llvm-project/blob/llvmorg-19.1.6/clang/lib/Analysis/CocoaConventions.cpp#L97-L145>
/// <https://github.com/llvm/llvm-project/blob/llvmorg-19.1.6/clang/lib/Analysis/RetainSummaryManager.cpp>
pub(crate) fn follows_create_rule(name: &str) -> bool {
    static RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"(Create|Copy)([^a-z]|$)|([^a-zA-Z]|^)(create|copy)([^a-z]|$)").unwrap()
    });

    RE.is_match(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_follows_create_rule() {
        assert!(follows_create_rule("ThingCreate"));
        assert!(follows_create_rule("CreateThing"));
        assert!(follows_create_rule("CopyCreateThing"));
        assert!(follows_create_rule("create_thing"));

        assert!(!follows_create_rule("Created"));
        assert!(!follows_create_rule("created"));
        assert!(!follows_create_rule("GetAbc"));
        assert!(!follows_create_rule("recreate"));

        assert!(follows_create_rule("CreatedCopy"));

        // A few real-world examples
        assert!(follows_create_rule("dispatch_data_create"));
        assert!(follows_create_rule("dispatch_data_create_map"));
        assert!(!follows_create_rule("dispatch_data_get_size"));
        assert!(follows_create_rule("MTLCreateSystemDefaultDevice"));
        assert!(follows_create_rule("MTLCopyAllDevices"));
        assert!(!follows_create_rule("MTLRemoveDeviceObserver"));
        assert!(follows_create_rule("CFArrayCreate"));
        assert!(follows_create_rule("CFArrayCreateCopy"));
        assert!(!follows_create_rule("CFArrayGetCount"));
        assert!(!follows_create_rule("CFArrayGetValues"));
    }
}
