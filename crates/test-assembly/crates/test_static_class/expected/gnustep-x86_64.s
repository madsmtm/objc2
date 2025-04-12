	.intel_syntax noprefix
	.section	.text.get_class,"ax",@progbits
	.globl	get_class
	.p2align	4
	.type	get_class,@function
get_class:
	mov	rax, qword ptr [rip + ._OBJC_CLASS_NSObject@GOTPCREL]
	ret
.Lfunc_end0:
	.size	get_class, .Lfunc_end0-get_class

	.section	.text.get_same_class,"ax",@progbits
	.globl	get_same_class
	.p2align	4
	.type	get_same_class,@function
get_same_class:
	mov	rax, qword ptr [rip + ._OBJC_CLASS_NSObject@GOTPCREL]
	ret
.Lfunc_end1:
	.size	get_same_class, .Lfunc_end1-get_same_class

	.section	.text.get_different_class,"ax",@progbits
	.globl	get_different_class
	.p2align	4
	.type	get_different_class,@function
get_different_class:
	mov	rax, qword ptr [rip + ._OBJC_CLASS_NSString@GOTPCREL]
	ret
.Lfunc_end2:
	.size	get_different_class, .Lfunc_end2-get_different_class

	.section	.text.unused_class,"ax",@progbits
	.globl	unused_class
	.p2align	4
	.type	unused_class,@function
unused_class:
	ret
.Lfunc_end3:
	.size	unused_class, .Lfunc_end3-unused_class

	.section	.text.use_fns,"ax",@progbits
	.globl	use_fns
	.p2align	4
	.type	use_fns,@function
use_fns:
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
	.size	use_fns, .Lfunc_end4-use_fns

	.section	.text.use_same_twice,"ax",@progbits
	.globl	use_same_twice
	.p2align	4
	.type	use_same_twice,@function
use_same_twice:
	mov	rax, rdi
	mov	rcx, qword ptr [rip + ._OBJC_CLASS_NSObject@GOTPCREL]
	mov	qword ptr [rdi], rcx
	mov	qword ptr [rdi + 8], rcx
	ret
.Lfunc_end5:
	.size	use_same_twice, .Lfunc_end5-use_same_twice

	.section	.text.use_in_loop,"ax",@progbits
	.globl	use_in_loop
	.p2align	4
	.type	use_in_loop,@function
use_in_loop:
	ret
.Lfunc_end6:
	.size	use_in_loop, .Lfunc_end6-use_in_loop

	.section	".note.GNU-stack","",@progbits
