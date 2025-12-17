	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_fn1_error_bool
	.p2align	4
_fn1_error_bool:
	push	ebp
	mov	ebp, esp
	push	esi
	push	eax
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	edx, dword ptr [ebp + 16]
	mov	dword ptr [ebp - 8], 0
	lea	esi, [ebp - 8]
	push	esi
	push	edx
	push	ecx
	push	eax
	call	_objc_msgSend
	add	esp, 16
	mov	ecx, eax
	xor	eax, eax
	test	cl, cl
	je	LBB0_1
LBB0_3:
	add	esp, 4
	pop	esi
	pop	ebp
	ret
LBB0_1:
	sub	esp, 12
	push	dword ptr [ebp - 8]
	call	_objc_retain
	add	esp, 16
	test	eax, eax
	jne	LBB0_3
	call	SYM(objc2[CRATE_ID]::__macros::msg_send::null_error::null_error, 0)
	add	esp, 4
	pop	esi
	pop	ebp
	ret

	.globl	_fn2_error_new
	.p2align	4
_fn2_error_new:
	push	ebp
	mov	ebp, esp
	sub	esp, 8
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	dword ptr [ebp - 4], 0
	sub	esp, 4
	lea	edx, [ebp - 4]
	push	edx
	push	ecx
	push	eax
	call	_objc_msgSend
	add	esp, 16
	test	eax, eax
	je	LBB1_2
	mov	edx, eax
	xor	eax, eax
LBB1_4:
	add	esp, 8
	pop	ebp
	ret
LBB1_2:
	sub	esp, 12
	push	dword ptr [ebp - 4]
	call	_objc_retain
	add	esp, 16
	mov	edx, eax
	mov	eax, 1
	test	edx, edx
	jne	LBB1_4
	call	SYM(objc2[CRATE_ID]::__macros::msg_send::null_error::null_error, 0)
	mov	edx, eax
	mov	eax, 1
	add	esp, 8
	pop	ebp
	ret

	.globl	_fn3_error_init
	.p2align	4
_fn3_error_init:
	push	ebp
	mov	ebp, esp
	sub	esp, 8
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	dword ptr [ebp - 4], 0
	sub	esp, 4
	lea	edx, [ebp - 4]
	push	edx
	push	ecx
	push	eax
	call	_objc_msgSend
	add	esp, 16
	test	eax, eax
	je	LBB2_2
	mov	edx, eax
	xor	eax, eax
LBB2_4:
	add	esp, 8
	pop	ebp
	ret
LBB2_2:
	sub	esp, 12
	push	dword ptr [ebp - 4]
	call	_objc_retain
	add	esp, 16
	mov	edx, eax
	mov	eax, 1
	test	edx, edx
	jne	LBB2_4
	call	SYM(objc2[CRATE_ID]::__macros::msg_send::null_error::null_error, 0)
	mov	edx, eax
	mov	eax, 1
	add	esp, 8
	pop	ebp
	ret

	.globl	_fn4_error_copy
	.p2align	4
_fn4_error_copy:
	push	ebp
	mov	ebp, esp
	sub	esp, 8
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	dword ptr [ebp - 4], 0
	sub	esp, 4
	lea	edx, [ebp - 4]
	push	edx
	push	ecx
	push	eax
	call	_objc_msgSend
	add	esp, 16
	test	eax, eax
	je	LBB3_2
	mov	edx, eax
	xor	eax, eax
LBB3_4:
	add	esp, 8
	pop	ebp
	ret
LBB3_2:
	sub	esp, 12
	push	dword ptr [ebp - 4]
	call	_objc_retain
	add	esp, 16
	mov	edx, eax
	mov	eax, 1
	test	edx, edx
	jne	LBB3_4
	call	SYM(objc2[CRATE_ID]::__macros::msg_send::null_error::null_error, 0)
	mov	edx, eax
	mov	eax, 1
	add	esp, 8
	pop	ebp
	ret

	.globl	_fn5_error_mutable_copy
	.p2align	4
_fn5_error_mutable_copy:
	push	ebp
	mov	ebp, esp
	sub	esp, 8
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	dword ptr [ebp - 4], 0
	sub	esp, 4
	lea	edx, [ebp - 4]
	push	edx
	push	ecx
	push	eax
	call	_objc_msgSend
	add	esp, 16
	test	eax, eax
	je	LBB4_2
	mov	edx, eax
	xor	eax, eax
LBB4_4:
	add	esp, 8
	pop	ebp
	ret
LBB4_2:
	sub	esp, 12
	push	dword ptr [ebp - 4]
	call	_objc_retain
	add	esp, 16
	mov	edx, eax
	mov	eax, 1
	test	edx, edx
	jne	LBB4_4
	call	SYM(objc2[CRATE_ID]::__macros::msg_send::null_error::null_error, 0)
	mov	edx, eax
	mov	eax, 1
	add	esp, 8
	pop	ebp
	ret

	.globl	_fn6_error_autoreleased
	.p2align	4
_fn6_error_autoreleased:
	push	ebp
	mov	ebp, esp
	sub	esp, 24
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	dword ptr [ebp - 4], 0
	lea	edx, [ebp - 4]
	mov	dword ptr [esp + 8], edx
	mov	dword ptr [esp + 4], ecx
	mov	dword ptr [esp], eax
	call	_objc_msgSend
	## InlineAsm Start

	mov	ebp, ebp

	## InlineAsm End
	mov	dword ptr [esp], eax
	call	_objc_retainAutoreleasedReturnValue
	test	eax, eax
	je	LBB5_2
	mov	edx, eax
	xor	eax, eax
LBB5_4:
	add	esp, 24
	pop	ebp
	ret
LBB5_2:
	mov	eax, dword ptr [ebp - 4]
	mov	dword ptr [esp], eax
	call	_objc_retain
	mov	edx, eax
	mov	eax, 1
	test	edx, edx
	jne	LBB5_4
	call	SYM(objc2[CRATE_ID]::__macros::msg_send::null_error::null_error, 0)
	mov	edx, eax
	mov	eax, 1
	add	esp, 24
	pop	ebp
	ret

.subsections_via_symbols
