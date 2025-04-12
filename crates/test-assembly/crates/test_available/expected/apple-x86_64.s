	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_always
	.p2align	4
_always:
	push	rbp
	mov	rbp, rsp
	mov	al, 1
	pop	rbp
	ret

	.globl	_never
	.p2align	4
_never:
	push	rbp
	mov	rbp, rsp
	xor	eax, eax
	pop	rbp
	ret

	.globl	_low
	.p2align	4
_low:
	push	rbp
	mov	rbp, rsp
	mov	al, 1
	pop	rbp
	ret

	.globl	_high
	.p2align	4
_high:
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	rbx, qword ptr [rip + SYM(objc2::__macro_helpers::os_version::apple::current_version::CURRENT_VERSION::GENERATED_ID, 0)@GOTPCREL]
	mov	eax, dword ptr [rbx]
	test	eax, eax
	je	LBB3_1
LBB3_2:
	cmp	eax, 983040
	setae	al
	add	rsp, 8
	pop	rbx
	pop	rbp
	ret
LBB3_1:
	call	SYM(objc2::__macro_helpers::os_version::apple::lookup_version::GENERATED_ID, 0)
	mov	dword ptr [rbx], eax
	jmp	LBB3_2

	.globl	_only_ios
	.p2align	4
_only_ios:
	push	rbp
	mov	rbp, rsp
	xor	eax, eax
	pop	rbp
	ret

	.globl	_two_checks
	.p2align	4
_two_checks:
	push	rbp
	mov	rbp, rsp
	push	r14
	push	rbx
	mov	r14, qword ptr [rip + SYM(objc2::__macro_helpers::os_version::apple::current_version::CURRENT_VERSION::GENERATED_ID, 0)@GOTPCREL]
	mov	ebx, dword ptr [r14]
	test	ebx, ebx
	je	LBB5_1
	mov	eax, dword ptr [r14]
	test	eax, eax
	je	LBB5_3
LBB5_4:
	cmp	ebx, 917504
	setae	cl
	cmp	eax, 983040
	setae	al
	and	al, cl
	pop	rbx
	pop	r14
	pop	rbp
	ret
LBB5_1:
	call	SYM(objc2::__macro_helpers::os_version::apple::lookup_version::GENERATED_ID, 0)
	mov	ebx, eax
	mov	dword ptr [r14], eax
	mov	eax, dword ptr [r14]
	test	eax, eax
	jne	LBB5_4
LBB5_3:
	call	SYM(objc2::__macro_helpers::os_version::apple::lookup_version::GENERATED_ID, 0)
	mov	dword ptr [r14], eax
	jmp	LBB5_4

.subsections_via_symbols
