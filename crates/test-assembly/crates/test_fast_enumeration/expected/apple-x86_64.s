	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_iter_create
	.p2align	4, 0x90
_iter_create:
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	rbx, rdi
	mov	qword ptr [rdi], rsi
	add	rdi, 8
	mov	esi, 208
	call	___bzero
	mov	rax, rbx
	add	rsp, 8
	pop	rbx
	pop	rbp
	ret

	.globl	_iter_once
	.p2align	4, 0x90
_iter_once:
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	rbx
	push	rax
	mov	rbx, rdi
	mov	rax, qword ptr [rdi + 200]
	cmp	rax, qword ptr [rdi + 208]
	jb	LBB1_4
	lea	rcx, [rbx + 8]
	mov	rdi, qword ptr [rbx]
	mov	rax, qword ptr [rip + SYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPCREL]
	mov	rsi, qword ptr [rax]
	test	rsi, rsi
	je	LBB1_2
LBB1_3:
	lea	rdx, [rbx + 136]
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
	add	rsp, 8
	pop	rbx
	pop	r14
	pop	r15
	pop	rbp
	ret
LBB1_2:
	mov	rax, qword ptr [rip + SYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPCREL]
	lea	rsi, [rip + l_anon.[ID].0]
	mov	r14, rdi
	mov	rdi, rax
	mov	r15, rcx
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	rdi, r14
	mov	rcx, r15
	mov	rsi, rax
	jmp	LBB1_3

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
	lea	r12, [rip + l_anon.[ID].0]
	xor	eax, eax
	jmp	LBB3_1
	.p2align	4, 0x90
LBB3_6:
	mov	rcx, qword ptr [rbp - 112]
	lea	rdx, [rax + 1]
	mov	qword ptr [rbp - 56], rdx
	mov	rdi, qword ptr [rcx + 8*rax]
	call	_use_obj
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
	lea	r12, [rip + l_anon.[ID].0]
	xor	ecx, ecx
	jmp	LBB4_1
	.p2align	4, 0x90
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
	lea	r12, [rip + l_anon.[ID].0]
	xor	eax, eax
	jmp	LBB5_1
	.p2align	4, 0x90
LBB5_6:
	mov	rcx, qword ptr [rbp - 112]
	lea	rdx, [rax + 1]
	mov	qword ptr [rbp - 56], rdx
	mov	rdi, qword ptr [rcx + 8*rax]
	call	_objc_retain
	mov	r13, rax
	mov	rdi, rax
	call	_use_obj
	mov	rdi, r13
	call	_objc_release
	mov	rdi, qword ptr [rbp - 256]
	mov	rax, qword ptr [rbp - 56]
	mov	rcx, qword ptr [rbp - 48]
LBB5_1:
	cmp	rax, rcx
	jb	LBB5_6
	mov	rsi, qword ptr [r15]
	test	rsi, rsi
	je	LBB5_3
LBB5_4:
	mov	r8d, 16
	mov	rdx, r14
	mov	rcx, rbx
	call	_objc_msgSend
	mov	qword ptr [rbp - 48], rax
	test	rax, rax
	je	LBB5_7
	xor	eax, eax
	jmp	LBB5_6
LBB5_3:
	mov	r13, rdi
	mov	rdi, r15
	mov	rsi, r12
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	rdi, r13
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
