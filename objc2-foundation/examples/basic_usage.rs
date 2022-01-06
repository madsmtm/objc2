use objc2::rc::autoreleasepool;
use objc2_foundation::{
    INSCopying, INSDictionary, INSString, NSArray, NSDictionary, NSObject, NSString,
};

fn main() {
    // Create and compare NSObjects
    let obj = NSObject::new();
    println!("{:?} == {:?}? {:?}", obj, obj, obj == obj);

    let obj2 = NSObject::new();
    println!("{:?} == {:?}? {:?}", obj, obj2, obj == obj2);

    // Create an NSArray from a Vec
    let objs = vec![obj, obj2];
    let array = NSArray::from_vec(objs);
    for obj in array.iter() {
        println!("{:?}", obj);
    }
    println!("{}", array.len());

    // Turn the NSArray back into a Vec
    let mut objs = NSArray::into_vec(array);
    let obj = objs.pop().unwrap();

    // Create an NSString from a str slice
    let string = NSString::from_str("Hello, world!");
    // Use an autoreleasepool to get the `str` contents of the NSString
    autoreleasepool(|pool| {
        println!("{}", string.as_str(pool));
        let string2 = string.copy();
        println!("{}", string2.as_str(pool));
    });

    // Create a dictionary mapping strings to objects
    let keys = &[&*string];
    let vals = vec![obj];
    let dict = NSDictionary::from_keys_and_objects(keys, vals);
    println!("{:?}", dict.get(&string));
    println!("{}", dict.len());
}
