	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.p2align	4, 0x90
SYM(block2[CRATE_ID]::stack::block_context_copy::<block2[CRATE_ID]::stack::StackBlock<(i32,), i32, test_block[CRATE_ID]::rc_block_drop::{closure#0}>>, 0):
	push	rbp
	mov	rbp, rsp
	pop	rbp
	ret

	.p2align	4, 0x90
SYM(block2[CRATE_ID]::stack::block_context_copy::<block2[CRATE_ID]::stack::StackBlock<(i32,), i32, test_block[CRATE_ID]::create_and_use_stack_block_drop::{closure#0}>>, 0):
	push	rbp
	mov	rbp, rsp
	pop	rbp
	ret

	.p2align	4, 0x90
SYM(block2[CRATE_ID]::stack::block_context_dispose::<block2[CRATE_ID]::stack::StackBlock<(i32,), i32, test_block[CRATE_ID]::rc_block_drop::{closure#0}>>, 0):
	push	rbp
	mov	rbp, rsp
	mov	rdi, qword ptr [rdi + 32]
	mov	esi, 4
	mov	edx, 4
	pop	rbp
	jmp	___rust_dealloc

	.p2align	4, 0x90
SYM(block2[CRATE_ID]::stack::block_context_dispose::<block2[CRATE_ID]::stack::StackBlock<(i32,), i32, test_block[CRATE_ID]::create_and_use_stack_block_drop::{closure#0}>>, 0):
	push	rbp
	mov	rbp, rsp
	mov	rdi, qword ptr [rdi + 32]
	mov	esi, 4
	mov	edx, 4
	pop	rbp
	jmp	___rust_dealloc

	.p2align	4, 0x90
SYM(<_ as block2[CRATE_ID]::stack::IntoBlock<(_,)>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::rc_block_drop::{closure#0}>, 0):
	push	rbp
	mov	rbp, rsp
	mov	eax, esi
	mov	rcx, qword ptr [rdi + 32]
	add	eax, dword ptr [rcx]
	pop	rbp
	ret

	.p2align	4, 0x90
SYM(<_ as block2[CRATE_ID]::stack::IntoBlock<(_,)>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::stack_block_to_rc::{closure#0}>, 0):
	push	rbp
	mov	rbp, rsp
	lea	eax, [rsi + 2]
	pop	rbp
	ret

	.p2align	4, 0x90
SYM(<_ as block2[CRATE_ID]::stack::IntoBlock<(_,)>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::create_and_use_rc_block::{closure#0}>, 0):
	push	rbp
	mov	rbp, rsp
	lea	eax, [rsi + 2]
	pop	rbp
	ret

	.p2align	4, 0x90
SYM(<_ as block2[CRATE_ID]::stack::IntoBlock<(_,)>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::create_and_use_stack_block::{closure#0}>, 0):
	push	rbp
	mov	rbp, rsp
	lea	eax, [rsi + 2]
	pop	rbp
	ret

	.p2align	4, 0x90
SYM(<_ as block2[CRATE_ID]::stack::IntoBlock<(_,)>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::create_and_use_stack_block_drop::{closure#0}>, 0):
	push	rbp
	mov	rbp, rsp
	mov	eax, esi
	mov	rcx, qword ptr [rdi + 32]
	add	eax, dword ptr [rcx]
	pop	rbp
	ret

	.p2align	4, 0x90
SYM(<_ as block2[CRATE_ID]::stack::IntoBlock<(_,)>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::rc_block::{closure#0}>, 0):
	push	rbp
	mov	rbp, rsp
	lea	eax, [rsi + 3]
	pop	rbp
	ret

	.globl	_stack_block_to_rc
	.p2align	4, 0x90
_stack_block_to_rc:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 32
	mov	rax, qword ptr [rip + __NSConcreteStackBlock@GOTPCREL]
	mov	qword ptr [rbp - 32], rax
	mov	qword ptr [rbp - 24], 0
	lea	rax, [rip + SYM(<_ as block2[CRATE_ID]::stack::IntoBlock<(_,)>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::stack_block_to_rc::{closure#0}>, 0)]
	mov	qword ptr [rbp - 16], rax
	lea	rax, [rip + l_anon.[ID].1]
	mov	qword ptr [rbp - 8], rax
	lea	rdi, [rbp - 32]
	call	__Block_copy
	add	rsp, 32
	pop	rbp
	ret

	.globl	_rc_block
	.p2align	4, 0x90
_rc_block:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 32
	mov	rax, qword ptr [rip + __NSConcreteStackBlock@GOTPCREL]
	mov	qword ptr [rbp - 32], rax
	mov	qword ptr [rbp - 24], 0
	lea	rax, [rip + SYM(<_ as block2[CRATE_ID]::stack::IntoBlock<(_,)>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::rc_block::{closure#0}>, 0)]
	mov	qword ptr [rbp - 16], rax
	lea	rax, [rip + l_anon.[ID].1]
	mov	qword ptr [rbp - 8], rax
	lea	rdi, [rbp - 32]
	call	__Block_copy
	add	rsp, 32
	pop	rbp
	ret

	.globl	_rc_block_drop
	.p2align	4, 0x90
_rc_block_drop:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 48
	mov	rax, qword ptr [rip + __NSConcreteStackBlock@GOTPCREL]
	mov	qword ptr [rbp - 40], rax
	mov	qword ptr [rbp - 32], 33554432
	lea	rax, [rip + SYM(<_ as block2[CRATE_ID]::stack::IntoBlock<(_,)>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::rc_block_drop::{closure#0}>, 0)]
	mov	qword ptr [rbp - 24], rax
	lea	rax, [rip + l_anon.[ID].0]
	mov	qword ptr [rbp - 16], rax
	mov	qword ptr [rbp - 8], rdi
	lea	rdi, [rbp - 40]
	call	__Block_copy
	add	rsp, 48
	pop	rbp
	ret

	.globl	_create_and_use_stack_block
	.p2align	4, 0x90
_create_and_use_stack_block:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 32
	mov	rax, qword ptr [rip + __NSConcreteStackBlock@GOTPCREL]
	mov	qword ptr [rbp - 32], rax
	mov	qword ptr [rbp - 24], 0
	lea	rax, [rip + SYM(<_ as block2[CRATE_ID]::stack::IntoBlock<(_,)>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::create_and_use_stack_block::{closure#0}>, 0)]
	mov	qword ptr [rbp - 16], rax
	lea	rax, [rip + l_anon.[ID].1]
	mov	qword ptr [rbp - 8], rax
	lea	rdi, [rbp - 32]
	call	_needs_block
	add	rsp, 32
	pop	rbp
	ret

	.globl	_create_and_use_stack_block_drop
	.p2align	4, 0x90
_create_and_use_stack_block_drop:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 48
	mov	rax, qword ptr [rip + __NSConcreteStackBlock@GOTPCREL]
	mov	qword ptr [rbp - 40], rax
	mov	qword ptr [rbp - 32], 33554432
	lea	rax, [rip + SYM(<_ as block2[CRATE_ID]::stack::IntoBlock<(_,)>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::create_and_use_stack_block_drop::{closure#0}>, 0)]
	mov	qword ptr [rbp - 24], rax
	lea	rax, [rip + l_anon.[ID].2]
	mov	qword ptr [rbp - 16], rax
	mov	qword ptr [rbp - 8], rdi
	lea	rdi, [rbp - 40]
	call	_needs_block
	mov	rdi, qword ptr [rbp - 8]
	mov	esi, 4
	mov	edx, 4
	call	___rust_dealloc
	add	rsp, 48
	pop	rbp
	ret

	.globl	_create_and_use_rc_block
	.p2align	4, 0x90
_create_and_use_rc_block:
	push	rbp
	mov	rbp, rsp
	push	rbx
	sub	rsp, 40
	mov	rax, qword ptr [rip + __NSConcreteStackBlock@GOTPCREL]
	mov	qword ptr [rbp - 40], rax
	mov	qword ptr [rbp - 32], 0
	lea	rax, [rip + SYM(<_ as block2[CRATE_ID]::stack::IntoBlock<(_,)>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::create_and_use_rc_block::{closure#0}>, 0)]
	mov	qword ptr [rbp - 24], rax
	lea	rax, [rip + l_anon.[ID].1]
	mov	qword ptr [rbp - 16], rax
	lea	rdi, [rbp - 40]
	call	__Block_copy
	mov	rbx, rax
	mov	rdi, rax
	call	_needs_block
	mov	rdi, rbx
	call	__Block_release
	add	rsp, 40
	pop	rbx
	pop	rbp
	ret

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].0:
	.asciz	"\000\000\000\000\000\000\000\000(\000\000\000\000\000\000"
	.quad	SYM(block2[CRATE_ID]::stack::block_context_copy::<block2[CRATE_ID]::stack::StackBlock<(i32,), i32, test_block[CRATE_ID]::rc_block_drop::{closure#0}>>, 0)
	.quad	SYM(block2[CRATE_ID]::stack::block_context_dispose::<block2[CRATE_ID]::stack::StackBlock<(i32,), i32, test_block[CRATE_ID]::rc_block_drop::{closure#0}>>, 0)

	.section	__TEXT,__const
	.p2align	3, 0x0
l_anon.[ID].1:
	.asciz	"\000\000\000\000\000\000\000\000 \000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].2:
	.asciz	"\000\000\000\000\000\000\000\000(\000\000\000\000\000\000"
	.quad	SYM(block2[CRATE_ID]::stack::block_context_copy::<block2[CRATE_ID]::stack::StackBlock<(i32,), i32, test_block[CRATE_ID]::create_and_use_stack_block_drop::{closure#0}>>, 0)
	.quad	SYM(block2[CRATE_ID]::stack::block_context_dispose::<block2[CRATE_ID]::stack::StackBlock<(i32,), i32, test_block[CRATE_ID]::create_and_use_stack_block_drop::{closure#0}>>, 0)

.subsections_via_symbols
