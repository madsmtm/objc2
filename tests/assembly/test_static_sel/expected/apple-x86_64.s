	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_get_sel
	.p2align	4, 0x90
_get_sel:
	push	rbp
	mov	rbp, rsp
	pop	rbp
	jmp	__ZN15test_static_sel7get_sel22objc_static_workaround17h1cbb0c161f2bac9aE

	.globl	_get_same_sel
	.p2align	4, 0x90
_get_same_sel:
	push	rbp
	mov	rbp, rsp
	pop	rbp
	jmp	__ZN15test_static_sel12get_same_sel22objc_static_workaround17hc1a2360c3d4bddd0E

	.globl	_get_common
	.p2align	4, 0x90
_get_common:
	push	rbp
	mov	rbp, rsp
	pop	rbp
	jmp	__ZN15test_static_sel10get_common22objc_static_workaround17h65f6e72e4d407b78E

	.globl	_get_different_sel
	.p2align	4, 0x90
_get_different_sel:
	push	rbp
	mov	rbp, rsp
	pop	rbp
	jmp	__ZN15test_static_sel17get_different_sel22objc_static_workaround17h87e0092168d40157E

	.globl	_unused_sel
	.p2align	4, 0x90
_unused_sel:
	push	rbp
	mov	rbp, rsp
	pop	rbp
	jmp	__ZN15test_static_sel10unused_sel22objc_static_workaround17h8da61a9c5502ca9aE

	.globl	_use_fns
	.p2align	4, 0x90
_use_fns:
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	r12
	push	rbx
	mov	rbx, rdi
	call	__ZN15test_static_sel7get_sel22objc_static_workaround17h1cbb0c161f2bac9aE
	mov	r14, rax
	call	__ZN15test_static_sel12get_same_sel22objc_static_workaround17hc1a2360c3d4bddd0E
	mov	r15, rax
	call	__ZN15test_static_sel17get_different_sel22objc_static_workaround17h87e0092168d40157E
	mov	r12, rax
	call	__ZN15test_static_sel7use_fns22objc_static_workaround17h38f61cd0d3e1beb6E
	mov	qword ptr [rbx], r14
	mov	qword ptr [rbx + 8], r15
	mov	qword ptr [rbx + 16], r12
	mov	qword ptr [rbx + 24], rax
	mov	rax, rbx
	pop	rbx
	pop	r12
	pop	r14
	pop	r15
	pop	rbp
	ret

	.globl	_use_same_twice
	.p2align	4, 0x90
_use_same_twice:
	push	rbp
	mov	rbp, rsp
	push	r14
	push	rbx
	mov	rbx, rdi
	call	__ZN15test_static_sel7get_sel22objc_static_workaround17h1cbb0c161f2bac9aE
	mov	r14, rax
	call	__ZN15test_static_sel7get_sel22objc_static_workaround17h1cbb0c161f2bac9aE
	mov	qword ptr [rbx], r14
	mov	qword ptr [rbx + 8], rax
	mov	rax, rbx
	pop	rbx
	pop	r14
	pop	rbp
	ret

	.globl	_use_in_loop
	.p2align	4, 0x90
_use_in_loop:
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	test	rdi, rdi
	je	LBB7_3
	mov	rbx, rdi
	.p2align	4, 0x90
LBB7_2:
	call	__ZN15test_static_sel11use_in_loop22objc_static_workaround17hb648681a47b1fb9eE
	dec	rbx
	jne	LBB7_2
LBB7_3:
	add	rsp, 8
	pop	rbx
	pop	rbp
	ret

	.p2align	4, 0x90
__ZN15test_static_sel7get_sel22objc_static_workaround17h1cbb0c161f2bac9aE:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + __ZN15test_static_sel7get_sel22objc_static_workaround3REF17he1cced5ed20eba38E]
	pop	rbp
	ret

	.p2align	4, 0x90
__ZN15test_static_sel12get_same_sel22objc_static_workaround17hc1a2360c3d4bddd0E:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + __ZN15test_static_sel12get_same_sel22objc_static_workaround3REF17h8453f9968f695650E]
	pop	rbp
	ret

	.p2align	4, 0x90
