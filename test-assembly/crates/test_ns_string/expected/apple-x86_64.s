	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_get_ascii
	.p2align	4, 0x90
_get_ascii:
	push	rbp
	mov	rbp, rsp
	lea	rdi, [rip + l___unnamed_1]
	mov	esi, 3
	call	SYM(objc2_foundation::__string_macro::is_ascii_no_nul::GENERATED_ID, 0)
	lea	rdx, [rip + SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 0)]
	lea	rcx, [rip + SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 1)]
	test	al, al
	cmovne	rcx, rdx
	mov	rax, rcx
	pop	rbp
	ret

	.globl	_get_utf16
	.p2align	4, 0x90
_get_utf16:
	push	rbp
	mov	rbp, rsp
	lea	rdi, [rip + l___unnamed_2]
	mov	esi, 5
	call	SYM(objc2_foundation::__string_macro::is_ascii_no_nul::GENERATED_ID, 0)
	lea	rdx, [rip + SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 0)]
	lea	rcx, [rip + SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 1)]
	test	al, al
	cmovne	rcx, rdx
	mov	rax, rcx
	pop	rbp
	ret

	.globl	_get_with_nul
	.p2align	4, 0x90
_get_with_nul:
	push	rbp
	mov	rbp, rsp
	lea	rdi, [rip + l___unnamed_3]
	mov	esi, 6
	call	SYM(objc2_foundation::__string_macro::is_ascii_no_nul::GENERATED_ID, 0)
	lea	rdx, [rip + SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 0)]
	lea	rcx, [rip + SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 1)]
	test	al, al
	cmovne	rcx, rdx
	mov	rax, rcx
	pop	rbp
	ret

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
SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 0):
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
SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 1):
	.quad	___CFConstantStringClassReference
	.asciz	"\320\007\000\000\000\000\000"
	.quad	SYM(test_ns_string[CRATE_ID]::get_ascii::UTF16, 0)
	.asciz	"\003\000\000\000\000\000\000"

	.section	__TEXT,__cstring,cstring_literals
SYM(test_ns_string[CRATE_ID]::get_utf16::ASCII, 0):
	.asciz	"\303\241b\304\207"

	.section	__DATA,__cfstring
	.p2align	3
SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 0):
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
SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 1):
	.quad	___CFConstantStringClassReference
	.asciz	"\320\007\000\000\000\000\000"
	.quad	SYM(test_ns_string[CRATE_ID]::get_utf16::UTF16, 0)
	.asciz	"\003\000\000\000\000\000\000"

	.section	__TEXT,__cstring,cstring_literals
SYM(test_ns_string[CRATE_ID]::get_with_nul::ASCII, 0):
	.asciz	"a\000b\000c\000"

	.section	__DATA,__cfstring
	.p2align	3
SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 0):
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
SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 1):
	.quad	___CFConstantStringClassReference
	.asciz	"\320\007\000\000\000\000\000"
	.quad	SYM(test_ns_string[CRATE_ID]::get_with_nul::UTF16, 0)
	.asciz	"\006\000\000\000\000\000\000"

.subsections_via_symbols
