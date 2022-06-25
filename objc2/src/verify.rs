use core::fmt;

use crate::runtime::{Class, Method, Object, Sel};
use crate::{Encode, EncodeArguments, Encoding};

#[allow(dead_code)]
pub(crate) enum VerificationError<'a> {
    NilReceiver(Sel),
    MethodNotFound(&'a Class, Sel),
    MismatchedReturn(&'a Method, Encoding<'static>),
    MismatchedArgumentsCount(&'a Method, usize),
    MismatchedArgument(&'a Method, usize, Encoding<'static>),
}

impl<'a> fmt::Display for VerificationError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            VerificationError::NilReceiver(sel) => {
                write!(f, "Messsaging {:?} to nil", sel)
            }
            VerificationError::MethodNotFound(cls, sel) => {
                write!(f, "Method {:?} not found on class {:?}", sel, cls)
            }
            VerificationError::MismatchedReturn(method, ret) => {
                let expected_ret = method.return_type();
                write!(
                    f,
                    "Return type code {} does not match expected {} for method {:?}",
                    ret,
                    expected_ret,
                    method.name()
                )
            }
            VerificationError::MismatchedArgumentsCount(method, count) => {
                let expected_count = method.arguments_count();
                write!(
                    f,
                    "Method {:?} accepts {} arguments, but {} were given",
                    method.name(),
                    expected_count,
                    count
                )
            }
            VerificationError::MismatchedArgument(method, i, arg) => {
                let expected = method.argument_type(i).unwrap();
                write!(
                    f,
                    "Method {:?} expected argument at index {} with type code {} but was given {}",
                    method.name(),
                    i,
                    expected,
                    arg
                )
            }
        }
    }
}

pub(crate) fn verify_message_signature<A, R>(
    cls: &Class,
    sel: Sel,
) -> Result<(), VerificationError<'_>>
where
    A: EncodeArguments,
    R: Encode,
{
    let method = match cls.instance_method(sel) {
        Some(method) => method,
        None => return Err(VerificationError::MethodNotFound(cls, sel)),
    };

    let ret = R::ENCODING;
    let expected_ret = method.return_type();
    if !ret.equivalent_to_str(&*expected_ret) {
        return Err(VerificationError::MismatchedReturn(method, ret));
    }

    let self_and_cmd = [<*mut Object>::ENCODING, Sel::ENCODING];
    let args = A::ENCODINGS;

    let count = self_and_cmd.len() + args.len();
    let expected_count = method.arguments_count();
    if count != expected_count {
        return Err(VerificationError::MismatchedArgumentsCount(method, count));
    }

    for (i, arg) in self_and_cmd.iter().chain(args).copied().enumerate() {
        let expected = method.argument_type(i).unwrap();
        if !arg.equivalent_to_str(&*expected) {
            return Err(VerificationError::MismatchedArgument(method, i, arg));
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
        assert_eq!(
            err.to_string(),
            "Method setFoo not found on class CustomObject"
        );

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
