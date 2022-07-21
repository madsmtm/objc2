	.section	__TEXT,__text,regular,pure_instructions
	.globl	_get_ascii
	.p2align	2
_get_ascii:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
Lloh0:
	adrp	x0, l___unnamed_1@PAGE
Lloh1:
	add	x0, x0, l___unnamed_1@PAGEOFF
	mov	w1, #3
	bl	SYM(objc2_foundation::__string_macro::is_ascii_no_nul::GENERATED_ID, 0)
Lloh2:
	adrp	x8, SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 0)@PAGE
Lloh3:
	add	x8, x8, SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 0)@PAGEOFF
Lloh4:
	adrp	x9, SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 1)@PAGE
Lloh5:
	add	x9, x9, SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 1)@PAGEOFF
	cmp	w0, #0
	csel	x0, x9, x8, ne
	ldp	x29, x30, [sp], #16
	ret
	.loh AdrpAdd	Lloh4, Lloh5
	.loh AdrpAdd	Lloh2, Lloh3
	.loh AdrpAdd	Lloh0, Lloh1

	.globl	_get_utf16
	.p2align	2
_get_utf16:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
Lloh6:
	adrp	x0, l___unnamed_2@PAGE
Lloh7:
	add	x0, x0, l___unnamed_2@PAGEOFF
	mov	w1, #5
	bl	SYM(objc2_foundation::__string_macro::is_ascii_no_nul::GENERATED_ID, 0)
Lloh8:
	adrp	x8, SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 0)@PAGE
Lloh9:
	add	x8, x8, SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 0)@PAGEOFF
Lloh10:
	adrp	x9, SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 1)@PAGE
Lloh11:
	add	x9, x9, SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 1)@PAGEOFF
	cmp	w0, #0
	csel	x0, x9, x8, ne
	ldp	x29, x30, [sp], #16
	ret
	.loh AdrpAdd	Lloh10, Lloh11
	.loh AdrpAdd	Lloh8, Lloh9
	.loh AdrpAdd	Lloh6, Lloh7

	.globl	_get_with_nul
	.p2align	2
_get_with_nul:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
Lloh12:
	adrp	x0, l___unnamed_3@PAGE
Lloh13:
	add	x0, x0, l___unnamed_3@PAGEOFF
	mov	w1, #6
	bl	SYM(objc2_foundation::__string_macro::is_ascii_no_nul::GENERATED_ID, 0)
Lloh14:
	adrp	x8, SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 0)@PAGE
Lloh15:
	add	x8, x8, SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 0)@PAGEOFF
Lloh16:
	adrp	x9, SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 1)@PAGE
Lloh17:
	add	x9, x9, SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 1)@PAGEOFF
	cmp	w0, #0
	csel	x0, x9, x8, ne
	ldp	x29, x30, [sp], #16
	ret
	.loh AdrpAdd	Lloh16, Lloh17
	.loh AdrpAdd	Lloh14, Lloh15
	.loh AdrpAdd	Lloh12, Lloh13

	.section	__DATA,__const
	.globl	_EMPTY
	.p2align	3
_EMPTY:
	.quad	SYM(test_ns_string[CRATE_ID]::EMPTY::CFSTRING, 0)

	.globl	_XYZ
	.p2align	3
_XYZ:
	.quad	SYM(test_ns_string[CRATE_ID]::XYZ::CFSTRING, 0)

	.section	__TEXT,__const
l___unnamed_1:
	.ascii	"abc"

l___unnamed_2:
	.ascii	"\303\241b\304\207"

l___unnamed_3:
	.asciz	"a\000b\000c"

	.section	__TEXT,__cstring,cstring_literals
SYM(test_ns_string[CRATE_ID]::EMPTY::ASCII, 0):
	.space	1

	.section	__DATA,__cfstring
	.globl	SYM(test_ns_string[CRATE_ID]::EMPTY::CFSTRING, 0)
	.p2align	3
SYM(test_ns_string[CRATE_ID]::EMPTY::CFSTRING, 0):
	.quad	___CFConstantStringClassReference
	.asciz	"\310\007\000\000\000\000\000"
	.quad	SYM(test_ns_string[CRATE_ID]::EMPTY::ASCII, 0)
	.space	8

	.section	__TEXT,__ustring
	.p2align	1
SYM(test_ns_string[CRATE_ID]::EMPTY::UTF16, 0):
	.space	2

	.section	__DATA,__cfstring
	.globl	SYM(test_ns_string[CRATE_ID]::EMPTY::CFSTRING, 1)
	.p2align	3
SYM(test_ns_string[CRATE_ID]::EMPTY::CFSTRING, 1):
	.quad	___CFConstantStringClassReference
	.asciz	"\320\007\000\000\000\000\000"
	.quad	SYM(test_ns_string[CRATE_ID]::EMPTY::UTF16, 0)
	.space	8

	.section	__TEXT,__cstring,cstring_literals
