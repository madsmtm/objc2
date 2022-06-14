	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_handle_alloc
	.p2align	4, 0x90
_handle_alloc:
	push	rbp
	mov	rbp, rsp
	call	_objc_msgSend
	test	rax, rax
	je	LBB0_2
	pop	rbp
	ret
LBB0_2:
	lea	rdi, [rip + l___unnamed_1]
	lea	rdx, [rip + l___unnamed_2]
	mov	esi, 17
	call	__ZN4core6option13expect_failed17h5222b7f29d1fceaaE

	.globl	_handle_init
	.p2align	4, 0x90
_handle_init:
	push	rbp
	mov	rbp, rsp
	pop	rbp
	jmp	_objc_msgSend

	.globl	_handle_alloc_init
	.p2align	4, 0x90
_handle_alloc_init:
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	rbx, rdx
	call	_objc_msgSend
	test	rax, rax
	je	LBB2_1
	mov	rdi, rax
	mov	rsi, rbx
	add	rsp, 8
	pop	rbx
	pop	rbp
	jmp	_objc_msgSend
LBB2_1:
	lea	rdi, [rip + l___unnamed_1]
	lea	rdx, [rip + l___unnamed_2]
	mov	esi, 17
	call	__ZN4core6option13expect_failed17h5222b7f29d1fceaaE

	.globl	_handle_alloc_release
	.p2align	4, 0x90
_handle_alloc_release:
	push	rbp
	mov	rbp, rsp
	call	_objc_msgSend
	test	rax, rax
	je	LBB3_1
	mov	rdi, rax
	pop	rbp
	jmp	_objc_release
LBB3_1:
	lea	rdi, [rip + l___unnamed_1]
	lea	rdx, [rip + l___unnamed_2]
	mov	esi, 17
	call	__ZN4core6option13expect_failed17h5222b7f29d1fceaaE

	.globl	_handle_alloc_init_release
	.p2align	4, 0x90
_handle_alloc_init_release:
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	rbx, rdx
	call	_objc_msgSend
	test	rax, rax
	je	LBB4_1
	mov	rdi, rax
	mov	rsi, rbx
	call	_objc_msgSend
	mov	rdi, rax
	add	rsp, 8
	pop	rbx
	pop	rbp
	jmp	_objc_release
LBB4_1:
	lea	rdi, [rip + l___unnamed_1]
	lea	rdx, [rip + l___unnamed_2]
	mov	esi, 17
	call	__ZN4core6option13expect_failed17h5222b7f29d1fceaaE

	.globl	_handle_copy
	.p2align	4, 0x90
_handle_copy:
	push	rbp
	mov	rbp, rsp
	pop	rbp
	jmp	_objc_msgSend

	.globl	_handle_autoreleased
	.p2align	4, 0x90
_handle_autoreleased:
	push	rbp
	mov	rbp, rsp
	call	_objc_msgSend
	mov	rdi, rax
	call	_objc_retainAutoreleasedReturnValue
	## InlineAsm Start

	nop

	## InlineAsm End
	pop	rbp
	ret

	.section	__TEXT,__const
l___unnamed_1:
	.ascii	"Failed allocating"

l___unnamed_3:
	.ascii	"$WORKSPACE/objc2/src/__macro_helpers.rs"

	.section	__DATA,__const
	.p2align	3
l___unnamed_2:
	.quad	l___unnamed_3
	.asciz	"B\000\000\000\000\000\000\000\037\000\000\000%\000\000"

.subsections_via_symbols
