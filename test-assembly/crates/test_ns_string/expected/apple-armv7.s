	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_get_ascii
	.p2align	2
	.code	32
_get_ascii:
	push	{r7, lr}
	mov	r7, sp
	movw	r0, :lower16:(l___unnamed_1-(LPC0_0+8))
	mov	r1, #3
	movt	r0, :upper16:(l___unnamed_1-(LPC0_0+8))
LPC0_0:
	add	r0, pc, r0
	bl	SYM(objc2_foundation::__string_macro::is_ascii::GENERATED_ID, 0)
	cmp	r0, #0
	beq	LBB0_2
	movw	r0, :lower16:(SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 0)-(LPC0_2+8))
	movt	r0, :upper16:(SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 0)-(LPC0_2+8))
LPC0_2:
	add	r0, pc, r0
	pop	{r7, lr}
	b	SYM(objc2_foundation::__string_macro::CFStringAscii::as_nsstring::GENERATED_ID, 0)
LBB0_2:
	movw	r0, :lower16:(SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 1)-(LPC0_1+8))
	movt	r0, :upper16:(SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 1)-(LPC0_1+8))
LPC0_1:
	add	r0, pc, r0
	pop	{r7, lr}
	b	SYM(objc2_foundation::__string_macro::CFStringUtf16::as_nsstring::GENERATED_ID, 0)

	.globl	_get_utf16
	.p2align	2
	.code	32
_get_utf16:
	push	{r7, lr}
	mov	r7, sp
	movw	r0, :lower16:(l___unnamed_2-(LPC1_0+8))
	mov	r1, #5
	movt	r0, :upper16:(l___unnamed_2-(LPC1_0+8))
LPC1_0:
	add	r0, pc, r0
	bl	SYM(objc2_foundation::__string_macro::is_ascii::GENERATED_ID, 0)
	cmp	r0, #0
	beq	LBB1_2
	movw	r0, :lower16:(SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 0)-(LPC1_2+8))
	movt	r0, :upper16:(SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 0)-(LPC1_2+8))
LPC1_2:
	add	r0, pc, r0
	pop	{r7, lr}
	b	SYM(objc2_foundation::__string_macro::CFStringAscii::as_nsstring::GENERATED_ID, 0)
LBB1_2:
	movw	r0, :lower16:(SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 1)-(LPC1_1+8))
	movt	r0, :upper16:(SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 1)-(LPC1_1+8))
LPC1_1:
	add	r0, pc, r0
	pop	{r7, lr}
	b	SYM(objc2_foundation::__string_macro::CFStringUtf16::as_nsstring::GENERATED_ID, 0)

	.globl	_get_with_nul
	.p2align	2
	.code	32
_get_with_nul:
	push	{r7, lr}
	mov	r7, sp
	movw	r0, :lower16:(l___unnamed_3-(LPC2_0+8))
	mov	r1, #6
	movt	r0, :upper16:(l___unnamed_3-(LPC2_0+8))
LPC2_0:
	add	r0, pc, r0
	bl	SYM(objc2_foundation::__string_macro::is_ascii::GENERATED_ID, 0)
	cmp	r0, #0
	beq	LBB2_2
	movw	r0, :lower16:(SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 0)-(LPC2_2+8))
	movt	r0, :upper16:(SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 0)-(LPC2_2+8))
LPC2_2:
	add	r0, pc, r0
	pop	{r7, lr}
	b	SYM(objc2_foundation::__string_macro::CFStringAscii::as_nsstring::GENERATED_ID, 0)
LBB2_2:
	movw	r0, :lower16:(SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 1)-(LPC2_1+8))
	movt	r0, :upper16:(SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 1)-(LPC2_1+8))
LPC2_1:
	add	r0, pc, r0
	pop	{r7, lr}
	b	SYM(objc2_foundation::__string_macro::CFStringUtf16::as_nsstring::GENERATED_ID, 0)

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

l___unnamed_4:
	.space	1

	.section	__DATA,__cfstring
	.globl	SYM(test_ns_string[CRATE_ID]::EMPTY::CFSTRING, 0)
	.p2align	2
