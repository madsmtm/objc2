	.text
	.intel_syntax noprefix
	.section	.text.handle_new,"ax",@progbits
	.globl	handle_new
	.p2align	4, 0x90
	.type	handle_new,@function
handle_new:
	push	r14
	push	rbx
	push	rax
	mov	r14, rsi
	mov	rbx, rdi
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rdi, rbx
	mov	rsi, r14
	add	rsp, 8
	pop	rbx
	pop	r14
	jmp	rax
.Lfunc_end0:
	.size	handle_new, .Lfunc_end0-handle_new

	.section	.text.handle_new_fallible,"ax",@progbits
	.globl	handle_new_fallible
	.p2align	4, 0x90
	.type	handle_new_fallible,@function
handle_new_fallible:
	push	r14
	push	rbx
	push	rax
	mov	r14, rsi
	mov	rbx, rdi
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rdi, rbx
	mov	rsi, r14
	call	rax
	test	rax, rax
	je	.LBB1_1
	add	rsp, 8
	pop	rbx
	pop	r14
	ret
.LBB1_1:
	mov	rdi, rbx
	mov	rsi, r14
	call	qword ptr [rip + SYM(objc2::__macro_helpers::new_failed::GENERATED_ID, 0)@GOTPCREL]
	ud2
.Lfunc_end1:
	.size	handle_new_fallible, .Lfunc_end1-handle_new_fallible

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
	mov	rdi, rbx
	mov	rsi, r14
	add	rsp, 8
	pop	rbx
	pop	r14
	jmp	rax
.Lfunc_end2:
	.size	handle_alloc, .Lfunc_end2-handle_alloc

	.section	.text.handle_alloc_fallible,"ax",@progbits
	.globl	handle_alloc_fallible
	.p2align	4, 0x90
	.type	handle_alloc_fallible,@function
handle_alloc_fallible:
	push	r14
	push	rbx
	push	rax
	mov	r14, rsi
	mov	rbx, rdi
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rdi, rbx
	mov	rsi, r14
	call	rax
	test	rax, rax
	je	.LBB3_1
	add	rsp, 8
	pop	rbx
	pop	r14
	ret
.LBB3_1:
	mov	rdi, rbx
	mov	rsi, r14
	call	qword ptr [rip + SYM(objc2::__macro_helpers::alloc_failed::GENERATED_ID, 0)@GOTPCREL]
	ud2
.Lfunc_end3:
	.size	handle_alloc_fallible, .Lfunc_end3-handle_alloc_fallible

	.section	.text.handle_init,"ax",@progbits
	.globl	handle_init
	.p2align	4, 0x90
	.type	handle_init,@function
handle_init:
	test	rdi, rdi
	je	.LBB4_1
	push	r14
	push	rbx
	push	rax
	mov	r14, rsi
	mov	rbx, rdi
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rdi, rbx
	mov	rsi, r14
	add	rsp, 8
	pop	rbx
	pop	r14
	jmp	rax
.LBB4_1:
	xor	eax, eax
	ret
.Lfunc_end4:
	.size	handle_init, .Lfunc_end4-handle_init

	.section	.text.handle_init_fallible,"ax",@progbits
	.globl	handle_init_fallible
	.p2align	4, 0x90
	.type	handle_init_fallible,@function
handle_init_fallible:
	push	r14
	push	rbx
	push	rax
	mov	r14, rsi
	mov	rbx, rdi
	test	rdi, rdi
	je	.LBB5_2
	mov	rdi, rbx
	mov	rsi, r14
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rdi, rbx
	mov	rsi, r14
	call	rax
	test	rax, rax
	je	.LBB5_2
	add	rsp, 8
	pop	rbx
	pop	r14
	ret
.LBB5_2:
	mov	rdi, rbx
	mov	rsi, r14
	call	qword ptr [rip + SYM(objc2::__macro_helpers::init_failed::GENERATED_ID, 0)@GOTPCREL]
	ud2
