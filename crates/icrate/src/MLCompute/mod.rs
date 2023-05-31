#[path = "../generated/MLCompute/mod.rs"]
mod generated;

pub use self::generated::*;

#[link(name = "MLCompute", kind = "framework")]
extern "C" {}

#[cfg(test)]
#[cfg(feature = "Foundation_NSNumber")]
#[cfg(feature = "Foundation_NSArray")]
mod tests {
    use super::*;
    use crate::Foundation::{NSArray, NSNumber};

    #[test]
    fn it_works() {
        unsafe {
            MLCTensor::tensorWithShape_dataType(
                &NSArray::from_vec(vec![NSNumber::new_u8(4), NSNumber::new_u8(4)]),
                MLCDataTypeFloat32,
            );
        }
    }
}
