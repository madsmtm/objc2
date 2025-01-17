	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_handle_new
	.p2align	4, 0x90
_handle_new:
	push	ebp
	mov	ebp, esp
	pop	ebp
	jmp	_objc_msgSend

	.globl	_handle_new_fallible
	.p2align	4, 0x90
_handle_new_fallible:
	push	ebp
	mov	ebp, esp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	call	L1$pb
L1$pb:
	pop	ebx
	mov	edi, dword ptr [ebp + 12]
	mov	esi, dword ptr [ebp + 8]
	sub	esp, 8
	push	edi
	push	esi
	call	_objc_msgSend
	add	esp, 16
	test	eax, eax
	je	LBB1_2
	add	esp, 12
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
LBB1_2:
	sub	esp, 4
	lea	eax, [ebx + l_anon.[ID].1-L1$pb]
	push	eax
	push	edi
	push	esi
	call	SYM(<objc2::__macro_helpers::method_family::RetainSemantics<1_u8> as objc2::__macro_helpers::msg_send_retained::MsgSendRetainedFailed>::failed::GENERATED_ID, 0)

	.globl	_handle_alloc
	.p2align	4, 0x90
_handle_alloc:
	push	ebp
	mov	ebp, esp
	pop	ebp
	jmp	_objc_msgSend

	.globl	_handle_init
	.p2align	4, 0x90
_handle_init:
	push	ebp
	mov	ebp, esp
	pop	ebp
	jmp	_objc_msgSend

	.globl	_handle_init_fallible
	.p2align	4, 0x90
_handle_init_fallible:
	push	ebp
	mov	ebp, esp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	call	L4$pb
L4$pb:
	pop	ebx
	mov	edi, dword ptr [ebp + 12]
	mov	esi, dword ptr [ebp + 8]
	sub	esp, 8
	push	edi
	push	esi
	call	_objc_msgSend
	add	esp, 16
	test	eax, eax
	je	LBB4_2
	add	esp, 12
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
LBB4_2:
	sub	esp, 4
	lea	eax, [ebx + l_anon.[ID].2-L4$pb]
	push	eax
	push	edi
	push	esi
	call	SYM(<objc2::__macro_helpers::method_family::RetainSemantics<3_u8> as objc2::__macro_helpers::msg_send_retained::MsgSendRetainedFailed>::failed::GENERATED_ID, 0)

	.globl	_handle_alloc_init
	.p2align	4, 0x90
_handle_alloc_init:
	push	ebp
	mov	ebp, esp
	push	esi
	push	eax
	mov	esi, dword ptr [ebp + 16]
	sub	esp, 8
	push	dword ptr [ebp + 12]
	push	dword ptr [ebp + 8]
	call	_objc_msgSend
	add	esp, 8
	push	esi
	push	eax
	call	_objc_msgSend
	add	esp, 20
	pop	esi
	pop	ebp
	ret

	.globl	_handle_alloc_release
	.p2align	4, 0x90
_handle_alloc_release:
	push	ebp
	mov	ebp, esp
	sub	esp, 8
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	dword ptr [esp + 4], ecx
	mov	dword ptr [esp], eax
	call	_objc_msgSend
	mov	dword ptr [esp], eax
	call	_objc_release
	add	esp, 8
	pop	ebp
	ret

	.globl	_handle_alloc_init_release
	.p2align	4, 0x90
_handle_alloc_init_release:
	push	ebp
	mov	ebp, esp
	push	esi
	sub	esp, 20
	mov	esi, dword ptr [ebp + 16]
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	dword ptr [esp + 4], ecx
	mov	dword ptr [esp], eax
	call	_objc_msgSend
	mov	dword ptr [esp + 4], esi
	mov	dword ptr [esp], eax
	call	_objc_msgSend
	mov	dword ptr [esp], eax
	call	_objc_release
	add	esp, 20
	pop	esi
	pop	ebp
	ret

	.globl	_handle_copy
	.p2align	4, 0x90
_handle_copy:
	push	ebp
	mov	ebp, esp
	pop	ebp
	jmp	_objc_msgSend

	.globl	_handle_copy_fallible
	.p2align	4, 0x90
_handle_copy_fallible:
	push	ebp
	mov	ebp, esp
	push	esi
	sub	esp, 20
	call	L9$pb
L9$pb:
	pop	esi
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	dword ptr [esp + 4], ecx
	mov	dword ptr [esp], eax
	call	_objc_msgSend
	test	eax, eax
	je	LBB9_2
	add	esp, 20
	pop	esi
	pop	ebp
	ret
LBB9_2:
	lea	eax, [esi + l_anon.[ID].3-L9$pb]
	mov	dword ptr [esp], eax
	call	SYM(<objc2::__macro_helpers::method_family::RetainSemantics<4_u8> as objc2::__macro_helpers::msg_send_retained::MsgSendRetainedFailed>::failed::GENERATED_ID, 0)

	.globl	_handle_mutable_copy
	.p2align	4, 0x90
_handle_mutable_copy:
	push	ebp
	mov	ebp, esp
	pop	ebp
	jmp	_objc_msgSend

	.globl	_handle_mutable_copy_fallible
	.p2align	4, 0x90
_handle_mutable_copy_fallible:
	push	ebp
	mov	ebp, esp
	push	esi
	sub	esp, 20
	call	L11$pb
L11$pb:
	pop	esi
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	dword ptr [esp + 4], ecx
	mov	dword ptr [esp], eax
	call	_objc_msgSend
	test	eax, eax
	je	LBB11_2
	add	esp, 20
	pop	esi
	pop	ebp
	ret
