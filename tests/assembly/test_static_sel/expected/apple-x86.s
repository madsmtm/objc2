	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_get_sel
	.p2align	4, 0x90
_get_sel:
	push	ebp
	mov	ebp, esp
	pop	ebp
	jmp	__ZN15test_static_sel7get_sel22objc_static_workaround17h81b7b987eca6a3a4E

	.globl	_get_same_sel
	.p2align	4, 0x90
_get_same_sel:
	push	ebp
	mov	ebp, esp
	pop	ebp
	jmp	__ZN15test_static_sel12get_same_sel22objc_static_workaround17h42fe6964f04a7ccaE

	.globl	_get_common
	.p2align	4, 0x90
_get_common:
	push	ebp
	mov	ebp, esp
	pop	ebp
	jmp	__ZN15test_static_sel10get_common22objc_static_workaround17h8ab540c389e482ebE

	.globl	_get_different_sel
	.p2align	4, 0x90
_get_different_sel:
	push	ebp
	mov	ebp, esp
	pop	ebp
	jmp	__ZN15test_static_sel17get_different_sel22objc_static_workaround17h728dd0c6759019a1E

	.globl	_unused_sel
	.p2align	4, 0x90
_unused_sel:
	push	ebp
	mov	ebp, esp
	pop	ebp
	jmp	__ZN15test_static_sel10unused_sel22objc_static_workaround17hde06c029659e697eE

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
	call	__ZN15test_static_sel7get_sel22objc_static_workaround17h81b7b987eca6a3a4E
	mov	dword ptr [ebp - 16], eax
	call	__ZN15test_static_sel12get_same_sel22objc_static_workaround17h42fe6964f04a7ccaE
	mov	ebx, eax
	call	__ZN15test_static_sel17get_different_sel22objc_static_workaround17h728dd0c6759019a1E
	mov	edi, eax
	call	__ZN15test_static_sel7use_fns22objc_static_workaround17ha83e06d45bafd771E
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
	call	__ZN15test_static_sel7get_sel22objc_static_workaround17h81b7b987eca6a3a4E
	mov	edi, eax
	call	__ZN15test_static_sel7get_sel22objc_static_workaround17h81b7b987eca6a3a4E
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
	call	__ZN15test_static_sel11use_in_loop22objc_static_workaround17h601e1f59fa6876fdE
	dec	esi
	jne	LBB7_1
LBB7_2:
	add	esp, 4
	pop	esi
	pop	ebp
	ret

	.p2align	4, 0x90
__ZN15test_static_sel7get_sel22objc_static_workaround17h81b7b987eca6a3a4E:
	push	ebp
	mov	ebp, esp
	call	L8$pb
L8$pb:
	pop	eax
	mov	eax, dword ptr [eax + __ZN15test_static_sel7get_sel22objc_static_workaround3REF17hc37abb4197ef951bE-L8$pb]
	pop	ebp
	ret

	.p2align	4, 0x90
__ZN15test_static_sel12get_same_sel22objc_static_workaround17h42fe6964f04a7ccaE:
	push	ebp
	mov	ebp, esp
	call	L9$pb
L9$pb:
	pop	eax
	mov	eax, dword ptr [eax + __ZN15test_static_sel12get_same_sel22objc_static_workaround3REF17hd3b05de0d9a96eecE-L9$pb]
	pop	ebp
	ret

	.p2align	4, 0x90
__ZN15test_static_sel10get_common22objc_static_workaround17h8ab540c389e482ebE:
	push	ebp
	mov	ebp, esp
	call	L10$pb
L10$pb:
	pop	eax
	mov	eax, dword ptr [eax + __ZN15test_static_sel10get_common22objc_static_workaround3REF17h3d670595527dbbb7E-L10$pb]
	pop	ebp
	ret

	.p2align	4, 0x90
__ZN15test_static_sel17get_different_sel22objc_static_workaround17h728dd0c6759019a1E:
	push	ebp
	mov	ebp, esp
	call	L11$pb
