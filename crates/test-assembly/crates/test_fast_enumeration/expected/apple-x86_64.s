	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_fn1_iter_create
	.p2align	4
_fn1_iter_create:
	push	rbp
	mov	rbp, rsp
	mov	rax, rdi
	mov	qword ptr [rdi + 192], 0
	mov	qword ptr [rdi + 184], 0
	mov	qword ptr [rdi + 176], 0
	mov	qword ptr [rdi + 168], 0
	mov	qword ptr [rdi + 160], 0
	mov	qword ptr [rdi + 8], 0
	mov	qword ptr [rdi + 16], 0
	mov	qword ptr [rdi + 24], 0
	mov	qword ptr [rdi + 32], 0
	mov	qword ptr [rdi + 40], 0
	mov	qword ptr [rdi + 48], 0
	mov	qword ptr [rdi + 56], 0
	mov	qword ptr [rdi + 64], 0
	mov	qword ptr [rdi + 72], 0
	mov	qword ptr [rdi + 80], 0
	mov	qword ptr [rdi + 88], 0
	mov	qword ptr [rdi + 96], 0
	mov	qword ptr [rdi + 104], 0
	mov	qword ptr [rdi + 112], 0
	mov	qword ptr [rdi + 120], 0
	mov	qword ptr [rdi + 128], 0
	mov	qword ptr [rdi], rsi
	mov	qword ptr [rdi + 152], 0
	mov	qword ptr [rdi + 144], 0
	mov	qword ptr [rdi + 136], 0
	mov	qword ptr [rdi + 208], 0
	mov	qword ptr [rdi + 200], 0
	pop	rbp
	ret

	.globl	_fn2_iter_once
	.p2align	4
_fn2_iter_once:
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	r12
	push	rbx
	mov	rbx, rdi
	mov	rax, qword ptr [rdi + 200]
	cmp	rax, qword ptr [rdi + 208]
	jb	LBB1_4
	lea	rcx, [rbx + 8]
	mov	rdi, qword ptr [rbx]
	lea	rdx, [rbx + 136]
	mov	rax, qword ptr [rip + SYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPCREL]
	mov	rsi, qword ptr [rax]
	test	rsi, rsi
	je	LBB1_2
LBB1_3:
	mov	r8d, 16
	call	_objc_msgSend
	mov	rcx, rax
	mov	qword ptr [rbx + 208], rax
	mov	qword ptr [rbx + 200], 0
	xor	eax, eax
	test	rcx, rcx
	je	LBB1_5
LBB1_4:
	mov	rcx, qword ptr [rbx + 144]
	lea	rdx, [rax + 1]
	mov	qword ptr [rbx + 200], rdx
	mov	rax, qword ptr [rcx + 8*rax]
LBB1_5:
	pop	rbx
	pop	r12
	pop	r14
	pop	r15
	pop	rbp
	ret
LBB1_2:
	mov	rax, qword ptr [rip + SYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPCREL]
	lea	rsi, [rip + L_anon.[ID].0]
	mov	r14, rdi
	mov	rdi, rax
	mov	r15, rcx
	mov	r12, rdx
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	rdi, r14
	mov	rdx, r12
	mov	rcx, r15
	mov	rsi, rax
	jmp	LBB1_3

	.globl	_fn3_use_obj
	.p2align	4
_fn3_use_obj:
	push	rbp
	mov	rbp, rsp
	mov	qword ptr [rbp - 8], rdi
	lea	rax, [rbp - 8]
	## InlineAsm Start
	## InlineAsm End
	pop	rbp
	ret

	.globl	_fn4_iter
	.p2align	4
_fn4_iter:
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	r13
	push	r12
	push	rbx
	sub	rsp, 216
	mov	qword ptr [rbp - 64], 0
	mov	qword ptr [rbp - 72], 0
	mov	qword ptr [rbp - 80], 0
	mov	qword ptr [rbp - 88], 0
	mov	qword ptr [rbp - 96], 0
	lea	rbx, [rbp - 248]
	mov	qword ptr [rbp - 248], 0
	mov	qword ptr [rbp - 240], 0
	mov	qword ptr [rbp - 232], 0
	mov	qword ptr [rbp - 224], 0
	mov	qword ptr [rbp - 216], 0
	mov	qword ptr [rbp - 208], 0
	mov	qword ptr [rbp - 200], 0
	mov	qword ptr [rbp - 192], 0
	mov	qword ptr [rbp - 184], 0
	mov	qword ptr [rbp - 176], 0
	mov	qword ptr [rbp - 168], 0
	mov	qword ptr [rbp - 160], 0
	mov	qword ptr [rbp - 152], 0
	mov	qword ptr [rbp - 144], 0
	mov	qword ptr [rbp - 136], 0
	mov	qword ptr [rbp - 128], 0
	mov	qword ptr [rbp - 256], rdi
	lea	r14, [rbp - 120]
	mov	qword ptr [rbp - 104], 0
	mov	qword ptr [rbp - 112], 0
	mov	qword ptr [rbp - 120], 0
	mov	qword ptr [rbp - 48], 0
	mov	qword ptr [rbp - 56], 0
	xor	ecx, ecx
	mov	r15, qword ptr [rip + SYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPCREL]
	lea	r12, [rip + L_anon.[ID].0]
	xor	eax, eax
	jmp	LBB3_1
	.p2align	4