__ZN15test_static_sel10get_common22objc_static_workaround17h65f6e72e4d407b78E:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + __ZN15test_static_sel10get_common22objc_static_workaround3REF17h8fda95d0c1daa73eE]
	pop	rbp
	ret

	.p2align	4, 0x90
__ZN15test_static_sel17get_different_sel22objc_static_workaround17h87e0092168d40157E:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + __ZN15test_static_sel17get_different_sel22objc_static_workaround3REF17he72c56ab87ae659fE]
	pop	rbp
	ret

	.p2align	4, 0x90
__ZN15test_static_sel10unused_sel22objc_static_workaround17h8da61a9c5502ca9aE:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + __ZN15test_static_sel10unused_sel22objc_static_workaround3REF17hc3e929842d683b1cE]
	pop	rbp
	ret

	.p2align	4, 0x90
__ZN15test_static_sel7use_fns22objc_static_workaround17h38f61cd0d3e1beb6E:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + __ZN15test_static_sel7use_fns22objc_static_workaround3REF17head6b4f5de386e04E]
	pop	rbp
	ret

	.p2align	4, 0x90
__ZN15test_static_sel11use_in_loop22objc_static_workaround17hb648681a47b1fb9eE:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + __ZN15test_static_sel11use_in_loop22objc_static_workaround3REF17habfe315ed82cbd83E]
	pop	rbp
	ret

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel7get_sel22objc_static_workaround4NAME17hc48a6857ae4ba570E:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	3
__ZN15test_static_sel7get_sel22objc_static_workaround3REF17he1cced5ed20eba38E:
	.quad	__ZN15test_static_sel7get_sel22objc_static_workaround4NAME17hc48a6857ae4ba570E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel12get_same_sel22objc_static_workaround4NAME17hcf1f699e31314f2fE:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	3
__ZN15test_static_sel12get_same_sel22objc_static_workaround3REF17h8453f9968f695650E:
	.quad	__ZN15test_static_sel12get_same_sel22objc_static_workaround4NAME17hcf1f699e31314f2fE

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel10get_common22objc_static_workaround4NAME17h3ea1273e7725867cE:
	.asciz	"alloc"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	3
__ZN15test_static_sel10get_common22objc_static_workaround3REF17h8fda95d0c1daa73eE:
	.quad	__ZN15test_static_sel10get_common22objc_static_workaround4NAME17h3ea1273e7725867cE

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel17get_different_sel22objc_static_workaround4NAME17h010a4d89cb2f6290E:
	.asciz	"i:am:different:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	3
__ZN15test_static_sel17get_different_sel22objc_static_workaround3REF17he72c56ab87ae659fE:
	.quad	__ZN15test_static_sel17get_different_sel22objc_static_workaround4NAME17h010a4d89cb2f6290E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel10unused_sel22objc_static_workaround4NAME17hf83654e332b09713E:
	.asciz	"unused"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	3
__ZN15test_static_sel10unused_sel22objc_static_workaround3REF17hc3e929842d683b1cE:
	.quad	__ZN15test_static_sel10unused_sel22objc_static_workaround4NAME17hf83654e332b09713E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel7use_fns22objc_static_workaround4NAME17h3b3319caba6cf9f3E:
	.asciz	"fourthSel"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	3
__ZN15test_static_sel7use_fns22objc_static_workaround3REF17head6b4f5de386e04E:
	.quad	__ZN15test_static_sel7use_fns22objc_static_workaround4NAME17h3b3319caba6cf9f3E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel11use_in_loop22objc_static_workaround4NAME17he5e1c1e1a2ca8028E:
	.asciz	"loopedSelector"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	3
__ZN15test_static_sel11use_in_loop22objc_static_workaround3REF17habfe315ed82cbd83E:
	.quad	__ZN15test_static_sel11use_in_loop22objc_static_workaround4NAME17he5e1c1e1a2ca8028E

.subsections_via_symbols
