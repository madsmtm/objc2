	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_handle_with_sel
	.p2align	4, 0x90
_handle_with_sel:
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	rbx, rdi
	call	__ZN24test_msg_send_static_sel15handle_with_sel22objc_static_workaround17hc26d924341981882E
	mov	rdi, rbx
	mov	rsi, rax
	add	rsp, 8
	pop	rbx
	pop	rbp
	jmp	_objc_msgSend

	.globl	_handle_alloc_init
	.p2align	4, 0x90
_handle_alloc_init:
	push	rbp
	mov	rbp, rsp
	push	r14
	push	rbx
	mov	rbx, rdi
	call	__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround17h54157f2130f0b136E
	mov	r14, rax
	call	__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround17h9609094b6546c715E
	mov	rdi, rbx
	mov	rsi, rax
	call	_objc_msgSend
	mov	rdi, rax
	mov	rsi, r14
	pop	rbx
	pop	r14
	pop	rbp
	jmp	_objc_msgSend

	.globl	_use_generic
	.p2align	4, 0x90
_use_generic:
	push	rbp
	mov	rbp, rsp
	push	r14
	push	rbx
	mov	rbx, rdi
	call	__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17h9a0f8d2843b3161bE
	mov	r14, rax
	call	__ZN24test_msg_send_static_sel7generic22objc_static_workaround17h47b00e7465514baaE
	mov	rdi, rbx
	mov	rsi, r14
	mov	rdx, rax
	call	_objc_msgSend
	call	__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17h4b3385059b35f065E
	mov	r14, rax
	call	__ZN24test_msg_send_static_sel7generic22objc_static_workaround17h47b00e7465514baaE
	mov	rdi, rbx
	mov	rsi, r14
	mov	rdx, rax
	call	_objc_msgSend
	call	__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17hc3bc04da3b46a52cE
	mov	r14, rax
	call	__ZN24test_msg_send_static_sel7generic22objc_static_workaround17h47b00e7465514baaE
	mov	rdi, rbx
	mov	rsi, r14
	mov	rdx, rax
	pop	rbx
	pop	r14
	pop	rbp
	jmp	_objc_msgSend

	.p2align	4, 0x90
__ZN24test_msg_send_static_sel15handle_with_sel22objc_static_workaround17hc26d924341981882E:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + __ZN24test_msg_send_static_sel15handle_with_sel22objc_static_workaround3REF17h389d6e8b38dbcd5aE]
	pop	rbp
	ret

	.p2align	4, 0x90
__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround17h54157f2130f0b136E:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + __ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround3REF17hcc4dca8f09f1c5b0E]
	pop	rbp
	ret

	.p2align	4, 0x90
__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround17h9609094b6546c715E:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + __ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround3REF17h7ea13d63769dc5d9E]
	pop	rbp
	ret

	.p2align	4, 0x90
__ZN24test_msg_send_static_sel7generic22objc_static_workaround17h47b00e7465514baaE:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + __ZN24test_msg_send_static_sel7generic22objc_static_workaround3REF17hc2551bb3b45b0465E]
	pop	rbp
	ret

	.p2align	4, 0x90
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17h9a0f8d2843b3161bE:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + __ZN24test_msg_send_static_sel11use_generic22objc_static_workaround3REF17h0720ebf54b124315E]
	pop	rbp
	ret

	.p2align	4, 0x90
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17h4b3385059b35f065E:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + __ZN24test_msg_send_static_sel11use_generic22objc_static_workaround3REF17h119b0b79569e4509E]
	pop	rbp
	ret

	.p2align	4, 0x90
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17hc3bc04da3b46a52cE:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + __ZN24test_msg_send_static_sel11use_generic22objc_static_workaround3REF17h79f9a2e492042379E]
	pop	rbp
	ret

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel15handle_with_sel22objc_static_workaround4NAME17h203f5fb6d284e62aE:
	.asciz	"someSelector"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	3
__ZN24test_msg_send_static_sel15handle_with_sel22objc_static_workaround3REF17h389d6e8b38dbcd5aE:
	.quad	__ZN24test_msg_send_static_sel15handle_with_sel22objc_static_workaround4NAME17h203f5fb6d284e62aE

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround4NAME17h9a964ceeb1e3c851E:
	.asciz	"init"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	3
__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround3REF17hcc4dca8f09f1c5b0E:
	.quad	__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround4NAME17h9a964ceeb1e3c851E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround4NAME17h76dc0bdfc53c4746E:
	.asciz	"alloc"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	3
__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround3REF17h7ea13d63769dc5d9E:
	.quad	__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround4NAME17h76dc0bdfc53c4746E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel7generic22objc_static_workaround4NAME17h2b80e03389f4de51E:
	.asciz	"generic:selector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	3
__ZN24test_msg_send_static_sel7generic22objc_static_workaround3REF17hc2551bb3b45b0465E:
	.quad	__ZN24test_msg_send_static_sel7generic22objc_static_workaround4NAME17h2b80e03389f4de51E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround4NAME17h325389a1196cfbceE:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	3
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround3REF17h0720ebf54b124315E:
	.quad	__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround4NAME17h325389a1196cfbceE

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround4NAME17h1ff5ca3b6260f1b5E:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	3
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround3REF17h119b0b79569e4509E:
	.quad	__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround4NAME17h1ff5ca3b6260f1b5E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround4NAME17h36b925d060e0191fE:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	3
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround3REF17h79f9a2e492042379E:
	.quad	__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround4NAME17h36b925d060e0191fE

.subsections_via_symbols
