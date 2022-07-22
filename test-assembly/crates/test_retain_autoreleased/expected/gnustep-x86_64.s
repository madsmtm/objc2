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
	call	rax
	mov	rdi, rax
	add	rsp, 8
	pop	rbx
	pop	r14
	jmp	qword ptr [rip + objc_retainAutoreleasedReturnValue@GOTPCREL]
.LBB0_1:
	lea	rdi, [rip + .L__unnamed_1]
	lea	rdx, [rip + .L__unnamed_2]
	mov	esi, 8
	call	qword ptr [rip + SYM(core::option::expect_failed::GENERATED_ID, 0)@GOTPCREL]
	ud2
.Lfunc_end0:
	.size	handle, .Lfunc_end0-handle

	.type	.L__unnamed_1,@object
	.section	.rodata.cst8,"aM",@progbits,8
.L__unnamed_1:
	.ascii	"Null IMP"
	.size	.L__unnamed_1, 8

	.type	.L__unnamed_3,@object
	.section	.rodata..L__unnamed_3,"a",@progbits
.L__unnamed_3:
	.ascii	"test-assembly/crates/test_retain_autoreleased/lib.rs"
	.size	.L__unnamed_3, 52

	.type	.L__unnamed_2,@object
	.section	.data.rel.ro..L__unnamed_2,"aw",@progbits
	.p2align	3
.L__unnamed_2:
	.quad	.L__unnamed_3
	.asciz	"4\000\000\000\000\000\000\000\t\000\000\000\034\000\000"
	.size	.L__unnamed_2, 24

	.section	".note.GNU-stack","",@progbits
