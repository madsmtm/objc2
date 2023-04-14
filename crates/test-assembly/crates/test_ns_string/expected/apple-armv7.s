	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_get_ascii
	.p2align	2
	.code	32
_get_ascii:
	movw	r0, :lower16:(SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 0)-(LPC0_0+8))
	movt	r0, :upper16:(SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 0)-(LPC0_0+8))
LPC0_0:
	add	r0, pc, r0
	bx	lr

	.globl	_get_utf16
	.p2align	2
	.code	32
_get_utf16:
	movw	r0, :lower16:(SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 0)-(LPC1_0+8))
	movt	r0, :upper16:(SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 0)-(LPC1_0+8))
LPC1_0:
	add	r0, pc, r0
	bx	lr

	.globl	_get_with_nul
	.p2align	2
	.code	32
_get_with_nul:
	movw	r0, :lower16:(SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 0)-(LPC2_0+8))
	movt	r0, :upper16:(SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 0)-(LPC2_0+8))
LPC2_0:
	add	r0, pc, r0
	bx	lr

	.section	__DATA,__const
	.globl	_EMPTY
	.p2align	2, 0x0
_EMPTY:
	.long	SYM(test_ns_string[CRATE_ID]::EMPTY::CFSTRING, 0)

	.globl	_XYZ
	.p2align	2, 0x0
_XYZ:
	.long	SYM(test_ns_string[CRATE_ID]::XYZ::CFSTRING, 0)

	.section	__TEXT,__cstring,cstring_literals
SYM(test_ns_string[CRATE_ID]::EMPTY::ASCII, 0):
	.space	1

	.section	__DATA,__cfstring
	.globl	SYM(test_ns_string[CRATE_ID]::EMPTY::CFSTRING, 0)
	.p2align	2, 0x0
SYM(test_ns_string[CRATE_ID]::EMPTY::CFSTRING, 0):
	.long	___CFConstantStringClassReference
	.asciz	"\310\007\000"
	.long	SYM(test_ns_string[CRATE_ID]::EMPTY::ASCII, 0)
	.space	4

	.section	__TEXT,__cstring,cstring_literals
SYM(test_ns_string[CRATE_ID]::XYZ::ASCII, 0):
	.asciz	"xyz"

	.section	__DATA,__cfstring
	.globl	SYM(test_ns_string[CRATE_ID]::XYZ::CFSTRING, 0)
	.p2align	2, 0x0
SYM(test_ns_string[CRATE_ID]::XYZ::CFSTRING, 0):
	.long	___CFConstantStringClassReference
	.asciz	"\310\007\000"
	.long	SYM(test_ns_string[CRATE_ID]::XYZ::ASCII, 0)
	.asciz	"\003\000\000"

	.section	__TEXT,__cstring,cstring_literals
SYM(test_ns_string[CRATE_ID]::get_ascii::ASCII, 0):
	.asciz	"abc"

	.section	__DATA,__cfstring
	.p2align	2, 0x0
SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 0):
	.long	___CFConstantStringClassReference
	.asciz	"\310\007\000"
	.long	SYM(test_ns_string[CRATE_ID]::get_ascii::ASCII, 0)
	.asciz	"\003\000\000"

	.section	__TEXT,__ustring
	.p2align	1, 0x0
SYM(test_ns_string[CRATE_ID]::get_utf16::UTF16, 0):
	.asciz	"\341\000b\000\007\001\000"

	.section	__DATA,__cfstring
	.p2align	2, 0x0
SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 0):
	.long	___CFConstantStringClassReference
	.asciz	"\320\007\000"
	.long	SYM(test_ns_string[CRATE_ID]::get_utf16::UTF16, 0)
	.asciz	"\003\000\000"

	.section	__TEXT,__ustring
	.p2align	1, 0x0
SYM(test_ns_string[CRATE_ID]::get_with_nul::UTF16, 0):
	.asciz	"a\000\000\000b\000\000\000c\000\000\000\000"

	.section	__DATA,__cfstring
	.p2align	2, 0x0
SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 0):
	.long	___CFConstantStringClassReference
	.asciz	"\320\007\000"
	.long	SYM(test_ns_string[CRATE_ID]::get_with_nul::UTF16, 0)
	.asciz	"\006\000\000"

.subsections_via_symbols
