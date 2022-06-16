	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_get_sel
	.p2align	4, 0x90
_get_sel:
	push	ebp
	mov	ebp, esp
	pop	ebp
	jmp	__ZN15test_static_sel7get_sel22objc_static_workaround17hbd28d1f4da2b6e6eE

	.globl	_get_same_sel
	.p2align	4, 0x90
_get_same_sel:
	push	ebp
	mov	ebp, esp
	pop	ebp
	jmp	__ZN15test_static_sel12get_same_sel22objc_static_workaround17hcf8c438d0e807292E

	.globl	_get_common
	.p2align	4, 0x90
_get_common:
	push	ebp
	mov	ebp, esp
	pop	ebp
	jmp	__ZN15test_static_sel10get_common22objc_static_workaround17hd73a38a3512c60a4E

	.globl	_get_different_sel
	.p2align	4, 0x90
_get_different_sel:
	push	ebp
	mov	ebp, esp
	pop	ebp
	jmp	__ZN15test_static_sel17get_different_sel22objc_static_workaround17h0437e7638527cbcbE

	.globl	_unused_sel
	.p2align	4, 0x90
_unused_sel:
	push	ebp
	mov	ebp, esp
	pop	ebp
	jmp	__ZN15test_static_sel10unused_sel22objc_static_workaround17h9d08c6f2f30054acE

	.globl	_use_fns
	.p2align	4, 0x90
_use_fns:
	push	ebp
	mov	ebp, esp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	mov	esi, dword ptr [ebp + 8]
	call	__ZN15test_static_sel7get_sel22objc_static_workaround17hbd28d1f4da2b6e6eE
	mov	dword ptr [ebp - 16], eax
	call	__ZN15test_static_sel12get_same_sel22objc_static_workaround17hcf8c438d0e807292E
	mov	ebx, eax
	call	__ZN15test_static_sel17get_different_sel22objc_static_workaround17h0437e7638527cbcbE
	mov	edi, eax
	call	__ZN15test_static_sel7use_fns22objc_static_workaround17hb2af229df09794e6E
	mov	ecx, dword ptr [ebp - 16]
	mov	dword ptr [esi], ecx
	mov	dword ptr [esi + 4], ebx
	mov	dword ptr [esi + 8], edi
	mov	dword ptr [esi + 12], eax
	mov	eax, esi
	add	esp, 12
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret	4

	.globl	_use_same_twice
	.p2align	4, 0x90
_use_same_twice:
	push	ebp
	mov	ebp, esp
	push	edi
	push	esi
	mov	esi, dword ptr [ebp + 8]
	call	__ZN15test_static_sel7get_sel22objc_static_workaround17hbd28d1f4da2b6e6eE
	mov	edi, eax
	call	__ZN15test_static_sel7get_sel22objc_static_workaround17hbd28d1f4da2b6e6eE
	mov	dword ptr [esi], edi
	mov	dword ptr [esi + 4], eax
	mov	eax, esi
	pop	esi
	pop	edi
	pop	ebp
	ret	4

	.globl	_use_in_loop
	.p2align	4, 0x90
_use_in_loop:
	push	ebp
	mov	ebp, esp
	push	esi
	push	eax
	mov	esi, dword ptr [ebp + 8]
	test	esi, esi
	je	LBB7_2
	.p2align	4, 0x90
LBB7_1:
	call	__ZN15test_static_sel11use_in_loop22objc_static_workaround17h0d03d9d1392db5daE
	dec	esi
	jne	LBB7_1
LBB7_2:
	add	esp, 4
	pop	esi
	pop	ebp
	ret

	.p2align	4, 0x90
__ZN15test_static_sel7get_sel22objc_static_workaround17hbd28d1f4da2b6e6eE:
	push	ebp
	mov	ebp, esp
	call	L8$pb
L8$pb:
	pop	eax
	mov	eax, dword ptr [eax + L_OBJC_SELECTOR_REFERENCES_68381ba894e318e9-L8$pb]
	pop	ebp
	ret

	.p2align	4, 0x90
__ZN15test_static_sel12get_same_sel22objc_static_workaround17hcf8c438d0e807292E:
	push	ebp
	mov	ebp, esp
	call	L9$pb
