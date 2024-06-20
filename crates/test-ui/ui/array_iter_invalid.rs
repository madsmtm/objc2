use objc2::rc::Retained;
use objc2_foundation::{NSArray, NSMutableArray};

fn main() {
    let arr: Retained<NSArray<NSArray>> = NSArray::new();
    for s in &mut arr {
        let s: &mut NSArray = s;
    }

    let arr: Retained<NSArray<NSMutableArray>> = NSArray::new();
    for s in &mut arr {
        let s: &mut NSMutableArray = s;
    }

    let arr: Retained<NSMutableArray<NSArray>> = NSMutableArray::new();
    for s in &mut arr {
        let s: &mut NSArray = s;
    }

    // Should succeed, included for completeness
    let arr: Retained<NSMutableArray<NSMutableArray>> = NSMutableArray::new();
    for s in &mut arr {
        let s: &mut NSArray = s;
    }

    // RetainedIntoIterator not available for mutable children
    let arr: Retained<NSArray<NSMutableArray>> = NSArray::new();
    for _ in arr {}
}
