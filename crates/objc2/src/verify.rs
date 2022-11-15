use core::fmt;
use core::hash::Hash;
use std::error::Error;

use crate::encode::{Encode, EncodeArguments, EncodeConvert, Encoding, EncodingBox};
use crate::runtime::{EncodingParseError, Method};

#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) enum Inner {
    MethodNotFound,
    EncodingParseError(EncodingParseError),
    MismatchedReturn(EncodingBox, Encoding),
    MismatchedArgumentsCount(usize, usize),
    MismatchedArgument(usize, EncodingBox, Encoding),
}

impl fmt::Display for Inner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MethodNotFound => write!(f, "method not found"),
            Self::EncodingParseError(e) => write!(f, "{e}"),
            Self::MismatchedReturn(expected, actual) => {
                write!(
                    f,
                    "expected return to have type code '{expected}', but found '{actual}'",
                )
            }
            Self::MismatchedArgumentsCount(expected, actual) => {
                write!(f, "expected {expected} arguments, but {actual} were given",)
            }
            Self::MismatchedArgument(i, expected, actual) => {
                write!(
                    f,
                    "expected argument at index {i} to have type code '{expected}', but found '{actual}'",
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
///
/// [`Class::verify_sel`]: crate::runtime::Class::verify_sel
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct VerificationError(Inner);

impl From<EncodingParseError> for VerificationError {
    fn from(e: EncodingParseError) -> Self {
        Self(Inner::EncodingParseError(e))
    }
}

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

pub(crate) fn verify_method_signature<A, R>(method: &Method) -> Result<(), VerificationError>
where
    A: EncodeArguments,
    R: EncodeConvert,
{
    let mut iter = method.types();

    // TODO: Verify stack layout
    let (expected, _stack_layout) = iter.extract_return()?;
    let actual = R::__Inner::ENCODING;
    if !actual.equivalent_to_box(&expected) {
        return Err(Inner::MismatchedReturn(expected, actual).into());
    }

    iter.verify_receiver()?;
    iter.verify_sel()?;

    let actual_count = A::ENCODINGS.len();

    for (i, actual) in A::ENCODINGS.iter().enumerate() {
        if let Some(res) = iter.next() {
            // TODO: Verify stack layout
            let (expected, _stack_layout) = res?;
            if !actual.equivalent_to_box(&expected) {
                return Err(Inner::MismatchedArgument(i, expected, actual.clone()).into());
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
    use crate::runtime::Sel;
    use crate::sel;
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
            "expected return to have type code 'v', but found 'Q'"
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
            "expected argument at index 0 to have type code 'I', but found ':'"
        );

        // Metaclass
        let metaclass = cls.metaclass();
        let err = metaclass
            .verify_sel::<(i32, i32, i32), i32>(sel!(addNumber:toNumber:))
            .unwrap_err();
        assert_eq!(err.to_string(), "expected 2 arguments, but 3 were given");
    }

    #[test]
    #[cfg(feature = "verify_message")]
    #[should_panic = "invalid message send to -[CustomObject foo]: expected return to have type code 'I', but found 'i'"]
    fn test_send_message_verified() {
        let obj = test_utils::custom_object();
        let _: i32 = unsafe { crate::msg_send![&obj, foo] };
    }

    #[test]
    #[cfg(feature = "verify_message")]
    #[should_panic = "invalid message send to +[CustomObject abcDef]: method not found"]
    fn test_send_message_verified_to_class() {
        let cls = test_utils::custom_class();
        let _: i32 = unsafe { crate::msg_send![cls, abcDef] };
    }

    #[test]
    fn test_marker_traits() {
        fn assert_marker_traits<T: Send + Sync + UnwindSafe + RefUnwindSafe + Unpin>() {}
        assert_marker_traits::<VerificationError>();
    }
}
