	.text
	.intel_syntax noprefix
	.section	.text.handle_alloc,"ax",@progbits
	.globl	handle_alloc
	.p2align	4, 0x90
	.type	handle_alloc,@function
handle_alloc:
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
	.size	handle_alloc, .Lfunc_end0-handle_alloc

	.section	.text.handle_init,"ax",@progbits
	.globl	handle_init
	.p2align	4, 0x90
	.type	handle_init,@function
handle_init:
	push	r14
	push	rbx
	push	rax
	test	rdi, rdi
	je	.LBB1_3
	mov	r14, rsi
	mov	rbx, rdi
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	test	rax, rax
	je	.LBB1_4
	mov	rdi, rbx
	mov	rsi, r14
	add	rsp, 8
	pop	rbx
	pop	r14
	jmp	rax
.LBB1_3:
	xor	eax, eax
	add	rsp, 8
	pop	rbx
	pop	r14
	ret
.LBB1_4:
	lea	rdi, [rip + .L__unnamed_1]
	lea	rdx, [rip + .L__unnamed_3]
	mov	esi, 8
	call	qword ptr [rip + SYM(core::option::expect_failed::GENERATED_ID, 0)@GOTPCREL]
	ud2
.Lfunc_end1:
	.size	handle_init, .Lfunc_end1-handle_init

	.section	.text.handle_alloc_init,"ax",@progbits
	.globl	handle_alloc_init
	.p2align	4, 0x90
	.type	handle_alloc_init,@function
handle_alloc_init:
	push	r15
	push	r14
	push	rbx
	mov	r14, rdx
	mov	r15, rsi
	mov	rbx, rdi
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	test	rax, rax
	je	.LBB2_1
	mov	rdi, rbx
	mov	rsi, r15
	call	rax
	test	rax, rax
	je	.LBB2_7
	mov	rbx, rax
	mov	rdi, rax
	mov	rsi, r14
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	test	rax, rax
	je	.LBB2_5
	mov	rdi, rbx
	mov	rsi, r14
	pop	rbx
	pop	r14
	pop	r15
	jmp	rax
.LBB2_7:
	xor	eax, eax
	pop	rbx
	pop	r14
	pop	r15
	ret
.LBB2_1:
	lea	rdi, [rip + .L__unnamed_1]
	lea	rdx, [rip + .L__unnamed_4]
	jmp	.LBB2_2
.LBB2_5:
	lea	rdi, [rip + .L__unnamed_1]
	lea	rdx, [rip + .L__unnamed_5]
.LBB2_2:
	mov	esi, 8
	call	qword ptr [rip + SYM(core::option::expect_failed::GENERATED_ID, 0)@GOTPCREL]
	ud2
.Lfunc_end2:
	.size	handle_alloc_init, .Lfunc_end2-handle_alloc_init

	.section	.text.handle_alloc_release,"ax",@progbits
	.globl	handle_alloc_release
	.p2align	4, 0x90
	.type	handle_alloc_release,@function
handle_alloc_release:
	push	r14
	push	rbx
	push	rax
	mov	r14, rsi
	mov	rbx, rdi
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	test	rax, rax
	je	.LBB3_1
	mov	rdi, rbx
	mov	rsi, r14
	call	rax
	mov	rdi, rax
	add	rsp, 8
	pop	rbx
	pop	r14
	jmp	qword ptr [rip + objc_release@GOTPCREL]
.LBB3_1:
	lea	rdi, [rip + .L__unnamed_1]
	lea	rdx, [rip + .L__unnamed_6]
	mov	esi, 8
	call	qword ptr [rip + SYM(core::option::expect_failed::GENERATED_ID, 0)@GOTPCREL]
	ud2
.Lfunc_end3:
	.size	handle_alloc_release, .Lfunc_end3-handle_alloc_release

	.section	.text.handle_alloc_init_release,"ax",@progbits
	.globl	handle_alloc_init_release
	.p2align	4, 0x90
	.type	handle_alloc_init_release,@function
handle_alloc_init_release:
	push	r15
	push	r14
	push	rbx
	mov	r14, rdx
	mov	r15, rsi
	mov	rbx, rdi
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	test	rax, rax
	je	.LBB4_1
	mov	rdi, rbx
	mov	rsi, r15
	call	rax
	test	rax, rax
	je	.LBB4_4
	mov	rbx, rax
	mov	rdi, rax
	mov	rsi, r14
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	test	rax, rax
	je	.LBB4_6
	mov	rdi, rbx
	mov	rsi, r14
	call	rax
	mov	rdi, rax
	pop	rbx
	pop	r14
	pop	r15
	jmp	qword ptr [rip + objc_release@GOTPCREL]
.LBB4_4:
	xor	edi, edi
	pop	rbx
	pop	r14
	pop	r15
	jmp	qword ptr [rip + objc_release@GOTPCREL]
.LBB4_1:
	lea	rdi, [rip + .L__unnamed_1]
	lea	rdx, [rip + .L__unnamed_7]
	jmp	.LBB4_2
.LBB4_6:
	lea	rdi, [rip + .L__unnamed_1]
	lea	rdx, [rip + .L__unnamed_8]
.LBB4_2:
	mov	esi, 8
	call	qword ptr [rip + SYM(core::option::expect_failed::GENERATED_ID, 0)@GOTPCREL]
	ud2
