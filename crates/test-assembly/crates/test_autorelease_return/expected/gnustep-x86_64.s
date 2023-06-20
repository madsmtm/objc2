	.text
	.intel_syntax noprefix
	.section	.text.simple,"ax",@progbits
	.globl	simple
	.p2align	4, 0x90
	.type	simple,@function
simple:
	jmp	qword ptr [rip + objc_autoreleaseReturnValue@GOTPCREL]
.Lfunc_end0:
	.size	simple, .Lfunc_end0-simple

	.section	.text.with_body,"ax",@progbits
	.globl	with_body
	.p2align	4, 0x90
	.type	with_body,@function
with_body:
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
	jmp	qword ptr [rip + objc_autoreleaseReturnValue@GOTPCREL]
.Lfunc_end1:
	.size	with_body, .Lfunc_end1-with_body

	.section	".note.GNU-stack","",@progbits
