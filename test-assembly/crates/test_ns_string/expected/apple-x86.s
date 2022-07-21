	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_get_ascii
	.p2align	4, 0x90
_get_ascii:
	push	ebp
	mov	ebp, esp
	push	esi
	push	eax
	call	L0$pb
L0$pb:
	pop	esi
	sub	esp, 8
	lea	eax, [esi + l___unnamed_1-L0$pb]
	push	3
	push	eax
	call	SYM(objc2_foundation::__string_macro::is_ascii_no_nul::GENERATED_ID, 0)
	add	esp, 16
	lea	edx, [esi + SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 0)-L0$pb]
	lea	ecx, [esi + SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 1)-L0$pb]
	test	al, al
	cmovne	ecx, edx
	mov	eax, ecx
	add	esp, 4
	pop	esi
	pop	ebp
	ret

	.globl	_get_utf16
	.p2align	4, 0x90
_get_utf16:
	push	ebp
	mov	ebp, esp
	push	esi
	push	eax
	call	L1$pb
L1$pb:
	pop	esi
	sub	esp, 8
	lea	eax, [esi + l___unnamed_2-L1$pb]
	push	5
	push	eax
	call	SYM(objc2_foundation::__string_macro::is_ascii_no_nul::GENERATED_ID, 0)
	add	esp, 16
	lea	edx, [esi + SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 0)-L1$pb]
	lea	ecx, [esi + SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 1)-L1$pb]
	test	al, al
	cmovne	ecx, edx
	mov	eax, ecx
	add	esp, 4
	pop	esi
	pop	ebp
	ret

	.globl	_get_with_nul
	.p2align	4, 0x90
_get_with_nul:
	push	ebp
	mov	ebp, esp
	push	esi
	push	eax
	call	L2$pb
L2$pb:
	pop	esi
	sub	esp, 8
	lea	eax, [esi + l___unnamed_3-L2$pb]
	push	6
	push	eax
	call	SYM(objc2_foundation::__string_macro::is_ascii_no_nul::GENERATED_ID, 0)
	add	esp, 16
	lea	edx, [esi + SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 0)-L2$pb]
	lea	ecx, [esi + SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 1)-L2$pb]
	test	al, al
	cmovne	ecx, edx
	mov	eax, ecx
	add	esp, 4
	pop	esi
	pop	ebp
	ret

	.section	__DATA,__const
	.globl	_EMPTY
	.p2align	2
_EMPTY:
	.long	SYM(test_ns_string[CRATE_ID]::EMPTY::CFSTRING, 0)

	.globl	_XYZ
	.p2align	2
_XYZ:
	.long	SYM(test_ns_string[CRATE_ID]::XYZ::CFSTRING, 0)

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
	.p2align	2
SYM(test_ns_string[CRATE_ID]::EMPTY::CFSTRING, 0):
	.long	___CFConstantStringClassReference
	.asciz	"\310\007\000"
	.long	SYM(test_ns_string[CRATE_ID]::EMPTY::ASCII, 0)
	.space	4

	.section	__TEXT,__ustring
	.p2align	1
SYM(test_ns_string[CRATE_ID]::EMPTY::UTF16, 0):
	.space	2

	.section	__DATA,__cfstring
	.globl	SYM(test_ns_string[CRATE_ID]::EMPTY::CFSTRING, 1)
	.p2align	2
SYM(test_ns_string[CRATE_ID]::EMPTY::CFSTRING, 1):
	.long	___CFConstantStringClassReference
	.asciz	"\320\007\000"
	.long	SYM(test_ns_string[CRATE_ID]::EMPTY::UTF16, 0)
	.space	4

	.section	__TEXT,__cstring,cstring_literals
SYM(test_ns_string[CRATE_ID]::XYZ::ASCII, 0):
	.asciz	"xyz"

	.section	__DATA,__cfstring
	.globl	SYM(test_ns_string[CRATE_ID]::XYZ::CFSTRING, 0)
	.p2align	2
