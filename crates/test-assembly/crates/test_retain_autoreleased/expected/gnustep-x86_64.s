	.text
	.intel_syntax noprefix
	.section	.text.handle,"ax",@progbits
	.globl	handle
	.p2align	4, 0x90
	.type	handle,@function
handle:
	push	r14
	push	rbx
	push	rax
	mov	rbx, rsi
	mov	r14, rdi
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rdi, r14
	mov	rsi, rbx
	call	rax
	mov	rdi, rax
	add	rsp, 8
	pop	rbx
	pop	r14
	jmp	qword ptr [rip + objc_retainAutoreleasedReturnValue@GOTPCREL]
.Lfunc_end0:
	.size	handle, .Lfunc_end0-handle

	.section	".note.GNU-stack","",@progbits
