	.text
	.intel_syntax noprefix
	.section	.text.iter_create,"ax",@progbits
	.globl	iter_create
	.p2align	4, 0x90
	.type	iter_create,@function
iter_create:
	mov	rax, rdi
	mov	qword ptr [rdi], rsi
	xorps	xmm0, xmm0
<<<<<<< HEAD
	movups	xmmword ptr [rdi + 8], xmm0
	movups	xmmword ptr [rdi + 24], xmm0
	movups	xmmword ptr [rdi + 40], xmm0
	movups	xmmword ptr [rdi + 56], xmm0
	movups	xmmword ptr [rdi + 72], xmm0
	movups	xmmword ptr [rdi + 88], xmm0
	movups	xmmword ptr [rdi + 104], xmm0
	movups	xmmword ptr [rdi + 120], xmm0
	movups	xmmword ptr [rdi + 136], xmm0
	movups	xmmword ptr [rdi + 152], xmm0
	movups	xmmword ptr [rdi + 168], xmm0
	movups	xmmword ptr [rdi + 184], xmm0
	movups	xmmword ptr [rdi + 200], xmm0
=======
	movups	xmmword ptr [rdi + 56], xmm0
	movups	xmmword ptr [rdi + 40], xmm0
	mov	qword ptr [rdi + 72], 0
	movups	xmmword ptr [rdi + 80], xmm0
	movups	xmmword ptr [rdi + 96], xmm0
	movups	xmmword ptr [rdi + 112], xmm0
	movups	xmmword ptr [rdi + 128], xmm0
	movups	xmmword ptr [rdi + 144], xmm0
	movups	xmmword ptr [rdi + 160], xmm0
	movups	xmmword ptr [rdi + 176], xmm0
	movups	xmmword ptr [rdi + 192], xmm0
	mov	qword ptr [rdi], 0
	movups	xmmword ptr [rdi + 16], xmm0
	mov	qword ptr [rdi + 32], 0
	movups	xmmword ptr [rdi + 208], xmm0
	mov	qword ptr [rdi + 224], rsi
>>>>>>> 11a7eeed2 (Fuzz array mutation)
	ret
.Lfunc_end0:
	.size	iter_create, .Lfunc_end0-iter_create

	.section	.text.iter_once,"ax",@progbits
	.globl	iter_once
	.p2align	4, 0x90
	.type	iter_once,@function
iter_once:
	push	r15
	push	r14
	push	r13
	push	r12
	push	rbx
	mov	rbx, rdi
<<<<<<< HEAD
	mov	rax, qword ptr [rdi + 200]
	cmp	rax, qword ptr [rdi + 208]
	jb	.LBB1_4
	lea	r14, [rbx + 8]
	mov	r15, qword ptr [rbx]
=======
	mov	rax, qword ptr [rdi + 208]
	cmp	rax, qword ptr [rdi + 216]
	jb	.LBB1_7
	mov	r15, qword ptr [rbx + 224]
	lea	r14, [rbx + 80]
	lea	r12, [rbx + 16]
>>>>>>> 11a7eeed2 (Fuzz array mutation)
	mov	rax, qword ptr [rip + SYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPCREL]
	mov	r12, qword ptr [rax]
	test	r12, r12
	je	.LBB1_2
.LBB1_3:
	lea	r13, [rbx + 136]
	mov	rdi, r15
	mov	rsi, r12
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	r8d, 16
	mov	rdi, r15
	mov	rsi, r12
	mov	rdx, r13
	mov	rcx, r14
	call	rax
	mov	qword ptr [rbx + 216], rax
	mov	qword ptr [rbx + 208], 0
	test	rax, rax
	je	.LBB1_4
	cmp	qword ptr [rbx + 24], 0
	je	.LBB1_14
	xor	eax, eax
.LBB1_7:
	mov	rcx, qword ptr [rbx + 32]
	test	rcx, rcx
	je	.LBB1_10
	mov	rcx, qword ptr [rcx]
	cmp	qword ptr [rbx], 0
	je	.LBB1_9
	cmp	qword ptr [rbx + 8], rcx
	je	.LBB1_10
	call	qword ptr [rip + SYM(icrate::additions::Foundation::iter::mutation_detected::GENERATED_ID, 0)@GOTPCREL]
