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
	call	SYM(objc2::__macro_helpers::retain_semantics::new_fail::GENERATED_ID, 0)

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
	call	SYM(objc2::__macro_helpers::retain_semantics::init_fail::GENERATED_ID, 0)

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
	call	SYM(objc2::__macro_helpers::retain_semantics::copy_fail::GENERATED_ID, 0)

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
	call	SYM(objc2::__macro_helpers::retain_semantics::mutable_copy_fail::GENERATED_ID, 0)

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
	call	SYM(objc2::__macro_helpers::retain_semantics::none_fail::GENERATED_ID, 0)

	.globl	_handle_with_out_param
	.p2align	4, 0x90
_handle_with_out_param:
Lfunc_begin0:
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	rbx
	push	rax
	mov	r14, rdx
	mov	rbx, qword ptr [rdx]
Ltmp0:
	call	_objc_msgSend
Ltmp1:
Ltmp2:
	mov	rdi, rax
	call	_objc_retainAutoreleasedReturnValue
Ltmp3:
	mov	r15, rax
	## InlineAsm Start

	nop

	## InlineAsm End
	mov	rdi, qword ptr [r14]
	call	_objc_retain
	mov	rdi, rbx
	call	_objc_release
	mov	rax, r15
	add	rsp, 8
	pop	rbx
	pop	r14
	pop	r15
	pop	rbp
	ret
LBB15_3:
Ltmp4:
	mov	r15, rax
	mov	rdi, qword ptr [r14]
Ltmp5:
	call	_objc_retain
Ltmp6:
Ltmp7:
	mov	rdi, rbx
	call	_objc_release
Ltmp8:
	mov	rdi, r15
	call	__Unwind_Resume
LBB15_6:
Ltmp9:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
Lfunc_end0:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table15:
Lexception0:
	.byte	255
	.byte	155
	.uleb128 Lttbase0-Lttbaseref0
Lttbaseref0:
	.byte	1
	.uleb128 Lcst_end0-Lcst_begin0
Lcst_begin0:
	.uleb128 Ltmp0-Lfunc_begin0
	.uleb128 Ltmp3-Ltmp0
	.uleb128 Ltmp4-Lfunc_begin0
	.byte	0
	.uleb128 Ltmp3-Lfunc_begin0
	.uleb128 Ltmp5-Ltmp3
	.byte	0
	.byte	0
	.uleb128 Ltmp5-Lfunc_begin0
	.uleb128 Ltmp8-Ltmp5
	.uleb128 Ltmp9-Lfunc_begin0
	.byte	1
	.uleb128 Ltmp8-Lfunc_begin0
	.uleb128 Lfunc_end0-Ltmp8
	.byte	0
	.byte	0
Lcst_end0:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase0:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__const
l_anon.[ID].0:
	.ascii	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].1:
	.quad	l_anon.[ID].0
	.asciz	"9\000\000\000\000\000\000\000\017\000\000\000\005\000\000"

	.p2align	3, 0x0
l_anon.[ID].2:
	.quad	l_anon.[ID].0
	.asciz	"9\000\000\000\000\000\000\000\036\000\000\000\005\000\000"

	.p2align	3, 0x0
l_anon.[ID].3:
	.quad	l_anon.[ID].0
	.asciz	"9\000\000\000\000\000\000\000:\000\000\000\005\000\000"

	.p2align	3, 0x0
l_anon.[ID].4:
	.quad	l_anon.[ID].0
	.asciz	"9\000\000\000\000\000\000\000D\000\000\000\005\000\000"

	.p2align	3, 0x0
l_anon.[ID].5:
	.quad	l_anon.[ID].0
	.asciz	"9\000\000\000\000\000\000\000X\000\000\000\005\000\000"

.subsections_via_symbols