LBB3_6:
	mov	rcx, qword ptr [rbp - 112]
	lea	rdx, [rax + 1]
	mov	qword ptr [rbp - 56], rdx
	mov	rdi, qword ptr [rcx + 8*rax]
	call	_fn3_use_obj
	mov	rdi, qword ptr [rbp - 256]
	mov	rax, qword ptr [rbp - 56]
	mov	rcx, qword ptr [rbp - 48]
LBB3_1:
	cmp	rax, rcx
	jb	LBB3_6
	mov	rsi, qword ptr [r15]
	test	rsi, rsi
	je	LBB3_3
LBB3_4:
	mov	r8d, 16
	mov	rdx, r14
	mov	rcx, rbx
	call	_objc_msgSend
	mov	qword ptr [rbp - 48], rax
	test	rax, rax
	je	LBB3_7
	xor	eax, eax
	jmp	LBB3_6
LBB3_3:
	mov	r13, rdi
	mov	rdi, r15
	mov	rsi, r12
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	rdi, r13
	mov	rsi, rax
	jmp	LBB3_4
LBB3_7:
	add	rsp, 216
	pop	rbx
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	pop	rbp
	ret

	.globl	_fn5_iter_noop
	.p2align	4
_fn5_iter_noop:
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	r13
	push	r12
	push	rbx
	sub	rsp, 216
	mov	qword ptr [rbp - 64], 0
	mov	qword ptr [rbp - 72], 0
	mov	qword ptr [rbp - 80], 0
	mov	qword ptr [rbp - 88], 0
	mov	qword ptr [rbp - 96], 0
	lea	rbx, [rbp - 248]
	mov	qword ptr [rbp - 248], 0
	mov	qword ptr [rbp - 240], 0
	mov	qword ptr [rbp - 232], 0
	mov	qword ptr [rbp - 224], 0
	mov	qword ptr [rbp - 216], 0
	mov	qword ptr [rbp - 208], 0
	mov	qword ptr [rbp - 200], 0
	mov	qword ptr [rbp - 192], 0
	mov	qword ptr [rbp - 184], 0
	mov	qword ptr [rbp - 176], 0
	mov	qword ptr [rbp - 168], 0
	mov	qword ptr [rbp - 160], 0
	mov	qword ptr [rbp - 152], 0
	mov	qword ptr [rbp - 144], 0
	mov	qword ptr [rbp - 136], 0
	mov	qword ptr [rbp - 128], 0
	mov	qword ptr [rbp - 256], rdi
	lea	r14, [rbp - 120]
	mov	qword ptr [rbp - 104], 0
	mov	qword ptr [rbp - 112], 0
	mov	qword ptr [rbp - 120], 0
	mov	qword ptr [rbp - 48], 0
	mov	qword ptr [rbp - 56], 0
	xor	eax, eax
	mov	r15, qword ptr [rip + SYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPCREL]
	lea	r12, [rip + L_anon.[ID].0]
	xor	ecx, ecx
	jmp	LBB4_1
	.p2align	4
LBB4_6:
	inc	rcx
	mov	qword ptr [rbp - 56], rcx
LBB4_1:
	cmp	rcx, rax
	jb	LBB4_6
	mov	rsi, qword ptr [r15]
	test	rsi, rsi
	je	LBB4_3
LBB4_4:
	mov	r8d, 16
	mov	rdx, r14
	mov	rcx, rbx
	call	_objc_msgSend
	mov	qword ptr [rbp - 48], rax
	test	rax, rax
	je	LBB4_7
	mov	rdi, qword ptr [rbp - 256]
	xor	ecx, ecx
	jmp	LBB4_6
LBB4_3:
	mov	r13, rdi
	mov	rdi, r15
	mov	rsi, r12
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	rdi, r13
	mov	rsi, rax
	jmp	LBB4_4
LBB4_7:
	add	rsp, 216
	pop	rbx
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	pop	rbp
	ret

	.globl	_fn6_iter_retained
	.p2align	4
