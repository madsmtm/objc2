	.intel_syntax noprefix
	.section	.text.fn1_iter_create,"ax",@progbits
	.globl	fn1_iter_create
	.p2align	4
	.type	fn1_iter_create,@function
fn1_iter_create:
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
	.size	fn1_iter_create, .Lfunc_end0-fn1_iter_create

	.section	.text.fn2_iter_once,"ax",@progbits
	.globl	fn2_iter_once
	.p2align	4
	.type	fn2_iter_once,@function
fn2_iter_once:
	push	r15
	push	r14
	push	r13
	push	r12
	push	rbx
	mov	rbx, rdi
	mov	rax, qword ptr [rdi + 200]
	cmp	rax, qword ptr [rdi + 208]
	jb	.LBB1_4
	lea	r14, [rbx + 8]
	mov	r12, qword ptr [rbx]
	lea	r15, [rbx + 136]
	mov	rax, qword ptr [rip + SYM(objc2_foundation[CRATE_ID]::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL, 0)@GOTPCREL]
	mov	r13, qword ptr [rax]
	test	r13, r13
	je	.LBB1_2
.LBB1_3:
	mov	rdi, r12
	mov	rsi, r13
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	r8d, 16
	mov	rdi, r12
	mov	rsi, r13
	mov	rdx, r15
	mov	rcx, r14
	call	rax
	mov	rcx, rax
	mov	qword ptr [rbx + 208], rax
	mov	qword ptr [rbx + 200], 0
	xor	eax, eax
	test	rcx, rcx
	je	.LBB1_5
.LBB1_4:
	mov	rcx, qword ptr [rbx + 144]
	lea	rdx, [rax + 1]
	mov	qword ptr [rbx + 200], rdx
	mov	rax, qword ptr [rcx + 8*rax]
.LBB1_5:
	pop	rbx
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	ret
.LBB1_2:
	mov	rdi, qword ptr [rip + SYM(objc2_foundation[CRATE_ID]::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL, 0)@GOTPCREL]
	lea	rsi, [rip + .Lanon.[ID].0]
	call	qword ptr [rip + SYM(<objc2[CRATE_ID]::__macros::sel::CachedSel>::fetch, 0)@GOTPCREL]
	mov	r13, rax
	jmp	.LBB1_3
.Lfunc_end1:
	.size	fn2_iter_once, .Lfunc_end1-fn2_iter_once

	.section	.text.fn3_use_obj,"ax",@progbits
	.globl	fn3_use_obj
	.p2align	4
	.type	fn3_use_obj,@function
fn3_use_obj:
	mov	qword ptr [rsp - 8], rdi
	lea	rax, [rsp - 8]
	#APP
	#NO_APP
	ret
.Lfunc_end2:
	.size	fn3_use_obj, .Lfunc_end2-fn3_use_obj

	.section	.text.fn4_iter,"ax",@progbits
	.globl	fn4_iter
	.p2align	4
	.type	fn4_iter,@function
fn4_iter:
	push	rbp
	push	r15
	push	r14
	push	r13
	push	r12
	push	rbx
	sub	rsp, 216
	mov	r13, rdi
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
	mov	r12, qword ptr [rip + fn3_use_obj@GOTPCREL]
	mov	r15, qword ptr [rip + SYM(objc2_foundation[CRATE_ID]::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL, 0)@GOTPCREL]
	mov	rbx, qword ptr [rip + objc_msg_lookup@GOTPCREL]
	xor	eax, eax
	jmp	.LBB3_1
	.p2align	4
.LBB3_6:
	mov	rcx, qword ptr [rsp + 144]
	lea	rdx, [rax + 1]
	mov	qword ptr [rsp + 200], rdx
	mov	rdi, qword ptr [rcx + 8*rax]
	call	r12
	mov	r13, qword ptr [rsp]
	mov	rax, qword ptr [rsp + 200]
	mov	rcx, qword ptr [rsp + 208]
.LBB3_1:
	cmp	rax, rcx
	jb	.LBB3_6
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
	lea	rcx, [rsp + 8]
	call	rax
	mov	qword ptr [rsp + 208], rax
	test	rax, rax
	je	.LBB3_7
	xor	eax, eax
	jmp	.LBB3_6
.LBB3_3:
	mov	rdi, r15
	lea	rsi, [rip + .Lanon.[ID].0]
	call	qword ptr [rip + SYM(<objc2[CRATE_ID]::__macros::sel::CachedSel>::fetch, 0)@GOTPCREL]
	mov	rbp, rax
	jmp	.LBB3_4
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
	.size	fn4_iter, .Lfunc_end3-fn4_iter

	.section	.text.fn5_iter_noop,"ax",@progbits
	.globl	fn5_iter_noop
	.p2align	4
	.type	fn5_iter_noop,@function
fn5_iter_noop:
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
	mov	r12, qword ptr [rip + SYM(objc2_foundation[CRATE_ID]::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL, 0)@GOTPCREL]
	mov	r13, qword ptr [rip + objc_msg_lookup@GOTPCREL]
	xor	ecx, ecx
	jmp	.LBB4_1
	.p2align	4
.LBB4_6:
	inc	rcx
	mov	qword ptr [rsp + 200], rcx
.LBB4_1:
	cmp	rcx, rax
	jb	.LBB4_6
	mov	rbp, qword ptr [r12]
	test	rbp, rbp
	je	.LBB4_3
