	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_nonnull_nonnull
	.p2align	4, 0x90
_nonnull_nonnull:
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	rbx
	push	rax
	mov	rbx, rdx
	mov	r14, qword ptr [rdx]
	call	_objc_msgSend
	mov	r15, rax
	mov	rdi, qword ptr [rbx]
	call	_objc_retain
	mov	rdi, r14
	call	_objc_release
	mov	rax, r15
	add	rsp, 8
	pop	rbx
	pop	r14
	pop	r15
	pop	rbp
	ret

	.globl	_null_nonnull
	.p2align	4, 0x90
_null_nonnull:
	test	rdx, rdx
	je	LBB1_2
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	rbx
	push	rax
	mov	rbx, qword ptr [rdx]
	mov	r14, rdx
	call	_objc_msgSend
	mov	r15, rax
	mov	rdi, qword ptr [r14]
	call	_objc_retain
	mov	rdi, rbx
	call	_objc_release
	mov	rax, r15
	add	rsp, 8
	pop	rbx
	pop	r14
	pop	r15
	pop	rbp
	ret
LBB1_2:
	xor	edx, edx
	jmp	_objc_msgSend

	.globl	_nonnull_null
	.p2align	4, 0x90
_nonnull_null:
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	rbx
	push	rax
	mov	r15, rdx
	mov	rbx, qword ptr [rdx]
	call	_objc_msgSend
	mov	r14, rax
	mov	rdi, qword ptr [r15]
	call	_objc_retain
	test	rbx, rbx
	je	LBB2_2
	mov	rdi, rbx
	call	_objc_release
LBB2_2:
	mov	rax, r14
	add	rsp, 8
	pop	rbx
	pop	r14
	pop	r15
	pop	rbp
	ret

	.globl	_null_null
	.p2align	4, 0x90
_null_null:
	test	rdx, rdx
	je	LBB3_4
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	rbx
	push	rax
	mov	rbx, qword ptr [rdx]
	mov	r15, rdx
	call	_objc_msgSend
	mov	r14, rax
	mov	rdi, qword ptr [r15]
	call	_objc_retain
	test	rbx, rbx
	je	LBB3_3
	mov	rdi, rbx
	call	_objc_release
LBB3_3:
	mov	rax, r14
	add	rsp, 8
	pop	rbx
	pop	r14
	pop	r15
	pop	rbp
	ret
LBB3_4:
	xor	edx, edx
	jmp	_objc_msgSend

	.globl	_two_nonnull_nonnull
	.p2align	4, 0x90
_two_nonnull_nonnull:
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	r13
	push	r12
	push	rbx
	push	rax
	mov	rbx, rcx
	mov	r14, rdx
	mov	r15, qword ptr [rdx]
	mov	r12, qword ptr [rcx]
	call	_objc_msgSend
	mov	r13, rax
	mov	rdi, qword ptr [r14]
	call	_objc_retain
	mov	rdi, r15
	call	_objc_release
	mov	rdi, qword ptr [rbx]
	call	_objc_retain
	mov	rdi, r12
	call	_objc_release
	mov	rax, r13
	add	rsp, 8
	pop	rbx
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	pop	rbp
	ret

	.globl	_call_with_none1
	.p2align	4, 0x90
_call_with_none1:
	push	rbp
	mov	rbp, rsp
	xor	edx, edx
	pop	rbp
	jmp	_objc_msgSend

	.globl	_call_with_none2
	.p2align	4, 0x90
_call_with_none2:
	push	rbp
	mov	rbp, rsp
	xor	edx, edx
	pop	rbp
	jmp	_objc_msgSend

	.globl	_call_with_none3
	.p2align	4, 0x90
_call_with_none3:
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	qword ptr [rbp - 16], 0
	lea	rdx, [rbp - 16]
	call	_objc_msgSend
	mov	rbx, rax
	mov	rdi, qword ptr [rbp - 16]
	call	_objc_retain
	mov	rdx, qword ptr [rbp - 16]
	mov	rax, rbx
	add	rsp, 8
	pop	rbx
	pop	rbp
	ret

	.globl	_call_with_none4
	.p2align	4, 0x90
_call_with_none4:
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	qword ptr [rbp - 16], 0
	lea	rdx, [rbp - 16]
	call	_objc_msgSend
	mov	rbx, rax
	mov	rdi, qword ptr [rbp - 16]
	call	_objc_retain
	mov	rdx, qword ptr [rbp - 16]
	mov	rax, rbx
	add	rsp, 8
	pop	rbx
	pop	rbp
	ret

	.globl	_call_with_some1
	.p2align	4, 0x90
_call_with_some1:
	push	rbp
	mov	rbp, rsp
	push	r14
	push	rbx
	sub	rsp, 16
	mov	rbx, rdx
	mov	qword ptr [rbp - 24], rdx
	lea	rdx, [rbp - 24]
	call	_objc_msgSend
	mov	r14, rax
	mov	rdi, qword ptr [rbp - 24]
	call	_objc_retain
	mov	rdi, rbx
	call	_objc_release
	mov	rdx, qword ptr [rbp - 24]
	mov	rax, r14
	add	rsp, 16
	pop	rbx
	pop	r14
	pop	rbp
	ret

	.globl	_call_with_some2
	.p2align	4, 0x90
_call_with_some2:
	push	rbp
	mov	rbp, rsp
	push	r14
	push	rbx
	sub	rsp, 16
	mov	rbx, rdx
	mov	qword ptr [rbp - 24], rdx
	lea	rdx, [rbp - 24]
	call	_objc_msgSend
	mov	r14, rax
	mov	rdi, qword ptr [rbp - 24]
	call	_objc_retain
	mov	rdi, rbx
	call	_objc_release
	mov	rdx, qword ptr [rbp - 24]
	mov	rax, r14
	add	rsp, 16
	pop	rbx
	pop	r14
	pop	rbp
	ret

	.globl	_call_with_some3
	.p2align	4, 0x90
_call_with_some3:
	push	rbp
	mov	rbp, rsp
	push	r14
	push	rbx
	sub	rsp, 16
	mov	rbx, rdx
	mov	qword ptr [rbp - 24], rdx
	lea	rdx, [rbp - 24]
	call	_objc_msgSend
	mov	r14, rax
	mov	rdi, qword ptr [rbp - 24]
	call	_objc_retain
	mov	rdi, rbx
	call	_objc_release
	mov	rdx, qword ptr [rbp - 24]
	mov	rax, r14
	add	rsp, 16
	pop	rbx
	pop	r14
	pop	rbp
	ret

.subsections_via_symbols
