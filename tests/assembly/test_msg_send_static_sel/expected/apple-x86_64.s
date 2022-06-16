	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_handle_with_sel
	.p2align	4, 0x90
_handle_with_sel:
	push	rbp
	mov	rbp, rsp
	lea	rsi, [rip + __ZN24test_msg_send_static_sel15handle_with_sel5do_it5VALUE17ha271e23b04b427ddE]
	pop	rbp
	jmp	_objc_msgSend

	.globl	_handle_alloc_init
	.p2align	4, 0x90
_handle_alloc_init:
	push	rbp
	mov	rbp, rsp
	lea	rsi, [rip + __ZN24test_msg_send_static_sel17handle_alloc_init5do_it5VALUE17h641723cf398488c1E]
	call	_objc_msgSend
	lea	rsi, [rip + __ZN24test_msg_send_static_sel17handle_alloc_init5do_it5VALUE17h18a88f0eacf39811E]
	mov	rdi, rax
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
	lea	rsi, [rip + __ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17h8a28567009fdf875E]
	lea	r14, [rip + __ZN24test_msg_send_static_sel7generic5do_it5VALUE17hd3f8e5714b07eb87E]
	mov	rdx, r14
	call	_objc_msgSend
	lea	rsi, [rip + __ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17h8efb56cf5369f387E]
	mov	rdi, rbx
	mov	rdx, r14
	call	_objc_msgSend
	lea	rsi, [rip + __ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17h014eee30b271725eE]
	mov	rdi, rbx
	mov	rdx, r14
	pop	rbx
	pop	r14
	pop	rbp
	jmp	_objc_msgSend

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel15handle_with_sel5do_it5VALUE17ha271e23b04b427ddE:
	.asciz	"someSelector"

__ZN24test_msg_send_static_sel17handle_alloc_init5do_it5VALUE17h18a88f0eacf39811E:
	.asciz	"init"

__ZN24test_msg_send_static_sel17handle_alloc_init5do_it5VALUE17h641723cf398488c1E:
	.asciz	"alloc"

__ZN24test_msg_send_static_sel7generic5do_it5VALUE17hd3f8e5714b07eb87E:
	.asciz	"generic:selector:"

__ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17h8a28567009fdf875E:
	.asciz	"performSelector:"

__ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17h8efb56cf5369f387E:
	.asciz	"performSelector:"

__ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17h014eee30b271725eE:
	.asciz	"performSelector:"

.subsections_via_symbols
