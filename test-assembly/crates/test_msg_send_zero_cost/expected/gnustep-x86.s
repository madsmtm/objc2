	.text
	.intel_syntax noprefix
	.section	.text.handle,"ax",@progbits
	.globl	handle
	.p2align	4, 0x90
	.type	handle,@function
handle:
	push	ebx
	push	edi
	push	esi
	mov	esi, dword ptr [esp + 16]
	mov	edi, dword ptr [esp + 20]
	call	.L0$pb
.L0$pb:
	pop	ebx
.Ltmp0:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp0-.L0$pb)
	sub	esp, 8
	push	edi
	push	esi
	call	objc_msg_lookup@PLT
	add	esp, 8
	push	edi
	push	esi
	call	eax
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.Lfunc_end0:
	.size	handle, .Lfunc_end0-handle

	.section	.text.handle_with_sel,"ax",@progbits
	.globl	handle_with_sel
	.p2align	4, 0x90
	.type	handle_with_sel,@function
handle_with_sel:
	push	ebx
	push	edi
	push	esi
	call	.L1$pb
.L1$pb:
	pop	ebx
	mov	esi, dword ptr [esp + 16]
.Ltmp1:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp1-.L1$pb)
	mov	eax, dword ptr [ebx + SEL_REF@GOT]
	mov	edi, dword ptr [eax]
	sub	esp, 8
	push	edi
	push	esi
	call	objc_msg_lookup@PLT
	add	esp, 8
	push	edi
	push	esi
	call	eax
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
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
	.p2align	2
SEL_REF:
	.long	SEL
	.size	SEL_REF, 4

	.section	".note.GNU-stack","",@progbits
