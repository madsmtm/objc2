use objc2_foundation::{NSArray, NSCopying, NSMutableArray};

fn main() {
    let arr: &mut NSArray<NSMutableArray> = &mut NSMutableArray::new();
    for _ in &mut *arr {
        let _ = &arr[0];
    }

    let arr: &mut NSMutableArray<NSArray> = &mut NSMutableArray::new();
    for s in arr.iter_retained() {
        arr.push(s);
    }

    let arr: &mut NSMutableArray<NSArray> = &mut NSMutableArray::new();
    for s in &*arr {
        arr.push(s.copy());
    }
}
