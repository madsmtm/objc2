use objc2::rc::Retained;
use objc2_foundation::{NSArray, NSMutableArray, NSMutableString, NSString};

fn main() {
    let arr: Retained<NSArray<NSString>> = NSArray::new();
    for s in &mut arr {
        let s: &mut NSString = s;
    }

    let arr: Retained<NSArray<NSMutableString>> = NSArray::new();
    for s in &mut arr {
        let s: &mut NSMutableString = s;
    }

    let arr: Retained<NSMutableArray<NSString>> = NSMutableArray::new();
    for s in &mut arr {
        let s: &mut NSString = s;
    }

    // Should succeed, included for completeness
    let arr: Retained<NSMutableArray<NSMutableString>> = NSMutableArray::new();
    for s in &mut arr {
        let s: &mut NSString = s;
    }

    // RetainedIntoIterator not available for mutable children
    let arr: Retained<NSArray<NSMutableString>> = NSArray::new();
    for _ in arr {}
}
