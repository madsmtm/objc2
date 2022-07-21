	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_get_ascii
	.p2align	4, 0x90
_get_ascii:
	push	rbp
	mov	rbp, rsp
	lea	rdi, [rip + l___unnamed_1]
	mov	esi, 3
	call	SYM(objc2_foundation::__string_macro::is_ascii::GENERATED_ID, 0)
	test	al, al
	je	LBB0_1
	lea	rdi, [rip + SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 0)]
	pop	rbp
	jmp	SYM(objc2_foundation::__string_macro::CFStringAscii::as_nsstring::GENERATED_ID, 0)
LBB0_1:
	lea	rdi, [rip + SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 1)]
	pop	rbp
	jmp	SYM(objc2_foundation::__string_macro::CFStringUtf16::as_nsstring::GENERATED_ID, 0)

	.globl	_get_utf16
	.p2align	4, 0x90
_get_utf16:
	push	rbp
	mov	rbp, rsp
	lea	rdi, [rip + l___unnamed_2]
	mov	esi, 5
	call	SYM(objc2_foundation::__string_macro::is_ascii::GENERATED_ID, 0)
	test	al, al
	je	LBB1_1
	lea	rdi, [rip + SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 0)]
	pop	rbp
	jmp	SYM(objc2_foundation::__string_macro::CFStringAscii::as_nsstring::GENERATED_ID, 0)
LBB1_1:
	lea	rdi, [rip + SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 1)]
	pop	rbp
	jmp	SYM(objc2_foundation::__string_macro::CFStringUtf16::as_nsstring::GENERATED_ID, 0)

	.globl	_get_with_nul
	.p2align	4, 0x90
_get_with_nul:
	push	rbp
	mov	rbp, rsp
	lea	rdi, [rip + l___unnamed_3]
	mov	esi, 6
	call	SYM(objc2_foundation::__string_macro::is_ascii::GENERATED_ID, 0)
	test	al, al
	je	LBB2_1
	lea	rdi, [rip + SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 0)]
	pop	rbp
	jmp	SYM(objc2_foundation::__string_macro::CFStringAscii::as_nsstring::GENERATED_ID, 0)
LBB2_1:
	lea	rdi, [rip + SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 1)]
	pop	rbp
	jmp	SYM(objc2_foundation::__string_macro::CFStringUtf16::as_nsstring::GENERATED_ID, 0)

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

l___unnamed_4:
	.space	1

	.section	__DATA,__cfstring
	.globl	SYM(test_ns_string[CRATE_ID]::EMPTY::CFSTRING, 0)
	.p2align	3
SYM(test_ns_string[CRATE_ID]::EMPTY::CFSTRING, 0):
	.quad	___CFConstantStringClassReference
	.asciz	"\310\007\000\000\000\000\000"
	.quad	l___unnamed_4
	.space	8

	.section	__TEXT,__const
	.p2align	1
l___unnamed_5:
	.space	2

	.section	__DATA,__cfstring
	.globl	SYM(test_ns_string[CRATE_ID]::EMPTY::CFSTRING, 1)
	.p2align	3
SYM(test_ns_string[CRATE_ID]::EMPTY::CFSTRING, 1):
	.quad	___CFConstantStringClassReference
	.asciz	"\320\007\000\000\000\000\000"
	.quad	l___unnamed_5
	.space	8

	.section	__TEXT,__literal4,4byte_literals
L___unnamed_6:
	.asciz	"xyz"

	.section	__DATA,__cfstring
	.globl	SYM(test_ns_string[CRATE_ID]::XYZ::CFSTRING, 0)
	.p2align	3
SYM(test_ns_string[CRATE_ID]::XYZ::CFSTRING, 0):
	.quad	___CFConstantStringClassReference
	.asciz	"\310\007\000\000\000\000\000"
	.quad	L___unnamed_6
	.asciz	"\003\000\000\000\000\000\000"

	.section	__TEXT,__literal8,8byte_literals
	.p2align	1
L___unnamed_7:
	.asciz	"x\000y\000z\000\000"

	.section	__DATA,__cfstring
	.globl	SYM(test_ns_string[CRATE_ID]::XYZ::CFSTRING, 1)
	.p2align	3
SYM(test_ns_string[CRATE_ID]::XYZ::CFSTRING, 1):
	.quad	___CFConstantStringClassReference
	.asciz	"\320\007\000\000\000\000\000"
	.quad	L___unnamed_7
	.asciz	"\003\000\000\000\000\000\000"

	.section	__TEXT,__literal4,4byte_literals
L___unnamed_8:
	.asciz	"abc"

	.section	__DATA,__cfstring
	.p2align	3
SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 0):
	.quad	___CFConstantStringClassReference
	.asciz	"\310\007\000\000\000\000\000"
	.quad	L___unnamed_8
	.asciz	"\003\000\000\000\000\000\000"

	.section	__TEXT,__literal8,8byte_literals
	.p2align	1
L___unnamed_9:
	.asciz	"a\000b\000c\000\000"

	.section	__DATA,__cfstring
	.p2align	3
SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 1):
	.quad	___CFConstantStringClassReference
	.asciz	"\320\007\000\000\000\000\000"
	.quad	L___unnamed_9
	.asciz	"\003\000\000\000\000\000\000"

	.section	__TEXT,__const
l___unnamed_10:
	.asciz	"\303\241b\304\207"

	.section	__DATA,__cfstring
	.p2align	3
SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 0):
	.quad	___CFConstantStringClassReference
	.asciz	"\310\007\000\000\000\000\000"
	.quad	l___unnamed_10
	.asciz	"\005\000\000\000\000\000\000"

	.section	__TEXT,__literal8,8byte_literals
	.p2align	1
L___unnamed_11:
	.asciz	"\341\000b\000\007\001\000"

	.section	__DATA,__cfstring
	.p2align	3
SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 1):
	.quad	___CFConstantStringClassReference
	.asciz	"\320\007\000\000\000\000\000"
	.quad	L___unnamed_11
	.asciz	"\003\000\000\000\000\000\000"

	.section	__TEXT,__const
l___unnamed_12:
	.asciz	"a\000b\000c\000"

	.section	__DATA,__cfstring
	.p2align	3
SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 0):
	.quad	___CFConstantStringClassReference
	.asciz	"\310\007\000\000\000\000\000"
	.quad	l___unnamed_12
	.asciz	"\006\000\000\000\000\000\000"

	.section	__TEXT,__const
	.p2align	1
l___unnamed_13:
	.asciz	"a\000\000\000b\000\000\000c\000\000\000\000"

	.section	__DATA,__cfstring
	.p2align	3
SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 1):
	.quad	___CFConstantStringClassReference
	.asciz	"\320\007\000\000\000\000\000"
	.quad	l___unnamed_13
	.asciz	"\006\000\000\000\000\000\000"

.subsections_via_symbols
