	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_handle_new
	.p2align	2
	.code	32
_handle_new:
	b	_objc_msgSend

	.globl	_handle_new_fallible
	.p2align	2
	.code	32
_handle_new_fallible:
	push	{r4, r5, r7, lr}
	add	r7, sp, #8
	mov	r4, r1
	mov	r5, r0
	bl	_objc_msgSend
	cmp	r0, #0
	popne	{r4, r5, r7, pc}
LBB1_1:
	movw	r2, :lower16:(l_anon.[ID].1-(LPC1_0+8))
	movt	r2, :upper16:(l_anon.[ID].1-(LPC1_0+8))
LPC1_0:
	add	r2, pc, r2
	mov	r0, r5
	mov	r1, r4
	mov	lr, pc
	b	SYM(objc2::__macro_helpers::retain_semantics::new_fail::GENERATED_ID, 0)

	.globl	_handle_alloc
	.p2align	2
	.code	32
_handle_alloc:
	b	_objc_msgSend

	.globl	_handle_init
	.p2align	2
	.code	32
_handle_init:
	b	_objc_msgSend

	.globl	_handle_init_fallible
	.p2align	2
	.code	32
_handle_init_fallible:
	push	{r4, r5, r7, lr}
	add	r7, sp, #8
	mov	r4, r1
	mov	r5, r0
	bl	_objc_msgSend
	cmp	r0, #0
	popne	{r4, r5, r7, pc}
LBB4_1:
	movw	r2, :lower16:(l_anon.[ID].2-(LPC4_0+8))
	movt	r2, :upper16:(l_anon.[ID].2-(LPC4_0+8))
LPC4_0:
	add	r2, pc, r2
	mov	r0, r5
	mov	r1, r4
	mov	lr, pc
	b	SYM(objc2::__macro_helpers::retain_semantics::init_fail::GENERATED_ID, 0)

	.globl	_handle_alloc_init
	.p2align	2
	.code	32
_handle_alloc_init:
	push	{r4, r7, lr}
	add	r7, sp, #4
	mov	r4, r2
	bl	_objc_msgSend
	mov	r1, r4
	pop	{r4, r7, lr}
	b	_objc_msgSend

	.globl	_handle_alloc_release
	.p2align	2
	.code	32
_handle_alloc_release:
	push	{r7, lr}
	mov	r7, sp
	bl	_objc_msgSend
	pop	{r7, lr}
	b	_objc_release

	.globl	_handle_alloc_init_release
	.p2align	2
	.code	32
_handle_alloc_init_release:
	push	{r4, r7, lr}
	add	r7, sp, #4
	mov	r4, r2
	bl	_objc_msgSend
	mov	r1, r4
	bl	_objc_msgSend
	pop	{r4, r7, lr}
	b	_objc_release

	.globl	_handle_copy
	.p2align	2
	.code	32
_handle_copy:
	b	_objc_msgSend

	.globl	_handle_copy_fallible
	.p2align	2
	.code	32
_handle_copy_fallible:
	push	{r7, lr}
	mov	r7, sp
	bl	_objc_msgSend
	cmp	r0, #0
	popne	{r7, pc}
LBB9_1:
	movw	r0, :lower16:(l_anon.[ID].3-(LPC9_0+8))
	movt	r0, :upper16:(l_anon.[ID].3-(LPC9_0+8))
LPC9_0:
	add	r0, pc, r0
	mov	lr, pc
	b	SYM(objc2::__macro_helpers::retain_semantics::copy_fail::GENERATED_ID, 0)

	.globl	_handle_mutable_copy
	.p2align	2
	.code	32
_handle_mutable_copy:
	b	_objc_msgSend

	.globl	_handle_mutable_copy_fallible
	.p2align	2
	.code	32
_handle_mutable_copy_fallible:
	push	{r7, lr}
	mov	r7, sp
	bl	_objc_msgSend
	cmp	r0, #0
	popne	{r7, pc}
