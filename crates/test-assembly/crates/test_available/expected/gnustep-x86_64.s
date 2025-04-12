	.intel_syntax noprefix
	.section	.text.always,"ax",@progbits
	.globl	always
	.p2align	4
	.type	always,@function
always:
	mov	al, 1
	ret
.Lfunc_end0:
	.size	always, .Lfunc_end0-always

	.section	.text.never,"ax",@progbits
	.globl	never
	.p2align	4
	.type	never,@function
never:
	xor	eax, eax
	ret
.Lfunc_end1:
	.size	never, .Lfunc_end1-never

	.section	.text.low,"ax",@progbits
	.globl	low
	.p2align	4
	.type	low,@function
low:
	mov	al, 1
	ret
.Lfunc_end2:
	.size	low, .Lfunc_end2-low

	.section	.text.high,"ax",@progbits
	.globl	high
	.p2align	4
	.type	high,@function
high:
	mov	al, 1
	ret
.Lfunc_end3:
	.size	high, .Lfunc_end3-high

	.section	.text.only_ios,"ax",@progbits
	.globl	only_ios
	.p2align	4
	.type	only_ios,@function
only_ios:
	xor	eax, eax
	ret
.Lfunc_end4:
	.size	only_ios, .Lfunc_end4-only_ios

	.section	.text.two_checks,"ax",@progbits
	.globl	two_checks
	.p2align	4
	.type	two_checks,@function
two_checks:
	mov	al, 1
	ret
.Lfunc_end5:
	.size	two_checks, .Lfunc_end5-two_checks

	.section	".note.GNU-stack","",@progbits