L9$pb:
	pop	eax
	mov	eax, dword ptr [eax + L_OBJC_SELECTOR_REFERENCES_cd2fd6e7d2adcc35-L9$pb]
	pop	ebp
	ret

	.p2align	4, 0x90
__ZN15test_static_sel10get_common22objc_static_workaround17hd73a38a3512c60a4E:
	push	ebp
	mov	ebp, esp
	call	L10$pb
L10$pb:
	pop	eax
	mov	eax, dword ptr [eax + L_OBJC_SELECTOR_REFERENCES_34d6c3ed70e85964-L10$pb]
	pop	ebp
	ret

	.p2align	4, 0x90
__ZN15test_static_sel17get_different_sel22objc_static_workaround17h0437e7638527cbcbE:
	push	ebp
	mov	ebp, esp
	call	L11$pb
L11$pb:
	pop	eax
	mov	eax, dword ptr [eax + L_OBJC_SELECTOR_REFERENCES_ab5e106a55f71e5b-L11$pb]
	pop	ebp
	ret

	.p2align	4, 0x90
__ZN15test_static_sel10unused_sel22objc_static_workaround17h9d08c6f2f30054acE:
	push	ebp
	mov	ebp, esp
	call	L12$pb
L12$pb:
	pop	eax
	mov	eax, dword ptr [eax + L_OBJC_SELECTOR_REFERENCES_6f2d5ee51a69c477-L12$pb]
	pop	ebp
	ret

	.p2align	4, 0x90
__ZN15test_static_sel7use_fns22objc_static_workaround17hb2af229df09794e6E:
	push	ebp
	mov	ebp, esp
	call	L13$pb
L13$pb:
	pop	eax
	mov	eax, dword ptr [eax + L_OBJC_SELECTOR_REFERENCES_1d27e854714b8860-L13$pb]
	pop	ebp
	ret

	.p2align	4, 0x90
__ZN15test_static_sel11use_in_loop22objc_static_workaround17h0d03d9d1392db5daE:
	push	ebp
	mov	ebp, esp
	call	L14$pb
L14$pb:
	pop	eax
	mov	eax, dword ptr [eax + L_OBJC_SELECTOR_REFERENCES_e56637a4c1a15b07-L14$pb]
	pop	ebp
	ret

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_68381ba894e318e9
L_OBJC_METH_VAR_NAME_68381ba894e318e9:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_68381ba894e318e9
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_68381ba894e318e9:
	.long	L_OBJC_METH_VAR_NAME_68381ba894e318e9

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_cd2fd6e7d2adcc35
L_OBJC_METH_VAR_NAME_cd2fd6e7d2adcc35:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_cd2fd6e7d2adcc35
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_cd2fd6e7d2adcc35:
	.long	L_OBJC_METH_VAR_NAME_cd2fd6e7d2adcc35

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_34d6c3ed70e85964
L_OBJC_METH_VAR_NAME_34d6c3ed70e85964:
	.asciz	"alloc"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_34d6c3ed70e85964
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_34d6c3ed70e85964:
	.long	L_OBJC_METH_VAR_NAME_34d6c3ed70e85964

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_ab5e106a55f71e5b
L_OBJC_METH_VAR_NAME_ab5e106a55f71e5b:
	.asciz	"i:am:different:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_ab5e106a55f71e5b
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_ab5e106a55f71e5b:
	.long	L_OBJC_METH_VAR_NAME_ab5e106a55f71e5b

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_6f2d5ee51a69c477
L_OBJC_METH_VAR_NAME_6f2d5ee51a69c477:
	.asciz	"unused"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_6f2d5ee51a69c477
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_6f2d5ee51a69c477:
	.long	L_OBJC_METH_VAR_NAME_6f2d5ee51a69c477

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_1d27e854714b8860
L_OBJC_METH_VAR_NAME_1d27e854714b8860:
	.asciz	"fourthSel"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_1d27e854714b8860
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_1d27e854714b8860:
	.long	L_OBJC_METH_VAR_NAME_1d27e854714b8860

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_e56637a4c1a15b07
L_OBJC_METH_VAR_NAME_e56637a4c1a15b07:
	.asciz	"loopedSelector"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_e56637a4c1a15b07
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_e56637a4c1a15b07:
	.long	L_OBJC_METH_VAR_NAME_e56637a4c1a15b07

.subsections_via_symbols
