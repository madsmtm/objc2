	.intel_syntax noprefix
	.section	.text.fn1_get_class,"ax",@progbits
	.globl	fn1_get_class
	.p2align	4
	.type	fn1_get_class,@function
fn1_get_class:
	mov	rax, qword ptr [rip + ._OBJC_CLASS_NSObject@GOTPCREL]
	ret
.Lfunc_end0:
	.size	fn1_get_class, .Lfunc_end0-fn1_get_class

	.section	.text.fn1_get_same_class,"ax",@progbits
	.globl	fn1_get_same_class
	.p2align	4
	.type	fn1_get_same_class,@function
fn1_get_same_class:
	mov	rax, qword ptr [rip + ._OBJC_CLASS_NSObject@GOTPCREL]
	ret
.Lfunc_end1:
	.size	fn1_get_same_class, .Lfunc_end1-fn1_get_same_class

	.section	.text.fn3_get_different_class,"ax",@progbits
	.globl	fn3_get_different_class
	.p2align	4
	.type	fn3_get_different_class,@function
fn3_get_different_class:
	mov	rax, qword ptr [rip + ._OBJC_CLASS_NSString@GOTPCREL]
	ret
.Lfunc_end2:
	.size	fn3_get_different_class, .Lfunc_end2-fn3_get_different_class

	.section	.text.fn4_unused_class,"ax",@progbits
	.globl	fn4_unused_class
	.p2align	4
	.type	fn4_unused_class,@function
fn4_unused_class:
	ret
.Lfunc_end3:
	.size	fn4_unused_class, .Lfunc_end3-fn4_unused_class

	.section	.text.fn5_use_fns,"ax",@progbits
	.globl	fn5_use_fns
	.p2align	4
	.type	fn5_use_fns,@function
fn5_use_fns:
	mov	rax, rdi
	mov	rcx, qword ptr [rip + ._OBJC_CLASS_NSObject@GOTPCREL]
	mov	qword ptr [rdi], rcx
	mov	qword ptr [rdi + 8], rcx
	mov	rcx, qword ptr [rip + ._OBJC_CLASS_NSString@GOTPCREL]
	mov	qword ptr [rdi + 16], rcx
	mov	rcx, qword ptr [rip + ._OBJC_CLASS_NSException@GOTPCREL]
	mov	qword ptr [rdi + 24], rcx
	ret
.Lfunc_end4:
	.size	fn5_use_fns, .Lfunc_end4-fn5_use_fns

	.section	.text.fn6_use_same_twice,"ax",@progbits
	.globl	fn6_use_same_twice
	.p2align	4
	.type	fn6_use_same_twice,@function
fn6_use_same_twice:
	mov	rax, rdi
	mov	rcx, qword ptr [rip + ._OBJC_CLASS_NSObject@GOTPCREL]
	mov	qword ptr [rdi], rcx
	mov	qword ptr [rdi + 8], rcx
	ret
.Lfunc_end5:
	.size	fn6_use_same_twice, .Lfunc_end5-fn6_use_same_twice

	.section	.text.fn7_use_in_loop,"ax",@progbits
	.globl	fn7_use_in_loop
	.p2align	4
	.type	fn7_use_in_loop,@function
fn7_use_in_loop:
	ret
.Lfunc_end6:
	.size	fn7_use_in_loop, .Lfunc_end6-fn7_use_in_loop

	.section	".note.GNU-stack","",@progbits
