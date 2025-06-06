	.section	__TEXT,__text,regular,pure_instructions
	.globl	_get_ascii
	.p2align	2
_get_ascii:
Lloh0:
	adrp	x0, SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 0)@PAGE
Lloh1:
	add	x0, x0, SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 0)@PAGEOFF
	ret
	.loh AdrpAdd	Lloh0, Lloh1

	.globl	_get_utf16
	.p2align	2
_get_utf16:
Lloh2:
	adrp	x0, SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 0)@PAGE
Lloh3:
	add	x0, x0, SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 0)@PAGEOFF
	ret
	.loh AdrpAdd	Lloh2, Lloh3

	.globl	_get_with_nul
	.p2align	2
_get_with_nul:
Lloh4:
	adrp	x0, SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 0)@PAGE
Lloh5:
	add	x0, x0, SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 0)@PAGEOFF
	ret
	.loh AdrpAdd	Lloh4, Lloh5

	.section	__DATA,__const
	.globl	_EMPTY
	.p2align	3, 0x0
_EMPTY:
	.quad	SYM(test_ns_string[CRATE_ID]::EMPTY::CFSTRING, 0)

	.globl	_XYZ
	.p2align	3, 0x0
_XYZ:
	.quad	SYM(test_ns_string[CRATE_ID]::XYZ::CFSTRING, 0)

	.globl	_NON_ASCII
	.p2align	3, 0x0
_NON_ASCII:
	.quad	SYM(test_ns_string[CRATE_ID]::NON_ASCII::CFSTRING, 0)

	.section	__TEXT,__cstring,cstring_literals
	.globl	SYM(test_ns_string[CRATE_ID]::EMPTY::ASCII, 0)
SYM(test_ns_string[CRATE_ID]::EMPTY::ASCII, 0):
	.space	1

	.globl	SYM(test_ns_string[CRATE_ID]::XYZ::ASCII, 0)
SYM(test_ns_string[CRATE_ID]::XYZ::ASCII, 0):
	.asciz	"xyz"

SYM(test_ns_string[CRATE_ID]::get_ascii::ASCII, 0):
	.asciz	"abc"

	.section	__DATA,__cfstring
	.globl	SYM(test_ns_string[CRATE_ID]::EMPTY::CFSTRING, 0)
	.p2align	3, 0x0
SYM(test_ns_string[CRATE_ID]::EMPTY::CFSTRING, 0):
	.quad	___CFConstantStringClassReference
	.asciz	"\310\007\000\000\000\000\000"
	.quad	SYM(test_ns_string[CRATE_ID]::EMPTY::ASCII, 0)
	.space	8

	.globl	SYM(test_ns_string[CRATE_ID]::XYZ::CFSTRING, 0)
	.p2align	3, 0x0
SYM(test_ns_string[CRATE_ID]::XYZ::CFSTRING, 0):
	.quad	___CFConstantStringClassReference
	.asciz	"\310\007\000\000\000\000\000"
	.quad	SYM(test_ns_string[CRATE_ID]::XYZ::ASCII, 0)
	.asciz	"\003\000\000\000\000\000\000"

	.section	__TEXT,__ustring
	.globl	SYM(test_ns_string[CRATE_ID]::NON_ASCII::UTF16, 0)
	.p2align	1, 0x0
SYM(test_ns_string[CRATE_ID]::NON_ASCII::UTF16, 0):
	.asciz	"=\330\000\336\000"

	.section	__DATA,__cfstring
	.globl	SYM(test_ns_string[CRATE_ID]::NON_ASCII::CFSTRING, 0)
	.p2align	3, 0x0
SYM(test_ns_string[CRATE_ID]::NON_ASCII::CFSTRING, 0):
	.quad	___CFConstantStringClassReference
	.asciz	"\320\007\000\000\000\000\000"
	.quad	SYM(test_ns_string[CRATE_ID]::NON_ASCII::UTF16, 0)
	.asciz	"\002\000\000\000\000\000\000"

	.p2align	3, 0x0
SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 0):
	.quad	___CFConstantStringClassReference
	.asciz	"\310\007\000\000\000\000\000"
	.quad	SYM(test_ns_string[CRATE_ID]::get_ascii::ASCII, 0)
	.asciz	"\003\000\000\000\000\000\000"

	.section	__TEXT,__ustring
	.p2align	1, 0x0
SYM(test_ns_string[CRATE_ID]::get_utf16::UTF16, 0):
	.asciz	"\341\000b\000\007\001\000"

	.section	__DATA,__cfstring
	.p2align	3, 0x0
SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 0):
	.quad	___CFConstantStringClassReference
	.asciz	"\320\007\000\000\000\000\000"
	.quad	SYM(test_ns_string[CRATE_ID]::get_utf16::UTF16, 0)
	.asciz	"\003\000\000\000\000\000\000"

	.section	__TEXT,__ustring
	.p2align	1, 0x0
SYM(test_ns_string[CRATE_ID]::get_with_nul::UTF16, 0):
	.asciz	"a\000\000\000b\000\000\000c\000\000\000\000"

	.section	__DATA,__cfstring
	.p2align	3, 0x0
SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 0):
	.quad	___CFConstantStringClassReference
	.asciz	"\320\007\000\000\000\000\000"
	.quad	SYM(test_ns_string[CRATE_ID]::get_with_nul::UTF16, 0)
	.asciz	"\006\000\000\000\000\000\000"

.subsections_via_symbols
