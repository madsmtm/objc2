	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_handle_with_sel
	.p2align	4, 0x90
_handle_with_sel:
	push	rbp
	mov	rbp, rsp
	mov	rsi, qword ptr [rip + __ZN24test_msg_send_static_sel15handle_with_sel5do_it3REF17ha4d87ecf6e77d3d8E]
	pop	rbp
	jmp	_objc_msgSend

	.globl	_handle_alloc_init
	.p2align	4, 0x90
_handle_alloc_init:
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	rbx, qword ptr [rip + __ZN24test_msg_send_static_sel17handle_alloc_init5do_it3REF17hf24926dd82f1328eE]
	mov	rsi, qword ptr [rip + __ZN24test_msg_send_static_sel17handle_alloc_init5do_it3REF17hba215860b5dbdc32E]
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
	mov	rsi, qword ptr [rip + __ZN24test_msg_send_static_sel11use_generic5do_it3REF17h845175a6204d5101E]
	mov	rdx, qword ptr [rip + __ZN24test_msg_send_static_sel7generic5do_it3REF17h4fb667c67ff2fee7E]
	call	_objc_msgSend
	mov	rsi, qword ptr [rip + __ZN24test_msg_send_static_sel11use_generic5do_it3REF17hc2064b35e83288e1E]
	mov	rdx, qword ptr [rip + __ZN24test_msg_send_static_sel7generic5do_it3REF17h4fb667c67ff2fee7E]
	mov	rdi, rbx
	call	_objc_msgSend
	mov	rsi, qword ptr [rip + __ZN24test_msg_send_static_sel11use_generic5do_it3REF17ha259adabc85c9a3dE]
	mov	rdx, qword ptr [rip + __ZN24test_msg_send_static_sel7generic5do_it3REF17h4fb667c67ff2fee7E]
	mov	rdi, rbx
	add	rsp, 8
	pop	rbx
	pop	rbp
	jmp	_objc_msgSend

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel15handle_with_sel5do_it4NAME17hebe8ea07b55a546cE:
	.asciz	"someSelector"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	3
__ZN24test_msg_send_static_sel15handle_with_sel5do_it3REF17ha4d87ecf6e77d3d8E:
	.quad	__ZN24test_msg_send_static_sel15handle_with_sel5do_it4NAME17hebe8ea07b55a546cE

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel17handle_alloc_init5do_it4NAME17hbe28dbabc7e7cfb8E:
	.asciz	"init"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	3
__ZN24test_msg_send_static_sel17handle_alloc_init5do_it3REF17hf24926dd82f1328eE:
	.quad	__ZN24test_msg_send_static_sel17handle_alloc_init5do_it4NAME17hbe28dbabc7e7cfb8E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel17handle_alloc_init5do_it4NAME17h51e1533dabc0a607E:
	.asciz	"alloc"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	3
__ZN24test_msg_send_static_sel17handle_alloc_init5do_it3REF17hba215860b5dbdc32E:
	.quad	__ZN24test_msg_send_static_sel17handle_alloc_init5do_it4NAME17h51e1533dabc0a607E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel7generic5do_it4NAME17h2f8048ba9a24f713E:
	.asciz	"generic:selector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	3
__ZN24test_msg_send_static_sel7generic5do_it3REF17h4fb667c67ff2fee7E:
	.quad	__ZN24test_msg_send_static_sel7generic5do_it4NAME17h2f8048ba9a24f713E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel11use_generic5do_it4NAME17hbb02489017cc6d09E:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	3
__ZN24test_msg_send_static_sel11use_generic5do_it3REF17h845175a6204d5101E:
	.quad	__ZN24test_msg_send_static_sel11use_generic5do_it4NAME17hbb02489017cc6d09E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel11use_generic5do_it4NAME17h7596ed8d4d33ce59E:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	3
__ZN24test_msg_send_static_sel11use_generic5do_it3REF17hc2064b35e83288e1E:
	.quad	__ZN24test_msg_send_static_sel11use_generic5do_it4NAME17h7596ed8d4d33ce59E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel11use_generic5do_it4NAME17he928f70016dd8da3E:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	3
__ZN24test_msg_send_static_sel11use_generic5do_it3REF17ha259adabc85c9a3dE:
	.quad	__ZN24test_msg_send_static_sel11use_generic5do_it4NAME17he928f70016dd8da3E

.subsections_via_symbols
