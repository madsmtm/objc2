	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_get_sel
	.p2align	4, 0x90
_get_sel:
	push	rbp
	mov	rbp, rsp
	pop	rbp
	jmp	__ZN15test_static_sel7get_sel22objc_static_workaround17hc12703947ea497bbE

	.globl	_get_same_sel
	.p2align	4, 0x90
_get_same_sel:
	push	rbp
	mov	rbp, rsp
	pop	rbp
	jmp	__ZN15test_static_sel12get_same_sel22objc_static_workaround17hca258d69d3e23d5eE

	.globl	_get_common_twice
	.p2align	4, 0x90
_get_common_twice:
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	call	__ZN15test_static_sel16get_common_twice22objc_static_workaround17h8dadd4b63e132a5cE
	mov	rbx, rax
	call	__ZN15test_static_sel16get_common_twice22objc_static_workaround17h23fd0bdb743e9e9cE
	mov	rdx, rax
	mov	rax, rbx
	add	rsp, 8
	pop	rbx
	pop	rbp
	ret

	.globl	_get_different_sel
	.p2align	4, 0x90
_get_different_sel:
	push	rbp
	mov	rbp, rsp
	pop	rbp
	jmp	__ZN15test_static_sel17get_different_sel22objc_static_workaround17h0647461fa2fb8e0aE

	.globl	_unused_sel
	.p2align	4, 0x90
_unused_sel:
	push	rbp
	mov	rbp, rsp
	pop	rbp
	jmp	__ZN15test_static_sel10unused_sel22objc_static_workaround17h89dff92e2e97ddb8E

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
	call	__ZN15test_static_sel7get_sel22objc_static_workaround17hc12703947ea497bbE
	mov	r14, rax
	call	__ZN15test_static_sel12get_same_sel22objc_static_workaround17hca258d69d3e23d5eE
	mov	r15, rax
	call	__ZN15test_static_sel17get_different_sel22objc_static_workaround17h0647461fa2fb8e0aE
	mov	r12, rax
	call	__ZN15test_static_sel7use_fns22objc_static_workaround17hc7ea4a687b01e5f9E
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
	call	__ZN15test_static_sel7get_sel22objc_static_workaround17hc12703947ea497bbE
	mov	r14, rax
	call	__ZN15test_static_sel7get_sel22objc_static_workaround17hc12703947ea497bbE
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
	call	__ZN15test_static_sel11use_in_loop22objc_static_workaround17h7c93fa32030b5498E
	dec	rbx
	jne	LBB7_2
LBB7_3:
	add	rsp, 8
	pop	rbx
	pop	rbp
	ret

	.p2align	4, 0x90
__ZN15test_static_sel7get_sel22objc_static_workaround17hc12703947ea497bbE:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_68381ba894e318e9]
	pop	rbp
	ret

	.p2align	4, 0x90
__ZN15test_static_sel12get_same_sel22objc_static_workaround17hca258d69d3e23d5eE:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_cd2fd6e7d2adcc35]
	pop	rbp
	ret

	.p2align	4, 0x90
__ZN15test_static_sel16get_common_twice22objc_static_workaround17h8dadd4b63e132a5cE:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_e4a45d49bfea5d77]
	pop	rbp
	ret

	.p2align	4, 0x90
__ZN15test_static_sel16get_common_twice22objc_static_workaround17h23fd0bdb743e9e9cE:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_33db9f67352fe9a7]
	pop	rbp
	ret

	.p2align	4, 0x90
__ZN15test_static_sel17get_different_sel22objc_static_workaround17h0647461fa2fb8e0aE:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_bb37877368f0b7a0]
	pop	rbp
	ret

	.p2align	4, 0x90
__ZN15test_static_sel10unused_sel22objc_static_workaround17h89dff92e2e97ddb8E:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_2c505e110d181b25]
	pop	rbp
	ret

	.p2align	4, 0x90
__ZN15test_static_sel7use_fns22objc_static_workaround17hc7ea4a687b01e5f9E:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_5419c3f7fc0a6f99]
	pop	rbp
	ret

	.p2align	4, 0x90
__ZN15test_static_sel11use_in_loop22objc_static_workaround17h7c93fa32030b5498E:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_9845965b987ed54b]
	pop	rbp
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
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_68381ba894e318e9:
	.quad	L_OBJC_METH_VAR_NAME_68381ba894e318e9

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
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_cd2fd6e7d2adcc35:
	.quad	L_OBJC_METH_VAR_NAME_cd2fd6e7d2adcc35

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
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_e4a45d49bfea5d77:
	.quad	L_OBJC_METH_VAR_NAME_e4a45d49bfea5d77

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
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_33db9f67352fe9a7:
	.quad	L_OBJC_METH_VAR_NAME_33db9f67352fe9a7

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
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_bb37877368f0b7a0:
	.quad	L_OBJC_METH_VAR_NAME_bb37877368f0b7a0

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
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_2c505e110d181b25:
	.quad	L_OBJC_METH_VAR_NAME_2c505e110d181b25

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
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_5419c3f7fc0a6f99:
	.quad	L_OBJC_METH_VAR_NAME_5419c3f7fc0a6f99

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
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_9845965b987ed54b:
	.quad	L_OBJC_METH_VAR_NAME_9845965b987ed54b

.subsections_via_symbols
