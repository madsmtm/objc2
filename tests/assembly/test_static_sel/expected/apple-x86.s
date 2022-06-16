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
	lea	eax, [eax + __ZN15test_static_sel7get_sel5do_it5VALUE17hdc6f5bc4c8522997E-L0$pb]
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
	lea	eax, [eax + __ZN15test_static_sel12get_same_sel5do_it5VALUE17hba6507a7c7d39ef8E-L1$pb]
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
	lea	eax, [eax + __ZN15test_static_sel10get_common5do_it5VALUE17ha16c61a23dba9432E-L2$pb]
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
	lea	eax, [eax + __ZN15test_static_sel17get_different_sel5do_it5VALUE17h56d326a3c9259e1eE-L3$pb]
	pop	ebp
	ret

	.globl	_unused_sel
	.p2align	4, 0x90
_unused_sel:
	push	ebp
	mov	ebp, esp
	pop	ebp
	ret

	.globl	_use_fns
	.p2align	4, 0x90
_use_fns:
	push	ebp
	mov	ebp, esp
	call	L5$pb
L5$pb:
	pop	ecx
	mov	eax, dword ptr [ebp + 8]
	lea	edx, [ecx + __ZN15test_static_sel7get_sel5do_it5VALUE17hdc6f5bc4c8522997E-L5$pb]
	mov	dword ptr [eax], edx
	lea	edx, [ecx + __ZN15test_static_sel12get_same_sel5do_it5VALUE17hba6507a7c7d39ef8E-L5$pb]
	mov	dword ptr [eax + 4], edx
	lea	edx, [ecx + __ZN15test_static_sel17get_different_sel5do_it5VALUE17h56d326a3c9259e1eE-L5$pb]
	mov	dword ptr [eax + 8], edx
	lea	ecx, [ecx + __ZN15test_static_sel7use_fns5do_it5VALUE17h7ce073afa896ddd6E-L5$pb]
	mov	dword ptr [eax + 12], ecx
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
	lea	ecx, [ecx + __ZN15test_static_sel7get_sel5do_it5VALUE17hdc6f5bc4c8522997E-L6$pb]
	mov	dword ptr [eax], ecx
	mov	dword ptr [eax + 4], ecx
	pop	ebp
	ret	4

	.globl	_use_in_loop
	.p2align	4, 0x90
_use_in_loop:
	push	ebp
	mov	ebp, esp
	pop	ebp
	ret

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel7get_sel5do_it5VALUE17hdc6f5bc4c8522997E:
	.asciz	"simple"

__ZN15test_static_sel12get_same_sel5do_it5VALUE17hba6507a7c7d39ef8E:
	.asciz	"simple"

__ZN15test_static_sel10get_common5do_it5VALUE17ha16c61a23dba9432E:
	.asciz	"alloc"

__ZN15test_static_sel17get_different_sel5do_it5VALUE17h56d326a3c9259e1eE:
	.asciz	"i:am:different:"

__ZN15test_static_sel7use_fns5do_it5VALUE17h7ce073afa896ddd6E:
	.asciz	"fourthSel"

.subsections_via_symbols
