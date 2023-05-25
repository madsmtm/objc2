	.text
	.intel_syntax noprefix
	.section	.text.iter_create,"ax",@progbits
	.globl	iter_create
	.p2align	4, 0x90
	.type	iter_create,@function
iter_create:
	mov	rax, rdi
	xorps	xmm0, xmm0
	movups	xmmword ptr [rdi + 176], xmm0
	movups	xmmword ptr [rdi + 160], xmm0
	mov	qword ptr [rdi + 192], 0
	movups	xmmword ptr [rdi + 8], xmm0
	movups	xmmword ptr [rdi + 24], xmm0
	movups	xmmword ptr [rdi + 40], xmm0
	movups	xmmword ptr [rdi + 56], xmm0
	movups	xmmword ptr [rdi + 72], xmm0
	movups	xmmword ptr [rdi + 88], xmm0
	movups	xmmword ptr [rdi + 104], xmm0
	movups	xmmword ptr [rdi + 120], xmm0
	mov	qword ptr [rdi], rsi
	movups	xmmword ptr [rdi + 136], xmm0
	mov	qword ptr [rdi + 152], 0
	movups	xmmword ptr [rdi + 200], xmm0
	ret
.Lfunc_end0:
	.size	iter_create, .Lfunc_end0-iter_create

	.section	.text.iter_once,"ax",@progbits
	.globl	iter_once
	.p2align	4, 0x90
	.type	iter_once,@function
