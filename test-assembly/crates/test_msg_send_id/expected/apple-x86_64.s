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
	lea	rdx, [rip + l_anon.[ID].1]
	mov	rdi, rbx
	mov	rsi, r14
	call	SYM(<objc2::__macro_helpers::RetainSemantics<_,_,_,_> as objc2::__macro_helpers::MsgSendIdFailed>::failed::GENERATED_ID, 0)

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
	lea	rdx, [rip + l_anon.[ID].2]
	mov	rdi, rbx
	mov	rsi, r14
	call	SYM(<objc2::__macro_helpers::RetainSemantics<_,_,_,_> as objc2::__macro_helpers::MsgSendIdFailed>::failed::GENERATED_ID, 1)

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
	lea	rdx, [rip + l_anon.[ID].3]
	mov	rdi, rbx
	mov	rsi, r14
	call	SYM(<objc2::__macro_helpers::RetainSemantics<_,_,_,_> as objc2::__macro_helpers::MsgSendIdFailed>::failed::GENERATED_ID, 2)

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
	lea	rdi, [rip + l_anon.[ID].4]
	call	SYM(<objc2::__macro_helpers::RetainSemantics<_,_,_,_> as objc2::__macro_helpers::MsgSendIdFailed>::failed::GENERATED_ID, 3)

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
	lea	rdx, [rip + l_anon.[ID].5]
	mov	rdi, rbx
	mov	rsi, r14
	call	SYM(<objc2::__macro_helpers::RetainSemantics<_,_,_,_> as objc2::__macro_helpers::MsgSendIdFailed>::failed::GENERATED_ID, 4)

	.section	__TEXT,__const
l_anon.[ID].0:
	.ascii	"$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	3
l_anon.[ID].1:
	.quad	l_anon.[ID].0
	.asciz	",\000\000\000\000\000\000\000\r\000\000\000\005\000\000"

	.p2align	3
l_anon.[ID].2:
	.quad	l_anon.[ID].0
	.asciz	",\000\000\000\000\000\000\000\027\000\000\000\005\000\000"

	.p2align	3
l_anon.[ID].3:
	.quad	l_anon.[ID].0
	.asciz	",\000\000\000\000\000\000\000'\000\000\000\005\000\000"

	.p2align	3
l_anon.[ID].4:
	.quad	l_anon.[ID].0
	.asciz	",\000\000\000\000\000\000\000F\000\000\000\005\000\000"

	.p2align	3
l_anon.[ID].5:
	.quad	l_anon.[ID].0
	.asciz	",\000\000\000\000\000\000\000P\000\000\000\005\000\000"

.subsections_via_symbols
