	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_get_sel
	.p2align	4, 0x90
_get_sel:
	push	ebp
	mov	ebp, esp
	call	L0$pb
L0$pb:
	pop	eax
	mov	eax, dword ptr [eax + __ZN15test_static_sel7get_sel5do_it3REF17h9474478a9beae4b3E-L0$pb]
	pop	ebp
	ret

	.globl	_get_same_sel
	.p2align	4, 0x90
_get_same_sel:
	push	ebp
	mov	ebp, esp
	call	L1$pb
L1$pb:
	pop	eax
	mov	eax, dword ptr [eax + __ZN15test_static_sel12get_same_sel5do_it3REF17h4e0f4268fe73e0d3E-L1$pb]
	pop	ebp
	ret

	.globl	_get_common
	.p2align	4, 0x90
_get_common:
	push	ebp
	mov	ebp, esp
	call	L2$pb
L2$pb:
	pop	eax
	mov	eax, dword ptr [eax + __ZN15test_static_sel10get_common5do_it3REF17habc9cf686995fb2cE-L2$pb]
	pop	ebp
	ret

	.globl	_get_different_sel
	.p2align	4, 0x90
_get_different_sel:
	push	ebp
	mov	ebp, esp
	call	L3$pb
L3$pb:
	pop	eax
	mov	eax, dword ptr [eax + __ZN15test_static_sel17get_different_sel5do_it3REF17h3cab7071782ba0e6E-L3$pb]
	pop	ebp
	ret

	.globl	_unused_sel
	.p2align	4, 0x90
_unused_sel:
	push	ebp
	mov	ebp, esp
	call	L4$pb
L4$pb:
	pop	eax
	mov	eax, dword ptr [eax + __ZN15test_static_sel10unused_sel5do_it3REF17h484805d909f89434E-L4$pb]
	pop	ebp
	ret

	.globl	_use_fns
	.p2align	4, 0x90
_use_fns:
	push	ebp
	mov	ebp, esp
	push	edi
	push	esi
	call	L5$pb
L5$pb:
	pop	ecx
	mov	eax, dword ptr [ebp + 8]
	mov	edx, dword ptr [ecx + __ZN15test_static_sel7get_sel5do_it3REF17h9474478a9beae4b3E-L5$pb]
	mov	esi, dword ptr [ecx + __ZN15test_static_sel12get_same_sel5do_it3REF17h4e0f4268fe73e0d3E-L5$pb]
	mov	edi, dword ptr [ecx + __ZN15test_static_sel17get_different_sel5do_it3REF17h3cab7071782ba0e6E-L5$pb]
	mov	ecx, dword ptr [ecx + __ZN15test_static_sel7use_fns5do_it3REF17h8cd819e3c1124776E-L5$pb]
	mov	dword ptr [eax], edx
	mov	dword ptr [eax + 4], esi
	mov	dword ptr [eax + 8], edi
	mov	dword ptr [eax + 12], ecx
	pop	esi
	pop	edi
	pop	ebp
	ret	4

	.globl	_use_same_twice
	.p2align	4, 0x90
_use_same_twice:
	push	ebp
	mov	ebp, esp
	call	L6$pb
L6$pb:
	pop	ecx
	mov	eax, dword ptr [ebp + 8]
	mov	edx, dword ptr [ecx + __ZN15test_static_sel7get_sel5do_it3REF17h9474478a9beae4b3E-L6$pb]
	mov	ecx, dword ptr [ecx + __ZN15test_static_sel7get_sel5do_it3REF17h9474478a9beae4b3E-L6$pb]
	mov	dword ptr [eax], edx
	mov	dword ptr [eax + 4], ecx
	pop	ebp
	ret	4

	.globl	_use_in_loop
	.p2align	4, 0x90
_use_in_loop:
	push	ebp
	mov	ebp, esp
	push	esi
	mov	edx, dword ptr [ebp + 8]
	test	edx, edx
	je	LBB7_6
	call	L7$pb
L7$pb:
	pop	eax
	lea	esi, [edx - 1]
	mov	ecx, edx
	and	ecx, 7
	cmp	esi, 7
	jb	LBB7_4
	and	edx, -8
	.p2align	4, 0x90