iter_once:
	push	rbp
	push	r15
	push	r14
	push	r13
	push	r12
	push	rbx
	push	rax
	mov	rbx, rdi
	mov	rax, qword ptr [rdi + 200]
	cmp	rax, qword ptr [rdi + 208]
	jb	.LBB1_1
	lea	r14, [rbx + 8]
	mov	r15, qword ptr [rbx]
	lea	r12, [rbx + 136]
	mov	rbp, qword ptr [rip + SYM(icrate::Foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPCREL]
	mov	r13, qword ptr [rbp]
	test	r13, r13
	jne	.LBB1_4
	lea	rdi, [rip + .Lanon.[ID].0]
	call	qword ptr [rip + SYM(objc2::runtime::Sel::register_unchecked::GENERATED_ID, 0)@GOTPCREL]
	mov	r13, rax
	mov	qword ptr [rbp], rax
.LBB1_4:
	mov	rdi, r15
	mov	rsi, r13
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	r8d, 16
	mov	rdi, r15
	mov	rsi, r13
	mov	rdx, r12
	mov	rcx, r14
	call	rax
	mov	rcx, rax
	mov	qword ptr [rbx + 208], rax
	mov	qword ptr [rbx + 200], 0
	xor	eax, eax
	test	rcx, rcx
	je	.LBB1_5
.LBB1_1:
	mov	rcx, qword ptr [rbx + 144]
	lea	rdx, [rax + 1]
	mov	qword ptr [rbx + 200], rdx
	mov	rax, qword ptr [rcx + 8*rax]
.LBB1_5:
	add	rsp, 8
	pop	rbx
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	pop	rbp
	ret
.Lfunc_end1:
	.size	iter_once, .Lfunc_end1-iter_once

	.section	.text.use_obj,"ax",@progbits
	.globl	use_obj
	.p2align	4, 0x90
	.type	use_obj,@function
use_obj:
	mov	qword ptr [rsp - 8], rdi
	lea	rax, [rsp - 8]
	#APP
	#NO_APP
	ret
.Lfunc_end2:
	.size	use_obj, .Lfunc_end2-use_obj

	.section	.text.iter,"ax",@progbits
	.globl	iter
	.p2align	4, 0x90
	.type	iter,@function
iter:
	push	rbp
	push	r15
	push	r14
	push	r13
	push	r12
	push	rbx
	sub	rsp, 216
	mov	r15, rdi
	xorps	xmm0, xmm0
	movups	xmmword ptr [rsp + 176], xmm0
	movups	xmmword ptr [rsp + 160], xmm0
	mov	qword ptr [rsp + 192], 0
	movups	xmmword ptr [rsp + 8], xmm0
	movups	xmmword ptr [rsp + 24], xmm0
	movups	xmmword ptr [rsp + 40], xmm0
	movups	xmmword ptr [rsp + 56], xmm0
	movups	xmmword ptr [rsp + 72], xmm0
	movups	xmmword ptr [rsp + 88], xmm0
	movups	xmmword ptr [rsp + 104], xmm0
	movups	xmmword ptr [rsp + 120], xmm0
	mov	qword ptr [rsp], rdi
	lea	r14, [rsp + 136]
	movups	xmmword ptr [rsp + 136], xmm0
	mov	qword ptr [rsp + 152], 0
	movups	xmmword ptr [rsp + 200], xmm0
	xor	ecx, ecx
	mov	rbp, qword ptr [rip + use_obj@GOTPCREL]
	mov	r12, qword ptr [rip + SYM(icrate::Foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPCREL]
	mov	rbx, qword ptr [rip + objc_msg_lookup@GOTPCREL]
	xor	eax, eax
	jmp	.LBB3_1
	.p2align	4, 0x90
.LBB3_6:
	mov	rcx, qword ptr [rsp + 144]
	lea	rdx, [rax + 1]
	mov	qword ptr [rsp + 200], rdx
	mov	rdi, qword ptr [rcx + 8*rax]
	call	rbp
	mov	r15, qword ptr [rsp]
	mov	rax, qword ptr [rsp + 200]
	mov	rcx, qword ptr [rsp + 208]
.LBB3_1:
	cmp	rax, rcx
	jb	.LBB3_6
	mov	r13, qword ptr [r12]
	test	r13, r13
	jne	.LBB3_4
	lea	rdi, [rip + .Lanon.[ID].0]
	call	qword ptr [rip + SYM(objc2::runtime::Sel::register_unchecked::GENERATED_ID, 0)@GOTPCREL]
	mov	r13, rax
	mov	qword ptr [r12], rax
.LBB3_4:
	mov	rdi, r15
	mov	rsi, r13
	call	rbx
	mov	r8d, 16
	mov	rdi, r15
	mov	rsi, r13
	mov	rdx, r14
	lea	rcx, [rsp + 8]
	call	rax
	mov	qword ptr [rsp + 208], rax
	test	rax, rax
	je	.LBB3_7
	xor	eax, eax
	jmp	.LBB3_6
.LBB3_7:
	add	rsp, 216
	pop	rbx
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	pop	rbp
	ret
.Lfunc_end3:
	.size	iter, .Lfunc_end3-iter

	.section	.text.iter_noop,"ax",@progbits
	.globl	iter_noop
	.p2align	4, 0x90
	.type	iter_noop,@function
iter_noop:
	push	rbp
	push	r15
	push	r14
	push	r13
	push	r12
	push	rbx
	sub	rsp, 216
	mov	r14, rdi
	xorps	xmm0, xmm0
	movups	xmmword ptr [rsp + 176], xmm0
	movups	xmmword ptr [rsp + 160], xmm0
	mov	qword ptr [rsp + 192], 0
	lea	rbx, [rsp + 8]
	movups	xmmword ptr [rsp + 8], xmm0
	movups	xmmword ptr [rsp + 24], xmm0
	movups	xmmword ptr [rsp + 40], xmm0
	movups	xmmword ptr [rsp + 56], xmm0
	movups	xmmword ptr [rsp + 72], xmm0
	movups	xmmword ptr [rsp + 88], xmm0
	movups	xmmword ptr [rsp + 104], xmm0
	movups	xmmword ptr [rsp + 120], xmm0
	mov	qword ptr [rsp], rdi
	lea	r15, [rsp + 136]
	movups	xmmword ptr [rsp + 136], xmm0
	mov	qword ptr [rsp + 152], 0
	movups	xmmword ptr [rsp + 200], xmm0
	xor	eax, eax
	mov	rbp, qword ptr [rip + SYM(icrate::Foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPCREL]
	mov	r12, qword ptr [rip + objc_msg_lookup@GOTPCREL]
	xor	ecx, ecx
	jmp	.LBB4_1
	.p2align	4, 0x90
.LBB4_6:
	inc	rcx
	mov	qword ptr [rsp + 200], rcx
.LBB4_1:
	cmp	rcx, rax
	jb	.LBB4_6
	mov	r13, qword ptr [rbp]
	test	r13, r13
	jne	.LBB4_4
	lea	rdi, [rip + .Lanon.[ID].0]
	call	qword ptr [rip + SYM(objc2::runtime::Sel::register_unchecked::GENERATED_ID, 0)@GOTPCREL]
	mov	r13, rax
	mov	qword ptr [rbp], rax
.LBB4_4:
	mov	rdi, r14
	mov	rsi, r13
	call	r12
	mov	r8d, 16
	mov	rdi, r14
	mov	rsi, r13
	mov	rdx, r15
	mov	rcx, rbx
	call	rax
	mov	qword ptr [rsp + 208], rax
	test	rax, rax
	je	.LBB4_7
	mov	r14, qword ptr [rsp]
	xor	ecx, ecx
	jmp	.LBB4_6
.LBB4_7:
	add	rsp, 216
	pop	rbx
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	pop	rbp
	ret
.Lfunc_end4:
	.size	iter_noop, .Lfunc_end4-iter_noop

	.section	.text.iter_retained,"ax",@progbits
	.globl	iter_retained
	.p2align	4, 0x90
	.type	iter_retained,@function
iter_retained:
	push	rbp
	push	r15
	push	r14
	push	r13
	push	r12
	push	rbx
	sub	rsp, 216
	mov	r12, rdi
	xorps	xmm0, xmm0
	movups	xmmword ptr [rsp + 176], xmm0
	movups	xmmword ptr [rsp + 160], xmm0
	mov	qword ptr [rsp + 192], 0
	movups	xmmword ptr [rsp + 8], xmm0
	movups	xmmword ptr [rsp + 24], xmm0
	movups	xmmword ptr [rsp + 40], xmm0
	movups	xmmword ptr [rsp + 56], xmm0
	movups	xmmword ptr [rsp + 72], xmm0
	movups	xmmword ptr [rsp + 88], xmm0
	movups	xmmword ptr [rsp + 104], xmm0
	movups	xmmword ptr [rsp + 120], xmm0
	mov	qword ptr [rsp], rdi
	movups	xmmword ptr [rsp + 136], xmm0
	mov	qword ptr [rsp + 152], 0
	movups	xmmword ptr [rsp + 200], xmm0
	xor	ecx, ecx
	mov	rbp, qword ptr [rip + objc_retain@GOTPCREL]
	mov	r15, qword ptr [rip + use_obj@GOTPCREL]
	mov	rbx, qword ptr [rip + objc_release@GOTPCREL]
	mov	r14, qword ptr [rip + SYM(icrate::Foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPCREL]
	xor	eax, eax
	jmp	.LBB5_1
	.p2align	4, 0x90
.LBB5_6:
	mov	rcx, qword ptr [rsp + 144]
	lea	rdx, [rax + 1]
	mov	qword ptr [rsp + 200], rdx
	mov	rdi, qword ptr [rcx + 8*rax]
	call	rbp
	mov	r12, rax
	mov	rdi, rax
	call	r15
	mov	rdi, r12
	call	rbx
	mov	r12, qword ptr [rsp]
	mov	rax, qword ptr [rsp + 200]
	mov	rcx, qword ptr [rsp + 208]
.LBB5_1:
	cmp	rax, rcx
	jb	.LBB5_6
	mov	r13, qword ptr [r14]
	test	r13, r13
	jne	.LBB5_4
	lea	rdi, [rip + .Lanon.[ID].0]
	call	qword ptr [rip + SYM(objc2::runtime::Sel::register_unchecked::GENERATED_ID, 0)@GOTPCREL]
	mov	r13, rax
	mov	qword ptr [r14], rax
.LBB5_4:
	mov	rdi, r12
	mov	rsi, r13
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	r8d, 16
	mov	rdi, r12
	mov	rsi, r13
	lea	rdx, [rsp + 136]
	lea	rcx, [rsp + 8]
	call	rax
	mov	qword ptr [rsp + 208], rax
	test	rax, rax
	je	.LBB5_7
	xor	eax, eax
	jmp	.LBB5_6
.LBB5_7:
	add	rsp, 216
	pop	rbx
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	pop	rbp
	ret
.Lfunc_end5:
	.size	iter_retained, .Lfunc_end5-iter_retained

	.type	.Lanon.[ID].0,@object
	.section	.rodata..Lanon.[ID].0,"a",@progbits
.Lanon.[ID].0:
	.asciz	"countByEnumeratingWithState:objects:count:"
	.size	.Lanon.[ID].0, 43

	.section	".note.GNU-stack","",@progbits
