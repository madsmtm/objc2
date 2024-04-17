use objc2::rc::Id;
use objc2_foundation::{NSArray, NSMutableArray, NSMutableString, NSString};

fn main() {
    let arr: Id<NSArray<NSString>> = NSArray::new();
    for s in &mut arr {
        let s: &mut NSString = s;
    }

    let arr: Id<NSArray<NSMutableString>> = NSArray::new();
    for s in &mut arr {
        let s: &mut NSMutableString = s;
    }

    let arr: Id<NSMutableArray<NSString>> = NSMutableArray::new();
    for s in &mut arr {
        let s: &mut NSString = s;
    }

    // Should succeed, included for completeness
    let arr: Id<NSMutableArray<NSMutableString>> = NSMutableArray::new();
    for s in &mut arr {
        let s: &mut NSString = s;
    }

    // IdIntoIterator not available for mutable children
    let arr: Id<NSArray<NSMutableString>> = NSArray::new();
    for _ in arr {}
}