LBB7_3:
	mov	esi, dword ptr [eax + __ZN15test_static_sel11use_in_loop5do_it3REF17h6c8decc396b6a708E-L7$pb]
	mov	esi, dword ptr [eax + __ZN15test_static_sel11use_in_loop5do_it3REF17h6c8decc396b6a708E-L7$pb]
	mov	esi, dword ptr [eax + __ZN15test_static_sel11use_in_loop5do_it3REF17h6c8decc396b6a708E-L7$pb]
	mov	esi, dword ptr [eax + __ZN15test_static_sel11use_in_loop5do_it3REF17h6c8decc396b6a708E-L7$pb]
	mov	esi, dword ptr [eax + __ZN15test_static_sel11use_in_loop5do_it3REF17h6c8decc396b6a708E-L7$pb]
	mov	esi, dword ptr [eax + __ZN15test_static_sel11use_in_loop5do_it3REF17h6c8decc396b6a708E-L7$pb]
	mov	esi, dword ptr [eax + __ZN15test_static_sel11use_in_loop5do_it3REF17h6c8decc396b6a708E-L7$pb]
	mov	esi, dword ptr [eax + __ZN15test_static_sel11use_in_loop5do_it3REF17h6c8decc396b6a708E-L7$pb]
	add	edx, -8
	jne	LBB7_3
LBB7_4:
	test	ecx, ecx
	je	LBB7_6
	.p2align	4, 0x90
LBB7_5:
	mov	edx, dword ptr [eax + __ZN15test_static_sel11use_in_loop5do_it3REF17h6c8decc396b6a708E-L7$pb]
	dec	ecx
	jne	LBB7_5
LBB7_6:
	pop	esi
	pop	ebp
	ret

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel7get_sel5do_it5VALUE17hdc6f5bc4c8522997E:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	2
__ZN15test_static_sel7get_sel5do_it3REF17h9474478a9beae4b3E:
	.long	__ZN15test_static_sel7get_sel5do_it5VALUE17hdc6f5bc4c8522997E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel12get_same_sel5do_it5VALUE17hba6507a7c7d39ef8E:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	2
__ZN15test_static_sel12get_same_sel5do_it3REF17h4e0f4268fe73e0d3E:
	.long	__ZN15test_static_sel12get_same_sel5do_it5VALUE17hba6507a7c7d39ef8E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel10get_common5do_it5VALUE17ha16c61a23dba9432E:
	.asciz	"alloc"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	2
__ZN15test_static_sel10get_common5do_it3REF17habc9cf686995fb2cE:
	.long	__ZN15test_static_sel10get_common5do_it5VALUE17ha16c61a23dba9432E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel17get_different_sel5do_it5VALUE17h56d326a3c9259e1eE:
	.asciz	"i:am:different:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	2
__ZN15test_static_sel17get_different_sel5do_it3REF17h3cab7071782ba0e6E:
	.long	__ZN15test_static_sel17get_different_sel5do_it5VALUE17h56d326a3c9259e1eE

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel10unused_sel5do_it5VALUE17h466f702aa6f94d5fE:
	.asciz	"unused"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	2
__ZN15test_static_sel10unused_sel5do_it3REF17h484805d909f89434E:
	.long	__ZN15test_static_sel10unused_sel5do_it5VALUE17h466f702aa6f94d5fE

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel7use_fns5do_it5VALUE17h7ce073afa896ddd6E:
	.asciz	"fourthSel"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	2
__ZN15test_static_sel7use_fns5do_it3REF17h8cd819e3c1124776E:
	.long	__ZN15test_static_sel7use_fns5do_it5VALUE17h7ce073afa896ddd6E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel11use_in_loop5do_it5VALUE17haac9abd8f4a72a6bE:
	.asciz	"loopedSelector"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	2
__ZN15test_static_sel11use_in_loop5do_it3REF17h6c8decc396b6a708E:
	.long	__ZN15test_static_sel11use_in_loop5do_it5VALUE17haac9abd8f4a72a6bE

.subsections_via_symbols
