	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
SYM(<_ as block2[CRATE_ID]::traits::IntoBlock<(_,), _>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::rc_block_drop::{closure#0}>, 0):
	ldr	x8, [x0, #32]
	ldr	w8, [x8]
	add	w0, w8, w1
	ret

	.p2align	2
SYM(<_ as block2[CRATE_ID]::traits::IntoBlock<(_,), _>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::stack_block_to_rc::{closure#0}>, 0):
	add	w0, w1, #2
	ret

	.p2align	2
SYM(<_ as block2[CRATE_ID]::traits::IntoBlock<(_,), _>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::create_and_use_rc_block::{closure#0}>, 0):
	add	w0, w1, #2
	ret

	.p2align	2
SYM(<_ as block2[CRATE_ID]::traits::IntoBlock<(_,), _>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::create_and_use_stack_block::{closure#0}>, 0):
	add	w0, w1, #2
	ret

	.p2align	2
SYM(<_ as block2[CRATE_ID]::traits::IntoBlock<(_,), _>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::create_and_use_stack_block_drop::{closure#0}>, 0):
	ldr	x8, [x0, #32]
	ldr	w8, [x8]
	add	w0, w8, w1
	ret

	.p2align	2
SYM(<_ as block2[CRATE_ID]::traits::IntoBlock<(_,), _>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::rc_block::{closure#0}>, 0):
	add	w0, w1, #2
	ret

	.p2align	2
SYM(<block2[CRATE_ID]::stack::StackBlock<(i32,), i32, test_block[CRATE_ID]::stack_block_to_rc::{closure#0}>>::clone_closure, 0):
	ret

	.p2align	2
SYM(<block2[CRATE_ID]::stack::StackBlock<(i32,), i32, test_block[CRATE_ID]::create_and_use_stack_block::{closure#0}>>::clone_closure, 0):
	ret

	.p2align	2
SYM(<block2[CRATE_ID]::stack::StackBlock<(i32,), i32, test_block[CRATE_ID]::create_and_use_stack_block_drop::{closure#0}>>::clone_closure, 0):
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x20, x1
	mov	x19, x0
Lloh0:
	adrp	x8, ___rust_no_alloc_shim_is_unstable@GOTPAGE
Lloh1:
	ldr	x8, [x8, ___rust_no_alloc_shim_is_unstable@GOTPAGEOFF]
	ldrb	wzr, [x8]
	mov	w0, #4
	mov	w1, #4
	bl	___rust_alloc
	cbz	x0, LBB8_2
	ldr	x8, [x20, #32]
	ldr	w8, [x8]
	str	w8, [x0]
	str	x0, [x19, #32]
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	ret
LBB8_2:
	mov	w0, #4
	mov	w1, #4
	bl	SYM(alloc::alloc::handle_alloc_error::GENERATED_ID, 0)
	.loh AdrpLdrGot	Lloh0, Lloh1

	.p2align	2
SYM(<block2[CRATE_ID]::stack::StackBlock<(i32,), i32, test_block[CRATE_ID]::rc_block_drop::{closure#0}>>::empty_clone_closure, 0):
	ret

	.p2align	2
SYM(<block2[CRATE_ID]::stack::StackBlock<(i32,), i32, test_block[CRATE_ID]::rc_block_drop::{closure#0}>>::drop_closure, 0):
	ldr	x0, [x0, #32]
	mov	w1, #4
	mov	w2, #4
	b	___rust_dealloc

	.p2align	2
SYM(<block2[CRATE_ID]::stack::StackBlock<(i32,), i32, test_block[CRATE_ID]::stack_block_to_rc::{closure#0}>>::drop_closure, 0):
	ret

	.p2align	2
SYM(<block2[CRATE_ID]::stack::StackBlock<(i32,), i32, test_block[CRATE_ID]::create_and_use_stack_block::{closure#0}>>::drop_closure, 0):
	ret

	.p2align	2
SYM(<block2[CRATE_ID]::stack::StackBlock<(i32,), i32, test_block[CRATE_ID]::create_and_use_stack_block_drop::{closure#0}>>::drop_closure, 0):
	ldr	x0, [x0, #32]
	mov	w1, #4
	mov	w2, #4
	b	___rust_dealloc

	.globl	_stack_block_to_rc
	.p2align	2
_stack_block_to_rc:
	sub	sp, sp, #48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
Lloh2:
	adrp	x8, __NSConcreteStackBlock@GOTPAGE
Lloh3:
	ldr	x8, [x8, __NSConcreteStackBlock@GOTPAGEOFF]
	mov	w9, #33554432
	stp	x8, x9, [sp]
Lloh4:
	adrp	x8, SYM(<_ as block2[CRATE_ID]::traits::IntoBlock<(_,), _>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::stack_block_to_rc::{closure#0}>, 0)@PAGE
Lloh5:
	add	x8, x8, SYM(<_ as block2[CRATE_ID]::traits::IntoBlock<(_,), _>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::stack_block_to_rc::{closure#0}>, 0)@PAGEOFF
Lloh6:
	adrp	x9, l_anon.[ID].2@PAGE
Lloh7:
	add	x9, x9, l_anon.[ID].2@PAGEOFF
	stp	x8, x9, [sp, #16]
	mov	x0, sp
	bl	__Block_copy
	cbz	x0, LBB14_2
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	ret
LBB14_2:
	bl	SYM(block2::rc_block::block_copy_fail::GENERATED_ID, 0)
	.loh AdrpAdd	Lloh6, Lloh7
	.loh AdrpAdd	Lloh4, Lloh5
	.loh AdrpLdrGot	Lloh2, Lloh3

	.globl	_rc_block
	.p2align	2
_rc_block:
	sub	sp, sp, #48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
Lloh8:
	adrp	x8, __NSConcreteStackBlock@GOTPAGE
Lloh9:
	ldr	x8, [x8, __NSConcreteStackBlock@GOTPAGEOFF]
	stp	x8, xzr, [sp]
Lloh10:
	adrp	x8, SYM(<_ as block2[CRATE_ID]::traits::IntoBlock<(_,), _>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::rc_block::{closure#0}>, 0)@PAGE
Lloh11:
	add	x8, x8, SYM(<_ as block2[CRATE_ID]::traits::IntoBlock<(_,), _>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::rc_block::{closure#0}>, 0)@PAGEOFF
Lloh12:
	adrp	x9, l_anon.[ID].1@PAGE
Lloh13:
	add	x9, x9, l_anon.[ID].1@PAGEOFF
	stp	x8, x9, [sp, #16]
	mov	x0, sp
	bl	__Block_copy
	cbz	x0, LBB15_2
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	ret
LBB15_2:
	bl	SYM(block2::rc_block::rc_new_fail::GENERATED_ID, 0)
	.loh AdrpAdd	Lloh12, Lloh13
	.loh AdrpAdd	Lloh10, Lloh11
	.loh AdrpLdrGot	Lloh8, Lloh9

	.globl	_rc_block_drop
	.p2align	2
_rc_block_drop:
	sub	sp, sp, #64
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
Lloh14:
	adrp	x8, __NSConcreteStackBlock@GOTPAGE
Lloh15:
	ldr	x8, [x8, __NSConcreteStackBlock@GOTPAGEOFF]
	mov	w9, #33554432
Lloh16:
	adrp	x10, SYM(<_ as block2[CRATE_ID]::traits::IntoBlock<(_,), _>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::rc_block_drop::{closure#0}>, 0)@PAGE
Lloh17:
	add	x10, x10, SYM(<_ as block2[CRATE_ID]::traits::IntoBlock<(_,), _>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::rc_block_drop::{closure#0}>, 0)@PAGEOFF
	stp	x8, x9, [sp, #8]
Lloh18:
	adrp	x8, l_anon.[ID].0@PAGE
Lloh19:
	add	x8, x8, l_anon.[ID].0@PAGEOFF
	stp	x10, x8, [sp, #24]
	str	x0, [sp, #40]
	add	x0, sp, #8
	bl	__Block_copy
	cbz	x0, LBB16_2
	ldp	x29, x30, [sp, #48]
	add	sp, sp, #64
	ret
LBB16_2:
	bl	SYM(block2::rc_block::rc_new_fail::GENERATED_ID, 0)
	.loh AdrpAdd	Lloh18, Lloh19
	.loh AdrpAdd	Lloh16, Lloh17
	.loh AdrpLdrGot	Lloh14, Lloh15

	.globl	_create_and_use_stack_block
	.p2align	2
_create_and_use_stack_block:
	sub	sp, sp, #48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
Lloh20:
	adrp	x8, __NSConcreteStackBlock@GOTPAGE
Lloh21:
	ldr	x8, [x8, __NSConcreteStackBlock@GOTPAGEOFF]
	mov	w9, #33554432
	stp	x8, x9, [sp]
Lloh22:
	adrp	x8, SYM(<_ as block2[CRATE_ID]::traits::IntoBlock<(_,), _>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::create_and_use_stack_block::{closure#0}>, 0)@PAGE
Lloh23:
	add	x8, x8, SYM(<_ as block2[CRATE_ID]::traits::IntoBlock<(_,), _>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::create_and_use_stack_block::{closure#0}>, 0)@PAGEOFF
Lloh24:
	adrp	x9, l_anon.[ID].3@PAGE
Lloh25:
	add	x9, x9, l_anon.[ID].3@PAGEOFF
	stp	x8, x9, [sp, #16]
	mov	x0, sp
	bl	_needs_block
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	ret
	.loh AdrpAdd	Lloh24, Lloh25
	.loh AdrpAdd	Lloh22, Lloh23
	.loh AdrpLdrGot	Lloh20, Lloh21

	.globl	_create_and_use_stack_block_drop
	.p2align	2
_create_and_use_stack_block_drop:
	sub	sp, sp, #80
	stp	x20, x19, [sp, #48]
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	mov	x19, x0
Lloh26:
	adrp	x8, __NSConcreteStackBlock@GOTPAGE
Lloh27:
	ldr	x8, [x8, __NSConcreteStackBlock@GOTPAGEOFF]
	mov	w9, #33554432
Lloh28:
	adrp	x10, SYM(<_ as block2[CRATE_ID]::traits::IntoBlock<(_,), _>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::create_and_use_stack_block_drop::{closure#0}>, 0)@PAGE
Lloh29:
	add	x10, x10, SYM(<_ as block2[CRATE_ID]::traits::IntoBlock<(_,), _>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::create_and_use_stack_block_drop::{closure#0}>, 0)@PAGEOFF
	stp	x8, x9, [sp, #8]
Lloh30:
	adrp	x8, l_anon.[ID].4@PAGE
Lloh31:
	add	x8, x8, l_anon.[ID].4@PAGEOFF
	stp	x10, x8, [sp, #24]
	str	x0, [sp, #40]
	add	x0, sp, #8
	bl	_needs_block
	mov	x0, x19
	mov	w1, #4
	mov	w2, #4
	bl	___rust_dealloc
	ldp	x29, x30, [sp, #64]
	ldp	x20, x19, [sp, #48]
	add	sp, sp, #80
	ret
	.loh AdrpAdd	Lloh30, Lloh31
	.loh AdrpAdd	Lloh28, Lloh29
	.loh AdrpLdrGot	Lloh26, Lloh27

	.globl	_create_and_use_rc_block
	.p2align	2
_create_and_use_rc_block:
	sub	sp, sp, #64
	stp	x20, x19, [sp, #32]
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
Lloh32:
	adrp	x8, __NSConcreteStackBlock@GOTPAGE
Lloh33:
	ldr	x8, [x8, __NSConcreteStackBlock@GOTPAGEOFF]
	stp	x8, xzr, [sp]
Lloh34:
	adrp	x8, SYM(<_ as block2[CRATE_ID]::traits::IntoBlock<(_,), _>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::create_and_use_rc_block::{closure#0}>, 0)@PAGE
Lloh35:
	add	x8, x8, SYM(<_ as block2[CRATE_ID]::traits::IntoBlock<(_,), _>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::create_and_use_rc_block::{closure#0}>, 0)@PAGEOFF
Lloh36:
	adrp	x9, l_anon.[ID].1@PAGE
Lloh37:
	add	x9, x9, l_anon.[ID].1@PAGEOFF
	stp	x8, x9, [sp, #16]
	mov	x0, sp
	bl	__Block_copy
	cbz	x0, LBB19_2
	mov	x19, x0
	bl	_needs_block
	mov	x0, x19
	bl	__Block_release
	ldp	x29, x30, [sp, #48]
	ldp	x20, x19, [sp, #32]
	add	sp, sp, #64
	ret
LBB19_2:
	bl	SYM(block2::rc_block::rc_new_fail::GENERATED_ID, 0)
	.loh AdrpAdd	Lloh36, Lloh37
	.loh AdrpAdd	Lloh34, Lloh35
	.loh AdrpLdrGot	Lloh32, Lloh33

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].0:
	.asciz	"\000\000\000\000\000\000\000\000(\000\000\000\000\000\000"
	.quad	SYM(<block2[CRATE_ID]::stack::StackBlock<(i32,), i32, test_block[CRATE_ID]::rc_block_drop::{closure#0}>>::empty_clone_closure, 0)
	.quad	SYM(<block2[CRATE_ID]::stack::StackBlock<(i32,), i32, test_block[CRATE_ID]::rc_block_drop::{closure#0}>>::drop_closure, 0)

	.section	__TEXT,__literal16,16byte_literals
	.p2align	3, 0x0
l_anon.[ID].1:
	.asciz	"\000\000\000\000\000\000\000\000 \000\000\000\000\000\000"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].2:
	.asciz	"\000\000\000\000\000\000\000\000 \000\000\000\000\000\000"
	.quad	SYM(<block2[CRATE_ID]::stack::StackBlock<(i32,), i32, test_block[CRATE_ID]::stack_block_to_rc::{closure#0}>>::clone_closure, 0)
	.quad	SYM(<block2[CRATE_ID]::stack::StackBlock<(i32,), i32, test_block[CRATE_ID]::stack_block_to_rc::{closure#0}>>::drop_closure, 0)

	.p2align	3, 0x0
l_anon.[ID].3:
	.asciz	"\000\000\000\000\000\000\000\000 \000\000\000\000\000\000"
	.quad	SYM(<block2[CRATE_ID]::stack::StackBlock<(i32,), i32, test_block[CRATE_ID]::create_and_use_stack_block::{closure#0}>>::clone_closure, 0)
	.quad	SYM(<block2[CRATE_ID]::stack::StackBlock<(i32,), i32, test_block[CRATE_ID]::create_and_use_stack_block::{closure#0}>>::drop_closure, 0)

	.p2align	3, 0x0
l_anon.[ID].4:
	.asciz	"\000\000\000\000\000\000\000\000(\000\000\000\000\000\000"
	.quad	SYM(<block2[CRATE_ID]::stack::StackBlock<(i32,), i32, test_block[CRATE_ID]::create_and_use_stack_block_drop::{closure#0}>>::clone_closure, 0)
	.quad	SYM(<block2[CRATE_ID]::stack::StackBlock<(i32,), i32, test_block[CRATE_ID]::create_and_use_stack_block_drop::{closure#0}>>::drop_closure, 0)

.subsections_via_symbols
