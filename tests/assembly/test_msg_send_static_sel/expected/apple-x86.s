	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_handle_with_sel
	.p2align	4, 0x90
_handle_with_sel:
	push	ebp
	mov	ebp, esp
	push	esi
	push	eax
	mov	esi, dword ptr [ebp + 8]
	call	__ZN24test_msg_send_static_sel15handle_with_sel22objc_static_workaround17h54173f14552949d2E
	sub	esp, 8
	push	eax
	push	esi
	call	_objc_msgSend
	add	esp, 20
	pop	esi
	pop	ebp
	ret

	.globl	_handle_alloc_init
	.p2align	4, 0x90
_handle_alloc_init:
	push	ebp
	mov	ebp, esp
	push	edi
	push	esi
	mov	edi, dword ptr [ebp + 8]
	call	__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround17h71624dd5125cec62E
	mov	esi, eax
	call	__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround17h3fb39dd7fdd57a60E
	sub	esp, 8
	push	eax
	push	edi
	call	_objc_msgSend
	add	esp, 8
	push	esi
	push	eax
	call	_objc_msgSend
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebp
	ret

	.globl	_use_generic
	.p2align	4, 0x90
_use_generic:
	push	ebp
	mov	ebp, esp
	push	edi
	push	esi
	mov	edi, dword ptr [ebp + 8]
	call	__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17h04600c743bfbbabbE
	mov	esi, eax
	call	__ZN24test_msg_send_static_sel7generic22objc_static_workaround17h1c856d65f822badbE
	sub	esp, 4
	push	eax
	push	esi
	push	edi
	call	_objc_msgSend
	add	esp, 16
	call	__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17hd72d07b382ff6ef9E
	mov	esi, eax
	call	__ZN24test_msg_send_static_sel7generic22objc_static_workaround17h1c856d65f822badbE
	sub	esp, 4
	push	eax
	push	esi
	push	edi
	call	_objc_msgSend
	add	esp, 16
	call	__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17hd8ba6c1819f0e9e4E
	mov	esi, eax
	call	__ZN24test_msg_send_static_sel7generic22objc_static_workaround17h1c856d65f822badbE
	sub	esp, 4
	push	eax
	push	esi
	push	edi
	call	_objc_msgSend
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebp
	ret

	.p2align	4, 0x90
__ZN24test_msg_send_static_sel15handle_with_sel22objc_static_workaround17h54173f14552949d2E:
	push	ebp
	mov	ebp, esp
	call	L3$pb
L3$pb:
	pop	eax
	mov	eax, dword ptr [eax + __ZN24test_msg_send_static_sel15handle_with_sel22objc_static_workaround3REF17h5e1ebbc4849be566E-L3$pb]
	pop	ebp
	ret

	.p2align	4, 0x90
__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround17h71624dd5125cec62E:
	push	ebp
	mov	ebp, esp
	call	L4$pb
L4$pb:
	pop	eax
	mov	eax, dword ptr [eax + __ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround3REF17ha2b6d0788f464ae7E-L4$pb]
	pop	ebp
	ret

	.p2align	4, 0x90
__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround17h3fb39dd7fdd57a60E:
	push	ebp
	mov	ebp, esp
	call	L5$pb
L5$pb:
	pop	eax
	mov	eax, dword ptr [eax + __ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround3REF17h361219fab0d04681E-L5$pb]
	pop	ebp
	ret

	.p2align	4, 0x90
__ZN24test_msg_send_static_sel7generic22objc_static_workaround17h1c856d65f822badbE:
	push	ebp
	mov	ebp, esp
	call	L6$pb
L6$pb:
	pop	eax
	mov	eax, dword ptr [eax + __ZN24test_msg_send_static_sel7generic22objc_static_workaround3REF17h2b1b65d00f097392E-L6$pb]
	pop	ebp
	ret

	.p2align	4, 0x90
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17h04600c743bfbbabbE:
	push	ebp
	mov	ebp, esp
	call	L7$pb
L7$pb:
	pop	eax
	mov	eax, dword ptr [eax + __ZN24test_msg_send_static_sel11use_generic22objc_static_workaround3REF17hc08656a638ae4ef4E-L7$pb]
	pop	ebp
	ret

	.p2align	4, 0x90
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17hd72d07b382ff6ef9E:
	push	ebp
	mov	ebp, esp
	call	L8$pb
L8$pb:
	pop	eax
	mov	eax, dword ptr [eax + __ZN24test_msg_send_static_sel11use_generic22objc_static_workaround3REF17h0e7e70309040c696E-L8$pb]
	pop	ebp
	ret

	.p2align	4, 0x90
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17hd8ba6c1819f0e9e4E:
	push	ebp
	mov	ebp, esp
	call	L9$pb
L9$pb:
	pop	eax
	mov	eax, dword ptr [eax + __ZN24test_msg_send_static_sel11use_generic22objc_static_workaround3REF17h48d6ef0c68f60e9aE-L9$pb]
	pop	ebp
	ret

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel15handle_with_sel22objc_static_workaround4NAME17ha22895543d278f79E:
	.asciz	"someSelector"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN24test_msg_send_static_sel15handle_with_sel22objc_static_workaround3REF17h5e1ebbc4849be566E:
	.long	__ZN24test_msg_send_static_sel15handle_with_sel22objc_static_workaround4NAME17ha22895543d278f79E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround4NAME17hb75868d081570b88E:
	.asciz	"init"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround3REF17ha2b6d0788f464ae7E:
	.long	__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround4NAME17hb75868d081570b88E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround4NAME17h4e50417aff254a08E:
	.asciz	"alloc"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround3REF17h361219fab0d04681E:
	.long	__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround4NAME17h4e50417aff254a08E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel7generic22objc_static_workaround4NAME17h4772d8f4b62a3fffE:
	.asciz	"generic:selector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN24test_msg_send_static_sel7generic22objc_static_workaround3REF17h2b1b65d00f097392E:
	.long	__ZN24test_msg_send_static_sel7generic22objc_static_workaround4NAME17h4772d8f4b62a3fffE

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround4NAME17had8c28baa7c5a8fbE:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround3REF17hc08656a638ae4ef4E:
	.long	__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround4NAME17had8c28baa7c5a8fbE

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround4NAME17h2832cf14fc32e7c8E:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround3REF17h0e7e70309040c696E:
	.long	__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround4NAME17h2832cf14fc32e7c8E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround4NAME17h306e66092660f4c2E:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround3REF17h48d6ef0c68f60e9aE:
	.long	__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround4NAME17h306e66092660f4c2E

.subsections_via_symbols