.Lfunc_end5:
	.size	handle_init_fallible, .Lfunc_end5-handle_init_fallible

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
	mov	rdi, rbx
	mov	rsi, r15
	call	rax
	test	rax, rax
	je	.LBB6_1
	mov	rbx, rax
	mov	rdi, rax
	mov	rsi, r14
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rdi, rbx
	mov	rsi, r14
	pop	rbx
	pop	r14
	pop	r15
	jmp	rax
.LBB6_1:
	xor	eax, eax
	pop	rbx
	pop	r14
	pop	r15
	ret
.Lfunc_end6:
	.size	handle_alloc_init, .Lfunc_end6-handle_alloc_init

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
	mov	rdi, rbx
	mov	rsi, r14
	call	rax
	mov	rdi, rax
	add	rsp, 8
	pop	rbx
	pop	r14
	jmp	qword ptr [rip + objc_release@GOTPCREL]
.Lfunc_end7:
	.size	handle_alloc_release, .Lfunc_end7-handle_alloc_release

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
	mov	rdi, rbx
	mov	rsi, r15
	call	rax
	test	rax, rax
	je	.LBB8_1
	mov	rbx, rax
	mov	rdi, rax
	mov	rsi, r14
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rdi, rbx
	mov	rsi, r14
	call	rax
	mov	rdi, rax
	pop	rbx
	pop	r14
	pop	r15
	jmp	qword ptr [rip + objc_release@GOTPCREL]
.LBB8_1:
	xor	edi, edi
	pop	rbx
	pop	r14
	pop	r15
	jmp	qword ptr [rip + objc_release@GOTPCREL]
.Lfunc_end8:
	.size	handle_alloc_init_release, .Lfunc_end8-handle_alloc_init_release

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
	mov	rdi, rbx
	mov	rsi, r14
	add	rsp, 8
	pop	rbx
	pop	r14
	jmp	rax
.Lfunc_end9:
	.size	handle_copy, .Lfunc_end9-handle_copy

	.section	.text.handle_copy_fallible,"ax",@progbits
	.globl	handle_copy_fallible
	.p2align	4, 0x90
	.type	handle_copy_fallible,@function
handle_copy_fallible:
	push	r14
	push	rbx
	push	rax
	mov	r14, rsi
	mov	rbx, rdi
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rdi, rbx
	mov	rsi, r14
	call	rax
	test	rax, rax
	je	.LBB10_1
	add	rsp, 8
	pop	rbx
	pop	r14
	ret
.LBB10_1:
	call	qword ptr [rip + SYM(objc2::__macro_helpers::copy_failed::GENERATED_ID, 0)@GOTPCREL]
	ud2
.Lfunc_end10:
	.size	handle_copy_fallible, .Lfunc_end10-handle_copy_fallible

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
	mov	rdi, rbx
	mov	rsi, r14
	call	rax
	mov	rdi, rax
	add	rsp, 8
	pop	rbx
	pop	r14
	jmp	qword ptr [rip + objc_retainAutoreleasedReturnValue@GOTPCREL]
.Lfunc_end11:
	.size	handle_autoreleased, .Lfunc_end11-handle_autoreleased

	.section	.text.handle_autoreleased_fallible,"ax",@progbits
	.globl	handle_autoreleased_fallible
	.p2align	4, 0x90
	.type	handle_autoreleased_fallible,@function
handle_autoreleased_fallible:
	push	r14
	push	rbx
	push	rax
	mov	r14, rsi
	mov	rbx, rdi
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rdi, rbx
	mov	rsi, r14
	call	rax
	mov	rdi, rax
	call	qword ptr [rip + objc_retainAutoreleasedReturnValue@GOTPCREL]
	test	rax, rax
	je	.LBB12_1
	add	rsp, 8
	pop	rbx
	pop	r14
	ret
.LBB12_1:
	mov	rdi, rbx
	mov	rsi, r14
	call	qword ptr [rip + SYM(objc2::__macro_helpers::normal_failed::GENERATED_ID, 0)@GOTPCREL]
	ud2
.Lfunc_end12:
	.size	handle_autoreleased_fallible, .Lfunc_end12-handle_autoreleased_fallible

	.section	".note.GNU-stack","",@progbits
