use core::{fmt, ptr};

use objc2::encode::{Encode, Encoding};

use crate::{
    SFNTLookupArrayHeader, SFNTLookupSegmentHeader, SFNTLookupSingleHeader,
    SFNTLookupTrimmedArrayHeader, SFNTLookupVectorHeader,
};

#[repr(C)]
#[derive(Clone, Copy)]
pub union SFNTLookupFormatSpecificHeader {
    the_array: SFNTLookupArrayHeader,
    segment: SFNTLookupSegmentHeader,
    single: SFNTLookupSingleHeader,
    trimmed_array: SFNTLookupTrimmedArrayHeader,
    vector: SFNTLookupVectorHeader,
}

impl fmt::Debug for SFNTLookupFormatSpecificHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SFNTLookupFormatSpecificHeader")
            .finish_non_exhaustive()
    }
}

impl PartialEq for SFNTLookupFormatSpecificHeader {
    fn eq(&self, other: &Self) -> bool {
        // This is probably wrong, but it's difficult to
        // implement a more correct comparison.
        ptr::eq(self, other)
    }
}

unsafe impl Encode for SFNTLookupFormatSpecificHeader {
    const ENCODING: Encoding = Encoding::Union(
        "SFNTLookupFormatSpecificHeader",
        &[
            <SFNTLookupArrayHeader>::ENCODING,
            <SFNTLookupSegmentHeader>::ENCODING,
            <SFNTLookupSingleHeader>::ENCODING,
            <SFNTLookupTrimmedArrayHeader>::ENCODING,
            <SFNTLookupVectorHeader>::ENCODING,
        ],
    );
}
