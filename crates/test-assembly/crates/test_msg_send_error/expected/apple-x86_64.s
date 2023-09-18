	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.p2align	4, 0x90
SYM(objc2[CRATE_ID]::__macro_helpers::msg_send_retained::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0):
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	rbx, rsi
	call	_objc_retain
	test	rax, rax
	je	LBB0_2
	add	rsp, 8
	pop	rbx
	pop	rbp
	ret
LBB0_2:
	lea	rdi, [rip + l_anon.[ID].0]
	mov	esi, 56
	mov	rdx, rbx
	call	SYM(core::option::expect_failed::GENERATED_ID, 0)

	.p2align	4, 0x90
SYM(objc2[CRATE_ID]::__macro_helpers::msg_send::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0):
	push	rbp
	mov	rbp, rsp
	call	_objc_retain
	test	rax, rax
	je	LBB1_2
	pop	rbp
	ret
LBB1_2:
	lea	rdi, [rip + l_anon.[ID].1]
	lea	rdx, [rip + l_anon.[ID].3]
	mov	esi, 54
	call	SYM(core::option::expect_failed::GENERATED_ID, 0)

	.globl	_error_bool
	.p2align	4, 0x90
_error_bool:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	qword ptr [rbp - 8], 0
	lea	rcx, [rbp - 8]
	call	_objc_msgSend
	test	al, al
	je	LBB2_2
	xor	eax, eax
	add	rsp, 16
	pop	rbp
	ret
LBB2_2:
	mov	rdi, qword ptr [rbp - 8]
	call	SYM(objc2[CRATE_ID]::__macro_helpers::msg_send::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0)
	add	rsp, 16
	pop	rbp
	ret

	.globl	_error_new
	.p2align	4, 0x90
_error_new:
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
	add	rsp, 16
	pop	rbp
	ret
LBB3_2:
	mov	rdi, qword ptr [rbp - 8]
	lea	rsi, [rip + l_anon.[ID].4]
	call	SYM(objc2[CRATE_ID]::__macro_helpers::msg_send_retained::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0)
	mov	rdx, rax
	mov	eax, 1
	add	rsp, 16
	pop	rbp
	ret

	.globl	_error_init
	.p2align	4, 0x90
_error_init:
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
	add	rsp, 16
	pop	rbp
	ret
LBB4_2:
	mov	rdi, qword ptr [rbp - 8]
	lea	rsi, [rip + l_anon.[ID].5]
	call	SYM(objc2[CRATE_ID]::__macro_helpers::msg_send_retained::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0)
	mov	rdx, rax
	mov	eax, 1
	add	rsp, 16
	pop	rbp
	ret

	.globl	_error_copy
	.p2align	4, 0x90
_error_copy:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	qword ptr [rbp - 8], 0
	lea	rdx, [rbp - 8]
	call	_objc_msgSend
	test	rax, rax
	je	LBB5_2
	mov	rdx, rax
	xor	eax, eax
	add	rsp, 16
	pop	rbp
	ret
LBB5_2:
	mov	rdi, qword ptr [rbp - 8]
	lea	rsi, [rip + l_anon.[ID].6]
	call	SYM(objc2[CRATE_ID]::__macro_helpers::msg_send_retained::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0)
	mov	rdx, rax
	mov	eax, 1
	add	rsp, 16
	pop	rbp
	ret

	.globl	_error_mutable_copy
	.p2align	4, 0x90
_error_mutable_copy:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	qword ptr [rbp - 8], 0
	lea	rdx, [rbp - 8]
	call	_objc_msgSend
	test	rax, rax
	je	LBB6_2
	mov	rdx, rax
	xor	eax, eax
	add	rsp, 16
	pop	rbp
	ret
LBB6_2:
	mov	rdi, qword ptr [rbp - 8]
	lea	rsi, [rip + l_anon.[ID].7]
	call	SYM(objc2[CRATE_ID]::__macro_helpers::msg_send_retained::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0)
	mov	rdx, rax
	mov	eax, 1
	add	rsp, 16
	pop	rbp
	ret

	.globl	_error_autoreleased
	.p2align	4, 0x90
_error_autoreleased:
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
	je	LBB7_2
	mov	rdx, rax
	xor	eax, eax
	add	rsp, 16
	pop	rbp
	ret
LBB7_2:
	mov	rdi, qword ptr [rbp - 8]
	lea	rsi, [rip + l_anon.[ID].8]
	call	SYM(objc2[CRATE_ID]::__macro_helpers::msg_send_retained::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0)
	mov	rdx, rax
	mov	eax, 1
	add	rsp, 16
	pop	rbp
	ret

	.section	__TEXT,__const
l_anon.[ID].0:
	.ascii	"error parameter should be set if the method returns NULL"

l_anon.[ID].1:
	.ascii	"error parameter should be set if the method returns NO"

l_anon.[ID].2:
	.ascii	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].3:
	.quad	l_anon.[ID].2
	.asciz	"6\000\000\000\000\000\000\000\n\000\000\000\005\000\000"

	.p2align	3, 0x0
l_anon.[ID].4:
	.quad	l_anon.[ID].2
	.asciz	"6\000\000\000\000\000\000\000\017\000\000\000\005\000\000"

	.p2align	3, 0x0
l_anon.[ID].5:
	.quad	l_anon.[ID].2
	.asciz	"6\000\000\000\000\000\000\000\026\000\000\000\005\000\000"

	.p2align	3, 0x0
l_anon.[ID].6:
	.quad	l_anon.[ID].2
	.asciz	"6\000\000\000\000\000\000\000\033\000\000\000\005\000\000"

	.p2align	3, 0x0
l_anon.[ID].7:
	.quad	l_anon.[ID].2
	.asciz	"6\000\000\000\000\000\000\000 \000\000\000\005\000\000"

	.p2align	3, 0x0
l_anon.[ID].8:
	.quad	l_anon.[ID].2
	.asciz	"6\000\000\000\000\000\000\000%\000\000\000\005\000\000"

.subsections_via_symbols
