	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.p2align	4, 0x90
SYM(objc2[CRATE_ID]::__macro_helpers::encountered_error::<objc2[CRATE_ID]::runtime::Object>, 0):
	push	ebp
	mov	ebp, esp
	push	edi
	push	esi
	mov	esi, edx
	call	L0$pb
L0$pb:
	pop	edi
	sub	esp, 12
	push	ecx
	call	_objc_retain
	add	esp, 16
	test	eax, eax
	je	LBB0_2
	pop	esi
	pop	edi
	pop	ebp
	ret
LBB0_2:
	sub	esp, 4
	lea	eax, [edi + l_anon.[ID].0-L0$pb]
	push	esi
	push	56
	push	eax
	call	SYM(core::option::expect_failed::GENERATED_ID, 0)

	.p2align	4, 0x90
SYM(objc2[CRATE_ID]::message::encountered_error::<objc2[CRATE_ID]::runtime::Object>, 0):
	push	ebp
	mov	ebp, esp
	push	esi
	push	eax
	call	L1$pb
L1$pb:
	pop	esi
	sub	esp, 12
	push	ecx
	call	_objc_retain
	add	esp, 16
	test	eax, eax
	je	LBB1_2
	add	esp, 4
	pop	esi
	pop	ebp
	ret
LBB1_2:
	sub	esp, 4
	lea	eax, [esi + l_anon.[ID].3-L1$pb]
	lea	ecx, [esi + l_anon.[ID].1-L1$pb]
	push	eax
	push	54
	push	ecx
	call	SYM(core::option::expect_failed::GENERATED_ID, 0)

	.globl	_error_bool
	.p2align	4, 0x90
_error_bool:
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
	je	LBB2_1
	add	esp, 4
	pop	esi
	pop	ebp
	ret
LBB2_1:
	mov	ecx, dword ptr [ebp - 8]
	call	SYM(objc2[CRATE_ID]::message::encountered_error::<objc2[CRATE_ID]::runtime::Object>, 0)
	add	esp, 4
	pop	esi
	pop	ebp
	ret

	.globl	_error_new
	.p2align	4, 0x90
_error_new:
	push	ebp
	mov	ebp, esp
	push	esi
	push	eax
	call	L3$pb
L3$pb:
	pop	esi
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	dword ptr [ebp - 8], 0
	sub	esp, 4
	lea	edx, [ebp - 8]
	push	edx
	push	ecx
	push	eax
	call	_objc_msgSend
	add	esp, 16
	test	eax, eax
	je	LBB3_2
	mov	edx, eax
	xor	eax, eax
	add	esp, 4
	pop	esi
	pop	ebp
	ret
LBB3_2:
	mov	ecx, dword ptr [ebp - 8]
	lea	edx, [esi + l_anon.[ID].4-L3$pb]
	call	SYM(objc2[CRATE_ID]::__macro_helpers::encountered_error::<objc2[CRATE_ID]::runtime::Object>, 0)
	mov	edx, eax
	mov	eax, 1
	add	esp, 4
	pop	esi
	pop	ebp
	ret

	.globl	_error_alloc
	.p2align	4, 0x90
_error_alloc:
	push	ebp
	mov	ebp, esp
	push	esi
	push	eax
	call	L4$pb
L4$pb:
	pop	esi
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	dword ptr [ebp - 8], 0
	sub	esp, 4
	lea	edx, [ebp - 8]
	push	edx
	push	ecx
	push	eax
	call	_objc_msgSend
	add	esp, 16
	test	eax, eax
	je	LBB4_2
	mov	edx, eax
	xor	eax, eax
	add	esp, 4
	pop	esi
	pop	ebp
	ret
LBB4_2:
	mov	ecx, dword ptr [ebp - 8]
	lea	edx, [esi + l_anon.[ID].5-L4$pb]
	call	SYM(objc2[CRATE_ID]::__macro_helpers::encountered_error::<objc2[CRATE_ID]::runtime::Object>, 0)
	mov	edx, eax
	mov	eax, 1
	add	esp, 4
	pop	esi
	pop	ebp
	ret

	.globl	_error_init
	.p2align	4, 0x90