.LBB1_9:
	mov	qword ptr [rbx], 1
	mov	qword ptr [rbx + 8], rcx
.LBB1_10:
	mov	rcx, qword ptr [rbx + 24]
	lea	rdx, [rax + 1]
	mov	qword ptr [rbx + 208], rdx
	mov	rax, qword ptr [rcx + 8*rax]
.LBB1_11:
	pop	rbx
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	ret
.LBB1_4:
	xor	eax, eax
	jmp	.LBB1_11
.LBB1_2:
	mov	rdi, qword ptr [rip + SYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPCREL]
	lea	rsi, [rip + .Lanon.[ID].0]
	call	qword ptr [rip + SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)@GOTPCREL]
	mov	r12, rax
	jmp	.LBB1_3
.LBB1_14:
	call	qword ptr [rip + SYM(icrate::additions::Foundation::iter::items_ptr_null::GENERATED_ID, 0)@GOTPCREL]
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
<<<<<<< HEAD
	sub	rsp, 216
=======
	sub	rsp, 232
>>>>>>> 11a7eeed2 (Fuzz array mutation)
	mov	r13, rdi
	xorps	xmm0, xmm0
	movups	xmmword ptr [rsp + 56], xmm0
	movups	xmmword ptr [rsp + 40], xmm0
	mov	qword ptr [rsp + 72], 0
	movups	xmmword ptr [rsp + 80], xmm0
	movups	xmmword ptr [rsp + 96], xmm0
	movups	xmmword ptr [rsp + 112], xmm0
	movups	xmmword ptr [rsp + 128], xmm0
	movups	xmmword ptr [rsp + 144], xmm0
	movups	xmmword ptr [rsp + 160], xmm0
	movups	xmmword ptr [rsp + 176], xmm0
	movups	xmmword ptr [rsp + 192], xmm0
	mov	qword ptr [rsp], 0
	lea	r14, [rsp + 16]
	movups	xmmword ptr [rsp + 16], xmm0
	mov	qword ptr [rsp + 32], 0
	movups	xmmword ptr [rsp + 208], xmm0
	mov	qword ptr [rsp + 224], rdi
	xor	ecx, ecx
	mov	r12, qword ptr [rip + use_obj@GOTPCREL]
	mov	r15, qword ptr [rip + SYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPCREL]
	mov	rbx, qword ptr [rip + objc_msg_lookup@GOTPCREL]
	xor	eax, eax
	jmp	.LBB3_1
	.p2align	4, 0x90
.LBB3_9:
	mov	qword ptr [rsp], 1
	mov	qword ptr [rsp + 8], rcx
.LBB3_10:
	mov	rcx, qword ptr [rsp + 24]
	lea	rdx, [rax + 1]
	mov	qword ptr [rsp + 208], rdx
	mov	rdi, qword ptr [rcx + 8*rax]
	call	r12
<<<<<<< HEAD
	mov	r13, qword ptr [rsp]
	mov	rax, qword ptr [rsp + 200]
	mov	rcx, qword ptr [rsp + 208]
.LBB3_1:
	cmp	rax, rcx
	jb	.LBB3_6
=======
	mov	r13, qword ptr [rsp + 224]
	mov	rax, qword ptr [rsp + 208]
	mov	rcx, qword ptr [rsp + 216]
.LBB3_1:
	cmp	rax, rcx
	jb	.LBB3_7
>>>>>>> 11a7eeed2 (Fuzz array mutation)
	mov	rbp, qword ptr [r15]
	test	rbp, rbp
	je	.LBB3_3
.LBB3_4:
	mov	rdi, r13
	mov	rsi, rbp
	call	rbx
	mov	r8d, 16
	mov	rdi, r13
	mov	rsi, rbp
	mov	rdx, r14
	lea	rcx, [rsp + 80]
	call	rax
	mov	qword ptr [rsp + 216], rax
	mov	qword ptr [rsp + 208], 0
	test	rax, rax
	je	.LBB3_13
	cmp	qword ptr [rsp + 24], 0
	je	.LBB3_14
	xor	eax, eax
.LBB3_7:
	mov	rcx, qword ptr [rsp + 32]
	test	rcx, rcx
	je	.LBB3_10
	mov	rcx, qword ptr [rcx]
	cmp	qword ptr [rsp], 0
	je	.LBB3_9
	cmp	qword ptr [rsp + 8], rcx
	je	.LBB3_10
	jmp	.LBB3_12