.Lfunc_end4:
	.size	handle_alloc_init_release, .Lfunc_end4-handle_alloc_init_release

	.section	.text.handle_copy,"ax",@progbits
	.globl	handle_copy
	.p2align	4, 0x90
	.type	handle_copy,@function
handle_copy:
	push	r14
	push	rbx
	push	rax
	mov	r14, rsi
	mov	rbx, rdi
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	test	rax, rax
	je	.LBB5_1
	mov	rdi, rbx
	mov	rsi, r14
	add	rsp, 8
	pop	rbx
	pop	r14
	jmp	rax
.LBB5_1:
	lea	rdi, [rip + .L__unnamed_1]
	lea	rdx, [rip + .L__unnamed_9]
	mov	esi, 8
	call	qword ptr [rip + SYM(core::option::expect_failed::GENERATED_ID, 0)@GOTPCREL]
	ud2
.Lfunc_end5:
	.size	handle_copy, .Lfunc_end5-handle_copy

	.section	.text.handle_autoreleased,"ax",@progbits
	.globl	handle_autoreleased
	.p2align	4, 0x90
	.type	handle_autoreleased,@function
handle_autoreleased:
	push	r14
	push	rbx
	push	rax
	mov	r14, rsi
	mov	rbx, rdi
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	test	rax, rax
	je	.LBB6_1
	mov	rdi, rbx
	mov	rsi, r14
	call	rax
	mov	rdi, rax
	add	rsp, 8
	pop	rbx
	pop	r14
	jmp	qword ptr [rip + objc_retainAutoreleasedReturnValue@GOTPCREL]
.LBB6_1:
	lea	rdi, [rip + .L__unnamed_1]
	lea	rdx, [rip + .L__unnamed_10]
	mov	esi, 8
	call	qword ptr [rip + SYM(core::option::expect_failed::GENERATED_ID, 0)@GOTPCREL]
	ud2
.Lfunc_end6:
	.size	handle_autoreleased, .Lfunc_end6-handle_autoreleased

	.type	.L__unnamed_1,@object
	.section	.rodata.cst8,"aM",@progbits,8
.L__unnamed_1:
	.ascii	"Null IMP"
	.size	.L__unnamed_1, 8

	.type	.L__unnamed_11,@object
	.section	.rodata..L__unnamed_11,"a",@progbits
.L__unnamed_11:
	.ascii	"test-assembly/crates/test_msg_send_id/lib.rs"
	.size	.L__unnamed_11, 44

	.type	.L__unnamed_2,@object
	.section	.data.rel.ro..L__unnamed_2,"aw",@progbits
	.p2align	3
.L__unnamed_2:
	.quad	.L__unnamed_11
	.asciz	",\000\000\000\000\000\000\000\b\000\000\000\005\000\000"
	.size	.L__unnamed_2, 24

	.type	.L__unnamed_3,@object
	.section	.data.rel.ro..L__unnamed_3,"aw",@progbits
	.p2align	3
.L__unnamed_3:
	.quad	.L__unnamed_11
	.asciz	",\000\000\000\000\000\000\000\020\000\000\000\005\000\000"
	.size	.L__unnamed_3, 24

	.type	.L__unnamed_4,@object
	.section	.data.rel.ro..L__unnamed_4,"aw",@progbits
	.p2align	3
.L__unnamed_4:
	.quad	.L__unnamed_11
	.asciz	",\000\000\000\000\000\000\000\025\000\000\000\017\000\000"
	.size	.L__unnamed_4, 24

	.type	.L__unnamed_5,@object
	.section	.data.rel.ro..L__unnamed_5,"aw",@progbits
	.p2align	3
.L__unnamed_5:
	.quad	.L__unnamed_11
	.asciz	",\000\000\000\000\000\000\000\026\000\000\000\005\000\000"
	.size	.L__unnamed_5, 24

	.type	.L__unnamed_6,@object
	.section	.data.rel.ro..L__unnamed_6,"aw",@progbits
	.p2align	3
.L__unnamed_6:
	.quad	.L__unnamed_11
	.asciz	",\000\000\000\000\000\000\000\034\000\000\000\t\000\000"
	.size	.L__unnamed_6, 24

	.type	.L__unnamed_7,@object
	.section	.data.rel.ro..L__unnamed_7,"aw",@progbits
	.p2align	3
.L__unnamed_7:
	.quad	.L__unnamed_11
	.asciz	",\000\000\000\000\000\000\000\"\000\000\000\017\000\000"
	.size	.L__unnamed_7, 24

	.type	.L__unnamed_8,@object
	.section	.data.rel.ro..L__unnamed_8,"aw",@progbits
	.p2align	3
.L__unnamed_8:
	.quad	.L__unnamed_11
	.asciz	",\000\000\000\000\000\000\000$\000\000\000\t\000\000"
	.size	.L__unnamed_8, 24

	.type	.L__unnamed_9,@object
	.section	.data.rel.ro..L__unnamed_9,"aw",@progbits
	.p2align	3
.L__unnamed_9:
	.quad	.L__unnamed_11
	.asciz	",\000\000\000\000\000\000\000*\000\000\000\005\000\000"
	.size	.L__unnamed_9, 24

	.type	.L__unnamed_10,@object
	.section	.data.rel.ro..L__unnamed_10,"aw",@progbits
	.p2align	3
.L__unnamed_10:
	.quad	.L__unnamed_11
	.asciz	",\000\000\000\000\000\000\000/\000\000\000\005\000\000"
	.size	.L__unnamed_10, 24

	.section	".note.GNU-stack","",@progbits
