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
	mov	rbx, rsi
	mov	r14, rdi
	call	_objc_msgSend
	test	rax, rax
	je	LBB1_2
	pop	rbx
	pop	r14
	pop	rbp
	ret
LBB1_2:
	lea	rdx, [rip + l_anon.[ID].1]
	mov	rdi, r14
	mov	rsi, rbx
	call	SYM(<objc2::__macro_helpers::method_family::RetainSemantics<1_u8> as objc2::__macro_helpers::msg_send_retained::MsgSendRetainedFailed>::failed::GENERATED_ID, 0)

	.globl	_handle_alloc
	.p2align	4, 0x90
_handle_alloc:
	push	rbp
	mov	rbp, rsp
	pop	rbp
	jmp	_objc_msgSend

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
	mov	rbx, rsi
	mov	r14, rdi
	call	_objc_msgSend
	test	rax, rax
	je	LBB4_2
	pop	rbx
	pop	r14
	pop	rbp
	ret
LBB4_2:
	lea	rdx, [rip + l_anon.[ID].2]
	mov	rdi, r14
	mov	rsi, rbx
	call	SYM(<objc2::__macro_helpers::method_family::RetainSemantics<3_u8> as objc2::__macro_helpers::msg_send_retained::MsgSendRetainedFailed>::failed::GENERATED_ID, 0)

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
	je	LBB9_2
	pop	rbp
	ret
LBB9_2:
	lea	rdi, [rip + l_anon.[ID].3]
	call	SYM(<objc2::__macro_helpers::method_family::RetainSemantics<4_u8> as objc2::__macro_helpers::msg_send_retained::MsgSendRetainedFailed>::failed::GENERATED_ID, 0)

	.globl	_handle_mutable_copy
	.p2align	4, 0x90
_handle_mutable_copy:
	push	rbp
	mov	rbp, rsp
	pop	rbp
	jmp	_objc_msgSend

	.globl	_handle_mutable_copy_fallible
	.p2align	4, 0x90
_handle_mutable_copy_fallible:
	push	rbp
	mov	rbp, rsp
	call	_objc_msgSend
	test	rax, rax
	je	LBB11_2
	pop	rbp
	ret
LBB11_2:
	lea	rdi, [rip + l_anon.[ID].4]
	call	SYM(<objc2::__macro_helpers::method_family::RetainSemantics<5_u8> as objc2::__macro_helpers::msg_send_retained::MsgSendRetainedFailed>::failed::GENERATED_ID, 0)

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

	.globl	_handle_autoreleased_with_arg
	.p2align	4, 0x90
_handle_autoreleased_with_arg:
	push	rbp
	mov	rbp, rsp
	movzx	edx, dl
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
	mov	rbx, rsi
	mov	r14, rdi
	call	_objc_msgSend
	mov	rdi, rax
	call	_objc_retainAutoreleasedReturnValue
	## InlineAsm Start

	nop

	## InlineAsm End
	test	rax, rax
	je	LBB14_2
	pop	rbx
	pop	r14
	pop	rbp
	ret
LBB14_2:
	lea	rdx, [rip + l_anon.[ID].5]
	mov	rdi, r14
	mov	rsi, rbx
	call	SYM(<objc2::__macro_helpers::method_family::RetainSemantics<6_u8> as objc2::__macro_helpers::msg_send_retained::MsgSendRetainedFailed>::failed::GENERATED_ID, 0)

	.globl	_handle_with_out_param
	.p2align	4, 0x90
_handle_with_out_param:
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	rbx
	push	rax
	mov	rbx, rdx
	mov	r14, qword ptr [rdx]
	call	_objc_msgSend
	mov	r15, rax
	mov	rdi, qword ptr [rbx]
	call	_objc_retain
	mov	rdi, r14
	call	_objc_release
	mov	rdi, r15
	call	_objc_retainAutoreleasedReturnValue
	## InlineAsm Start

	nop

	## InlineAsm End
	add	rsp, 8
	pop	rbx
	pop	r14
	pop	r15
	pop	rbp
	ret

	.section	__TEXT,__const
l_anon.[ID].0:
	.ascii	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].1:
	.quad	l_anon.[ID].0
	.asciz	"9\000\000\000\000\000\000\000\r\000\000\000\005\000\000"

	.p2align	3, 0x0
l_anon.[ID].2:
	.quad	l_anon.[ID].0
	.asciz	"9\000\000\000\000\000\000\000\034\000\000\000\005\000\000"

	.p2align	3, 0x0
l_anon.[ID].3:
	.quad	l_anon.[ID].0
	.asciz	"9\000\000\000\000\000\000\0008\000\000\000\005\000\000"

	.p2align	3, 0x0
l_anon.[ID].4:
	.quad	l_anon.[ID].0
	.asciz	"9\000\000\000\000\000\000\000B\000\000\000\005\000\000"

	.p2align	3, 0x0
l_anon.[ID].5:
	.quad	l_anon.[ID].0
	.asciz	"9\000\000\000\000\000\000\000V\000\000\000\005\000\000"

.subsections_via_symbols
