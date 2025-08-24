	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_fn1_get_ascii
	.p2align	4
_fn1_get_ascii:
	push	rbp
	mov	rbp, rsp
	lea	rax, [rip + SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 0)]
	pop	rbp
	ret

	.globl	_fn2_get_utf16
	.p2align	4
_fn2_get_utf16:
	push	rbp
	mov	rbp, rsp
	lea	rax, [rip + SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 0)]
	pop	rbp
	ret

	.globl	_fn3_get_with_nul
	.p2align	4
_fn3_get_with_nul:
	push	rbp
	mov	rbp, rsp
	lea	rax, [rip + SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 0)]
	pop	rbp
	ret

	.section	__DATA,__const
	.globl	_S1_EMPTY
	.p2align	3, 0x0
_S1_EMPTY:
	.quad	SYM(test_ns_string[CRATE_ID]::EMPTY::CFSTRING, 0)

	.globl	_S2_XYZ
	.p2align	3, 0x0
_S2_XYZ:
	.quad	SYM(test_ns_string[CRATE_ID]::XYZ::CFSTRING, 0)

	.globl	_S3_NON_ASCII
	.p2align	3, 0x0
_S3_NON_ASCII:
	.quad	SYM(test_ns_string[CRATE_ID]::NON_ASCII::CFSTRING, 0)

	.section	__TEXT,__cstring,cstring_literals
	.globl	SYM(test_ns_string[CRATE_ID]::XYZ::ASCII, 0)
SYM(test_ns_string[CRATE_ID]::XYZ::ASCII, 0):
	.asciz	"xyz"

	.globl	SYM(test_ns_string[CRATE_ID]::EMPTY::ASCII, 0)
SYM(test_ns_string[CRATE_ID]::EMPTY::ASCII, 0):
	.space	1

SYM(test_ns_string[CRATE_ID]::get_ascii::ASCII, 0):
	.asciz	"abc"

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

	.globl	SYM(test_ns_string[CRATE_ID]::XYZ::CFSTRING, 0)
	.p2align	3, 0x0
SYM(test_ns_string[CRATE_ID]::XYZ::CFSTRING, 0):
	.quad	___CFConstantStringClassReference
	.asciz	"\310\007\000\000\000\000\000"
	.quad	SYM(test_ns_string[CRATE_ID]::XYZ::ASCII, 0)
	.asciz	"\003\000\000\000\000\000\000"

	.globl	SYM(test_ns_string[CRATE_ID]::EMPTY::CFSTRING, 0)
	.p2align	3, 0x0
SYM(test_ns_string[CRATE_ID]::EMPTY::CFSTRING, 0):
	.quad	___CFConstantStringClassReference
	.asciz	"\310\007\000\000\000\000\000"
	.quad	SYM(test_ns_string[CRATE_ID]::EMPTY::ASCII, 0)
	.space	8

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

.subsections_via_symbols
