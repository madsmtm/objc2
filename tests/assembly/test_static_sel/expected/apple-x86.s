	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_get_sel
	.p2align	4, 0x90
_get_sel:
	push	ebp
	mov	ebp, esp
	pop	ebp
	jmp	__RNvNvCsiw6xRX1pmbi_15test_static_sel7get_sel22objc_static_workaround

	.globl	_get_same_sel
	.p2align	4, 0x90
_get_same_sel:
	push	ebp
	mov	ebp, esp
	pop	ebp
	jmp	__RNvNvCsiw6xRX1pmbi_15test_static_sel12get_same_sel22objc_static_workaround

	.globl	_get_common_twice
	.p2align	4, 0x90
_get_common_twice:
	push	ebp
	mov	ebp, esp
	push	esi
	push	eax
	call	__RNvNvCsiw6xRX1pmbi_15test_static_sel16get_common_twice22objc_static_workaround
	mov	esi, eax
	call	__RNvNvCsiw6xRX1pmbi_15test_static_sel16get_common_twices_22objc_static_workaround
	mov	edx, eax
	mov	eax, esi
	add	esp, 4
	pop	esi
	pop	ebp
	ret

	.globl	_get_different_sel
	.p2align	4, 0x90
_get_different_sel:
	push	ebp
	mov	ebp, esp
	pop	ebp
	jmp	__RNvNvCsiw6xRX1pmbi_15test_static_sel17get_different_sel22objc_static_workaround

	.globl	_unused_sel
	.p2align	4, 0x90
_unused_sel:
	push	ebp
	mov	ebp, esp
	pop	ebp
	jmp	__RNvNvCsiw6xRX1pmbi_15test_static_sel10unused_sel22objc_static_workaround

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
	call	__RNvNvCsiw6xRX1pmbi_15test_static_sel7get_sel22objc_static_workaround
	mov	dword ptr [ebp - 16], eax
	call	__RNvNvCsiw6xRX1pmbi_15test_static_sel12get_same_sel22objc_static_workaround
	mov	ebx, eax
	call	__RNvNvCsiw6xRX1pmbi_15test_static_sel17get_different_sel22objc_static_workaround
	mov	edi, eax
	call	__RNvNvCsiw6xRX1pmbi_15test_static_sel7use_fns22objc_static_workaround
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
	call	__RNvNvCsiw6xRX1pmbi_15test_static_sel7get_sel22objc_static_workaround
	mov	edi, eax
	call	__RNvNvCsiw6xRX1pmbi_15test_static_sel7get_sel22objc_static_workaround
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
	call	__RNvNvCsiw6xRX1pmbi_15test_static_sel11use_in_loop22objc_static_workaround
	dec	esi
	jne	LBB7_1
LBB7_2:
	add	esp, 4
	pop	esi
	pop	ebp
	ret

	.p2align	4, 0x90
__RNvNvCsiw6xRX1pmbi_15test_static_sel7get_sel22objc_static_workaround:
	push	ebp
	mov	ebp, esp
	call	L8$pb
L8$pb:
	pop	eax
	mov	eax, dword ptr [eax + L_OBJC_SELECTOR_REFERENCES_68381ba894e318e9-L8$pb]
	pop	ebp
	ret

	.p2align	4, 0x90
__RNvNvCsiw6xRX1pmbi_15test_static_sel12get_same_sel22objc_static_workaround:
	push	ebp
	mov	ebp, esp
	call	L9$pb
L9$pb:
	pop	eax
	mov	eax, dword ptr [eax + L_OBJC_SELECTOR_REFERENCES_cd2fd6e7d2adcc35-L9$pb]
	pop	ebp
	ret

	.p2align	4, 0x90
__RNvNvCsiw6xRX1pmbi_15test_static_sel16get_common_twice22objc_static_workaround:
	push	ebp
	mov	ebp, esp
	call	L10$pb
L10$pb:
	pop	eax
	mov	eax, dword ptr [eax + L_OBJC_SELECTOR_REFERENCES_e4a45d49bfea5d77-L10$pb]
	pop	ebp
	ret

	.p2align	4, 0x90
__RNvNvCsiw6xRX1pmbi_15test_static_sel16get_common_twices_22objc_static_workaround:
	push	ebp
	mov	ebp, esp
	call	L11$pb
L11$pb:
	pop	eax
	mov	eax, dword ptr [eax + L_OBJC_SELECTOR_REFERENCES_33db9f67352fe9a7-L11$pb]
	pop	ebp
	ret

	.p2align	4, 0x90
__RNvNvCsiw6xRX1pmbi_15test_static_sel17get_different_sel22objc_static_workaround:
	push	ebp
	mov	ebp, esp
	call	L12$pb
L12$pb:
	pop	eax
	mov	eax, dword ptr [eax + L_OBJC_SELECTOR_REFERENCES_bb37877368f0b7a0-L12$pb]
	pop	ebp
	ret

	.p2align	4, 0x90
