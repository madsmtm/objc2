	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.p2align	4
SYM(<_ as block2[CRATE_ID]::traits::IntoBlock<(_,), _>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::rc_block_drop::{closure#0}>, 0):
	push	rbp
	mov	rbp, rsp
	mov	eax, esi
	mov	rcx, qword ptr [rdi + 32]
	add	eax, dword ptr [rcx]
	pop	rbp
	ret

	.p2align	4
SYM(<_ as block2[CRATE_ID]::traits::IntoBlock<(_,), _>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::stack_block_to_rc::{closure#0}>, 0):
	push	rbp
	mov	rbp, rsp
	lea	eax, [rsi + 2]
	pop	rbp
	ret

	.p2align	4
SYM(<_ as block2[CRATE_ID]::traits::IntoBlock<(_,), _>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::create_and_use_rc_block::{closure#0}>, 0):
	push	rbp
	mov	rbp, rsp
	lea	eax, [rsi + 2]
	pop	rbp
	ret

	.p2align	4
SYM(<_ as block2[CRATE_ID]::traits::IntoBlock<(_,), _>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::create_and_use_stack_block::{closure#0}>, 0):
	push	rbp
	mov	rbp, rsp
	lea	eax, [rsi + 2]
	pop	rbp
	ret

	.p2align	4
SYM(<_ as block2[CRATE_ID]::traits::IntoBlock<(_,), _>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::create_and_use_stack_block_drop::{closure#0}>, 0):
	push	rbp
	mov	rbp, rsp
	mov	eax, esi
	mov	rcx, qword ptr [rdi + 32]
	add	eax, dword ptr [rcx]
	pop	rbp
	ret

	.p2align	4
SYM(<_ as block2[CRATE_ID]::traits::IntoBlock<(_,), _>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::rc_block::{closure#0}>, 0):
	push	rbp
	mov	rbp, rsp
	lea	eax, [rsi + 2]
	pop	rbp
	ret

	.p2align	4
SYM(<block2[CRATE_ID]::stack::StackBlock<(i32,), i32, test_block[CRATE_ID]::stack_block_to_rc::{closure#0}>>::clone_closure, 0):
	push	rbp
	mov	rbp, rsp
	pop	rbp
	ret

	.p2align	4
SYM(<block2[CRATE_ID]::stack::StackBlock<(i32,), i32, test_block[CRATE_ID]::create_and_use_stack_block::{closure#0}>>::clone_closure, 0):
	push	rbp
	mov	rbp, rsp
	pop	rbp
	ret

	.p2align	4
SYM(<block2[CRATE_ID]::stack::StackBlock<(i32,), i32, test_block[CRATE_ID]::create_and_use_stack_block_drop::{closure#0}>>::clone_closure, 0):
	push	rbp
	mov	rbp, rsp
	push	r14
	push	rbx
	mov	r14, rsi
	mov	rbx, rdi
	mov	rax, qword ptr [rip + ___rust_no_alloc_shim_is_unstable@GOTPCREL]
	movzx	eax, byte ptr [rax]
	mov	edi, 4
	mov	esi, 4
	call	SYM(__rustc[CRATE_ID]::__rust_alloc, 0)
	test	rax, rax
	je	LBB8_2
	mov	rcx, qword ptr [r14 + 32]
	mov	ecx, dword ptr [rcx]
	mov	dword ptr [rax], ecx
	mov	qword ptr [rbx + 32], rax
	pop	rbx
	pop	r14
	pop	rbp
	ret
LBB8_2:
	mov	edi, 4
	mov	esi, 4
	call	SYM(alloc::alloc::handle_alloc_error::GENERATED_ID, 0)

	.p2align	4
SYM(<block2[CRATE_ID]::stack::StackBlock<(i32,), i32, test_block[CRATE_ID]::rc_block_drop::{closure#0}>>::empty_clone_closure, 0):
	push	rbp
	mov	rbp, rsp
	pop	rbp
	ret

	.p2align	4
SYM(<block2[CRATE_ID]::stack::StackBlock<(i32,), i32, test_block[CRATE_ID]::rc_block_drop::{closure#0}>>::drop_closure, 0):
	push	rbp
	mov	rbp, rsp
	mov	rdi, qword ptr [rdi + 32]
	mov	esi, 4
	mov	edx, 4
	pop	rbp
	jmp	SYM(__rustc[CRATE_ID]::__rust_dealloc, 0)

	.p2align	4
SYM(<block2[CRATE_ID]::stack::StackBlock<(i32,), i32, test_block[CRATE_ID]::stack_block_to_rc::{closure#0}>>::drop_closure, 0):
	push	rbp
	mov	rbp, rsp
	pop	rbp
	ret

	.p2align	4
SYM(<block2[CRATE_ID]::stack::StackBlock<(i32,), i32, test_block[CRATE_ID]::create_and_use_stack_block::{closure#0}>>::drop_closure, 0):
	push	rbp
	mov	rbp, rsp
	pop	rbp
	ret

	.p2align	4
SYM(<block2[CRATE_ID]::stack::StackBlock<(i32,), i32, test_block[CRATE_ID]::create_and_use_stack_block_drop::{closure#0}>>::drop_closure, 0):
	push	rbp
	mov	rbp, rsp
	mov	rdi, qword ptr [rdi + 32]
	mov	esi, 4
	mov	edx, 4
	pop	rbp
	jmp	SYM(__rustc[CRATE_ID]::__rust_dealloc, 0)

	.globl	_stack_block_to_rc
	.p2align	4
_stack_block_to_rc:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 32
	mov	rax, qword ptr [rip + __NSConcreteStackBlock@GOTPCREL]
	mov	qword ptr [rbp - 32], rax
	mov	qword ptr [rbp - 24], 33554432
	lea	rax, [rip + SYM(<_ as block2[CRATE_ID]::traits::IntoBlock<(_,), _>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::stack_block_to_rc::{closure#0}>, 0)]
	mov	qword ptr [rbp - 16], rax
	lea	rax, [rip + l_anon.[ID].0]
	mov	qword ptr [rbp - 8], rax
	lea	rdi, [rbp - 32]
	call	__Block_copy
	test	rax, rax
	je	LBB14_2
	add	rsp, 32
	pop	rbp
	ret
LBB14_2:
	call	SYM(block2::rc_block::block_copy_fail::GENERATED_ID, 0)

	.globl	_rc_block
	.p2align	4
_rc_block:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 32
	mov	rax, qword ptr [rip + __NSConcreteStackBlock@GOTPCREL]
	mov	qword ptr [rbp - 32], rax
	mov	qword ptr [rbp - 24], 0
	lea	rax, [rip + SYM(<_ as block2[CRATE_ID]::traits::IntoBlock<(_,), _>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::rc_block::{closure#0}>, 0)]
	mov	qword ptr [rbp - 16], rax
	lea	rax, [rip + L_anon.[ID].4]
	mov	qword ptr [rbp - 8], rax
	lea	rdi, [rbp - 32]
	call	__Block_copy
	test	rax, rax
	je	LBB15_2
	add	rsp, 32
	pop	rbp
	ret
LBB15_2:
	call	SYM(block2::rc_block::rc_new_fail::GENERATED_ID, 0)

	.globl	_rc_block_drop
	.p2align	4
_rc_block_drop:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 48
	mov	rax, qword ptr [rip + __NSConcreteStackBlock@GOTPCREL]
	mov	qword ptr [rbp - 40], rax
	mov	qword ptr [rbp - 32], 33554432
	lea	rax, [rip + SYM(<_ as block2[CRATE_ID]::traits::IntoBlock<(_,), _>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::rc_block_drop::{closure#0}>, 0)]
	mov	qword ptr [rbp - 24], rax
	lea	rax, [rip + l_anon.[ID].3]
	mov	qword ptr [rbp - 16], rax
	mov	qword ptr [rbp - 8], rdi
	lea	rdi, [rbp - 40]
	call	__Block_copy
	test	rax, rax
	je	LBB16_2
	add	rsp, 48
	pop	rbp
	ret
LBB16_2:
	call	SYM(block2::rc_block::rc_new_fail::GENERATED_ID, 0)

	.globl	_create_and_use_stack_block
	.p2align	4
_create_and_use_stack_block:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 32
	mov	rax, qword ptr [rip + __NSConcreteStackBlock@GOTPCREL]
	mov	qword ptr [rbp - 32], rax
	mov	qword ptr [rbp - 24], 33554432
	lea	rax, [rip + SYM(<_ as block2[CRATE_ID]::traits::IntoBlock<(_,), _>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::create_and_use_stack_block::{closure#0}>, 0)]
	mov	qword ptr [rbp - 16], rax
	lea	rax, [rip + l_anon.[ID].1]
	mov	qword ptr [rbp - 8], rax
	lea	rdi, [rbp - 32]
	call	_needs_block
	add	rsp, 32
	pop	rbp
	ret

	.globl	_create_and_use_stack_block_drop
	.p2align	4
_create_and_use_stack_block_drop:
	push	rbp
	mov	rbp, rsp
	push	rbx
	sub	rsp, 40
	mov	rbx, rdi
	mov	rax, qword ptr [rip + __NSConcreteStackBlock@GOTPCREL]
	mov	qword ptr [rbp - 48], rax
	mov	qword ptr [rbp - 40], 33554432
	lea	rax, [rip + SYM(<_ as block2[CRATE_ID]::traits::IntoBlock<(_,), _>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::create_and_use_stack_block_drop::{closure#0}>, 0)]
	mov	qword ptr [rbp - 32], rax
	lea	rax, [rip + l_anon.[ID].2]
	mov	qword ptr [rbp - 24], rax
	mov	qword ptr [rbp - 16], rdi
	lea	rdi, [rbp - 48]
	call	_needs_block
	mov	esi, 4
	mov	edx, 4
	mov	rdi, rbx
	call	SYM(__rustc[CRATE_ID]::__rust_dealloc, 0)
	add	rsp, 40
	pop	rbx
	pop	rbp
	ret

	.globl	_create_and_use_rc_block
	.p2align	4
_create_and_use_rc_block:
	push	rbp
	mov	rbp, rsp
	push	rbx
	sub	rsp, 40
	mov	rax, qword ptr [rip + __NSConcreteStackBlock@GOTPCREL]
	mov	qword ptr [rbp - 40], rax
	mov	qword ptr [rbp - 32], 0
	lea	rax, [rip + SYM(<_ as block2[CRATE_ID]::traits::IntoBlock<(_,), _>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::create_and_use_rc_block::{closure#0}>, 0)]
	mov	qword ptr [rbp - 24], rax
	lea	rax, [rip + L_anon.[ID].4]
	mov	qword ptr [rbp - 16], rax
	lea	rdi, [rbp - 40]
	call	__Block_copy
	test	rax, rax
	je	LBB19_2
	mov	rbx, rax
	mov	rdi, rax
	call	_needs_block
	mov	rdi, rbx
	call	__Block_release
	add	rsp, 40
	pop	rbx
	pop	rbp
	ret
LBB19_2:
	call	SYM(block2::rc_block::rc_new_fail::GENERATED_ID, 0)

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].0:
	.asciz	"\000\000\000\000\000\000\000\000 \000\000\000\000\000\000"
	.quad	SYM(<block2[CRATE_ID]::stack::StackBlock<(i32,), i32, test_block[CRATE_ID]::stack_block_to_rc::{closure#0}>>::clone_closure, 0)
	.quad	SYM(<block2[CRATE_ID]::stack::StackBlock<(i32,), i32, test_block[CRATE_ID]::stack_block_to_rc::{closure#0}>>::drop_closure, 0)

	.p2align	3, 0x0
l_anon.[ID].1:
	.asciz	"\000\000\000\000\000\000\000\000 \000\000\000\000\000\000"
	.quad	SYM(<block2[CRATE_ID]::stack::StackBlock<(i32,), i32, test_block[CRATE_ID]::create_and_use_stack_block::{closure#0}>>::clone_closure, 0)
	.quad	SYM(<block2[CRATE_ID]::stack::StackBlock<(i32,), i32, test_block[CRATE_ID]::create_and_use_stack_block::{closure#0}>>::drop_closure, 0)

	.p2align	3, 0x0
l_anon.[ID].2:
	.asciz	"\000\000\000\000\000\000\000\000(\000\000\000\000\000\000"
	.quad	SYM(<block2[CRATE_ID]::stack::StackBlock<(i32,), i32, test_block[CRATE_ID]::create_and_use_stack_block_drop::{closure#0}>>::clone_closure, 0)
	.quad	SYM(<block2[CRATE_ID]::stack::StackBlock<(i32,), i32, test_block[CRATE_ID]::create_and_use_stack_block_drop::{closure#0}>>::drop_closure, 0)

	.p2align	3, 0x0
l_anon.[ID].3:
	.asciz	"\000\000\000\000\000\000\000\000(\000\000\000\000\000\000"
	.quad	SYM(<block2[CRATE_ID]::stack::StackBlock<(i32,), i32, test_block[CRATE_ID]::rc_block_drop::{closure#0}>>::empty_clone_closure, 0)
	.quad	SYM(<block2[CRATE_ID]::stack::StackBlock<(i32,), i32, test_block[CRATE_ID]::rc_block_drop::{closure#0}>>::drop_closure, 0)

	.section	__TEXT,__literal16,16byte_literals
	.p2align	3, 0x0
L_anon.[ID].4:
	.asciz	"\000\000\000\000\000\000\000\000 \000\000\000\000\000\000"

.subsections_via_symbols
