	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_get_sel
	.p2align	4, 0x90
_get_sel:
	push	rbp
	mov	rbp, rsp
	pop	rbp
	jmp	__ZN15test_static_sel7get_sel22objc_static_workaround17hb9e2bf6c39c8e225E

	.globl	_get_same_sel
	.p2align	4, 0x90
_get_same_sel:
	push	rbp
	mov	rbp, rsp
	pop	rbp
	jmp	__ZN15test_static_sel12get_same_sel22objc_static_workaround17h9d8d966f2cacb6adE

	.globl	_get_common
	.p2align	4, 0x90
_get_common:
	push	rbp
	mov	rbp, rsp
	pop	rbp
	jmp	__ZN15test_static_sel10get_common22objc_static_workaround17h294a2c5d7fde0460E

	.globl	_get_different_sel
	.p2align	4, 0x90
_get_different_sel:
	push	rbp
	mov	rbp, rsp
	pop	rbp
	jmp	__ZN15test_static_sel17get_different_sel22objc_static_workaround17hec1d04928285e256E

	.globl	_unused_sel
	.p2align	4, 0x90
_unused_sel:
	push	rbp
	mov	rbp, rsp
	pop	rbp
	jmp	__ZN15test_static_sel10unused_sel22objc_static_workaround17h1c4a686117a5fc41E

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
	call	__ZN15test_static_sel7get_sel22objc_static_workaround17hb9e2bf6c39c8e225E
	mov	r14, rax
	call	__ZN15test_static_sel12get_same_sel22objc_static_workaround17h9d8d966f2cacb6adE
	mov	r15, rax
	call	__ZN15test_static_sel17get_different_sel22objc_static_workaround17hec1d04928285e256E
	mov	r12, rax
	call	__ZN15test_static_sel7use_fns22objc_static_workaround17h276ee743b1a511dcE
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
	call	__ZN15test_static_sel7get_sel22objc_static_workaround17hb9e2bf6c39c8e225E
	mov	r14, rax
	call	__ZN15test_static_sel7get_sel22objc_static_workaround17hb9e2bf6c39c8e225E
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
	call	__ZN15test_static_sel11use_in_loop22objc_static_workaround17hba715066d6ed34d0E
	dec	rbx
	jne	LBB7_2
LBB7_3:
	add	rsp, 8
	pop	rbx
	pop	rbp
	ret

	.p2align	4, 0x90
__ZN15test_static_sel7get_sel22objc_static_workaround17hb9e2bf6c39c8e225E:
	push	rbp
	mov	rbp, rsp
	call	__ZN5objc210image_info17h37aa854479a69509E
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_68381ba894e318e9]
	pop	rbp
	ret

	.p2align	4, 0x90
__ZN15test_static_sel12get_same_sel22objc_static_workaround17h9d8d966f2cacb6adE:
	push	rbp
	mov	rbp, rsp
	call	__ZN5objc210image_info17h37aa854479a69509E
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_cd2fd6e7d2adcc35]
	pop	rbp
	ret

	.p2align	4, 0x90
__ZN15test_static_sel10get_common22objc_static_workaround17h294a2c5d7fde0460E:
	push	rbp
	mov	rbp, rsp
	call	__ZN5objc210image_info17h37aa854479a69509E
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_34d6c3ed70e85964]
	pop	rbp
	ret

	.p2align	4, 0x90
__ZN15test_static_sel17get_different_sel22objc_static_workaround17hec1d04928285e256E:
	push	rbp
	mov	rbp, rsp
	call	__ZN5objc210image_info17h37aa854479a69509E
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_ab5e106a55f71e5b]
	pop	rbp
	ret

	.p2align	4, 0x90
__ZN15test_static_sel10unused_sel22objc_static_workaround17h1c4a686117a5fc41E:
	push	rbp
	mov	rbp, rsp
	call	__ZN5objc210image_info17h37aa854479a69509E
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_6f2d5ee51a69c477]
	pop	rbp
	ret

	.p2align	4, 0x90
__ZN15test_static_sel7use_fns22objc_static_workaround17h276ee743b1a511dcE:
	push	rbp
	mov	rbp, rsp
	call	__ZN5objc210image_info17h37aa854479a69509E
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_1d27e854714b8860]
	pop	rbp
	ret

	.p2align	4, 0x90
__ZN15test_static_sel11use_in_loop22objc_static_workaround17hba715066d6ed34d0E:
	push	rbp
	mov	rbp, rsp
	call	__ZN5objc210image_info17h37aa854479a69509E
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_e56637a4c1a15b07]
	pop	rbp
	ret

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_68381ba894e318e9
L_OBJC_METH_VAR_NAME_68381ba894e318e9:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_68381ba894e318e9
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_68381ba894e318e9:
	.quad	L_OBJC_METH_VAR_NAME_68381ba894e318e9

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_cd2fd6e7d2adcc35
L_OBJC_METH_VAR_NAME_cd2fd6e7d2adcc35:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_cd2fd6e7d2adcc35
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_cd2fd6e7d2adcc35:
	.quad	L_OBJC_METH_VAR_NAME_cd2fd6e7d2adcc35

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_34d6c3ed70e85964
L_OBJC_METH_VAR_NAME_34d6c3ed70e85964:
	.asciz	"alloc"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_34d6c3ed70e85964
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_34d6c3ed70e85964:
	.quad	L_OBJC_METH_VAR_NAME_34d6c3ed70e85964

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_ab5e106a55f71e5b
L_OBJC_METH_VAR_NAME_ab5e106a55f71e5b:
	.asciz	"i:am:different:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_ab5e106a55f71e5b
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_ab5e106a55f71e5b:
	.quad	L_OBJC_METH_VAR_NAME_ab5e106a55f71e5b

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_6f2d5ee51a69c477
L_OBJC_METH_VAR_NAME_6f2d5ee51a69c477:
	.asciz	"unused"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_6f2d5ee51a69c477
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_6f2d5ee51a69c477:
	.quad	L_OBJC_METH_VAR_NAME_6f2d5ee51a69c477

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_1d27e854714b8860
L_OBJC_METH_VAR_NAME_1d27e854714b8860:
	.asciz	"fourthSel"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_1d27e854714b8860
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_1d27e854714b8860:
	.quad	L_OBJC_METH_VAR_NAME_1d27e854714b8860

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_e56637a4c1a15b07
L_OBJC_METH_VAR_NAME_e56637a4c1a15b07:
	.asciz	"loopedSelector"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_e56637a4c1a15b07
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_e56637a4c1a15b07:
	.quad	L_OBJC_METH_VAR_NAME_e56637a4c1a15b07

.subsections_via_symbols