SYM(test_ns_string[CRATE_ID]::XYZ::CFSTRING, 0):
	.long	___CFConstantStringClassReference
	.asciz	"\310\007\000"
	.long	SYM(test_ns_string[CRATE_ID]::XYZ::ASCII, 0)
	.asciz	"\003\000\000"

	.section	__TEXT,__ustring
	.p2align	1
SYM(test_ns_string[CRATE_ID]::XYZ::UTF16, 0):
	.asciz	"x\000y\000z\000\000"

	.section	__DATA,__cfstring
	.globl	SYM(test_ns_string[CRATE_ID]::XYZ::CFSTRING, 1)
	.p2align	2
SYM(test_ns_string[CRATE_ID]::XYZ::CFSTRING, 1):
	.long	___CFConstantStringClassReference
	.asciz	"\320\007\000"
	.long	SYM(test_ns_string[CRATE_ID]::XYZ::UTF16, 0)
	.asciz	"\003\000\000"

	.section	__TEXT,__cstring,cstring_literals
SYM(test_ns_string[CRATE_ID]::get_ascii::ASCII, 0):
	.asciz	"abc"

	.section	__DATA,__cfstring
	.p2align	2
SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 0):
	.long	___CFConstantStringClassReference
	.asciz	"\310\007\000"
	.long	SYM(test_ns_string[CRATE_ID]::get_ascii::ASCII, 0)
	.asciz	"\003\000\000"

	.section	__TEXT,__ustring
	.p2align	1
SYM(test_ns_string[CRATE_ID]::get_ascii::UTF16, 0):
	.asciz	"a\000b\000c\000\000"

	.section	__DATA,__cfstring
	.p2align	2
SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 1):
	.long	___CFConstantStringClassReference
	.asciz	"\320\007\000"
	.long	SYM(test_ns_string[CRATE_ID]::get_ascii::UTF16, 0)
	.asciz	"\003\000\000"

	.section	__TEXT,__cstring,cstring_literals
SYM(test_ns_string[CRATE_ID]::get_utf16::ASCII, 0):
	.asciz	"\303\241b\304\207"

	.section	__DATA,__cfstring
	.p2align	2
SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 0):
	.long	___CFConstantStringClassReference
	.asciz	"\310\007\000"
	.long	SYM(test_ns_string[CRATE_ID]::get_utf16::ASCII, 0)
	.asciz	"\005\000\000"

	.section	__TEXT,__ustring
	.p2align	1
SYM(test_ns_string[CRATE_ID]::get_utf16::UTF16, 0):
	.asciz	"\341\000b\000\007\001\000"

	.section	__DATA,__cfstring
	.p2align	2
SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 1):
	.long	___CFConstantStringClassReference
	.asciz	"\320\007\000"
	.long	SYM(test_ns_string[CRATE_ID]::get_utf16::UTF16, 0)
	.asciz	"\003\000\000"

	.section	__TEXT,__cstring,cstring_literals
SYM(test_ns_string[CRATE_ID]::get_with_nul::ASCII, 0):
	.asciz	"a\000b\000c\000"

	.section	__DATA,__cfstring
	.p2align	2
SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 0):
	.long	___CFConstantStringClassReference
	.asciz	"\310\007\000"
	.long	SYM(test_ns_string[CRATE_ID]::get_with_nul::ASCII, 0)
	.asciz	"\006\000\000"

	.section	__TEXT,__ustring
	.p2align	1
SYM(test_ns_string[CRATE_ID]::get_with_nul::UTF16, 0):
	.asciz	"a\000\000\000b\000\000\000c\000\000\000\000"

	.section	__DATA,__cfstring
	.p2align	2
SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 1):
	.long	___CFConstantStringClassReference
	.asciz	"\320\007\000"
	.long	SYM(test_ns_string[CRATE_ID]::get_with_nul::UTF16, 0)
	.asciz	"\006\000\000"

.subsections_via_symbols