SYM(test_ns_string[CRATE_ID]::EMPTY::CFSTRING, 0):
	.long	___CFConstantStringClassReference
	.asciz	"\310\007\000"
	.long	l___unnamed_4
	.space	4

	.section	__TEXT,__const
	.p2align	1
l___unnamed_5:
	.space	2

	.section	__DATA,__cfstring
	.globl	SYM(test_ns_string[CRATE_ID]::EMPTY::CFSTRING, 1)
	.p2align	2
SYM(test_ns_string[CRATE_ID]::EMPTY::CFSTRING, 1):
	.long	___CFConstantStringClassReference
	.asciz	"\320\007\000"
	.long	l___unnamed_5
	.space	4

	.section	__TEXT,__literal4,4byte_literals
L___unnamed_6:
	.asciz	"xyz"

	.section	__DATA,__cfstring
	.globl	SYM(test_ns_string[CRATE_ID]::XYZ::CFSTRING, 0)
	.p2align	2
SYM(test_ns_string[CRATE_ID]::XYZ::CFSTRING, 0):
	.long	___CFConstantStringClassReference
	.asciz	"\310\007\000"
	.long	L___unnamed_6
	.asciz	"\003\000\000"

	.section	__TEXT,__literal8,8byte_literals
	.p2align	1
L___unnamed_7:
	.asciz	"x\000y\000z\000\000"

	.section	__DATA,__cfstring
	.globl	SYM(test_ns_string[CRATE_ID]::XYZ::CFSTRING, 1)
	.p2align	2
SYM(test_ns_string[CRATE_ID]::XYZ::CFSTRING, 1):
	.long	___CFConstantStringClassReference
	.asciz	"\320\007\000"
	.long	L___unnamed_7
	.asciz	"\003\000\000"

	.section	__TEXT,__literal4,4byte_literals
L___unnamed_8:
	.asciz	"abc"

	.section	__DATA,__cfstring
	.p2align	2
SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 0):
	.long	___CFConstantStringClassReference
	.asciz	"\310\007\000"
	.long	L___unnamed_8
	.asciz	"\003\000\000"

	.section	__TEXT,__literal8,8byte_literals
	.p2align	1
L___unnamed_9:
	.asciz	"a\000b\000c\000\000"

	.section	__DATA,__cfstring
	.p2align	2
SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 1):
	.long	___CFConstantStringClassReference
	.asciz	"\320\007\000"
	.long	L___unnamed_9
	.asciz	"\003\000\000"

	.section	__TEXT,__const
l___unnamed_10:
	.asciz	"\303\241b\304\207"

	.section	__DATA,__cfstring
	.p2align	2
SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 0):
	.long	___CFConstantStringClassReference
	.asciz	"\310\007\000"
	.long	l___unnamed_10
	.asciz	"\005\000\000"

	.section	__TEXT,__literal8,8byte_literals
	.p2align	1
L___unnamed_11:
	.asciz	"\341\000b\000\007\001\000"

	.section	__DATA,__cfstring
	.p2align	2
SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 1):
	.long	___CFConstantStringClassReference
	.asciz	"\320\007\000"
	.long	L___unnamed_11
	.asciz	"\003\000\000"

	.section	__TEXT,__const
l___unnamed_12:
	.asciz	"a\000b\000c\000"

	.section	__DATA,__cfstring
	.p2align	2
SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 0):
	.long	___CFConstantStringClassReference
	.asciz	"\310\007\000"
	.long	l___unnamed_12
	.asciz	"\006\000\000"

	.section	__TEXT,__const
	.p2align	1
l___unnamed_13:
	.asciz	"a\000\000\000b\000\000\000c\000\000\000\000"

	.section	__DATA,__cfstring
	.p2align	2
SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 1):
	.long	___CFConstantStringClassReference
	.asciz	"\320\007\000"
	.long	l___unnamed_13
	.asciz	"\006\000\000"

.subsections_via_symbols