.LBB3_3:
	mov	rdi, r15
	lea	rsi, [rip + .Lanon.[ID].0]
	call	qword ptr [rip + SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)@GOTPCREL]
	mov	rbp, rax
	jmp	.LBB3_4
.LBB3_13:
	add	rsp, 232
	pop	rbx
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	pop	rbp
	ret
.LBB3_14:
	call	qword ptr [rip + SYM(icrate::additions::Foundation::iter::items_ptr_null::GENERATED_ID, 0)@GOTPCREL]
.LBB3_12:
	call	qword ptr [rip + SYM(icrate::additions::Foundation::iter::mutation_detected::GENERATED_ID, 0)@GOTPCREL]
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
	sub	rsp, 232
	mov	r13, rdi
	xorps	xmm0, xmm0
	movups	xmmword ptr [rsp + 56], xmm0
	movups	xmmword ptr [rsp + 40], xmm0
	mov	qword ptr [rsp + 72], 0
	lea	rbx, [rsp + 80]
	movups	xmmword ptr [rsp + 80], xmm0
	movups	xmmword ptr [rsp + 96], xmm0
	movups	xmmword ptr [rsp + 112], xmm0
	movups	xmmword ptr [rsp + 128], xmm0
	movups	xmmword ptr [rsp + 144], xmm0
	movups	xmmword ptr [rsp + 160], xmm0
	movups	xmmword ptr [rsp + 176], xmm0
	movups	xmmword ptr [rsp + 192], xmm0
	mov	qword ptr [rsp], 0
	lea	r14, [rsp + 16]
	movups	xmmword ptr [rsp + 16], xmm0
	mov	qword ptr [rsp + 32], 0
	movups	xmmword ptr [rsp + 208], xmm0
	mov	qword ptr [rsp + 224], rdi
	xor	ecx, ecx
	mov	r15, qword ptr [rip + SYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPCREL]
	mov	r12, qword ptr [rip + objc_msg_lookup@GOTPCREL]
	xor	eax, eax
	xor	edx, edx
	jmp	.LBB4_1
	.p2align	4, 0x90
.LBB4_9:
	mov	qword ptr [rsp], 1
	mov	qword ptr [rsp + 8], rsi
.LBB4_11:
	inc	rdx
	mov	qword ptr [rsp + 208], rdx
	mov	r13, qword ptr [rsp + 224]
.LBB4_1:
	cmp	rdx, rax
	jb	.LBB4_7
	mov	rbp, qword ptr [r15]
	test	rbp, rbp
	je	.LBB4_3
.LBB4_4:
	mov	rdi, r13
	mov	rsi, rbp
	call	r12
	mov	r8d, 16
	mov	rdi, r13
	mov	rsi, rbp
	mov	rdx, r14
	mov	rcx, rbx
	call	rax
	mov	qword ptr [rsp + 216], rax
	mov	qword ptr [rsp + 208], 0
	test	rax, rax
	je	.LBB4_12
	cmp	qword ptr [rsp + 24], 0
	je	.LBB4_13
	mov	rcx, qword ptr [rsp + 32]
	xor	edx, edx
.LBB4_7:
	test	rcx, rcx
	je	.LBB4_11
	mov	rsi, qword ptr [rcx]
	cmp	qword ptr [rsp], 0
	je	.LBB4_9
	cmp	qword ptr [rsp + 8], rsi
	je	.LBB4_11
	jmp	.LBB4_14
.LBB4_3:
	mov	rdi, r15
	lea	rsi, [rip + .Lanon.[ID].0]
	call	qword ptr [rip + SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)@GOTPCREL]
	mov	rbp, rax
	jmp	.LBB4_4
.LBB4_12:
	add	rsp, 232
	pop	rbx
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	pop	rbp
	ret
.LBB4_13:
	call	qword ptr [rip + SYM(icrate::additions::Foundation::iter::items_ptr_null::GENERATED_ID, 0)@GOTPCREL]
