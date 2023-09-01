use std::os::raw::c_ushort;

extern_struct!(
    pub struct NSDecimal {
        // signed   int _exponent:8;
        // unsigned int _length:4;
        // unsigned int _isNegative:1;
        // unsigned int _isCompact:1;
        // unsigned int _reserved:18;
        _inner: i32,
        _mantissa: [c_ushort; 8],
    }
);