.LBB4_4:
	mov	rdi, r14
	mov	rsi, rbp
	call	r13
	mov	r8d, 16
	mov	rdi, r14
	mov	rsi, rbp
	mov	rdx, r15
	mov	rcx, rbx
	call	rax
	mov	qword ptr [rsp + 208], rax
	test	rax, rax
	je	.LBB4_7
	mov	r14, qword ptr [rsp]
	xor	ecx, ecx
	jmp	.LBB4_6
.LBB4_3:
	mov	rdi, r12
	lea	rsi, [rip + .Lanon.[ID].0]
	call	qword ptr [rip + SYM(<objc2[CRATE_ID]::__macros::sel::CachedSel>::fetch, 0)@GOTPCREL]
	mov	rbp, rax
	jmp	.LBB4_4
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
	.size	fn5_iter_noop, .Lfunc_end4-fn5_iter_noop

	.section	.text.fn6_iter_retained,"ax",@progbits
	.globl	fn6_iter_retained
	.p2align	4
	.type	fn6_iter_retained,@function
fn6_iter_retained:
	push	rbp
	push	r15
	push	r14
	push	r13
	push	r12
	push	rbx
	sub	rsp, 232
	mov	r13, rdi
	xorps	xmm0, xmm0
	movups	xmmword ptr [rsp + 192], xmm0
	movups	xmmword ptr [rsp + 176], xmm0
	mov	qword ptr [rsp + 208], 0
	movups	xmmword ptr [rsp + 24], xmm0
	movups	xmmword ptr [rsp + 40], xmm0
	movups	xmmword ptr [rsp + 56], xmm0
	movups	xmmword ptr [rsp + 72], xmm0
	movups	xmmword ptr [rsp + 88], xmm0
	movups	xmmword ptr [rsp + 104], xmm0
	movups	xmmword ptr [rsp + 120], xmm0
	movups	xmmword ptr [rsp + 136], xmm0
	mov	qword ptr [rsp], 0
	mov	qword ptr [rsp + 16], rdi
	movups	xmmword ptr [rsp + 152], xmm0
	mov	qword ptr [rsp + 168], 0
	movups	xmmword ptr [rsp + 216], xmm0
	xor	ecx, ecx
	mov	r12, qword ptr [rip + objc_retain@GOTPCREL]
	mov	rbx, qword ptr [rip + fn3_use_obj@GOTPCREL]
	mov	r14, qword ptr [rip + objc_release@GOTPCREL]
	mov	r15, qword ptr [rip + objc_msg_lookup@GOTPCREL]
	xor	eax, eax
	jmp	.LBB5_1
	.p2align	4
.LBB5_11:
	mov	qword ptr [rsp], 1
	mov	qword ptr [rsp + 8], rcx
.LBB5_12:
	mov	rcx, qword ptr [rsp + 160]
	lea	rdx, [rax + 1]
	mov	qword ptr [rsp + 216], rdx
	mov	rdi, qword ptr [rcx + 8*rax]
	call	r12
	mov	r13, rax
	mov	rdi, rax
	call	rbx
	mov	rdi, r13
	call	r14
	mov	r13, qword ptr [rsp + 16]
	mov	rax, qword ptr [rsp + 216]
	mov	rcx, qword ptr [rsp + 224]
.LBB5_1:
	cmp	rax, rcx
	jb	.LBB5_7
	mov	rax, qword ptr [rip + SYM(objc2_foundation[CRATE_ID]::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL, 0)@GOTPCREL]
	mov	rbp, qword ptr [rax]
	test	rbp, rbp
	je	.LBB5_3
.LBB5_4:
	mov	rdi, r13
	mov	rsi, rbp
	call	r15
	mov	r8d, 16
	mov	rdi, r13
	mov	rsi, rbp
	lea	rdx, [rsp + 152]
	lea	rcx, [rsp + 24]
	call	rax
	mov	qword ptr [rsp + 224], rax
	mov	qword ptr [rsp + 216], 0
	test	rax, rax
	je	.LBB5_13
	cmp	qword ptr [rsp + 160], 0
	je	.LBB5_14
	xor	eax, eax
.LBB5_7:
	mov	rcx, qword ptr [rsp + 168]
	test	rcx, rcx
	je	.LBB5_12
	mov	rcx, qword ptr [rcx]
	cmp	dword ptr [rsp], 1
	jne	.LBB5_11
	cmp	qword ptr [rsp + 8], rcx
	je	.LBB5_12
	jmp	.LBB5_10
.LBB5_3:
	mov	rdi, qword ptr [rip + SYM(objc2_foundation[CRATE_ID]::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL, 0)@GOTPCREL]
	lea	rsi, [rip + .Lanon.[ID].0]
	call	qword ptr [rip + SYM(<objc2[CRATE_ID]::__macros::sel::CachedSel>::fetch, 0)@GOTPCREL]
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
	call	qword ptr [rip + SYM(objc2_foundation[CRATE_ID]::iter::items_ptr_null, 0)@GOTPCREL]
.LBB5_10:
	call	qword ptr [rip + SYM(objc2_foundation[CRATE_ID]::iter::mutation_detected, 0)@GOTPCREL]
.Lfunc_end5:
	.size	fn6_iter_retained, .Lfunc_end5-fn6_iter_retained

	.type	.Lanon.[ID].0,@object
	.section	.rodata.str1.1,"aMS",@progbits,1
.Lanon.[ID].0:
	.asciz	"countByEnumeratingWithState:objects:count:"
	.size	.Lanon.[ID].0, 43

	.section	".note.GNU-stack","",@progbits