.LBB4_14:
	call	qword ptr [rip + SYM(icrate::additions::Foundation::iter::mutation_detected::GENERATED_ID, 0)@GOTPCREL]
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
	sub	rsp, 232
	mov	r13, rdi
	xorps	xmm0, xmm0
	movups	xmmword ptr [rsp + 56], xmm0
	movups	xmmword ptr [rsp + 40], xmm0
	mov	qword ptr [rsp + 72], 0
	movups	xmmword ptr [rsp + 80], xmm0
	movups	xmmword ptr [rsp + 96], xmm0
	movups	xmmword ptr [rsp + 112], xmm0
	movups	xmmword ptr [rsp + 128], xmm0
	movups	xmmword ptr [rsp + 144], xmm0
	movups	xmmword ptr [rsp + 160], xmm0
	movups	xmmword ptr [rsp + 176], xmm0
	movups	xmmword ptr [rsp + 192], xmm0
	mov	qword ptr [rsp], 0
	movups	xmmword ptr [rsp + 16], xmm0
	mov	qword ptr [rsp + 32], 0
	movups	xmmword ptr [rsp + 208], xmm0
	mov	qword ptr [rsp + 224], rdi
	xor	ecx, ecx
	mov	r12, qword ptr [rip + objc_retain@GOTPCREL]
	mov	rbx, qword ptr [rip + use_obj@GOTPCREL]
	mov	r14, qword ptr [rip + objc_release@GOTPCREL]
	mov	r15, qword ptr [rip + SYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPCREL]
	xor	eax, eax
	jmp	.LBB5_1
	.p2align	4, 0x90
.LBB5_9:
	mov	qword ptr [rsp], 1
	mov	qword ptr [rsp + 8], rcx
.LBB5_10:
	mov	rcx, qword ptr [rsp + 24]
	lea	rdx, [rax + 1]
	mov	qword ptr [rsp + 208], rdx
	mov	rdi, qword ptr [rcx + 8*rax]
	call	r12
	mov	r13, rax
	mov	rdi, rax
	call	rbx
	mov	rdi, r13
	call	r14
	mov	r13, qword ptr [rsp + 224]
	mov	rax, qword ptr [rsp + 208]
	mov	rcx, qword ptr [rsp + 216]
.LBB5_1:
	cmp	rax, rcx
	jb	.LBB5_7
	mov	rbp, qword ptr [r15]
	test	rbp, rbp
	je	.LBB5_3
.LBB5_4:
	mov	rdi, r13
	mov	rsi, rbp
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	r8d, 16
	mov	rdi, r13
	mov	rsi, rbp
	lea	rdx, [rsp + 16]
	lea	rcx, [rsp + 80]
	call	rax
	mov	qword ptr [rsp + 216], rax
	mov	qword ptr [rsp + 208], 0
	test	rax, rax
	je	.LBB5_13
	cmp	qword ptr [rsp + 24], 0
	je	.LBB5_14
	xor	eax, eax
.LBB5_7:
	mov	rcx, qword ptr [rsp + 32]
	test	rcx, rcx
	je	.LBB5_10
	mov	rcx, qword ptr [rcx]
	cmp	qword ptr [rsp], 0
	je	.LBB5_9
	cmp	qword ptr [rsp + 8], rcx
	je	.LBB5_10
	jmp	.LBB5_12
.LBB5_3:
	mov	rdi, r15
	lea	rsi, [rip + .Lanon.[ID].0]
	call	qword ptr [rip + SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)@GOTPCREL]
	mov	rbp, rax
	jmp	.LBB5_4
.LBB5_13:
	add	rsp, 232
	pop	rbx
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	pop	rbp
	ret
.LBB5_14:
	call	qword ptr [rip + SYM(icrate::additions::Foundation::iter::items_ptr_null::GENERATED_ID, 0)@GOTPCREL]
.LBB5_12:
	call	qword ptr [rip + SYM(icrate::additions::Foundation::iter::mutation_detected::GENERATED_ID, 0)@GOTPCREL]
.Lfunc_end5:
	.size	iter_retained, .Lfunc_end5-iter_retained

	.type	.Lanon.[ID].0,@object
	.section	.rodata..Lanon.[ID].0,"a",@progbits
.Lanon.[ID].0:
	.asciz	"countByEnumeratingWithState:objects:count:"
	.size	.Lanon.[ID].0, 43

	.section	".note.GNU-stack","",@progbits
