	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_handle_new
	.p2align	4, 0x90
_handle_new:
	push	rbp
	mov	rbp, rsp
	pop	rbp
	jmp	_objc_msgSend

	.globl	_handle_new_fallible
	.p2align	4, 0x90
_handle_new_fallible:
	push	rbp
	mov	rbp, rsp
	push	r14
	push	rbx
	mov	r14, rsi
	mov	rbx, rdi
	call	_objc_msgSend
	test	rax, rax
	je	LBB1_2
	pop	rbx
	pop	r14
	pop	rbp
	ret
LBB1_2:
	mov	rdi, rbx
	mov	rsi, r14
	call	SYM(objc2::__macro_helpers::new_failed::GENERATED_ID, 0)

	.globl	_handle_alloc
	.p2align	4, 0x90
_handle_alloc:
	push	rbp
	mov	rbp, rsp
	pop	rbp
	jmp	_objc_msgSend

	.globl	_handle_alloc_fallible
	.p2align	4, 0x90
_handle_alloc_fallible:
	push	rbp
	mov	rbp, rsp
	push	r14
	push	rbx
	mov	r14, rsi
	mov	rbx, rdi
	call	_objc_msgSend
	test	rax, rax
	je	LBB3_2
	pop	rbx
	pop	r14
	pop	rbp
	ret
LBB3_2:
	mov	rdi, rbx
	mov	rsi, r14
	call	SYM(objc2::__macro_helpers::alloc_failed::GENERATED_ID, 0)

	.globl	_handle_init
	.p2align	4, 0x90
_handle_init:
	push	rbp
	mov	rbp, rsp
	pop	rbp
	jmp	_objc_msgSend

	.globl	_handle_init_fallible
	.p2align	4, 0x90
_handle_init_fallible:
	push	rbp
	mov	rbp, rsp
	push	r14
	push	rbx
	mov	r14, rsi
	mov	rbx, rdi
	call	_objc_msgSend
	test	rax, rax
	je	LBB5_2
	pop	rbx
	pop	r14
	pop	rbp
	ret
LBB5_2:
	mov	rdi, rbx
	mov	rsi, r14
	call	SYM(objc2::__macro_helpers::init_failed::GENERATED_ID, 0)

	.globl	_handle_alloc_init
	.p2align	4, 0x90
_handle_alloc_init:
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	rbx, rdx
	call	_objc_msgSend
	mov	rdi, rax
	mov	rsi, rbx
	add	rsp, 8
	pop	rbx
	pop	rbp
	jmp	_objc_msgSend

	.globl	_handle_alloc_release
	.p2align	4, 0x90
_handle_alloc_release:
	push	rbp
	mov	rbp, rsp
	call	_objc_msgSend
	mov	rdi, rax
	pop	rbp
	jmp	_objc_release

	.globl	_handle_alloc_init_release
	.p2align	4, 0x90
_handle_alloc_init_release:
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	rbx, rdx
	call	_objc_msgSend
	mov	rdi, rax
	mov	rsi, rbx
	call	_objc_msgSend
	mov	rdi, rax
	add	rsp, 8
	pop	rbx
	pop	rbp
	jmp	_objc_release

	.globl	_handle_copy
	.p2align	4, 0x90
_handle_copy:
	push	rbp
	mov	rbp, rsp
	pop	rbp
	jmp	_objc_msgSend

	.globl	_handle_copy_fallible
	.p2align	4, 0x90
_handle_copy_fallible:
	push	rbp
	mov	rbp, rsp
	call	_objc_msgSend
	test	rax, rax
	je	LBB10_2
	pop	rbp
	ret
LBB10_2:
	call	SYM(objc2::__macro_helpers::copy_failed::GENERATED_ID, 0)

	.globl	_handle_autoreleased
	.p2align	4, 0x90
_handle_autoreleased:
	push	rbp
	mov	rbp, rsp
	call	_objc_msgSend
	mov	rdi, rax
	call	_objc_retainAutoreleasedReturnValue
	## InlineAsm Start

	nop

	## InlineAsm End
	pop	rbp
	ret

	.globl	_handle_autoreleased_fallible
	.p2align	4, 0x90
_handle_autoreleased_fallible:
	push	rbp
	mov	rbp, rsp
	push	r14
	push	rbx
	mov	r14, rsi
	mov	rbx, rdi
	call	_objc_msgSend
	mov	rdi, rax
	call	_objc_retainAutoreleasedReturnValue
	## InlineAsm Start

	nop

	## InlineAsm End
	test	rax, rax
	je	LBB12_2
	pop	rbx
	pop	r14
	pop	rbp
	ret
LBB12_2:
	mov	rdi, rbx
	mov	rsi, r14
	call	SYM(objc2::__macro_helpers::normal_failed::GENERATED_ID, 0)

.subsections_via_symbols