__RNvNvCsiw6xRX1pmbi_15test_static_sel10unused_sel22objc_static_workaround:
	push	ebp
	mov	ebp, esp
	call	L13$pb
L13$pb:
	pop	eax
	mov	eax, dword ptr [eax + L_OBJC_SELECTOR_REFERENCES_2c505e110d181b25-L13$pb]
	pop	ebp
	ret

	.p2align	4, 0x90
__RNvNvCsiw6xRX1pmbi_15test_static_sel7use_fns22objc_static_workaround:
	push	ebp
	mov	ebp, esp
	call	L14$pb
L14$pb:
	pop	eax
	mov	eax, dword ptr [eax + L_OBJC_SELECTOR_REFERENCES_5419c3f7fc0a6f99-L14$pb]
	pop	ebp
	ret

	.p2align	4, 0x90
__RNvNvCsiw6xRX1pmbi_15test_static_sel11use_in_loop22objc_static_workaround:
	push	ebp
	mov	ebp, esp
	call	L15$pb
L15$pb:
	pop	eax
	mov	eax, dword ptr [eax + L_OBJC_SELECTOR_REFERENCES_9845965b987ed54b-L15$pb]
	pop	ebp
	ret

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_68381ba894e318e9
	.p2align	2
L_OBJC_IMAGE_INFO_68381ba894e318e9:
	.space	8

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_68381ba894e318e9
L_OBJC_METH_VAR_NAME_68381ba894e318e9:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_68381ba894e318e9
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_68381ba894e318e9:
	.long	L_OBJC_METH_VAR_NAME_68381ba894e318e9

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_cd2fd6e7d2adcc35
	.p2align	2
L_OBJC_IMAGE_INFO_cd2fd6e7d2adcc35:
	.space	8

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_cd2fd6e7d2adcc35
L_OBJC_METH_VAR_NAME_cd2fd6e7d2adcc35:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_cd2fd6e7d2adcc35
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_cd2fd6e7d2adcc35:
	.long	L_OBJC_METH_VAR_NAME_cd2fd6e7d2adcc35

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_e4a45d49bfea5d77
	.p2align	2
L_OBJC_IMAGE_INFO_e4a45d49bfea5d77:
	.space	8

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_e4a45d49bfea5d77
L_OBJC_METH_VAR_NAME_e4a45d49bfea5d77:
	.asciz	"alloc"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_e4a45d49bfea5d77
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_e4a45d49bfea5d77:
	.long	L_OBJC_METH_VAR_NAME_e4a45d49bfea5d77

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_33db9f67352fe9a7
	.p2align	2
L_OBJC_IMAGE_INFO_33db9f67352fe9a7:
	.space	8

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_33db9f67352fe9a7
L_OBJC_METH_VAR_NAME_33db9f67352fe9a7:
	.asciz	"alloc"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_33db9f67352fe9a7
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_33db9f67352fe9a7:
	.long	L_OBJC_METH_VAR_NAME_33db9f67352fe9a7

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_bb37877368f0b7a0
	.p2align	2
L_OBJC_IMAGE_INFO_bb37877368f0b7a0:
	.space	8

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_bb37877368f0b7a0
L_OBJC_METH_VAR_NAME_bb37877368f0b7a0:
	.asciz	"i:am:different:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_bb37877368f0b7a0
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_bb37877368f0b7a0:
	.long	L_OBJC_METH_VAR_NAME_bb37877368f0b7a0

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_2c505e110d181b25
	.p2align	2
L_OBJC_IMAGE_INFO_2c505e110d181b25:
	.space	8

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_2c505e110d181b25
L_OBJC_METH_VAR_NAME_2c505e110d181b25:
	.asciz	"unused"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_2c505e110d181b25
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_2c505e110d181b25:
	.long	L_OBJC_METH_VAR_NAME_2c505e110d181b25

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_5419c3f7fc0a6f99
	.p2align	2
L_OBJC_IMAGE_INFO_5419c3f7fc0a6f99:
	.space	8

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_5419c3f7fc0a6f99
L_OBJC_METH_VAR_NAME_5419c3f7fc0a6f99:
	.asciz	"fourthSel"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_5419c3f7fc0a6f99
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_5419c3f7fc0a6f99:
	.long	L_OBJC_METH_VAR_NAME_5419c3f7fc0a6f99

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_9845965b987ed54b
	.p2align	2
L_OBJC_IMAGE_INFO_9845965b987ed54b:
	.space	8

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_9845965b987ed54b
L_OBJC_METH_VAR_NAME_9845965b987ed54b:
	.asciz	"loopedSelector"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_9845965b987ed54b
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_9845965b987ed54b:
	.long	L_OBJC_METH_VAR_NAME_9845965b987ed54b

.subsections_via_symbols
