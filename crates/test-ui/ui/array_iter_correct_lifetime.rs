use icrate::Foundation::{NSArray, NSCopying, NSMutableArray, NSMutableString, NSString};

fn main() {
    let arr: &mut NSArray<NSMutableString> = &mut NSMutableArray::new();
    for _ in &mut *arr {
        let _ = &arr[0];
    }

    let arr: &mut NSMutableArray<NSString> = &mut NSMutableArray::new();
    for s in arr.iter_retained() {
        arr.push(s);
    }

    let arr: &mut NSMutableArray<NSString> = &mut NSMutableArray::new();
    for s in &*arr {
        arr.push(s.copy());
    }
}