LBB11_2:
	lea	eax, [esi + l_anon.[ID].4-L11$pb]
	mov	dword ptr [esp], eax
	call	SYM(<objc2::__macro_helpers::method_family::RetainSemantics<5_u8> as objc2::__macro_helpers::msg_send_retained::MsgSendRetainedFailed>::failed::GENERATED_ID, 0)

	.globl	_handle_autoreleased
	.p2align	4, 0x90
_handle_autoreleased:
	push	ebp
	mov	ebp, esp
	sub	esp, 8
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	dword ptr [esp + 4], ecx
	mov	dword ptr [esp], eax
	call	_objc_msgSend
	## InlineAsm Start

	mov	ebp, ebp

	## InlineAsm End
	mov	dword ptr [esp], eax
	call	_objc_retainAutoreleasedReturnValue
	add	esp, 8
	pop	ebp
	ret

	.globl	_handle_autoreleased_with_arg
	.p2align	4, 0x90
_handle_autoreleased_with_arg:
	push	ebp
	mov	ebp, esp
	sub	esp, 8
	movzx	eax, byte ptr [ebp + 16]
	sub	esp, 4
	push	eax
	push	dword ptr [ebp + 12]
	push	dword ptr [ebp + 8]
	call	_objc_msgSend
	add	esp, 16
	## InlineAsm Start

	mov	ebp, ebp

	## InlineAsm End
	sub	esp, 12
	push	eax
	call	_objc_retainAutoreleasedReturnValue
	add	esp, 24
	pop	ebp
	ret

	.globl	_handle_autoreleased_fallible
	.p2align	4, 0x90
_handle_autoreleased_fallible:
	push	ebp
	mov	ebp, esp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	call	L14$pb
L14$pb:
	pop	ebx
	mov	edi, dword ptr [ebp + 12]
	mov	esi, dword ptr [ebp + 8]
	sub	esp, 8
	push	edi
	push	esi
	call	_objc_msgSend
	add	esp, 16
	## InlineAsm Start

	mov	ebp, ebp

	## InlineAsm End
	sub	esp, 12
	push	eax
	call	_objc_retainAutoreleasedReturnValue
	add	esp, 16
	test	eax, eax
	je	LBB14_2
	add	esp, 12
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
LBB14_2:
	sub	esp, 4
	lea	eax, [ebx + l_anon.[ID].5-L14$pb]
	push	eax
	push	edi
	push	esi
	call	SYM(<objc2::__macro_helpers::method_family::RetainSemantics<6_u8> as objc2::__macro_helpers::msg_send_retained::MsgSendRetainedFailed>::failed::GENERATED_ID, 0)

	.globl	_handle_with_out_param
	.p2align	4, 0x90
_handle_with_out_param:
Lfunc_begin0:
	push	ebp
	mov	ebp, esp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	mov	ebx, dword ptr [ebp + 16]
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	edi, dword ptr [ebx]
Ltmp0:
	mov	dword ptr [esp + 8], ebx
	mov	dword ptr [esp + 4], ecx
	mov	dword ptr [esp], eax
	call	_objc_msgSend
Ltmp1:
	mov	esi, eax
	mov	eax, dword ptr [ebx]
	mov	dword ptr [esp], eax
	call	_objc_retain
	mov	dword ptr [esp], edi
	call	_objc_release
	## InlineAsm Start

	mov	ebp, ebp

	## InlineAsm End
	mov	dword ptr [esp], esi
	call	_objc_retainAutoreleasedReturnValue
	add	esp, 12
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
LBB15_2:
Ltmp2:
	mov	esi, eax
	mov	eax, dword ptr [ebx]
Ltmp3:
	mov	dword ptr [esp], eax
	call	_objc_retain
Ltmp4:
Ltmp5:
	mov	dword ptr [esp], edi
	call	_objc_release
Ltmp6:
	mov	dword ptr [esp], esi
	call	__Unwind_Resume
LBB15_5:
Ltmp7:
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
	.uleb128 Ltmp1-Ltmp0
	.uleb128 Ltmp2-Lfunc_begin0
	.byte	0
	.uleb128 Ltmp1-Lfunc_begin0
	.uleb128 Ltmp3-Ltmp1
	.byte	0
	.byte	0
	.uleb128 Ltmp3-Lfunc_begin0
	.uleb128 Ltmp6-Ltmp3
	.uleb128 Ltmp7-Lfunc_begin0
	.byte	1
	.uleb128 Ltmp6-Lfunc_begin0
	.uleb128 Lfunc_end0-Ltmp6
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
	.p2align	2, 0x0
l_anon.[ID].1:
	.long	l_anon.[ID].0
	.asciz	"9\000\000\000\r\000\000\000\005\000\000"

	.p2align	2, 0x0
l_anon.[ID].2:
	.long	l_anon.[ID].0
	.asciz	"9\000\000\000\034\000\000\000\005\000\000"

	.p2align	2, 0x0
l_anon.[ID].3:
	.long	l_anon.[ID].0
	.asciz	"9\000\000\0008\000\000\000\005\000\000"

	.p2align	2, 0x0
l_anon.[ID].4:
	.long	l_anon.[ID].0
	.asciz	"9\000\000\000B\000\000\000\005\000\000"

	.p2align	2, 0x0
l_anon.[ID].5:
	.long	l_anon.[ID].0
	.asciz	"9\000\000\000V\000\000\000\005\000\000"

	.section	__IMPORT,__pointers,non_lazy_symbol_pointers
L_rust_eh_personality$non_lazy_ptr:
	.indirect_symbol	_rust_eh_personality
	.long	0

.subsections_via_symbols