_fn6_iter_retained:
Lfunc_begin0:
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	r13
	push	r12
	push	rbx
	sub	rsp, 232
	mov	qword ptr [rbp - 64], 0
	mov	qword ptr [rbp - 72], 0
	mov	qword ptr [rbp - 80], 0
	mov	qword ptr [rbp - 88], 0
	mov	qword ptr [rbp - 96], 0
	lea	rbx, [rbp - 248]
	mov	qword ptr [rbp - 248], 0
	mov	qword ptr [rbp - 240], 0
	mov	qword ptr [rbp - 232], 0
	mov	qword ptr [rbp - 224], 0
	mov	qword ptr [rbp - 216], 0
	mov	qword ptr [rbp - 208], 0
	mov	qword ptr [rbp - 200], 0
	mov	qword ptr [rbp - 192], 0
	mov	qword ptr [rbp - 184], 0
	mov	qword ptr [rbp - 176], 0
	mov	qword ptr [rbp - 168], 0
	mov	qword ptr [rbp - 160], 0
	mov	qword ptr [rbp - 152], 0
	mov	qword ptr [rbp - 144], 0
	mov	qword ptr [rbp - 136], 0
	mov	qword ptr [rbp - 128], 0
	mov	qword ptr [rbp - 272], 0
	mov	qword ptr [rbp - 256], rdi
	lea	r14, [rbp - 120]
	mov	qword ptr [rbp - 120], 0
	mov	qword ptr [rbp - 112], 0
	mov	qword ptr [rbp - 104], 0
	mov	qword ptr [rbp - 48], 0
	mov	qword ptr [rbp - 56], 0
	xor	ecx, ecx
	mov	r15, qword ptr [rip + SYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPCREL]
	lea	r12, [rip + L_anon.[ID].0]
	xor	eax, eax
	cmp	rax, rcx
	jb	LBB5_7
	.p2align	4
LBB5_2:
	mov	rsi, qword ptr [r15]
	test	rsi, rsi
	je	LBB5_3
LBB5_4:
	mov	r8d, 16
	mov	rdx, r14
	mov	rcx, rbx
	call	_objc_msgSend
	mov	qword ptr [rbp - 48], rax
	mov	qword ptr [rbp - 56], 0
	test	rax, rax
	je	LBB5_14
	cmp	qword ptr [rbp - 112], 0
	je	LBB5_18
	xor	eax, eax
LBB5_7:
	mov	rcx, qword ptr [rbp - 104]
	test	rcx, rcx
	je	LBB5_12
	mov	rcx, qword ptr [rcx]
	cmp	dword ptr [rbp - 272], 1
	jne	LBB5_11
	cmp	qword ptr [rbp - 264], rcx
	je	LBB5_12
	jmp	LBB5_10
	.p2align	4
LBB5_11:
	mov	qword ptr [rbp - 272], 1
	mov	qword ptr [rbp - 264], rcx
LBB5_12:
	mov	rcx, qword ptr [rbp - 112]
	lea	rdx, [rax + 1]
	mov	qword ptr [rbp - 56], rdx
	mov	rdi, qword ptr [rcx + 8*rax]
	call	_objc_retain
	mov	r13, rax
Ltmp0:
	mov	rdi, rax
	call	_fn3_use_obj
Ltmp1:
	mov	rdi, r13
	call	_objc_release
	mov	rdi, qword ptr [rbp - 256]
	mov	rax, qword ptr [rbp - 56]
	mov	rcx, qword ptr [rbp - 48]
	cmp	rax, rcx
	jae	LBB5_2
	jmp	LBB5_7
LBB5_3:
	mov	r13, rdi
	mov	rdi, r15
	mov	rsi, r12
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	rdi, r13
	mov	rsi, rax
	jmp	LBB5_4
LBB5_14:
	add	rsp, 232
	pop	rbx
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	pop	rbp
	ret
LBB5_18:
	call	SYM(objc2_foundation::iter::items_ptr_null::GENERATED_ID, 0)
LBB5_10:
	call	SYM(objc2_foundation::iter::mutation_detected::GENERATED_ID, 0)
LBB5_16:
Ltmp2:
	mov	rbx, rax
Ltmp3:
	mov	rdi, r13
	call	_objc_release
Ltmp4:
	mov	rdi, rbx
	call	__Unwind_Resume
LBB5_15:
Ltmp5:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
Lfunc_end0:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table5:
Lexception0:
	.byte	255
	.byte	155
	.uleb128 Lttbase0-Lttbaseref0
Lttbaseref0:
	.byte	1
	.uleb128 Lcst_end0-Lcst_begin0
Lcst_begin0:
	.uleb128 Lfunc_begin0-Lfunc_begin0
	.uleb128 Ltmp0-Lfunc_begin0
	.byte	0
	.byte	0
	.uleb128 Ltmp0-Lfunc_begin0
	.uleb128 Ltmp1-Ltmp0
	.uleb128 Ltmp2-Lfunc_begin0
	.byte	0
	.uleb128 Ltmp1-Lfunc_begin0
	.uleb128 Ltmp3-Ltmp1
	.byte	0
	.byte	0
	.uleb128 Ltmp3-Lfunc_begin0
	.uleb128 Ltmp4-Ltmp3
	.uleb128 Ltmp5-Lfunc_begin0
	.byte	1
	.uleb128 Ltmp4-Lfunc_begin0
	.uleb128 Lfunc_end0-Ltmp4
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
	.asciz	"countByEnumeratingWithState:objects:count:"

.subsections_via_symbols
