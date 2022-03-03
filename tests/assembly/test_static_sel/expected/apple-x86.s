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
	mov	eax, dword ptr [eax + __ZN15test_static_sel7get_sel5do_it3REF17h228c7cd1be7c0069E-L0$pb]
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
	mov	eax, dword ptr [eax + __ZN15test_static_sel12get_same_sel5do_it3REF17h2d56ac8d4cf2a355E-L1$pb]
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
	mov	eax, dword ptr [eax + __ZN15test_static_sel10get_common5do_it3REF17hbce043dcabf60bd8E-L2$pb]
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
	mov	eax, dword ptr [eax + __ZN15test_static_sel17get_different_sel5do_it3REF17h46c19195af72a6afE-L3$pb]
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
	mov	eax, dword ptr [eax + __ZN15test_static_sel10unused_sel5do_it3REF17hb665a83c664ebefbE-L4$pb]
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
	mov	edx, dword ptr [ecx + __ZN15test_static_sel7get_sel5do_it3REF17h228c7cd1be7c0069E-L5$pb]
	mov	esi, dword ptr [ecx + __ZN15test_static_sel12get_same_sel5do_it3REF17h2d56ac8d4cf2a355E-L5$pb]
	mov	edi, dword ptr [ecx + __ZN15test_static_sel17get_different_sel5do_it3REF17h46c19195af72a6afE-L5$pb]
	mov	ecx, dword ptr [ecx + __ZN15test_static_sel7use_fns5do_it3REF17h4ce7e1055d208b06E-L5$pb]
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
	mov	edx, dword ptr [ecx + __ZN15test_static_sel7get_sel5do_it3REF17h228c7cd1be7c0069E-L6$pb]
	mov	ecx, dword ptr [ecx + __ZN15test_static_sel7get_sel5do_it3REF17h228c7cd1be7c0069E-L6$pb]
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
	mov	esi, dword ptr [eax + __ZN15test_static_sel11use_in_loop5do_it3REF17h82744bfcc90580aeE-L7$pb]
	mov	esi, dword ptr [eax + __ZN15test_static_sel11use_in_loop5do_it3REF17h82744bfcc90580aeE-L7$pb]
	mov	esi, dword ptr [eax + __ZN15test_static_sel11use_in_loop5do_it3REF17h82744bfcc90580aeE-L7$pb]
	mov	esi, dword ptr [eax + __ZN15test_static_sel11use_in_loop5do_it3REF17h82744bfcc90580aeE-L7$pb]
	mov	esi, dword ptr [eax + __ZN15test_static_sel11use_in_loop5do_it3REF17h82744bfcc90580aeE-L7$pb]
	mov	esi, dword ptr [eax + __ZN15test_static_sel11use_in_loop5do_it3REF17h82744bfcc90580aeE-L7$pb]
	mov	esi, dword ptr [eax + __ZN15test_static_sel11use_in_loop5do_it3REF17h82744bfcc90580aeE-L7$pb]
	mov	esi, dword ptr [eax + __ZN15test_static_sel11use_in_loop5do_it3REF17h82744bfcc90580aeE-L7$pb]
	add	edx, -8
	jne	LBB7_3
LBB7_4:
	test	ecx, ecx
	je	LBB7_6
	.p2align	4, 0x90
LBB7_5:
	mov	edx, dword ptr [eax + __ZN15test_static_sel11use_in_loop5do_it3REF17h82744bfcc90580aeE-L7$pb]
	dec	ecx
	jne	LBB7_5
LBB7_6:
	pop	esi
	pop	ebp
	ret

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel7get_sel5do_it4NAME17h6c6d257dd8fb3617E:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	2
__ZN15test_static_sel7get_sel5do_it3REF17h228c7cd1be7c0069E:
	.long	__ZN15test_static_sel7get_sel5do_it4NAME17h6c6d257dd8fb3617E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel12get_same_sel5do_it4NAME17h3c19b4f1b67d375cE:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	2
__ZN15test_static_sel12get_same_sel5do_it3REF17h2d56ac8d4cf2a355E:
	.long	__ZN15test_static_sel12get_same_sel5do_it4NAME17h3c19b4f1b67d375cE

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel10get_common5do_it4NAME17hcb2f2f30b61e60e6E:
	.asciz	"alloc"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	2
__ZN15test_static_sel10get_common5do_it3REF17hbce043dcabf60bd8E:
	.long	__ZN15test_static_sel10get_common5do_it4NAME17hcb2f2f30b61e60e6E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel17get_different_sel5do_it4NAME17hf72482e04f464b16E:
	.asciz	"i:am:different:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	2
__ZN15test_static_sel17get_different_sel5do_it3REF17h46c19195af72a6afE:
	.long	__ZN15test_static_sel17get_different_sel5do_it4NAME17hf72482e04f464b16E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel10unused_sel5do_it4NAME17h47c97c765e901021E:
	.asciz	"unused"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	2
__ZN15test_static_sel10unused_sel5do_it3REF17hb665a83c664ebefbE:
	.long	__ZN15test_static_sel10unused_sel5do_it4NAME17h47c97c765e901021E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel7use_fns5do_it4NAME17hf963d790d76ff9aaE:
	.asciz	"fourthSel"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	2
__ZN15test_static_sel7use_fns5do_it3REF17h4ce7e1055d208b06E:
	.long	__ZN15test_static_sel7use_fns5do_it4NAME17hf963d790d76ff9aaE

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel11use_in_loop5do_it4NAME17h13b3be67c555f0a5E:
	.asciz	"loopedSelector"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	2
__ZN15test_static_sel11use_in_loop5do_it3REF17h82744bfcc90580aeE:
	.long	__ZN15test_static_sel11use_in_loop5do_it4NAME17h13b3be67c555f0a5E

.subsections_via_symbols
