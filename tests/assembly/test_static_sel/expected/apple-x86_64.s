	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_get_sel
	.p2align	4, 0x90
_get_sel:
	push	rbp
	mov	rbp, rsp
	lea	rax, [rip + __ZN15test_static_sel7get_sel5do_it5VALUE17h338b867de999a683E]
	pop	rbp
	ret

	.globl	_get_same_sel
	.p2align	4, 0x90
_get_same_sel:
	push	rbp
	mov	rbp, rsp
	lea	rax, [rip + __ZN15test_static_sel12get_same_sel5do_it5VALUE17hfda78dbf2d31d676E]
	pop	rbp
	ret

	.globl	_get_common
	.p2align	4, 0x90
_get_common:
	push	rbp
	mov	rbp, rsp
	lea	rax, [rip + __ZN15test_static_sel10get_common5do_it5VALUE17h6913f03b163cd283E]
	pop	rbp
	ret

	.globl	_get_different_sel
	.p2align	4, 0x90
_get_different_sel:
	push	rbp
	mov	rbp, rsp
	lea	rax, [rip + __ZN15test_static_sel17get_different_sel5do_it5VALUE17h88a6ccd34e9782e8E]
	pop	rbp
	ret

	.globl	_unused_sel
	.p2align	4, 0x90
_unused_sel:
	push	rbp
	mov	rbp, rsp
	pop	rbp
	ret

	.globl	_use_fns
	.p2align	4, 0x90
_use_fns:
	push	rbp
	mov	rbp, rsp
	mov	rax, rdi
	lea	rcx, [rip + __ZN15test_static_sel7get_sel5do_it5VALUE17h338b867de999a683E]
	mov	qword ptr [rdi], rcx
	lea	rcx, [rip + __ZN15test_static_sel12get_same_sel5do_it5VALUE17hfda78dbf2d31d676E]
	mov	qword ptr [rdi + 8], rcx
	lea	rcx, [rip + __ZN15test_static_sel17get_different_sel5do_it5VALUE17h88a6ccd34e9782e8E]
	mov	qword ptr [rdi + 16], rcx
	lea	rcx, [rip + __ZN15test_static_sel7use_fns5do_it5VALUE17h3053c06c1c5263daE]
	mov	qword ptr [rdi + 24], rcx
	pop	rbp
	ret

	.globl	_use_same_twice
	.p2align	4, 0x90
_use_same_twice:
	push	rbp
	mov	rbp, rsp
	mov	rax, rdi
	lea	rcx, [rip + __ZN15test_static_sel7get_sel5do_it5VALUE17h338b867de999a683E]
	mov	qword ptr [rdi], rcx
	mov	qword ptr [rdi + 8], rcx
	pop	rbp
	ret

	.globl	_use_in_loop
	.p2align	4, 0x90
_use_in_loop:
	push	rbp
	mov	rbp, rsp
	pop	rbp
	ret

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel7get_sel5do_it5VALUE17h338b867de999a683E:
	.asciz	"simple"

__ZN15test_static_sel12get_same_sel5do_it5VALUE17hfda78dbf2d31d676E:
	.asciz	"simple"

__ZN15test_static_sel10get_common5do_it5VALUE17h6913f03b163cd283E:
	.asciz	"alloc"

__ZN15test_static_sel17get_different_sel5do_it5VALUE17h88a6ccd34e9782e8E:
	.asciz	"i:am:different:"

__ZN15test_static_sel7use_fns5do_it5VALUE17h3053c06c1c5263daE:
	.asciz	"fourthSel"

.subsections_via_symbols
