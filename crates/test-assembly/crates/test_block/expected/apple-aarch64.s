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
	mov	x19, x0
	ldr	x20, [x1, #32]
	bl	SYM(__rustc[CRATE_ID]::__rust_no_alloc_shim_is_unstable_v2, 0)
	mov	w0, #4
	mov	w1, #4
	bl	SYM(__rustc[CRATE_ID]::__rust_alloc, 0)
	cbz	x0, LBB8_2
	ldr	w8, [x20]
	str	w8, [x0]
	str	x0, [x19, #32]
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	ret
LBB8_2:
	mov	w0, #4
	mov	w1, #4
	bl	SYM(alloc::alloc::handle_alloc_error::GENERATED_ID, 0)

	.p2align	2
SYM(<block2[CRATE_ID]::stack::StackBlock<(i32,), i32, test_block[CRATE_ID]::rc_block_drop::{closure#0}>>::empty_clone_closure, 0):
	ret

	.p2align	2
SYM(<block2[CRATE_ID]::stack::StackBlock<(i32,), i32, test_block[CRATE_ID]::rc_block_drop::{closure#0}>>::drop_closure, 0):
	ldr	x0, [x0, #32]
	mov	w1, #4
	mov	w2, #4
	b	SYM(__rustc[CRATE_ID]::__rust_dealloc, 0)

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
	b	SYM(__rustc[CRATE_ID]::__rust_dealloc, 0)

	.globl	_fn1_stack_block_to_rc
	.p2align	2
_fn1_stack_block_to_rc:
	sub	sp, sp, #48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
Lloh0:
	adrp	x8, __NSConcreteStackBlock@GOTPAGE
Lloh1:
	ldr	x8, [x8, __NSConcreteStackBlock@GOTPAGEOFF]
	mov	w9, #33554432
	stp	x8, x9, [sp]
Lloh2:
	adrp	x8, SYM(<_ as block2[CRATE_ID]::traits::IntoBlock<(_,), _>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::stack_block_to_rc::{closure#0}>, 0)@PAGE
Lloh3:
	add	x8, x8, SYM(<_ as block2[CRATE_ID]::traits::IntoBlock<(_,), _>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::stack_block_to_rc::{closure#0}>, 0)@PAGEOFF
Lloh4:
	adrp	x9, l_anon.[ID].0@PAGE
Lloh5:
	add	x9, x9, l_anon.[ID].0@PAGEOFF
	stp	x8, x9, [sp, #16]
	mov	x0, sp
	bl	__Block_copy
	cbz	x0, LBB14_2
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	ret
LBB14_2:
	bl	SYM(block2::rc_block::block_copy_fail::GENERATED_ID, 0)
	.loh AdrpAdd	Lloh4, Lloh5
	.loh AdrpAdd	Lloh2, Lloh3
	.loh AdrpLdrGot	Lloh0, Lloh1

	.globl	_fn2_rc_block
	.p2align	2
_fn2_rc_block:
	sub	sp, sp, #48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
Lloh6:
	adrp	x8, __NSConcreteStackBlock@GOTPAGE
Lloh7:
	ldr	x8, [x8, __NSConcreteStackBlock@GOTPAGEOFF]
	stp	x8, xzr, [sp]
Lloh8:
	adrp	x8, SYM(<_ as block2[CRATE_ID]::traits::IntoBlock<(_,), _>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::rc_block::{closure#0}>, 0)@PAGE
Lloh9:
	add	x8, x8, SYM(<_ as block2[CRATE_ID]::traits::IntoBlock<(_,), _>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::rc_block::{closure#0}>, 0)@PAGEOFF
Lloh10:
	adrp	x9, l_anon.[ID].4@PAGE
Lloh11:
	add	x9, x9, l_anon.[ID].4@PAGEOFF
	stp	x8, x9, [sp, #16]
	mov	x0, sp
	bl	__Block_copy
	cbz	x0, LBB15_2
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	ret
LBB15_2:
	bl	SYM(block2::rc_block::rc_new_fail::GENERATED_ID, 0)
	.loh AdrpAdd	Lloh10, Lloh11
	.loh AdrpAdd	Lloh8, Lloh9
	.loh AdrpLdrGot	Lloh6, Lloh7

	.globl	_fn3_rc_block_drop
	.p2align	2
_fn3_rc_block_drop:
	sub	sp, sp, #64
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
Lloh12:
	adrp	x8, __NSConcreteStackBlock@GOTPAGE
Lloh13:
	ldr	x8, [x8, __NSConcreteStackBlock@GOTPAGEOFF]
	mov	w9, #33554432
Lloh14:
	adrp	x10, SYM(<_ as block2[CRATE_ID]::traits::IntoBlock<(_,), _>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::rc_block_drop::{closure#0}>, 0)@PAGE
Lloh15:
	add	x10, x10, SYM(<_ as block2[CRATE_ID]::traits::IntoBlock<(_,), _>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::rc_block_drop::{closure#0}>, 0)@PAGEOFF
	stp	x8, x9, [sp, #8]
Lloh16:
	adrp	x8, l_anon.[ID].3@PAGE
Lloh17:
	add	x8, x8, l_anon.[ID].3@PAGEOFF
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
	.loh AdrpAdd	Lloh16, Lloh17
	.loh AdrpAdd	Lloh14, Lloh15
	.loh AdrpLdrGot	Lloh12, Lloh13

	.globl	_fn4_create_and_use_stack_block
	.p2align	2
_fn4_create_and_use_stack_block:
	sub	sp, sp, #48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
Lloh18:
	adrp	x8, __NSConcreteStackBlock@GOTPAGE
Lloh19:
	ldr	x8, [x8, __NSConcreteStackBlock@GOTPAGEOFF]
	mov	w9, #33554432
	stp	x8, x9, [sp]
Lloh20:
	adrp	x8, SYM(<_ as block2[CRATE_ID]::traits::IntoBlock<(_,), _>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::create_and_use_stack_block::{closure#0}>, 0)@PAGE
Lloh21:
	add	x8, x8, SYM(<_ as block2[CRATE_ID]::traits::IntoBlock<(_,), _>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::create_and_use_stack_block::{closure#0}>, 0)@PAGEOFF
Lloh22:
	adrp	x9, l_anon.[ID].1@PAGE
Lloh23:
	add	x9, x9, l_anon.[ID].1@PAGEOFF
	stp	x8, x9, [sp, #16]
	mov	x0, sp
	bl	_needs_block
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	ret
	.loh AdrpAdd	Lloh22, Lloh23
	.loh AdrpAdd	Lloh20, Lloh21
	.loh AdrpLdrGot	Lloh18, Lloh19

	.globl	_fn5_create_and_use_stack_block_drop
	.p2align	2
_fn5_create_and_use_stack_block_drop:
	sub	sp, sp, #80
	stp	x20, x19, [sp, #48]
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	mov	x19, x0
Lloh24:
	adrp	x8, __NSConcreteStackBlock@GOTPAGE
Lloh25:
	ldr	x8, [x8, __NSConcreteStackBlock@GOTPAGEOFF]
	mov	w9, #33554432
Lloh26:
	adrp	x10, SYM(<_ as block2[CRATE_ID]::traits::IntoBlock<(_,), _>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::create_and_use_stack_block_drop::{closure#0}>, 0)@PAGE
Lloh27:
	add	x10, x10, SYM(<_ as block2[CRATE_ID]::traits::IntoBlock<(_,), _>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::create_and_use_stack_block_drop::{closure#0}>, 0)@PAGEOFF
	stp	x8, x9, [sp, #8]
Lloh28:
	adrp	x8, l_anon.[ID].2@PAGE
Lloh29:
	add	x8, x8, l_anon.[ID].2@PAGEOFF
	stp	x10, x8, [sp, #24]
	str	x0, [sp, #40]
	add	x0, sp, #8
	bl	_needs_block
	mov	x0, x19
	mov	w1, #4
	mov	w2, #4
	bl	SYM(__rustc[CRATE_ID]::__rust_dealloc, 0)
	ldp	x29, x30, [sp, #64]
	ldp	x20, x19, [sp, #48]
	add	sp, sp, #80
	ret
	.loh AdrpAdd	Lloh28, Lloh29
	.loh AdrpAdd	Lloh26, Lloh27
	.loh AdrpLdrGot	Lloh24, Lloh25

	.globl	_fn6_create_and_use_rc_block
	.p2align	2
_fn6_create_and_use_rc_block:
	sub	sp, sp, #64
	stp	x20, x19, [sp, #32]
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
Lloh30:
	adrp	x8, __NSConcreteStackBlock@GOTPAGE
Lloh31:
	ldr	x8, [x8, __NSConcreteStackBlock@GOTPAGEOFF]
	stp	x8, xzr, [sp]
Lloh32:
	adrp	x8, SYM(<_ as block2[CRATE_ID]::traits::IntoBlock<(_,), _>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::create_and_use_rc_block::{closure#0}>, 0)@PAGE
Lloh33:
	add	x8, x8, SYM(<_ as block2[CRATE_ID]::traits::IntoBlock<(_,), _>>::__get_invoke_stack_block::invoke::<i32, i32, test_block[CRATE_ID]::create_and_use_rc_block::{closure#0}>, 0)@PAGEOFF
Lloh34:
	adrp	x9, l_anon.[ID].4@PAGE
Lloh35:
	add	x9, x9, l_anon.[ID].4@PAGEOFF
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
	.loh AdrpAdd	Lloh34, Lloh35
	.loh AdrpAdd	Lloh32, Lloh33
	.loh AdrpLdrGot	Lloh30, Lloh31

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
l_anon.[ID].4:
	.asciz	"\000\000\000\000\000\000\000\000 \000\000\000\000\000\000"

.subsections_via_symbols
