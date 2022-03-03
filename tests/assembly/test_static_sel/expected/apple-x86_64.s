	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_get_sel
	.p2align	4, 0x90
_get_sel:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + __ZN15test_static_sel7get_sel5do_it3REF17h782882fcc02d9d8aE]
	pop	rbp
	ret

	.globl	_get_same_sel
	.p2align	4, 0x90
_get_same_sel:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + __ZN15test_static_sel12get_same_sel5do_it3REF17h2d67e358172fa45dE]
	pop	rbp
	ret

	.globl	_get_common
	.p2align	4, 0x90
_get_common:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + __ZN15test_static_sel10get_common5do_it3REF17hbd5f5d11cc1d359fE]
	pop	rbp
	ret

	.globl	_get_different_sel
	.p2align	4, 0x90
_get_different_sel:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + __ZN15test_static_sel17get_different_sel5do_it3REF17h95db9b19a0d861e2E]
	pop	rbp
	ret

	.globl	_unused_sel
	.p2align	4, 0x90
_unused_sel:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + __ZN15test_static_sel10unused_sel5do_it3REF17hc65ecc2272129c12E]
	pop	rbp
	ret

	.globl	_use_fns
	.p2align	4, 0x90
_use_fns:
	push	rbp
	mov	rbp, rsp
	mov	rax, rdi
	mov	rcx, qword ptr [rip + __ZN15test_static_sel7get_sel5do_it3REF17h782882fcc02d9d8aE]
	mov	rdx, qword ptr [rip + __ZN15test_static_sel12get_same_sel5do_it3REF17h2d67e358172fa45dE]
	mov	rsi, qword ptr [rip + __ZN15test_static_sel17get_different_sel5do_it3REF17h95db9b19a0d861e2E]
	mov	rdi, qword ptr [rip + __ZN15test_static_sel7use_fns5do_it3REF17h588d7388fcda7afcE]
	mov	qword ptr [rax], rcx
	mov	qword ptr [rax + 8], rdx
	mov	qword ptr [rax + 16], rsi
	mov	qword ptr [rax + 24], rdi
	pop	rbp
	ret

	.globl	_use_same_twice
	.p2align	4, 0x90
_use_same_twice:
	push	rbp
	mov	rbp, rsp
	mov	rax, rdi
	mov	rcx, qword ptr [rip + __ZN15test_static_sel7get_sel5do_it3REF17h782882fcc02d9d8aE]
	mov	rdx, qword ptr [rip + __ZN15test_static_sel7get_sel5do_it3REF17h782882fcc02d9d8aE]
	mov	qword ptr [rdi], rcx
	mov	qword ptr [rdi + 8], rdx
	pop	rbp
	ret

	.globl	_use_in_loop
	.p2align	4, 0x90
_use_in_loop:
	push	rbp
	mov	rbp, rsp
	test	rdi, rdi
	je	LBB7_6
	lea	rcx, [rdi - 1]
	mov	eax, edi
	and	eax, 7
	cmp	rcx, 7
	jb	LBB7_4
	and	rdi, -8
	.p2align	4, 0x90
LBB7_3:
	mov	rcx, qword ptr [rip + __ZN15test_static_sel11use_in_loop5do_it3REF17he7523abb0b72310bE]
	mov	rcx, qword ptr [rip + __ZN15test_static_sel11use_in_loop5do_it3REF17he7523abb0b72310bE]
	mov	rcx, qword ptr [rip + __ZN15test_static_sel11use_in_loop5do_it3REF17he7523abb0b72310bE]
	mov	rcx, qword ptr [rip + __ZN15test_static_sel11use_in_loop5do_it3REF17he7523abb0b72310bE]
	mov	rcx, qword ptr [rip + __ZN15test_static_sel11use_in_loop5do_it3REF17he7523abb0b72310bE]
	mov	rcx, qword ptr [rip + __ZN15test_static_sel11use_in_loop5do_it3REF17he7523abb0b72310bE]
	mov	rcx, qword ptr [rip + __ZN15test_static_sel11use_in_loop5do_it3REF17he7523abb0b72310bE]
	mov	rcx, qword ptr [rip + __ZN15test_static_sel11use_in_loop5do_it3REF17he7523abb0b72310bE]
	add	rdi, -8
	jne	LBB7_3
LBB7_4:
	test	rax, rax
	je	LBB7_6
	.p2align	4, 0x90
LBB7_5:
	mov	rcx, qword ptr [rip + __ZN15test_static_sel11use_in_loop5do_it3REF17he7523abb0b72310bE]
	dec	rax
	jne	LBB7_5
LBB7_6:
	pop	rbp
	ret

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel7get_sel5do_it4NAME17h9e9003947df30d5bE:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3
__ZN15test_static_sel7get_sel5do_it3REF17h782882fcc02d9d8aE:
	.quad	__ZN15test_static_sel7get_sel5do_it4NAME17h9e9003947df30d5bE

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel12get_same_sel5do_it4NAME17h8b1a946a162bf505E:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3
__ZN15test_static_sel12get_same_sel5do_it3REF17h2d67e358172fa45dE:
	.quad	__ZN15test_static_sel12get_same_sel5do_it4NAME17h8b1a946a162bf505E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel10get_common5do_it4NAME17h584eb2aaa202befbE:
	.asciz	"alloc"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3
__ZN15test_static_sel10get_common5do_it3REF17hbd5f5d11cc1d359fE:
	.quad	__ZN15test_static_sel10get_common5do_it4NAME17h584eb2aaa202befbE

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel17get_different_sel5do_it4NAME17h771c17e46b379ccbE:
	.asciz	"i:am:different:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3
__ZN15test_static_sel17get_different_sel5do_it3REF17h95db9b19a0d861e2E:
	.quad	__ZN15test_static_sel17get_different_sel5do_it4NAME17h771c17e46b379ccbE

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel10unused_sel5do_it4NAME17h00f91d535bf5e92eE:
	.asciz	"unused"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3
__ZN15test_static_sel10unused_sel5do_it3REF17hc65ecc2272129c12E:
	.quad	__ZN15test_static_sel10unused_sel5do_it4NAME17h00f91d535bf5e92eE

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel7use_fns5do_it4NAME17hff47d49a940f1428E:
	.asciz	"fourthSel"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3
__ZN15test_static_sel7use_fns5do_it3REF17h588d7388fcda7afcE:
	.quad	__ZN15test_static_sel7use_fns5do_it4NAME17hff47d49a940f1428E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel11use_in_loop5do_it4NAME17hfb93cf0b2231dd8eE:
	.asciz	"loopedSelector"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3
__ZN15test_static_sel11use_in_loop5do_it3REF17he7523abb0b72310bE:
	.quad	__ZN15test_static_sel11use_in_loop5do_it4NAME17hfb93cf0b2231dd8eE

.subsections_via_symbols
