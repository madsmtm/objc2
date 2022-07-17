use alloc::string::{String, ToString};
use core::fmt;
use core::hash::Hash;
use std::error::Error;

use crate::runtime::{Class, Object, Sel};
use crate::{Encode, EncodeArguments, Encoding};

#[derive(Debug, PartialEq, Eq, Hash)]
enum Inner {
    MethodNotFound,
    MismatchedReturn(String, Encoding<'static>),
    MismatchedArgumentsCount(usize, usize),
    MismatchedArgument(usize, String, Encoding<'static>),
}

impl fmt::Display for Inner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MethodNotFound => {
                write!(f, "method not found")
            }
            Self::MismatchedReturn(expected, actual) => {
                write!(
                    f,
                    "expected return to have type code {}, but found {}",
                    expected, actual
                )
            }
            Self::MismatchedArgumentsCount(expected, actual) => {
                write!(
                    f,
                    "expected {} arguments, but {} were given",
                    expected, actual
                )
            }
            Self::MismatchedArgument(i, expected, actual) => {
                write!(
                    f,
                    "expected argument at index {} to have type code {}, but found {}",
                    i, expected, actual
                )
            }
        }
    }
}

/// Failed verifying selector on a class.
///
/// This is returned in the error case of [`Class::verify_sel`], see that for
/// details.
///
/// This implements [`Error`], and a description of the error can be retrieved
/// using [`fmt::Display`].
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct VerificationError(Inner);

impl From<Inner> for VerificationError {
    fn from(inner: Inner) -> Self {
        Self(inner)
    }
}

impl fmt::Display for VerificationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Delegate to inner
        fmt::Display::fmt(&self.0, f)
    }
}

impl Error for VerificationError {}

pub(crate) fn verify_message_signature<A, R>(cls: &Class, sel: Sel) -> Result<(), VerificationError>
where
    A: EncodeArguments,
    R: Encode,
{
    let method = match cls.instance_method(sel) {
        Some(method) => method,
        None => return Err(Inner::MethodNotFound.into()),
    };

    // TODO: Verify stack layout?
    let mut iter = method.types();

    // Verify return type (first type in `types`)
    let (expected, _stack_layout) = iter
        .next()
        .expect("Expected to find return type in encoding type")
        .unwrap_or_else(|e| panic!("{}", e));
    let actual = R::ENCODING;
    if !actual.equivalent_to_str(expected) {
        return Err(Inner::MismatchedReturn(expected.to_string(), actual).into());
    }

    // Verify receiver type
    let (expected, _stack_layout) = iter
        .next()
        .expect("Expected to find receiver type in encoding type")
        .unwrap_or_else(|e| panic!("{}", e));
    let actual = <*mut Object>::ENCODING;
    if !actual.equivalent_to_str(expected) {
        panic!("Invalid receiver encoding {} in method {:?}", expected, sel);
    }

    // Verify selector type
    let (expected, _stack_layout) = iter
        .next()
        .expect("Expected to find selector type in encoding type")
        .unwrap_or_else(|e| panic!("{}", e));
    let actual = Sel::ENCODING;
    if !actual.equivalent_to_str(expected) {
        panic!("Invalid selector encoding {} in method {:?}", expected, sel);
    }

    let actual_count = A::ENCODINGS.len();

    for (i, &actual) in A::ENCODINGS.iter().enumerate() {
        if let Some(res) = iter.next() {
            let (expected, _stack_layout) = res.unwrap_or_else(|e| panic!("{}", e));
            if !actual.equivalent_to_str(expected) {
                return Err(Inner::MismatchedArgument(i, expected.to_string(), actual).into());
            }
        } else {
            return Err(Inner::MismatchedArgumentsCount(i, actual_count).into());
        }
    }

    let remaining = iter.count();
    if remaining != 0 {
        return Err(Inner::MismatchedArgumentsCount(actual_count + remaining, actual_count).into());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils;
    use alloc::string::ToString;
    use core::panic::{RefUnwindSafe, UnwindSafe};

    #[test]
    fn test_verify_message() {
        let cls = test_utils::custom_class();

        assert!(cls.verify_sel::<(), u32>(sel!(foo)).is_ok());
        assert!(cls.verify_sel::<(u32,), ()>(sel!(setFoo:)).is_ok());

        let metaclass = cls.metaclass();
        metaclass
            .verify_sel::<(i32, i32), i32>(sel!(addNumber:toNumber:))
            .unwrap();
    }

    #[test]
    fn test_verify_message_errors() {
        let cls = test_utils::custom_class();

        // Unimplemented selector (missing colon)
        let err = cls.verify_sel::<(), ()>(sel!(setFoo)).unwrap_err();
        assert_eq!(err.to_string(), "method not found");

        // Incorrect return type
        let err = cls.verify_sel::<(u32,), u64>(sel!(setFoo:)).unwrap_err();
        assert_eq!(
            err.to_string(),
            "expected return to have type code v, but found Q"
        );

        // Too many arguments
        let err = cls.verify_sel::<(u32, i8), ()>(sel!(setFoo:)).unwrap_err();
        assert_eq!(err.to_string(), "expected 1 arguments, but 2 were given");

        // Too few arguments
        let err = cls.verify_sel::<(), ()>(sel!(setFoo:)).unwrap_err();
        assert_eq!(err.to_string(), "expected 1 arguments, but 0 were given");

        // Incorrect argument type
        let err = cls.verify_sel::<(Sel,), ()>(sel!(setFoo:)).unwrap_err();
        assert_eq!(
            err.to_string(),
            "expected argument at index 0 to have type code I, but found :"
        );

        // Metaclass
        let metaclass = cls.metaclass();
        let err = metaclass
            .verify_sel::<(i32, i32, i32), i32>(sel!(addNumber:toNumber:))
            .unwrap_err();
        assert_eq!(err.to_string(), "expected 2 arguments, but 3 were given");
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic = "invalid message send to -[CustomObject foo]: expected return to have type code I, but found i"]
    fn test_send_message_verified() {
        let obj = test_utils::custom_object();
        let _: i32 = unsafe { msg_send![&obj, foo] };
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic = "invalid message send to +[CustomObject abcDef]: method not found"]
    fn test_send_message_verified_to_class() {
        let cls = test_utils::custom_class();
        let _: i32 = unsafe { msg_send![cls, abcDef] };
    }

    #[test]
    fn test_marker_traits() {
        fn assert_marker_traits<T: Send + Sync + UnwindSafe + RefUnwindSafe + Unpin>() {}
        assert_marker_traits::<VerificationError>();
    }
}
