use core::fmt;
use malloc_buf::Malloc;

use crate::runtime::{Class, Object, Sel};
use crate::{Encode, EncodeArguments, Encoding};

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) enum VerificationError {
    NilReceiver(Sel),
    MethodNotFound(Sel),
    MismatchedReturn(Sel, Malloc<str>, Encoding<'static>),
    MismatchedArgumentsCount(Sel, usize, usize),
    MismatchedArgument(Sel, usize, Malloc<str>, Encoding<'static>),
}

impl fmt::Display for VerificationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NilReceiver(sel) => {
                write!(f, "Messsaging {:?} to nil", sel)
            }
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

pub(crate) fn verify_message_signature<A, R>(cls: &Class, sel: Sel) -> Result<(), VerificationError>
where
    A: EncodeArguments,
    R: Encode,
{
    let method = match cls.instance_method(sel) {
        Some(method) => method,
        None => return Err(VerificationError::MethodNotFound(sel)),
    };

    let actual = R::ENCODING;
    let expected = method.return_type();
    if !actual.equivalent_to_str(&*expected) {
        return Err(VerificationError::MismatchedReturn(sel, expected, actual));
    }

    let self_and_cmd = [<*mut Object>::ENCODING, Sel::ENCODING];
    let args = A::ENCODINGS;

    let actual = self_and_cmd.len() + args.len();
    let expected = method.arguments_count();
    if actual != expected {
        return Err(VerificationError::MismatchedArgumentsCount(
            sel, expected, actual,
        ));
    }

    for (i, actual) in self_and_cmd.iter().chain(args).copied().enumerate() {
        let expected = method.argument_type(i).unwrap();
        if !actual.equivalent_to_str(&*expected) {
            return Err(VerificationError::MismatchedArgument(
                sel, i, expected, actual,
            ));
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