_error_init:
	push	ebp
	mov	ebp, esp
	push	esi
	push	eax
	call	L5$pb
L5$pb:
	pop	esi
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	dword ptr [ebp - 8], 0
	sub	esp, 4
	lea	edx, [ebp - 8]
	push	edx
	push	ecx
	push	eax
	call	_objc_msgSend
	add	esp, 16
	test	eax, eax
	je	LBB5_2
	mov	edx, eax
	xor	eax, eax
	add	esp, 4
	pop	esi
	pop	ebp
	ret
LBB5_2:
	mov	ecx, dword ptr [ebp - 8]
	lea	edx, [esi + l_anon.[ID].6-L5$pb]
	call	SYM(objc2[CRATE_ID]::__macro_helpers::encountered_error::<objc2[CRATE_ID]::runtime::Object>, 0)
	mov	edx, eax
	mov	eax, 1
	add	esp, 4
	pop	esi
	pop	ebp
	ret

	.globl	_error_copy
	.p2align	4, 0x90
_error_copy:
	push	ebp
	mov	ebp, esp
	push	esi
	push	eax
	call	L6$pb
L6$pb:
	pop	esi
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	dword ptr [ebp - 8], 0
	sub	esp, 4
	lea	edx, [ebp - 8]
	push	edx
	push	ecx
	push	eax
	call	_objc_msgSend
	add	esp, 16
	test	eax, eax
	je	LBB6_2
	mov	edx, eax
	xor	eax, eax
	add	esp, 4
	pop	esi
	pop	ebp
	ret
LBB6_2:
	mov	ecx, dword ptr [ebp - 8]
	lea	edx, [esi + l_anon.[ID].7-L6$pb]
	call	SYM(objc2[CRATE_ID]::__macro_helpers::encountered_error::<objc2[CRATE_ID]::runtime::Object>, 0)
	mov	edx, eax
	mov	eax, 1
	add	esp, 4
	pop	esi
	pop	ebp
	ret

	.globl	_error_autoreleased
	.p2align	4, 0x90
_error_autoreleased:
	push	ebp
	mov	ebp, esp
	push	esi
	push	eax
	call	L7$pb
L7$pb:
	pop	esi
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	dword ptr [ebp - 8], 0
	sub	esp, 4
	lea	edx, [ebp - 8]
	push	edx
	push	ecx
	push	eax
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
	je	LBB7_2
	mov	edx, eax
	xor	eax, eax
	add	esp, 4
	pop	esi
	pop	ebp
	ret
LBB7_2:
	mov	ecx, dword ptr [ebp - 8]
	lea	edx, [esi + l_anon.[ID].8-L7$pb]
	call	SYM(objc2[CRATE_ID]::__macro_helpers::encountered_error::<objc2[CRATE_ID]::runtime::Object>, 0)
	mov	edx, eax
	mov	eax, 1
	add	esp, 4
	pop	esi
	pop	ebp
	ret

	.section	__TEXT,__const
l_anon.[ID].0:
	.ascii	"error parameter should be set if the method returns NULL"

l_anon.[ID].1:
	.ascii	"error parameter should be set if the method returns NO"

l_anon.[ID].2:
	.ascii	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	2, 0x0
l_anon.[ID].3:
	.long	l_anon.[ID].2
	.asciz	"6\000\000\000\013\000\000\000\005\000\000"

	.p2align	2, 0x0
l_anon.[ID].4:
	.long	l_anon.[ID].2
	.asciz	"6\000\000\000\020\000\000\000\005\000\000"

	.p2align	2, 0x0
l_anon.[ID].5:
	.long	l_anon.[ID].2
	.asciz	"6\000\000\000\025\000\000\000\005\000\000"

	.p2align	2, 0x0
l_anon.[ID].6:
	.long	l_anon.[ID].2
	.asciz	"6\000\000\000\032\000\000\000\005\000\000"

	.p2align	2, 0x0
l_anon.[ID].7:
	.long	l_anon.[ID].2
	.asciz	"6\000\000\000\037\000\000\000\005\000\000"

	.p2align	2, 0x0
l_anon.[ID].8:
	.long	l_anon.[ID].2
	.asciz	"6\000\000\000$\000\000\000\005\000\000"

.subsections_via_symbols
