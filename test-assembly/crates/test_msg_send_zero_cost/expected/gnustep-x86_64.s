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
	mov	r14, rsi
	mov	rbx, rdi
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	test	rax, rax
	je	.LBB0_1
	mov	rdi, rbx
	mov	rsi, r14
	add	rsp, 8
	pop	rbx
	pop	r14
	jmp	rax
.LBB0_1:
	lea	rdi, [rip + .L__unnamed_1]
	lea	rdx, [rip + .L__unnamed_2]
	mov	esi, 8
	call	qword ptr [rip + SYM(core::option::expect_failed::GENERATED_ID, 0)@GOTPCREL]
	ud2
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
	mov	r14, rdi
	mov	rax, qword ptr [rip + SEL_REF@GOTPCREL]
	mov	rbx, qword ptr [rax]
	mov	rsi, rbx
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	test	rax, rax
	je	.LBB1_1
	mov	rdi, r14
	mov	rsi, rbx
	add	rsp, 8
	pop	rbx
	pop	r14
	jmp	rax
.LBB1_1:
	lea	rdi, [rip + .L__unnamed_1]
	lea	rdx, [rip + .L__unnamed_3]
	mov	esi, 8
	call	qword ptr [rip + SYM(core::option::expect_failed::GENERATED_ID, 0)@GOTPCREL]
	ud2
.Lfunc_end1:
	.size	handle_with_sel, .Lfunc_end1-handle_with_sel

	.type	.L__unnamed_1,@object
	.section	.rodata.cst8,"aM",@progbits,8
.L__unnamed_1:
	.ascii	"Null IMP"
	.size	.L__unnamed_1, 8

	.type	.L__unnamed_4,@object
	.section	.rodata..L__unnamed_4,"a",@progbits
.L__unnamed_4:
	.ascii	"$DIR/lib.rs"
	.size	.L__unnamed_4, 51

	.type	.L__unnamed_2,@object
	.section	.data.rel.ro..L__unnamed_2,"aw",@progbits
	.p2align	3
.L__unnamed_2:
	.quad	.L__unnamed_4
	.asciz	"3\000\000\000\000\000\000\000\n\000\000\000\005\000\000"
	.size	.L__unnamed_2, 24

	.type	SEL,@object
	.section	.rodata.SEL,"a",@progbits
	.globl	SEL
SEL:
	.asciz	"someSelector"
	.size	SEL, 13

	.type	SEL_REF,@object
	.section	.data.rel.ro.SEL_REF,"aw",@progbits
	.globl	SEL_REF
	.p2align	3
SEL_REF:
	.quad	SEL
	.size	SEL_REF, 8

	.type	.L__unnamed_3,@object
	.section	.data.rel.ro..L__unnamed_3,"aw",@progbits
	.p2align	3
.L__unnamed_3:
	.quad	.L__unnamed_4
	.asciz	"3\000\000\000\000\000\000\000\032\000\000\000\005\000\000"
	.size	.L__unnamed_3, 24

	.section	".note.GNU-stack","",@progbits
