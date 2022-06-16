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
	push	dword ptr [eax + __ZN24test_msg_send_static_sel15handle_with_sel5do_it3REF17h65cc25d2143f786fE-L0$pb]
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
	mov	esi, dword ptr [eax + __ZN24test_msg_send_static_sel17handle_alloc_init5do_it3REF17h498e51a4429d4cebE-L1$pb]
	sub	esp, 8
	push	dword ptr [eax + __ZN24test_msg_send_static_sel17handle_alloc_init5do_it3REF17h97f4b0b87078d28fE-L1$pb]
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
	push	dword ptr [esi + __ZN24test_msg_send_static_sel7generic5do_it3REF17h3a04d62bb3a1f3bdE-L2$pb]
	push	dword ptr [esi + __ZN24test_msg_send_static_sel11use_generic5do_it3REF17hedb1f43f6216ce47E-L2$pb]
	push	edi
	call	_objc_msgSend
	add	esp, 12
	push	dword ptr [esi + __ZN24test_msg_send_static_sel7generic5do_it3REF17h3a04d62bb3a1f3bdE-L2$pb]
	push	dword ptr [esi + __ZN24test_msg_send_static_sel11use_generic5do_it3REF17h38ebe7d8068fa384E-L2$pb]
	push	edi
	call	_objc_msgSend
	add	esp, 12
	push	dword ptr [esi + __ZN24test_msg_send_static_sel7generic5do_it3REF17h3a04d62bb3a1f3bdE-L2$pb]
	push	dword ptr [esi + __ZN24test_msg_send_static_sel11use_generic5do_it3REF17h4def92294f8ef6c8E-L2$pb]
	push	edi
	call	_objc_msgSend
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebp
	ret

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel15handle_with_sel5do_it5VALUE17hc9de2e17c2d2dbd7E:
	.asciz	"someSelector"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	2
__ZN24test_msg_send_static_sel15handle_with_sel5do_it3REF17h65cc25d2143f786fE:
	.long	__ZN24test_msg_send_static_sel15handle_with_sel5do_it5VALUE17hc9de2e17c2d2dbd7E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel17handle_alloc_init5do_it5VALUE17h80cf87b1cbcff62cE:
	.asciz	"init"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	2
__ZN24test_msg_send_static_sel17handle_alloc_init5do_it3REF17h498e51a4429d4cebE:
	.long	__ZN24test_msg_send_static_sel17handle_alloc_init5do_it5VALUE17h80cf87b1cbcff62cE

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel17handle_alloc_init5do_it5VALUE17h6331c35d7f3ca7ecE:
	.asciz	"alloc"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	2
__ZN24test_msg_send_static_sel17handle_alloc_init5do_it3REF17h97f4b0b87078d28fE:
	.long	__ZN24test_msg_send_static_sel17handle_alloc_init5do_it5VALUE17h6331c35d7f3ca7ecE

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel7generic5do_it5VALUE17h5dc54c5eb2d3cedbE:
	.asciz	"generic:selector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	2
__ZN24test_msg_send_static_sel7generic5do_it3REF17h3a04d62bb3a1f3bdE:
	.long	__ZN24test_msg_send_static_sel7generic5do_it5VALUE17h5dc54c5eb2d3cedbE

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17hde48cd15665388c1E:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	2
__ZN24test_msg_send_static_sel11use_generic5do_it3REF17hedb1f43f6216ce47E:
	.long	__ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17hde48cd15665388c1E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17h8d540a1bd5813fe5E:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	2
__ZN24test_msg_send_static_sel11use_generic5do_it3REF17h38ebe7d8068fa384E:
	.long	__ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17h8d540a1bd5813fe5E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17h05f27bd87a98eec0E:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	2
__ZN24test_msg_send_static_sel11use_generic5do_it3REF17h4def92294f8ef6c8E:
	.long	__ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17h05f27bd87a98eec0E

.subsections_via_symbols
