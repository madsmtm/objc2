	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_fn1_error_bool
	.p2align	4
_fn1_error_bool:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	qword ptr [rbp - 8], 0
	lea	rcx, [rbp - 8]
	call	_objc_msgSend
	test	al, al
	je	LBB0_2
	xor	eax, eax
LBB0_4:
	add	rsp, 16
	pop	rbp
	ret
LBB0_2:
	mov	rdi, qword ptr [rbp - 8]
	call	_objc_retain
	test	rax, rax
	jne	LBB0_4
	call	SYM(objc2::__macros::msg_send::null_error::null_error::GENERATED_ID, 0)
	add	rsp, 16
	pop	rbp
	ret

	.globl	_fn2_error_new
	.p2align	4
_fn2_error_new:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	qword ptr [rbp - 8], 0
	lea	rdx, [rbp - 8]
	call	_objc_msgSend
	test	rax, rax
	je	LBB1_2
	mov	rdx, rax
	xor	eax, eax
LBB1_4:
	add	rsp, 16
	pop	rbp
	ret
LBB1_2:
	mov	rdi, qword ptr [rbp - 8]
	call	_objc_retain
	mov	rdx, rax
	mov	eax, 1
	test	rdx, rdx
	jne	LBB1_4
	call	SYM(objc2::__macros::msg_send::null_error::null_error::GENERATED_ID, 0)
	mov	rdx, rax
	mov	eax, 1
	add	rsp, 16
	pop	rbp
	ret

	.globl	_fn3_error_init
	.p2align	4
_fn3_error_init:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	qword ptr [rbp - 8], 0
	lea	rdx, [rbp - 8]
	call	_objc_msgSend
	test	rax, rax
	je	LBB2_2
	mov	rdx, rax
	xor	eax, eax
LBB2_4:
	add	rsp, 16
	pop	rbp
	ret
LBB2_2:
	mov	rdi, qword ptr [rbp - 8]
	call	_objc_retain
	mov	rdx, rax
	mov	eax, 1
	test	rdx, rdx
	jne	LBB2_4
	call	SYM(objc2::__macros::msg_send::null_error::null_error::GENERATED_ID, 0)
	mov	rdx, rax
	mov	eax, 1
	add	rsp, 16
	pop	rbp
	ret

	.globl	_fn4_error_copy
	.p2align	4
_fn4_error_copy:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	qword ptr [rbp - 8], 0
	lea	rdx, [rbp - 8]
	call	_objc_msgSend
	test	rax, rax
	je	LBB3_2
	mov	rdx, rax
	xor	eax, eax
LBB3_4:
	add	rsp, 16
	pop	rbp
	ret
LBB3_2:
	mov	rdi, qword ptr [rbp - 8]
	call	_objc_retain
	mov	rdx, rax
	mov	eax, 1
	test	rdx, rdx
	jne	LBB3_4
	call	SYM(objc2::__macros::msg_send::null_error::null_error::GENERATED_ID, 0)
	mov	rdx, rax
	mov	eax, 1
	add	rsp, 16
	pop	rbp
	ret

	.globl	_fn5_error_mutable_copy
	.p2align	4
_fn5_error_mutable_copy:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	qword ptr [rbp - 8], 0
	lea	rdx, [rbp - 8]
	call	_objc_msgSend
	test	rax, rax
	je	LBB4_2
	mov	rdx, rax
	xor	eax, eax
LBB4_4:
	add	rsp, 16
	pop	rbp
	ret
LBB4_2:
	mov	rdi, qword ptr [rbp - 8]
	call	_objc_retain
	mov	rdx, rax
	mov	eax, 1
	test	rdx, rdx
	jne	LBB4_4
	call	SYM(objc2::__macros::msg_send::null_error::null_error::GENERATED_ID, 0)
	mov	rdx, rax
	mov	eax, 1
	add	rsp, 16
	pop	rbp
	ret

	.globl	_fn6_error_autoreleased
	.p2align	4
_fn6_error_autoreleased:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	qword ptr [rbp - 8], 0
	lea	rdx, [rbp - 8]
	call	_objc_msgSend
	mov	rdi, rax
	call	_objc_retainAutoreleasedReturnValue
	## InlineAsm Start

	nop

	## InlineAsm End
	test	rax, rax
	je	LBB5_2
	mov	rdx, rax
	xor	eax, eax
LBB5_4:
	add	rsp, 16
	pop	rbp
	ret
LBB5_2:
	mov	rdi, qword ptr [rbp - 8]
	call	_objc_retain
	mov	rdx, rax
	mov	eax, 1
	test	rdx, rdx
	jne	LBB5_4
	call	SYM(objc2::__macros::msg_send::null_error::null_error::GENERATED_ID, 0)
	mov	rdx, rax
	mov	eax, 1
	add	rsp, 16
	pop	rbp
	ret

.subsections_via_symbols
