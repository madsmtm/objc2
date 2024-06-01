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
	add	rsp, 8
	pop	rbx
	pop	r14
	jmp	rax
.Lfunc_end0:
	.size	handle, .Lfunc_end0-handle

	.section	.text.handle_with_sel,"ax",@progbits
	.globl	handle_with_sel
	.p2align	4, 0x90
	.type	handle_with_sel,@function
handle_with_sel:
	push	r14
	push	rbx
	push	rax
	mov	rbx, rdi
	mov	rax, qword ptr [rip + SEL_REF@GOTPCREL]
	mov	r14, qword ptr [rax]
	mov	rsi, r14
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rdi, rbx
	mov	rsi, r14
	add	rsp, 8
	pop	rbx
	pop	r14
	jmp	rax
.Lfunc_end1:
	.size	handle_with_sel, .Lfunc_end1-handle_with_sel

	.type	SEL,@object
	.section	.rodata.SEL,"a",@progbits
	.globl	SEL
SEL:
	.asciz	"someSelector"
	.size	SEL, 13

	.type	SEL_REF,@object
	.section	.data.rel.ro.SEL_REF,"aw",@progbits
	.globl	SEL_REF
	.p2align	3, 0x0
SEL_REF:
	.quad	SEL
	.size	SEL_REF, 8

	.section	".note.GNU-stack","",@progbits
