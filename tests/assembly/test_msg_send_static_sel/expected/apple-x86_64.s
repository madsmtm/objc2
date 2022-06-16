	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_handle_with_sel
	.p2align	4, 0x90
_handle_with_sel:
	push	rbp
	mov	rbp, rsp
	mov	rsi, qword ptr [rip + __ZN24test_msg_send_static_sel15handle_with_sel5do_it3REF17hf309c5158f98b1fcE]
	pop	rbp
	jmp	_objc_msgSend

	.globl	_handle_alloc_init
	.p2align	4, 0x90
_handle_alloc_init:
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	rbx, qword ptr [rip + __ZN24test_msg_send_static_sel17handle_alloc_init5do_it3REF17h9733de6ecff5dbb2E]
	mov	rsi, qword ptr [rip + __ZN24test_msg_send_static_sel17handle_alloc_init5do_it3REF17hb3954197f6bdef1fE]
	call	_objc_msgSend
	mov	rdi, rax
	mov	rsi, rbx
	add	rsp, 8
	pop	rbx
	pop	rbp
	jmp	_objc_msgSend

	.globl	_use_generic
	.p2align	4, 0x90
_use_generic:
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	rbx, rdi
	mov	rsi, qword ptr [rip + __ZN24test_msg_send_static_sel11use_generic5do_it3REF17hef0b1c0e70be6ec1E]
	mov	rdx, qword ptr [rip + __ZN24test_msg_send_static_sel7generic5do_it3REF17h5d75f9d29adef9eeE]
	call	_objc_msgSend
	mov	rsi, qword ptr [rip + __ZN24test_msg_send_static_sel11use_generic5do_it3REF17hba5110112d711d53E]
	mov	rdx, qword ptr [rip + __ZN24test_msg_send_static_sel7generic5do_it3REF17h5d75f9d29adef9eeE]
	mov	rdi, rbx
	call	_objc_msgSend
	mov	rsi, qword ptr [rip + __ZN24test_msg_send_static_sel11use_generic5do_it3REF17h1c094ab04f32e2afE]
	mov	rdx, qword ptr [rip + __ZN24test_msg_send_static_sel7generic5do_it3REF17h5d75f9d29adef9eeE]
	mov	rdi, rbx
	add	rsp, 8
	pop	rbx
	pop	rbp
	jmp	_objc_msgSend

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel15handle_with_sel5do_it5VALUE17ha271e23b04b427ddE:
	.asciz	"someSelector"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3
__ZN24test_msg_send_static_sel15handle_with_sel5do_it3REF17hf309c5158f98b1fcE:
	.quad	__ZN24test_msg_send_static_sel15handle_with_sel5do_it5VALUE17ha271e23b04b427ddE

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel17handle_alloc_init5do_it5VALUE17h18a88f0eacf39811E:
	.asciz	"init"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3
__ZN24test_msg_send_static_sel17handle_alloc_init5do_it3REF17h9733de6ecff5dbb2E:
	.quad	__ZN24test_msg_send_static_sel17handle_alloc_init5do_it5VALUE17h18a88f0eacf39811E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel17handle_alloc_init5do_it5VALUE17h641723cf398488c1E:
	.asciz	"alloc"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3
__ZN24test_msg_send_static_sel17handle_alloc_init5do_it3REF17hb3954197f6bdef1fE:
	.quad	__ZN24test_msg_send_static_sel17handle_alloc_init5do_it5VALUE17h641723cf398488c1E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel7generic5do_it5VALUE17hd3f8e5714b07eb87E:
	.asciz	"generic:selector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3
__ZN24test_msg_send_static_sel7generic5do_it3REF17h5d75f9d29adef9eeE:
	.quad	__ZN24test_msg_send_static_sel7generic5do_it5VALUE17hd3f8e5714b07eb87E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17h8a28567009fdf875E:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3
__ZN24test_msg_send_static_sel11use_generic5do_it3REF17hef0b1c0e70be6ec1E:
	.quad	__ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17h8a28567009fdf875E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17h8efb56cf5369f387E:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3
__ZN24test_msg_send_static_sel11use_generic5do_it3REF17hba5110112d711d53E:
	.quad	__ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17h8efb56cf5369f387E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17h014eee30b271725eE:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3
__ZN24test_msg_send_static_sel11use_generic5do_it3REF17h1c094ab04f32e2afE:
	.quad	__ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17h014eee30b271725eE

.subsections_via_symbols