LBB11_1:
	movw	r0, :lower16:(l_anon.[ID].4-(LPC11_0+8))
	movt	r0, :upper16:(l_anon.[ID].4-(LPC11_0+8))
LPC11_0:
	add	r0, pc, r0
	mov	lr, pc
	b	SYM(objc2::__macro_helpers::retain_semantics::mutable_copy_fail::GENERATED_ID, 0)

	.globl	_handle_autoreleased
	.p2align	2
	.code	32
_handle_autoreleased:
	push	{r7, lr}
	mov	r7, sp
	bl	_objc_msgSend
	@ InlineAsm Start
	mov	r7, r7
	@ InlineAsm End
	pop	{r7, lr}
	b	_objc_retainAutoreleasedReturnValue

	.globl	_handle_autoreleased_with_arg
	.p2align	2
	.code	32
_handle_autoreleased_with_arg:
	push	{r7, lr}
	mov	r7, sp
	uxtb	r2, r2
	bl	_objc_msgSend
	@ InlineAsm Start
	mov	r7, r7
	@ InlineAsm End
	pop	{r7, lr}
	b	_objc_retainAutoreleasedReturnValue

	.globl	_handle_autoreleased_fallible
	.p2align	2
	.code	32
_handle_autoreleased_fallible:
	push	{r4, r5, r7, lr}
	add	r7, sp, #8
	mov	r4, r1
	mov	r5, r0
	bl	_objc_msgSend
	@ InlineAsm Start
	mov	r7, r7
	@ InlineAsm End
	bl	_objc_retainAutoreleasedReturnValue
	cmp	r0, #0
	popne	{r4, r5, r7, pc}
LBB14_1:
	movw	r2, :lower16:(l_anon.[ID].5-(LPC14_0+8))
	movt	r2, :upper16:(l_anon.[ID].5-(LPC14_0+8))
LPC14_0:
	add	r2, pc, r2
	mov	r0, r5
	mov	r1, r4
	mov	lr, pc
	b	SYM(objc2::__macro_helpers::retain_semantics::none_fail::GENERATED_ID, 0)

	.globl	_handle_with_out_param
	.p2align	2
	.code	32