L11$pb:
	pop	eax
	mov	eax, dword ptr [eax + __ZN15test_static_sel17get_different_sel22objc_static_workaround3REF17h5c87d7158e6ff828E-L11$pb]
	pop	ebp
	ret

	.p2align	4, 0x90
__ZN15test_static_sel10unused_sel22objc_static_workaround17hde06c029659e697eE:
	push	ebp
	mov	ebp, esp
	call	L12$pb
L12$pb:
	pop	eax
	mov	eax, dword ptr [eax + __ZN15test_static_sel10unused_sel22objc_static_workaround3REF17hb7b3ebb79de7386dE-L12$pb]
	pop	ebp
	ret

	.p2align	4, 0x90
__ZN15test_static_sel7use_fns22objc_static_workaround17ha83e06d45bafd771E:
	push	ebp
	mov	ebp, esp
	call	L13$pb
L13$pb:
	pop	eax
	mov	eax, dword ptr [eax + __ZN15test_static_sel7use_fns22objc_static_workaround3REF17h1a4a387de8da6e06E-L13$pb]
	pop	ebp
	ret

	.p2align	4, 0x90
__ZN15test_static_sel11use_in_loop22objc_static_workaround17h601e1f59fa6876fdE:
	push	ebp
	mov	ebp, esp
	call	L14$pb
L14$pb:
	pop	eax
	mov	eax, dword ptr [eax + __ZN15test_static_sel11use_in_loop22objc_static_workaround3REF17hafed3287c99de420E-L14$pb]
	pop	ebp
	ret

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel7get_sel22objc_static_workaround4NAME17h90deb32c3cc9afbfE:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN15test_static_sel7get_sel22objc_static_workaround3REF17hc37abb4197ef951bE:
	.long	__ZN15test_static_sel7get_sel22objc_static_workaround4NAME17h90deb32c3cc9afbfE

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel12get_same_sel22objc_static_workaround4NAME17h79fb7ea50d2382ecE:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN15test_static_sel12get_same_sel22objc_static_workaround3REF17hd3b05de0d9a96eecE:
	.long	__ZN15test_static_sel12get_same_sel22objc_static_workaround4NAME17h79fb7ea50d2382ecE

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel10get_common22objc_static_workaround4NAME17h4c782c7fdf617c72E:
	.asciz	"alloc"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN15test_static_sel10get_common22objc_static_workaround3REF17h3d670595527dbbb7E:
	.long	__ZN15test_static_sel10get_common22objc_static_workaround4NAME17h4c782c7fdf617c72E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel17get_different_sel22objc_static_workaround4NAME17h69a9c6cf0d4df9cbE:
	.asciz	"i:am:different:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN15test_static_sel17get_different_sel22objc_static_workaround3REF17h5c87d7158e6ff828E:
	.long	__ZN15test_static_sel17get_different_sel22objc_static_workaround4NAME17h69a9c6cf0d4df9cbE

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel10unused_sel22objc_static_workaround4NAME17h7123d51632cd4661E:
	.asciz	"unused"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN15test_static_sel10unused_sel22objc_static_workaround3REF17hb7b3ebb79de7386dE:
	.long	__ZN15test_static_sel10unused_sel22objc_static_workaround4NAME17h7123d51632cd4661E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel7use_fns22objc_static_workaround4NAME17ha768eb127f2dc8a6E:
	.asciz	"fourthSel"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN15test_static_sel7use_fns22objc_static_workaround3REF17h1a4a387de8da6e06E:
	.long	__ZN15test_static_sel7use_fns22objc_static_workaround4NAME17ha768eb127f2dc8a6E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel11use_in_loop22objc_static_workaround4NAME17hff06965bc2df53daE:
	.asciz	"loopedSelector"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN15test_static_sel11use_in_loop22objc_static_workaround3REF17hafed3287c99de420E:
	.long	__ZN15test_static_sel11use_in_loop22objc_static_workaround4NAME17hff06965bc2df53daE

.subsections_via_symbols
