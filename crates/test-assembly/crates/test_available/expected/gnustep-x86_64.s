	.intel_syntax noprefix
	.section	.text.fn1_always,"ax",@progbits
	.globl	fn1_always
	.p2align	4
	.type	fn1_always,@function
fn1_always:
	mov	al, 1
	ret
.Lfunc_end0:
	.size	fn1_always, .Lfunc_end0-fn1_always

	.section	.text.fn2_never,"ax",@progbits
	.globl	fn2_never
	.p2align	4
	.type	fn2_never,@function
fn2_never:
	xor	eax, eax
	ret
.Lfunc_end1:
	.size	fn2_never, .Lfunc_end1-fn2_never

	.section	.text.fn3_low,"ax",@progbits
	.globl	fn3_low
	.p2align	4
	.type	fn3_low,@function
fn3_low:
	mov	al, 1
	ret
.Lfunc_end2:
	.size	fn3_low, .Lfunc_end2-fn3_low

	.section	.text.fn4_high,"ax",@progbits
	.globl	fn4_high
	.p2align	4
	.type	fn4_high,@function
fn4_high:
	mov	al, 1
	ret
.Lfunc_end3:
	.size	fn4_high, .Lfunc_end3-fn4_high

	.section	.text.fn5_only_ios,"ax",@progbits
	.globl	fn5_only_ios
	.p2align	4
	.type	fn5_only_ios,@function
fn5_only_ios:
	xor	eax, eax
	ret
.Lfunc_end4:
	.size	fn5_only_ios, .Lfunc_end4-fn5_only_ios

	.section	.text.fn6_two_checks,"ax",@progbits
	.globl	fn6_two_checks
	.p2align	4
	.type	fn6_two_checks,@function
fn6_two_checks:
	mov	al, 1
	ret
.Lfunc_end5:
	.size	fn6_two_checks, .Lfunc_end5-fn6_two_checks

	.section	".note.GNU-stack","",@progbits
