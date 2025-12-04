use objc2::encode::{Encode, Encoding, RefEncode};

/// Handle of the GPU resource used for binding resources to argument tables,
/// navigating resource view pools and storing resources in an argument buffer
///
/// MTLResourceID represents a specific GPU resource. This handle can be
/// mutated by modifying textureID or samplerID values to get to individual
/// resource views in a resource view pool.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtlresourceid?language=objc)
#[repr(C)]
// Not Default, see `header-translator/src/rust_type.rs` `has_zero_default`
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLResourceID {
    pub(crate) _impl: u64,
}

unsafe impl Encode for MTLResourceID {
    #[allow(unexpected_cfgs)]
    const ENCODING: Encoding = Encoding::Struct(
        "MTLResourceID",
        // NOTE: This check only works on Rust 1.91 and above. If we were to
        // use `target_abi = "sim"`, it'd work on Rust 1.78, but it would also
        // cause a compilation error on lower versions (and our MSRV is still
        // at Rust 1.71, so that's probably unacceptable).
        if cfg!(target_env = "sim") {
            &[Encoding::Union("?", &[<u64>::ENCODING, <u64>::ENCODING])]
        } else {
            &[<u64>::ENCODING]
        },
    );
}

unsafe impl RefEncode for MTLResourceID {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

impl MTLResourceID {
    /// Construct a `MTLResourceID` from an ID previously gotten via `to_raw`.
    ///
    /// # Safety
    ///
    /// The documentation for `MTLResourceID` says:
    ///
    /// > A MTLResourceID represents a specific GPU resource, mutating this
    /// > handle is undefined unless the mutation results in the value
    /// > equalling an already existing handle of the same resource type.
    ///
    /// So we've tentatively marked this method as `unsafe`, with the safety
    /// requirement that the ID must be valid, i.e. have previously come from
    /// [`to_raw`][Self::to_raw] or similar.
    ///
    /// If you disagree with this assessment, feel free to open an issue!
    pub const unsafe fn from_raw(id: u64) -> Self {
        Self { _impl: id }
    }

    /// Get the underlying data of the ID.
    ///
    /// May be useful for FFI purposes.
    pub const fn to_raw(self) -> u64 {
        self._impl
    }
}

#[cfg(test)]
mod tests {
    use std::string::ToString;

    use super::*;

    #[test]
    fn encoding() {
        #[allow(unexpected_cfgs)]
        let expected = if cfg!(target_env = "sim") {
            "{MTLResourceID=(?=QQ)}"
        } else {
            "{MTLResourceID=Q}"
        };
        assert_eq!(MTLResourceID::ENCODING.to_string(), expected);
    }
}
