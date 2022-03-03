	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_handle_with_sel
	.p2align	4, 0x90
_handle_with_sel:
	push	ebp
	mov	ebp, esp
	sub	esp, 8
	call	L0$pb
L0$pb:
	pop	eax
	sub	esp, 8
	push	dword ptr [eax + __ZN24test_msg_send_static_sel15handle_with_sel5do_it3REF17h494943a1fc7f70e1E-L0$pb]
	push	dword ptr [ebp + 8]
	call	_objc_msgSend
	add	esp, 24
	pop	ebp
	ret

	.globl	_handle_alloc_init
	.p2align	4, 0x90
_handle_alloc_init:
	push	ebp
	mov	ebp, esp
	push	esi
	push	eax
	call	L1$pb
L1$pb:
	pop	eax
	mov	esi, dword ptr [eax + __ZN24test_msg_send_static_sel17handle_alloc_init5do_it3REF17he8c072a7529a9166E-L1$pb]
	sub	esp, 8
	push	dword ptr [eax + __ZN24test_msg_send_static_sel17handle_alloc_init5do_it3REF17h0a9b51fcf825b29fE-L1$pb]
	push	dword ptr [ebp + 8]
	call	_objc_msgSend
	add	esp, 8
	push	esi
	push	eax
	call	_objc_msgSend
	add	esp, 20
	pop	esi
	pop	ebp
	ret

	.globl	_use_generic
	.p2align	4, 0x90
_use_generic:
	push	ebp
	mov	ebp, esp
	push	edi
	push	esi
	call	L2$pb
L2$pb:
	pop	esi
	mov	edi, dword ptr [ebp + 8]
	sub	esp, 4
	push	dword ptr [esi + __ZN24test_msg_send_static_sel7generic5do_it3REF17hd7d99d1530d1aa53E-L2$pb]
	push	dword ptr [esi + __ZN24test_msg_send_static_sel11use_generic5do_it3REF17h92275f9ad829b8d2E-L2$pb]
	push	edi
	call	_objc_msgSend
	add	esp, 12
	push	dword ptr [esi + __ZN24test_msg_send_static_sel7generic5do_it3REF17hd7d99d1530d1aa53E-L2$pb]
	push	dword ptr [esi + __ZN24test_msg_send_static_sel11use_generic5do_it3REF17hec8af9726862a51fE-L2$pb]
	push	edi
	call	_objc_msgSend
	add	esp, 12
	push	dword ptr [esi + __ZN24test_msg_send_static_sel7generic5do_it3REF17hd7d99d1530d1aa53E-L2$pb]
	push	dword ptr [esi + __ZN24test_msg_send_static_sel11use_generic5do_it3REF17h4ca08a2f79beb685E-L2$pb]
	push	edi
	call	_objc_msgSend
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebp
	ret

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel15handle_with_sel5do_it4NAME17heabddaa6f4c9743dE:
	.asciz	"someSelector"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN24test_msg_send_static_sel15handle_with_sel5do_it3REF17h494943a1fc7f70e1E:
	.long	__ZN24test_msg_send_static_sel15handle_with_sel5do_it4NAME17heabddaa6f4c9743dE

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel17handle_alloc_init5do_it4NAME17ha7bfa87c9398e80eE:
	.asciz	"init"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN24test_msg_send_static_sel17handle_alloc_init5do_it3REF17he8c072a7529a9166E:
	.long	__ZN24test_msg_send_static_sel17handle_alloc_init5do_it4NAME17ha7bfa87c9398e80eE

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel17handle_alloc_init5do_it4NAME17h91c65e3829956b99E:
	.asciz	"alloc"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN24test_msg_send_static_sel17handle_alloc_init5do_it3REF17h0a9b51fcf825b29fE:
	.long	__ZN24test_msg_send_static_sel17handle_alloc_init5do_it4NAME17h91c65e3829956b99E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel7generic5do_it4NAME17h327a9f2556c00325E:
	.asciz	"generic:selector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN24test_msg_send_static_sel7generic5do_it3REF17hd7d99d1530d1aa53E:
	.long	__ZN24test_msg_send_static_sel7generic5do_it4NAME17h327a9f2556c00325E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel11use_generic5do_it4NAME17hbf99e9b9cc3551bbE:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN24test_msg_send_static_sel11use_generic5do_it3REF17h92275f9ad829b8d2E:
	.long	__ZN24test_msg_send_static_sel11use_generic5do_it4NAME17hbf99e9b9cc3551bbE

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel11use_generic5do_it4NAME17h38dee9b38eec502dE:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN24test_msg_send_static_sel11use_generic5do_it3REF17hec8af9726862a51fE:
	.long	__ZN24test_msg_send_static_sel11use_generic5do_it4NAME17h38dee9b38eec502dE

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel11use_generic5do_it4NAME17h21b73c7a226720e8E:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN24test_msg_send_static_sel11use_generic5do_it3REF17h4ca08a2f79beb685E:
	.long	__ZN24test_msg_send_static_sel11use_generic5do_it4NAME17h21b73c7a226720e8E

.subsections_via_symbols
