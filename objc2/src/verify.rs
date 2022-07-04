use core::fmt;
use core::hash::{Hash, Hasher};
use malloc_buf::Malloc;
use std::error::Error;

use crate::runtime::{Class, Object, Sel};
use crate::{Encode, EncodeArguments, Encoding};

/// Workaround for `Malloc<str>` not implementing common traits
#[derive(Debug)]
struct MallocEncoding(Malloc<str>);

impl PartialEq for MallocEncoding {
    fn eq(&self, other: &Self) -> bool {
        *self.0 == *other.0
    }
}

impl Eq for MallocEncoding {}

impl Hash for MallocEncoding {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&*self.0, state)
    }
}

impl fmt::Display for MallocEncoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&*self.0, f)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Inner {
    MethodNotFound(Sel),
    MismatchedReturn(Sel, MallocEncoding, Encoding<'static>),
    MismatchedArgumentsCount(Sel, usize, usize),
    MismatchedArgument(Sel, usize, MallocEncoding, Encoding<'static>),
}

impl fmt::Display for Inner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MethodNotFound(sel) => {
                write!(f, "Method {:?} not found on class", sel)
            }
            Self::MismatchedReturn(sel, expected, actual) => {
                write!(
                    f,
                    "Return type code {} does not match expected {} for method {:?}",
                    actual, expected, sel
                )
            }
            Self::MismatchedArgumentsCount(sel, expected, actual) => {
                write!(
                    f,
                    "Method {:?} accepts {} arguments, but {} were given",
                    sel, expected, actual
                )
            }
            Self::MismatchedArgument(sel, i, expected, actual) => {
                write!(
                    f,
                    "Method {:?} expected argument at index {} with type code {} but was given {}",
                    sel, i, expected, actual
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
        None => return Err(Inner::MethodNotFound(sel).into()),
    };

    let actual = R::ENCODING;
    let expected = method.return_type();
    if !actual.equivalent_to_str(&*expected) {
        return Err(Inner::MismatchedReturn(sel, MallocEncoding(expected), actual).into());
    }

    let self_and_cmd = [<*mut Object>::ENCODING, Sel::ENCODING];
    let args = A::ENCODINGS;

    let actual = self_and_cmd.len() + args.len();
    let expected = method.arguments_count();
    if actual != expected {
        return Err(Inner::MismatchedArgumentsCount(sel, expected, actual).into());
    }

    for (i, actual) in self_and_cmd.iter().chain(args).copied().enumerate() {
        let expected = method.argument_type(i).unwrap();
        if !actual.equivalent_to_str(&*expected) {
            return Err(Inner::MismatchedArgument(sel, i, MallocEncoding(expected), actual).into());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils;
    use alloc::string::ToString;

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
        assert_eq!(err.to_string(), "Method setFoo not found on class");

        // Incorrect return type
        let err = cls.verify_sel::<(u32,), u64>(sel!(setFoo:)).unwrap_err();
        assert_eq!(
            err.to_string(),
            "Return type code Q does not match expected v for method setFoo:"
        );

        // Too many arguments
        let err = cls.verify_sel::<(u32, i8), ()>(sel!(setFoo:)).unwrap_err();
        assert_eq!(
            err.to_string(),
            "Method setFoo: accepts 3 arguments, but 4 were given"
        );

        // Too few arguments
        let err = cls.verify_sel::<(), ()>(sel!(setFoo:)).unwrap_err();
        assert_eq!(
            err.to_string(),
            "Method setFoo: accepts 3 arguments, but 2 were given"
        );

        // Incorrect argument type
        let err = cls.verify_sel::<(Sel,), ()>(sel!(setFoo:)).unwrap_err();
        assert_eq!(
            err.to_string(),
            "Method setFoo: expected argument at index 2 with type code I but was given :"
        );
    }

    #[test]
    #[cfg(feature = "verify_message")]
    #[should_panic = "Return type code i does not match expected I for method foo"]
    fn test_send_message_verified() {
        let obj = test_utils::custom_object();
        let _: i32 = unsafe { msg_send![&obj, foo] };
    }
}
