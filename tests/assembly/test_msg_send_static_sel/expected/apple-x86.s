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
	lea	eax, [eax + __ZN24test_msg_send_static_sel15handle_with_sel5do_it5VALUE17hc9de2e17c2d2dbd7E-L0$pb]
	push	eax
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
	pop	esi
	sub	esp, 8
	lea	eax, [esi + __ZN24test_msg_send_static_sel17handle_alloc_init5do_it5VALUE17h6331c35d7f3ca7ecE-L1$pb]
	push	eax
	push	dword ptr [ebp + 8]
	call	_objc_msgSend
	add	esp, 8
	lea	ecx, [esi + __ZN24test_msg_send_static_sel17handle_alloc_init5do_it5VALUE17h80cf87b1cbcff62cE-L1$pb]
	push	ecx
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
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	call	L2$pb
L2$pb:
	pop	edi
	mov	esi, dword ptr [ebp + 8]
	sub	esp, 4
	lea	ebx, [edi + __ZN24test_msg_send_static_sel7generic5do_it5VALUE17h5dc54c5eb2d3cedbE-L2$pb]
	lea	eax, [edi + __ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17hde48cd15665388c1E-L2$pb]
	push	ebx
	push	eax
	push	esi
	call	_objc_msgSend
	add	esp, 12
	lea	eax, [edi + __ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17h8d540a1bd5813fe5E-L2$pb]
	push	ebx
	push	eax
	push	esi
	call	_objc_msgSend
	add	esp, 12
	lea	eax, [edi + __ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17h05f27bd87a98eec0E-L2$pb]
	push	ebx
	push	eax
	push	esi
	call	_objc_msgSend
	add	esp, 28
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel15handle_with_sel5do_it5VALUE17hc9de2e17c2d2dbd7E:
	.asciz	"someSelector"

__ZN24test_msg_send_static_sel17handle_alloc_init5do_it5VALUE17h80cf87b1cbcff62cE:
	.asciz	"init"

__ZN24test_msg_send_static_sel17handle_alloc_init5do_it5VALUE17h6331c35d7f3ca7ecE:
	.asciz	"alloc"

__ZN24test_msg_send_static_sel7generic5do_it5VALUE17h5dc54c5eb2d3cedbE:
	.asciz	"generic:selector:"

__ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17hde48cd15665388c1E:
	.asciz	"performSelector:"

__ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17h8d540a1bd5813fe5E:
	.asciz	"performSelector:"

__ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17h05f27bd87a98eec0E:
	.asciz	"performSelector:"

.subsections_via_symbols
