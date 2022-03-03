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
	call	__ZN24test_msg_send_static_sel15handle_with_sel22objc_static_workaround17ha942fb2a10f030e2E
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
	call	__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround17hd4cb31996e6a0379E
	mov	r14, rax
	call	__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround17h5f238466fcc6efaeE
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
	call	__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17h3464617adbf28153E
	mov	r14, rax
	call	__ZN24test_msg_send_static_sel7generic22objc_static_workaround17h71fc2d7ad9beb033E
	mov	rdi, rbx
	mov	rsi, r14
	mov	rdx, rax
	call	_objc_msgSend
	call	__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17hfa132600ad7bf08dE
	mov	r14, rax
	call	__ZN24test_msg_send_static_sel7generic22objc_static_workaround17h71fc2d7ad9beb033E
	mov	rdi, rbx
	mov	rsi, r14
	mov	rdx, rax
	call	_objc_msgSend
	call	__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17hbd4ba246c21e9c56E
	mov	r14, rax
	call	__ZN24test_msg_send_static_sel7generic22objc_static_workaround17h71fc2d7ad9beb033E
	mov	rdi, rbx
	mov	rsi, r14
	mov	rdx, rax
	pop	rbx
	pop	r14
	pop	rbp
	jmp	_objc_msgSend

	.p2align	4, 0x90
__ZN24test_msg_send_static_sel15handle_with_sel22objc_static_workaround17ha942fb2a10f030e2E:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_40f5b12005284286]
	pop	rbp
	ret

	.p2align	4, 0x90
__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround17hd4cb31996e6a0379E:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_904c14aa63c4eec9]
	pop	rbp
	ret

	.p2align	4, 0x90
__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround17h5f238466fcc6efaeE:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_b1ab35d3713395f9]
	pop	rbp
	ret

	.p2align	4, 0x90
__ZN24test_msg_send_static_sel7generic22objc_static_workaround17h71fc2d7ad9beb033E:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_31f63858e271db32]
	pop	rbp
	ret

	.p2align	4, 0x90
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17h3464617adbf28153E:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_cdfe92d39025fdf4]
	pop	rbp
	ret

	.p2align	4, 0x90
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17hfa132600ad7bf08dE:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_79bd65c86d46fbf1]
	pop	rbp
	ret

	.p2align	4, 0x90
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17hbd4ba246c21e9c56E:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_8e0840c6b39b7720]
	pop	rbp
	ret

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_40f5b12005284286
	.p2align	2
L_OBJC_IMAGE_INFO_40f5b12005284286:
	.space	8

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_40f5b12005284286
L_OBJC_METH_VAR_NAME_40f5b12005284286:
	.asciz	"someSelector"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_40f5b12005284286
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_40f5b12005284286:
	.quad	L_OBJC_METH_VAR_NAME_40f5b12005284286

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_904c14aa63c4eec9
	.p2align	2
L_OBJC_IMAGE_INFO_904c14aa63c4eec9:
	.space	8

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_904c14aa63c4eec9
L_OBJC_METH_VAR_NAME_904c14aa63c4eec9:
	.asciz	"init"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_904c14aa63c4eec9
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_904c14aa63c4eec9:
	.quad	L_OBJC_METH_VAR_NAME_904c14aa63c4eec9

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_b1ab35d3713395f9
	.p2align	2
L_OBJC_IMAGE_INFO_b1ab35d3713395f9:
	.space	8

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_b1ab35d3713395f9
L_OBJC_METH_VAR_NAME_b1ab35d3713395f9:
	.asciz	"alloc"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_b1ab35d3713395f9
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_b1ab35d3713395f9:
	.quad	L_OBJC_METH_VAR_NAME_b1ab35d3713395f9

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_31f63858e271db32
	.p2align	2
L_OBJC_IMAGE_INFO_31f63858e271db32:
	.space	8

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_31f63858e271db32
L_OBJC_METH_VAR_NAME_31f63858e271db32:
	.asciz	"generic:selector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_31f63858e271db32
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_31f63858e271db32:
	.quad	L_OBJC_METH_VAR_NAME_31f63858e271db32

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_cdfe92d39025fdf4
	.p2align	2
L_OBJC_IMAGE_INFO_cdfe92d39025fdf4:
	.space	8

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_cdfe92d39025fdf4
L_OBJC_METH_VAR_NAME_cdfe92d39025fdf4:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_cdfe92d39025fdf4
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_cdfe92d39025fdf4:
	.quad	L_OBJC_METH_VAR_NAME_cdfe92d39025fdf4

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_79bd65c86d46fbf1
	.p2align	2
L_OBJC_IMAGE_INFO_79bd65c86d46fbf1:
	.space	8

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_79bd65c86d46fbf1
L_OBJC_METH_VAR_NAME_79bd65c86d46fbf1:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_79bd65c86d46fbf1
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_79bd65c86d46fbf1:
	.quad	L_OBJC_METH_VAR_NAME_79bd65c86d46fbf1

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_8e0840c6b39b7720
	.p2align	2
L_OBJC_IMAGE_INFO_8e0840c6b39b7720:
	.space	8

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_8e0840c6b39b7720
L_OBJC_METH_VAR_NAME_8e0840c6b39b7720:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_8e0840c6b39b7720
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_8e0840c6b39b7720:
	.quad	L_OBJC_METH_VAR_NAME_8e0840c6b39b7720

.subsections_via_symbols
