	.text
	.intel_syntax noprefix
	.section	.text.get_class,"ax",@progbits
	.globl	get_class
	.p2align	4, 0x90
	.type	get_class,@function
get_class:
	call	.L0$pb
.L0$pb:
	pop	eax
.Ltmp0:
	add	eax, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp0-.L0$pb)
	mov	eax, dword ptr [eax + _OBJC_CLASS_NSObject@GOT]
	ret
.Lfunc_end0:
	.size	get_class, .Lfunc_end0-get_class

	.section	.text.get_same_class,"ax",@progbits
	.globl	get_same_class
	.p2align	4, 0x90
	.type	get_same_class,@function
get_same_class:
	call	.L1$pb
.L1$pb:
	pop	eax
.Ltmp1:
	add	eax, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp1-.L1$pb)
	mov	eax, dword ptr [eax + _OBJC_CLASS_NSObject@GOT]
	ret
.Lfunc_end1:
	.size	get_same_class, .Lfunc_end1-get_same_class

	.section	.text.get_different_class,"ax",@progbits
	.globl	get_different_class
	.p2align	4, 0x90
	.type	get_different_class,@function
get_different_class:
	call	.L2$pb
.L2$pb:
	pop	eax
.Ltmp2:
	add	eax, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp2-.L2$pb)
	mov	eax, dword ptr [eax + _OBJC_CLASS_NSString@GOT]
	ret
.Lfunc_end2:
	.size	get_different_class, .Lfunc_end2-get_different_class

	.section	.text.unused_class,"ax",@progbits
	.globl	unused_class
	.p2align	4, 0x90
	.type	unused_class,@function
unused_class:
	ret
.Lfunc_end3:
	.size	unused_class, .Lfunc_end3-unused_class

	.section	.text.use_fns,"ax",@progbits
	.globl	use_fns
	.p2align	4, 0x90
	.type	use_fns,@function
use_fns:
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
	.size	use_fns, .Lfunc_end4-use_fns

	.section	.text.use_same_twice,"ax",@progbits
	.globl	use_same_twice
	.p2align	4, 0x90
	.type	use_same_twice,@function
use_same_twice:
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
	.size	use_same_twice, .Lfunc_end5-use_same_twice

	.section	.text.use_in_loop,"ax",@progbits
	.globl	use_in_loop
	.p2align	4, 0x90
	.type	use_in_loop,@function
use_in_loop:
	ret
.Lfunc_end6:
	.size	use_in_loop, .Lfunc_end6-use_in_loop

	.section	".note.GNU-stack","",@progbits
