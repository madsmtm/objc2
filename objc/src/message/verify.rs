use std::fmt;

use crate::runtime::{Class, Method, Object, Sel};
use crate::{Encode, EncodeArguments, Encoding};

pub enum VerificationError<'a> {
    NilReceiver(Sel),
    MethodNotFound(&'a Class, Sel),
    MismatchedReturn(&'a Method, Encoding<'static>),
    MismatchedArgumentsCount(&'a Method, usize),
    MismatchedArgument(&'a Method, usize, Encoding<'static>),
}

impl<'a> fmt::Display for VerificationError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

pub fn verify_message_signature<A, R>(cls: &Class, sel: Sel) -> Result<(), VerificationError>
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
    if ret != *expected_ret {
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
        if arg != *expected {
            return Err(VerificationError::MismatchedArgument(method, i, arg));
        }
    }

    Ok(())
}
