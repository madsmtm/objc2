	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_fn01_handle_new
	.p2align	4
_fn01_handle_new:
	push	ebp
	mov	ebp, esp
	pop	ebp
	jmp	_objc_msgSend

	.globl	_fn02_handle_new_fallible
	.p2align	4
_fn02_handle_new_fallible:
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
	call	SYM(objc2[CRATE_ID]::__macros::retain_semantics::new_fail, 0)

	.globl	_fn03_handle_alloc
	.p2align	4
_fn03_handle_alloc:
	push	ebp
	mov	ebp, esp
	pop	ebp
	jmp	_objc_msgSend

	.globl	_fn04_handle_init
	.p2align	4
_fn04_handle_init:
	push	ebp
	mov	ebp, esp
	pop	ebp
	jmp	_objc_msgSend

	.globl	_fn05_handle_init_fallible
	.p2align	4
_fn05_handle_init_fallible:
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
	call	SYM(objc2[CRATE_ID]::__macros::retain_semantics::init_fail, 0)

	.globl	_fn06_handle_alloc_init
	.p2align	4
_fn06_handle_alloc_init:
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

	.globl	_fn07_handle_alloc_release
	.p2align	4
_fn07_handle_alloc_release:
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

	.globl	_fn08_handle_alloc_init_release
	.p2align	4
_fn08_handle_alloc_init_release:
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

	.globl	_fn09_handle_copy
	.p2align	4
_fn09_handle_copy:
	push	ebp
	mov	ebp, esp
	pop	ebp
	jmp	_objc_msgSend

	.globl	_fn10_handle_copy_fallible
	.p2align	4
_fn10_handle_copy_fallible:
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
	call	SYM(objc2[CRATE_ID]::__macros::retain_semantics::copy_fail, 0)

	.globl	_fn11_handle_mutable_copy
	.p2align	4
_fn11_handle_mutable_copy:
	push	ebp
	mov	ebp, esp
	pop	ebp
	jmp	_objc_msgSend

	.globl	_fn12_handle_mutable_copy_fallible
	.p2align	4
_fn12_handle_mutable_copy_fallible:
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
	call	SYM(objc2[CRATE_ID]::__macros::retain_semantics::mutable_copy_fail, 0)

	.globl	_fn13_handle_autoreleased
	.p2align	4
_fn13_handle_autoreleased:
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

	.globl	_fn14_handle_autoreleased_with_arg
	.p2align	4
_fn14_handle_autoreleased_with_arg:
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

	.globl	_fn15_handle_autoreleased_fallible
	.p2align	4
_fn15_handle_autoreleased_fallible:
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
	call	SYM(objc2[CRATE_ID]::__macros::retain_semantics::none_fail, 0)

	.globl	_fn16_handle_with_out_param
	.p2align	4
_fn16_handle_with_out_param:
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
	## InlineAsm Start

	mov	ebp, ebp

	## InlineAsm End
Ltmp2:
	mov	dword ptr [esp], eax
	call	_objc_retainAutoreleasedReturnValue
Ltmp3:
	mov	esi, eax
	mov	eax, dword ptr [ebx]
	mov	dword ptr [esp], eax
	call	_objc_retain
	mov	dword ptr [esp], edi
	call	_objc_release
	mov	eax, esi
	add	esp, 12
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
LBB15_3:
Ltmp4:
	mov	esi, eax
	mov	eax, dword ptr [ebx]
Ltmp5:
	mov	dword ptr [esp], eax
	call	_objc_retain
Ltmp6:
Ltmp7:
	mov	dword ptr [esp], edi
	call	_objc_release
Ltmp8:
	mov	dword ptr [esp], esi
	call	__Unwind_Resume
LBB15_6:
Ltmp9:
	call	SYM(core[CRATE_ID]::panicking::panic_in_cleanup, 0)
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

	.section	__TEXT,__cstring,cstring_literals
L_anon.[ID].0:
	.asciz	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	2, 0x0
l_anon.[ID].1:
	.long	L_anon.[ID].0
	.asciz	"9\000\000\000\017\000\000\000\005\000\000"

	.p2align	2, 0x0
l_anon.[ID].2:
	.long	L_anon.[ID].0
	.asciz	"9\000\000\000\036\000\000\000\005\000\000"

	.p2align	2, 0x0
l_anon.[ID].3:
	.long	L_anon.[ID].0
	.asciz	"9\000\000\000:\000\000\000\005\000\000"

	.p2align	2, 0x0
l_anon.[ID].4:
	.long	L_anon.[ID].0
	.asciz	"9\000\000\000D\000\000\000\005\000\000"

	.p2align	2, 0x0
l_anon.[ID].5:
	.long	L_anon.[ID].0
	.asciz	"9\000\000\000X\000\000\000\005\000\000"

	.section	__IMPORT,__pointers,non_lazy_symbol_pointers
L_rust_eh_personality$non_lazy_ptr:
	.indirect_symbol	_rust_eh_personality
	.long	0

.subsections_via_symbols