_handle_with_out_param:
Lfunc_begin0:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	push	{r8, r10, r11}
	sub	r4, sp, #64
	bfc	r4, #0, #4
	mov	sp, r4
	vst1.64	{d8, d9, d10, d11}, [r4:128]!
	vst1.64	{d12, d13, d14, d15}, [r4:128]
	sub	sp, sp, #64
	mov	r4, r1
	mov	r5, r0
	str	r2, [sp, #4]
	ldr	r0, [sp, #4]
	ldr	r0, [r0]
	str	r0, [sp, #8]
	movw	r0, :lower16:(L_rust_eh_personality$non_lazy_ptr-(LPC15_2+8))
	movt	r0, :upper16:(L_rust_eh_personality$non_lazy_ptr-(LPC15_2+8))
LPC15_2:
	ldr	r0, [pc, r0]
	ldr	r6, [sp, #4]
	str	r0, [sp, #36]
	ldr	r0, LCPI15_0
LPC15_0:
	add	r0, pc, r0
	str	r0, [sp, #40]
	str	r7, [sp, #44]
	str	sp, [sp, #52]
	ldr	r0, LCPI15_1
LPC15_1:
	add	r0, pc, r0
	str	r0, [sp, #48]
	mov	r0, #1
	str	r0, [sp, #16]
	add	r0, sp, #12
	bl	__Unwind_SjLj_Register
Ltmp0:
	mov	r0, r5
	mov	r1, r4
	mov	r2, r6
	bl	_objc_msgSend
Ltmp1:
	@ InlineAsm Start
	mov	r7, r7
	@ InlineAsm End
	mov	r1, #4
	str	r1, [sp, #16]
Ltmp2:
	bl	_objc_retainAutoreleasedReturnValue
Ltmp3:
	mov	r4, r0
	ldr	r0, [sp, #4]
	ldr	r0, [r0]
	mvn	r5, #0
	str	r5, [sp, #16]
	bl	_objc_retain
	ldr	r0, [sp, #8]
	str	r5, [sp, #16]
	bl	_objc_release
	add	r0, sp, #12
	bl	__Unwind_SjLj_Unregister
	mov	r0, r4
	add	r4, sp, #64
	vld1.64	{d8, d9, d10, d11}, [r4:128]!
	vld1.64	{d12, d13, d14, d15}, [r4:128]
	sub	sp, r7, #24
	pop	{r8, r10, r11}
	pop	{r4, r5, r6, r7, pc}
LBB15_3:
	lsl	r0, r0, #2
	adr	r1, LJTI15_0
	ldr	r0, [r0, r1]
	add	pc, r0, r1
	.p2align	2
LJTI15_0:
	.data_region jt32
	.long	LBB15_5-LJTI15_0
	.long	LBB15_10-LJTI15_0
	.long	LBB15_10-LJTI15_0
	.long	LBB15_5-LJTI15_0
	.end_data_region
LBB15_5:
Ltmp4:
	ldr	r0, [sp, #20]
	str	r0, [sp]
	ldr	r0, [sp, #4]
	ldr	r0, [r0]
	mov	r1, #2
	str	r1, [sp, #16]
Ltmp5:
	bl	_objc_retain
Ltmp6:
	ldr	r0, [sp, #8]
	mov	r1, #3
	str	r1, [sp, #16]
Ltmp7:
	bl	_objc_release
Ltmp8:
	b	LBB15_9
LBB15_7:
	ldr	r0, [sp, #16]
	cmp	r0, #4
	bls	LBB15_3
	trap
LBB15_9:
	mvn	r0, #0
	str	r0, [sp, #16]
	ldr	r0, [sp]
	mov	lr, pc
	b	__Unwind_SjLj_Resume
LBB15_10:
Ltmp9:
	ldr	r0, [sp, #20]
	ldr	r0, [sp, #24]
	mov	lr, pc
	b	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
	.p2align	2
	.data_region
LCPI15_0:
	.long	Lexception0-(LPC15_0+8)
LCPI15_1:
	.long	LBB15_7-(LPC15_1+8)
	.end_data_region
Lfunc_end0:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table15:
Lexception0:
	.byte	255
	.byte	155
	.uleb128 Lttbase0-Lttbaseref0
Lttbaseref0:
	.byte	3
	.uleb128 Lcst_end0-Lcst_begin0
Lcst_begin0:
	.byte	0
	.byte	0
	.byte	1
	.byte	1
	.byte	2
	.byte	1
	.byte	3
	.byte	0
Lcst_end0:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase0:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__const
l_anon.[ID].0:
	.ascii	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	2, 0x0
l_anon.[ID].1:
	.long	l_anon.[ID].0
	.asciz	"9\000\000\000\017\000\000\000\005\000\000"

	.p2align	2, 0x0
l_anon.[ID].2:
	.long	l_anon.[ID].0
	.asciz	"9\000\000\000\036\000\000\000\005\000\000"

	.p2align	2, 0x0
l_anon.[ID].3:
	.long	l_anon.[ID].0
	.asciz	"9\000\000\000:\000\000\000\005\000\000"

	.p2align	2, 0x0
l_anon.[ID].4:
	.long	l_anon.[ID].0
	.asciz	"9\000\000\000D\000\000\000\005\000\000"

	.p2align	2, 0x0
l_anon.[ID].5:
	.long	l_anon.[ID].0
	.asciz	"9\000\000\000X\000\000\000\005\000\000"

	.section	__DATA,__nl_symbol_ptr,non_lazy_symbol_pointers
	.p2align	2, 0x0
L_rust_eh_personality$non_lazy_ptr:
	.indirect_symbol	_rust_eh_personality
	.long	0

.subsections_via_symbols
