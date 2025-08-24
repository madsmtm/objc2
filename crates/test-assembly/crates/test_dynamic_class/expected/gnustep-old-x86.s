	.intel_syntax noprefix
	.section	.text.fn1_get_class,"ax",@progbits
	.globl	fn1_get_class
	.p2align	4
	.type	fn1_get_class,@function
fn1_get_class:
	call	.L0$pb
.L0$pb:
	pop	eax
.Ltmp0:
	add	eax, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp0-.L0$pb)
	mov	eax, dword ptr [eax + _OBJC_CLASS_NSObject@GOT]
	ret
.Lfunc_end0:
	.size	fn1_get_class, .Lfunc_end0-fn1_get_class

	.section	.text.fn1_get_same_class,"ax",@progbits
	.globl	fn1_get_same_class
	.p2align	4
	.type	fn1_get_same_class,@function
fn1_get_same_class:
	call	.L1$pb
.L1$pb:
	pop	eax
.Ltmp1:
	add	eax, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp1-.L1$pb)
	mov	eax, dword ptr [eax + _OBJC_CLASS_NSObject@GOT]
	ret
.Lfunc_end1:
	.size	fn1_get_same_class, .Lfunc_end1-fn1_get_same_class

	.section	.text.fn3_get_different_class,"ax",@progbits
	.globl	fn3_get_different_class
	.p2align	4
	.type	fn3_get_different_class,@function
fn3_get_different_class:
	call	.L2$pb
.L2$pb:
	pop	eax
.Ltmp2:
	add	eax, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp2-.L2$pb)
	mov	eax, dword ptr [eax + _OBJC_CLASS_NSString@GOT]
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
	call	.L4$pb
.L4$pb:
	pop	ecx
	mov	eax, dword ptr [esp + 4]
.Ltmp3:
	add	ecx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp3-.L4$pb)
	mov	edx, dword ptr [ecx + _OBJC_CLASS_NSObject@GOT]
	mov	dword ptr [eax], edx
	mov	dword ptr [eax + 4], edx
	mov	edx, dword ptr [ecx + _OBJC_CLASS_NSString@GOT]
	mov	ecx, dword ptr [ecx + _OBJC_CLASS_NSException@GOT]
	mov	dword ptr [eax + 8], edx
	mov	dword ptr [eax + 12], ecx
	ret	4
.Lfunc_end4:
	.size	fn5_use_fns, .Lfunc_end4-fn5_use_fns

	.section	.text.fn6_use_same_twice,"ax",@progbits
	.globl	fn6_use_same_twice
	.p2align	4
	.type	fn6_use_same_twice,@function
fn6_use_same_twice:
	call	.L5$pb
.L5$pb:
	pop	ecx
	mov	eax, dword ptr [esp + 4]
.Ltmp4:
	add	ecx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp4-.L5$pb)
	mov	ecx, dword ptr [ecx + _OBJC_CLASS_NSObject@GOT]
	mov	dword ptr [eax], ecx
	mov	dword ptr [eax + 4], ecx
	ret	4
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
