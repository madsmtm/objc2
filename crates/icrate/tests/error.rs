#![cfg(feature = "Foundation_NSError")]
#![cfg(feature = "Foundation_NSString")]
use icrate::ns_string;
use icrate::Foundation::{NSError, NSURLErrorDomain};

#[test]
fn basic() {
    let error = NSError::new(-999, unsafe { NSURLErrorDomain });
    let expected = if cfg!(feature = "apple") {
        "The operation couldn’t be completed. (NSURLErrorDomain error -999.)"
    } else {
        "NSURLErrorDomain -999"
    };
    assert_eq!(format!("{error}"), expected);
}

#[test]
fn custom_domain() {
    let error = NSError::new(42, ns_string!("MyDomain"));
    assert_eq!(error.code(), 42);
    assert_eq!(&*error.domain(), ns_string!("MyDomain"));
    let expected = if cfg!(feature = "apple") {
        "The operation couldn’t be completed. (MyDomain error 42.)"
    } else {
        "MyDomain 42"
    };
    assert_eq!(format!("{error}"), expected);
}