SYM(test_ns_string[CRATE_ID]::XYZ::ASCII, 0):
	.asciz	"xyz"

	.section	__DATA,__cfstring
	.globl	SYM(test_ns_string[CRATE_ID]::XYZ::CFSTRING, 0)
	.p2align	3
SYM(test_ns_string[CRATE_ID]::XYZ::CFSTRING, 0):
	.quad	___CFConstantStringClassReference
	.asciz	"\310\007\000\000\000\000\000"
	.quad	SYM(test_ns_string[CRATE_ID]::XYZ::ASCII, 0)
	.asciz	"\003\000\000\000\000\000\000"

	.section	__TEXT,__ustring
	.p2align	1
SYM(test_ns_string[CRATE_ID]::XYZ::UTF16, 0):
	.asciz	"x\000y\000z\000\000"

	.section	__DATA,__cfstring
	.globl	SYM(test_ns_string[CRATE_ID]::XYZ::CFSTRING, 1)
	.p2align	3
SYM(test_ns_string[CRATE_ID]::XYZ::CFSTRING, 1):
	.quad	___CFConstantStringClassReference
	.asciz	"\320\007\000\000\000\000\000"
	.quad	SYM(test_ns_string[CRATE_ID]::XYZ::UTF16, 0)
	.asciz	"\003\000\000\000\000\000\000"

	.section	__TEXT,__cstring,cstring_literals
SYM(test_ns_string[CRATE_ID]::get_ascii::ASCII, 0):
	.asciz	"abc"

	.section	__DATA,__cfstring
	.p2align	3
SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 1):
	.quad	___CFConstantStringClassReference
	.asciz	"\310\007\000\000\000\000\000"
	.quad	SYM(test_ns_string[CRATE_ID]::get_ascii::ASCII, 0)
	.asciz	"\003\000\000\000\000\000\000"

	.section	__TEXT,__ustring
	.p2align	1
SYM(test_ns_string[CRATE_ID]::get_ascii::UTF16, 0):
	.asciz	"a\000b\000c\000\000"

	.section	__DATA,__cfstring
	.p2align	3
SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 0):
	.quad	___CFConstantStringClassReference
	.asciz	"\320\007\000\000\000\000\000"
	.quad	SYM(test_ns_string[CRATE_ID]::get_ascii::UTF16, 0)
	.asciz	"\003\000\000\000\000\000\000"

	.section	__TEXT,__cstring,cstring_literals
SYM(test_ns_string[CRATE_ID]::get_utf16::ASCII, 0):
	.asciz	"\303\241b\304\207"

	.section	__DATA,__cfstring
	.p2align	3
SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 1):
	.quad	___CFConstantStringClassReference
	.asciz	"\310\007\000\000\000\000\000"
	.quad	SYM(test_ns_string[CRATE_ID]::get_utf16::ASCII, 0)
	.asciz	"\005\000\000\000\000\000\000"

	.section	__TEXT,__ustring
	.p2align	1
SYM(test_ns_string[CRATE_ID]::get_utf16::UTF16, 0):
	.asciz	"\341\000b\000\007\001\000"

	.section	__DATA,__cfstring
	.p2align	3
SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 0):
	.quad	___CFConstantStringClassReference
	.asciz	"\320\007\000\000\000\000\000"
	.quad	SYM(test_ns_string[CRATE_ID]::get_utf16::UTF16, 0)
	.asciz	"\003\000\000\000\000\000\000"

	.section	__TEXT,__cstring,cstring_literals
SYM(test_ns_string[CRATE_ID]::get_with_nul::ASCII, 0):
	.asciz	"a\000b\000c\000"

	.section	__DATA,__cfstring
	.p2align	3
SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 1):
	.quad	___CFConstantStringClassReference
	.asciz	"\310\007\000\000\000\000\000"
	.quad	SYM(test_ns_string[CRATE_ID]::get_with_nul::ASCII, 0)
	.asciz	"\006\000\000\000\000\000\000"

	.section	__TEXT,__ustring
	.p2align	1
SYM(test_ns_string[CRATE_ID]::get_with_nul::UTF16, 0):
	.asciz	"a\000\000\000b\000\000\000c\000\000\000\000"

	.section	__DATA,__cfstring
	.p2align	3
SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 0):
	.quad	___CFConstantStringClassReference
	.asciz	"\320\007\000\000\000\000\000"
	.quad	SYM(test_ns_string[CRATE_ID]::get_with_nul::UTF16, 0)
	.asciz	"\006\000\000\000\000\000\000"

.subsections_via_symbols
