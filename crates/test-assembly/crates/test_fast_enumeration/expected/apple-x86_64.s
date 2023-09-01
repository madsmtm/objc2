	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_iter_create
	.p2align	4, 0x90
_iter_create:
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

	.globl	_iter_once
	.p2align	4, 0x90
_iter_once:
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	r12
	push	rbx
	mov	rbx, rdi
	mov	rax, qword ptr [rdi + 200]
	cmp	rax, qword ptr [rdi + 208]
	jb	LBB1_1
	lea	r14, [rbx + 8]
	mov	r15, qword ptr [rbx]
	lea	r12, [rbx + 136]
	mov	rax, qword ptr [rip + SYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPCREL]
	mov	rsi, qword ptr [rax]
	test	rsi, rsi
	je	LBB1_3
LBB1_4:
	mov	r8d, 16
	mov	rdi, r15
	mov	rdx, r12
	mov	rcx, r14
	call	_objc_msgSend
	mov	rcx, rax
	mov	qword ptr [rbx + 208], rax
	mov	qword ptr [rbx + 200], 0
	xor	eax, eax
	test	rcx, rcx
	je	LBB1_5
LBB1_1:
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
LBB1_3:
	mov	rdi, qword ptr [rip + SYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPCREL]
	lea	rsi, [rip + l_anon.[ID].0]
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	rsi, rax
	jmp	LBB1_4

	.globl	_use_obj
	.p2align	4, 0x90
_use_obj:
	push	rbp
	mov	rbp, rsp
	mov	qword ptr [rbp - 8], rdi
	lea	rax, [rbp - 8]
	## InlineAsm Start
	## InlineAsm End
	pop	rbp
	ret

	.globl	_iter
	.p2align	4, 0x90
_iter:
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	r13
	push	r12
	push	rbx
	sub	rsp, 216
	mov	r14, rdi
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
	lea	r15, [rbp - 120]
	mov	qword ptr [rbp - 104], 0
	mov	qword ptr [rbp - 112], 0
	mov	qword ptr [rbp - 120], 0
	mov	qword ptr [rbp - 48], 0
	mov	qword ptr [rbp - 56], 0
	xor	ecx, ecx
	mov	r12, qword ptr [rip + SYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPCREL]
	lea	r13, [rip + l_anon.[ID].0]
	xor	eax, eax
	jmp	LBB3_1
	.p2align	4, 0x90
LBB3_6:
	mov	rcx, qword ptr [rbp - 112]
	lea	rdx, [rax + 1]
	mov	qword ptr [rbp - 56], rdx
	mov	rdi, qword ptr [rcx + 8*rax]
	call	_use_obj
	mov	r14, qword ptr [rbp - 256]
	mov	rax, qword ptr [rbp - 56]
	mov	rcx, qword ptr [rbp - 48]
LBB3_1:
	cmp	rax, rcx
	jb	LBB3_6
	mov	rsi, qword ptr [r12]
	test	rsi, rsi
	je	LBB3_3
LBB3_4:
	mov	r8d, 16
	mov	rdi, r14
	mov	rdx, r15
	mov	rcx, rbx
	call	_objc_msgSend
	mov	qword ptr [rbp - 48], rax
	test	rax, rax
	je	LBB3_7
	xor	eax, eax
	jmp	LBB3_6
LBB3_3:
	mov	rdi, r12
	mov	rsi, r13
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
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

	.globl	_iter_noop
	.p2align	4, 0x90
_iter_noop:
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	r13
	push	r12
	push	rbx
	sub	rsp, 216
	mov	r14, rdi
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
	lea	r15, [rbp - 120]
	mov	qword ptr [rbp - 104], 0
	mov	qword ptr [rbp - 112], 0
	mov	qword ptr [rbp - 120], 0
	mov	qword ptr [rbp - 48], 0
	mov	qword ptr [rbp - 56], 0
	xor	eax, eax
	mov	r12, qword ptr [rip + SYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPCREL]
	lea	r13, [rip + l_anon.[ID].0]
	xor	ecx, ecx
	jmp	LBB4_1
	.p2align	4, 0x90
LBB4_6:
	inc	rcx
	mov	qword ptr [rbp - 56], rcx
LBB4_1:
	cmp	rcx, rax
	jb	LBB4_6
	mov	rsi, qword ptr [r12]
	test	rsi, rsi
	je	LBB4_3
LBB4_4:
	mov	r8d, 16
	mov	rdi, r14
	mov	rdx, r15
	mov	rcx, rbx
	call	_objc_msgSend
	mov	qword ptr [rbp - 48], rax
	test	rax, rax
	je	LBB4_7
	mov	r14, qword ptr [rbp - 256]
	xor	ecx, ecx
	jmp	LBB4_6
LBB4_3:
	mov	rdi, r12
	mov	rsi, r13
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
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

	.globl	_iter_retained
	.p2align	4, 0x90
_iter_retained:
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	r13
	push	r12
	push	rbx
	sub	rsp, 216
	mov	r15, rdi
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
	mov	r12, qword ptr [rip + SYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPCREL]
	lea	r13, [rip + l_anon.[ID].0]
	xor	eax, eax
	jmp	LBB5_1
	.p2align	4, 0x90
LBB5_6:
	mov	rcx, qword ptr [rbp - 112]
	lea	rdx, [rax + 1]
	mov	qword ptr [rbp - 56], rdx
	mov	rdi, qword ptr [rcx + 8*rax]
	call	_objc_retain
	mov	r15, rax
	mov	rdi, rax
	call	_use_obj
	mov	rdi, r15
	call	_objc_release
	mov	r15, qword ptr [rbp - 256]
	mov	rax, qword ptr [rbp - 56]
	mov	rcx, qword ptr [rbp - 48]
LBB5_1:
	cmp	rax, rcx
	jb	LBB5_6
	mov	rsi, qword ptr [r12]
	test	rsi, rsi
	je	LBB5_3
LBB5_4:
	mov	r8d, 16
	mov	rdi, r15
	mov	rdx, r14
	mov	rcx, rbx
	call	_objc_msgSend
	mov	qword ptr [rbp - 48], rax
	test	rax, rax
	je	LBB5_7
	xor	eax, eax
	jmp	LBB5_6
LBB5_3:
	mov	rdi, r12
	mov	rsi, r13
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	rsi, rax
	jmp	LBB5_4
LBB5_7:
	add	rsp, 216
	pop	rbx
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	pop	rbp
	ret

	.section	__TEXT,__const
l_anon.[ID].0:
	.asciz	"countByEnumeratingWithState:objects:count:"

.subsections_via_symbols
