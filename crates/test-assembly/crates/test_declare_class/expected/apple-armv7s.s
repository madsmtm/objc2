	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.p2align	2
	.code	32
SYM(core[CRATE_ID]::ptr::drop_in_place::<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}>, 0):
	bx	lr

	.p2align	2
	.code	32
SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0):
	push	{r4, r5, r7, lr}
	add	r7, sp, #8
	push	{r8, r10, r11}
	sub	sp, sp, #12
	ldr	r0, [r0]
	mov	r2, #0
	ldrb	r1, [r0]
	strb	r2, [r0]
	cmp	r1, #0
	beq	LBB1_5
	movw	r1, :lower16:(LL_OBJC_CLASSLIST_REFERENCES_$_NSObject$non_lazy_ptr-(LPC1_0+8))
	movt	r1, :upper16:(LL_OBJC_CLASSLIST_REFERENCES_$_NSObject$non_lazy_ptr-(LPC1_0+8))
	movw	r0, :lower16:(l_anon.[ID].11-(LPC1_1+8))
LPC1_0:
	ldr	r1, [pc, r1]
	movt	r0, :upper16:(l_anon.[ID].11-(LPC1_1+8))
LPC1_1:
	add	r0, pc, r0
	ldr	r2, [r1]
	mov	r1, #15
	bl	SYM(objc2::declare::ClassBuilder::new::GENERATED_ID, 0)
	cmp	r0, #0
	beq	LBB1_6
	movw	r1, :lower16:(L_anon.[ID].12-(LPC1_2+8))
	mov	r2, #0
	movt	r1, :upper16:(L_anon.[ID].12-(LPC1_2+8))
	movw	r3, :lower16:(l_anon.[ID].13-(LPC1_3+8))
	movt	r3, :upper16:(l_anon.[ID].13-(LPC1_3+8))
	add	r4, sp, #8
LPC1_3:
	add	r3, pc, r3
	str	r0, [sp, #8]
	strd	r2, r3, [sp]
LPC1_2:
	add	r1, pc, r1
	mov	r0, r4
	mov	r2, #4
	mov	r3, #1
	bl	SYM(objc2::declare::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
	movw	r8, :lower16:(l_anon.[ID].2-(LPC1_4+8))
	mov	r0, #2
	movt	r8, :upper16:(l_anon.[ID].2-(LPC1_4+8))
	movw	r1, :lower16:(L_anon.[ID].14-(LPC1_5+8))
	movt	r1, :upper16:(L_anon.[ID].14-(LPC1_5+8))
LPC1_4:
	add	r8, pc, r8
	stm	sp, {r0, r8}
LPC1_5:
	add	r1, pc, r1
	mov	r0, r4
	mov	r2, #4
	mov	r3, #4
	bl	SYM(objc2::declare::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
	movw	r0, :lower16:(LL_OBJC_SELECTOR_REFERENCES_dealloc$non_lazy_ptr-(LPC1_6+8))
	mov	r3, #0
	movt	r0, :upper16:(LL_OBJC_SELECTOR_REFERENCES_dealloc$non_lazy_ptr-(LPC1_6+8))
	movw	r11, :lower16:(SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::__objc2_dealloc, 0)-(LPC1_7+8))
LPC1_6:
	ldr	r0, [pc, r0]
	movt	r11, :upper16:(SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::__objc2_dealloc, 0)-(LPC1_7+8))
LPC1_7:
	add	r11, pc, r11
	ldr	r1, [r0]
	movw	r10, :lower16:(l_anon.[ID].3-(LPC1_8+8))
	movt	r10, :upper16:(l_anon.[ID].3-(LPC1_8+8))
	movw	r5, :lower16:(l_anon.[ID].1-(LPC1_9+8))
	movt	r5, :upper16:(l_anon.[ID].1-(LPC1_9+8))
LPC1_8:
	add	r10, pc, r10
LPC1_9:
	add	r5, pc, r5
	mov	r0, r4
	mov	r2, r5
	strd	r10, r11, [sp]
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	movw	r0, :lower16:(LL_OBJC_SELECTOR_REFERENCES_init$non_lazy_ptr-(LPC1_10+8))
	mov	r2, r5
	movt	r0, :upper16:(LL_OBJC_SELECTOR_REFERENCES_init$non_lazy_ptr-(LPC1_10+8))
	mov	r3, #0
LPC1_10:
	ldr	r0, [pc, r0]
	ldr	r1, [r0]
	movw	r9, :lower16:(_init-(LPC1_11+8))
	movt	r9, :upper16:(_init-(LPC1_11+8))
	mov	r0, r4
LPC1_11:
	add	r9, pc, r9
	strd	r8, r9, [sp]
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	movw	r11, :lower16:(_class_method-(LPC1_12+8))
	mov	r0, r4
	movt	r11, :upper16:(_class_method-(LPC1_12+8))
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_d874ee9262978be2-(LPC1_13+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_d874ee9262978be2-(LPC1_13+8))
LPC1_12:
	add	r11, pc, r11
LPC1_13:
	ldr	r1, [pc, r1]
	mov	r2, r5
	mov	r3, #0
	strd	r10, r11, [sp]
	bl	SYM(objc2::declare::ClassBuilder::add_class_method_inner::GENERATED_ID, 0)
	movw	r11, :lower16:(_method-(LPC1_14+8))
	mov	r0, r4
	movt	r11, :upper16:(_method-(LPC1_14+8))
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_4539fd1dbda0cddc-(LPC1_15+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_4539fd1dbda0cddc-(LPC1_15+8))
LPC1_14:
	add	r11, pc, r11
LPC1_15:
	ldr	r1, [pc, r1]
	mov	r2, r5
	mov	r3, #0
	strd	r10, r11, [sp]
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	movw	r3, :lower16:(_method_bool-(LPC1_16+8))
	mov	r0, r4
	movt	r3, :upper16:(_method_bool-(LPC1_16+8))
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_2b1b3a94e0ece2e5-(LPC1_17+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_2b1b3a94e0ece2e5-(LPC1_17+8))
LPC1_16:
	add	r3, pc, r3
LPC1_17:
	ldr	r1, [pc, r1]
	movw	r10, :lower16:(l_anon.[ID].4-(LPC1_18+8))
	movt	r10, :upper16:(l_anon.[ID].4-(LPC1_18+8))
LPC1_18:
	add	r10, pc, r10
	str	r10, [sp]
	mov	r2, r10
	str	r3, [sp, #4]
	mov	r3, #1
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	movw	r9, :lower16:(_method_id-(LPC1_19+8))
	mov	r0, r4
	movt	r9, :upper16:(_method_id-(LPC1_19+8))
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_f7f521670860b0ce-(LPC1_20+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_f7f521670860b0ce-(LPC1_20+8))
LPC1_19:
	add	r9, pc, r9
LPC1_20:
	ldr	r1, [pc, r1]
	mov	r2, r5
	mov	r3, #0
	strd	r8, r9, [sp]
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	movw	r9, :lower16:(_method_id_with_param-(LPC1_21+8))
	mov	r0, r4
	movt	r9, :upper16:(_method_id_with_param-(LPC1_21+8))
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_6addfcf634c6232f-(LPC1_22+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_6addfcf634c6232f-(LPC1_22+8))
LPC1_21:
	add	r9, pc, r9
LPC1_22:
	ldr	r1, [pc, r1]
	mov	r2, r10
	mov	r3, #1
	strd	r8, r9, [sp]
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	movw	r0, :lower16:(l_anon.[ID].17-(LPC1_23+8))
	mov	r1, #9
	movt	r0, :upper16:(l_anon.[ID].17-(LPC1_23+8))
LPC1_23:
	add	r0, pc, r0
	bl	SYM(objc2::runtime::AnyProtocol::get::GENERATED_ID, 0)
	cmp	r0, #0
	beq	LBB1_4
	mov	r1, r0
	add	r0, sp, #8
	bl	SYM(objc2::declare::ClassBuilder::add_protocol::GENERATED_ID, 0)
LBB1_4:
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_4a8c690dbc9d8166-(LPC1_24+8))
	add	r0, sp, #8
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_4a8c690dbc9d8166-(LPC1_24+8))
	mov	r3, #1
LPC1_24:
	ldr	r1, [pc, r1]
	movw	r2, :lower16:(l_anon.[ID].7-(LPC1_25+8))
	movt	r2, :upper16:(l_anon.[ID].7-(LPC1_25+8))
	movw	r9, :lower16:(_copyWithZone-(LPC1_26+8))
	movt	r9, :upper16:(_copyWithZone-(LPC1_26+8))
LPC1_25:
	add	r2, pc, r2
LPC1_26:
	add	r9, pc, r9
	strd	r8, r9, [sp]
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	ldr	r0, [sp, #8]
	bl	SYM(objc2::declare::ClassBuilder::register::GENERATED_ID, 0)
	sub	sp, r7, #20
	pop	{r8, r10, r11}
	pop	{r4, r5, r7, pc}
LBB1_5:
	movw	r0, :lower16:(l_anon.[ID].8-(LPC1_27+8))
	mov	r1, #43
	movt	r0, :upper16:(l_anon.[ID].8-(LPC1_27+8))
	movw	r2, :lower16:(l_anon.[ID].10-(LPC1_28+8))
	movt	r2, :upper16:(l_anon.[ID].10-(LPC1_28+8))
LPC1_27:
	add	r0, pc, r0
LPC1_28:
	add	r2, pc, r2
	mov	lr, pc
	b	SYM(core::panicking::panic::GENERATED_ID, 0)
LBB1_6:
	movw	r0, :lower16:(l_anon.[ID].11-(LPC1_29+8))
	mov	r1, #15
	movt	r0, :upper16:(l_anon.[ID].11-(LPC1_29+8))
	movw	r2, :lower16:(l_anon.[ID].16-(LPC1_30+8))
	movt	r2, :upper16:(l_anon.[ID].16-(LPC1_30+8))
LPC1_29:
	add	r0, pc, r0
LPC1_30:
	add	r2, pc, r2
	mov	lr, pc
	b	SYM(objc2::__macro_helpers::declare_class::failed_declaring_class::GENERATED_ID, 0)

	.p2align	2
	.code	32
SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0):
	push	{r7, lr}
	mov	r7, sp
	sub	sp, sp, #4
	ldr	r0, [r0]
	str	r0, [sp]
	mov	r0, sp
	bl	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)
	mov	sp, r7
	pop	{r7, pc}

	.globl	_get_class
	.p2align	2
	.code	32
_get_class:
	push	{r7, lr}
	mov	r7, sp
	sub	sp, sp, #12
	movw	r0, :lower16:(SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)-(LPC3_0+8))
	movt	r0, :upper16:(SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)-(LPC3_0+8))
LPC3_0:
	add	r0, pc, r0
	ldr	r0, [r0]
	dmb	ish
	cmp	r0, #3
	bne	LBB3_3
LBB3_1:
	movw	r0, :lower16:(l_anon.[ID].11-(LPC3_4+8))
	mov	r1, #15
	movt	r0, :upper16:(l_anon.[ID].11-(LPC3_4+8))
LPC3_4:
	add	r0, pc, r0
	bl	SYM(objc2::runtime::AnyClass::get::GENERATED_ID, 0)
	cmp	r0, #0
	movne	sp, r7
	popne	{r7, pc}
LBB3_2:
	movw	r0, :lower16:(l_anon.[ID].8-(LPC3_5+8))
	mov	r1, #43
	movt	r0, :upper16:(l_anon.[ID].8-(LPC3_5+8))
	movw	r2, :lower16:(l_anon.[ID].16-(LPC3_6+8))
	movt	r2, :upper16:(l_anon.[ID].16-(LPC3_6+8))
LPC3_5:
	add	r0, pc, r0
LPC3_6:
	add	r2, pc, r2
	mov	lr, pc
	b	SYM(core::panicking::panic::GENERATED_ID, 0)
LBB3_3:
	movw	r0, :lower16:(SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)-(LPC3_1+8))
	mov	r2, #1
	movt	r0, :upper16:(SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)-(LPC3_1+8))
	movw	r3, :lower16:(l_anon.[ID].0-(LPC3_2+8))
	movt	r3, :upper16:(l_anon.[ID].0-(LPC3_2+8))
	movw	r1, :lower16:(l_anon.[ID].16-(LPC3_3+8))
	movt	r1, :upper16:(l_anon.[ID].16-(LPC3_3+8))
	strb	r2, [r7, #-5]
LPC3_3:
	add	r1, pc, r1
	sub	r2, r7, #5
	str	r2, [r7, #-4]
LPC3_1:
	add	r0, pc, r0
	str	r1, [sp]
LPC3_2:
	add	r3, pc, r3
	sub	r2, r7, #4
	mov	r1, #0
	bl	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	b	LBB3_1

	.globl	_get_obj
	.p2align	2
	.code	32
_get_obj:
	push	{r4, r7, lr}
	add	r7, sp, #4
	movw	r0, :lower16:(LL_OBJC_SELECTOR_REFERENCES_new$non_lazy_ptr-(LPC4_0+8))
	movt	r0, :upper16:(LL_OBJC_SELECTOR_REFERENCES_new$non_lazy_ptr-(LPC4_0+8))
LPC4_0:
	ldr	r0, [pc, r0]
	ldr	r4, [r0]
	bl	_get_class
	mov	r1, r4
	bl	_objc_msgSend
	pop	{r4, r7, pc}

	.globl	_access_ivars
	.p2align	2
	.code	32
_access_ivars:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	bl	_get_obj
	mov	r4, r0
	bl	_object_getClass
	movw	r1, :lower16:(L_anon.[ID].12-(LPC5_0+8))
	mov	r2, #4
	movt	r1, :upper16:(L_anon.[ID].12-(LPC5_0+8))
	movw	r3, :lower16:(l_anon.[ID].13-(LPC5_1+8))
	movt	r3, :upper16:(l_anon.[ID].13-(LPC5_1+8))
LPC5_0:
	add	r1, pc, r1
LPC5_1:
	add	r3, pc, r3
	bl	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	ldrb	r5, [r4, r0]
	mov	r0, r4
	bl	_object_getClass
	movw	r1, :lower16:(L_anon.[ID].14-(LPC5_2+8))
	mov	r2, #4
	movt	r1, :upper16:(L_anon.[ID].14-(LPC5_2+8))
	movw	r3, :lower16:(l_anon.[ID].2-(LPC5_3+8))
	movt	r3, :upper16:(l_anon.[ID].2-(LPC5_3+8))
LPC5_2:
	add	r1, pc, r1
LPC5_3:
	add	r3, pc, r3
	bl	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	ldr	r6, [r4, r0]
	mov	r0, r4
	bl	_objc_release
	mov	r0, r5
	mov	r1, r6
	pop	{r4, r5, r6, r7, pc}

	.globl	SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	2
	.code	32
SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
	push	{r7, lr}
	mov	r7, sp
	sub	sp, sp, #12
	movw	r0, :lower16:(SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)-(LPC6_0+8))
	movt	r0, :upper16:(SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)-(LPC6_0+8))
LPC6_0:
	add	r0, pc, r0
	ldr	r0, [r0]
	dmb	ish
	cmp	r0, #3
	bne	LBB6_3
LBB6_1:
	movw	r0, :lower16:(l_anon.[ID].11-(LPC6_4+8))
	mov	r1, #15
	movt	r0, :upper16:(l_anon.[ID].11-(LPC6_4+8))
LPC6_4:
	add	r0, pc, r0
	bl	SYM(objc2::runtime::AnyClass::get::GENERATED_ID, 0)
	cmp	r0, #0
	movne	sp, r7
	popne	{r7, pc}
LBB6_2:
	movw	r0, :lower16:(l_anon.[ID].8-(LPC6_5+8))
	mov	r1, #43
	movt	r0, :upper16:(l_anon.[ID].8-(LPC6_5+8))
	movw	r2, :lower16:(l_anon.[ID].16-(LPC6_6+8))
	movt	r2, :upper16:(l_anon.[ID].16-(LPC6_6+8))
LPC6_5:
	add	r0, pc, r0
LPC6_6:
	add	r2, pc, r2
	mov	lr, pc
	b	SYM(core::panicking::panic::GENERATED_ID, 0)
LBB6_3:
	movw	r0, :lower16:(SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)-(LPC6_1+8))
	mov	r2, #1
	movt	r0, :upper16:(SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)-(LPC6_1+8))
	movw	r3, :lower16:(l_anon.[ID].0-(LPC6_2+8))
	movt	r3, :upper16:(l_anon.[ID].0-(LPC6_2+8))
	movw	r1, :lower16:(l_anon.[ID].16-(LPC6_3+8))
	movt	r1, :upper16:(l_anon.[ID].16-(LPC6_3+8))
	strb	r2, [r7, #-5]
LPC6_3:
	add	r1, pc, r1
	sub	r2, r7, #5
	str	r2, [r7, #-4]
LPC6_1:
	add	r0, pc, r0
	str	r1, [sp]
LPC6_2:
	add	r3, pc, r3
	sub	r2, r7, #4
	mov	r1, #0
	bl	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	b	LBB6_1

	.p2align	2
	.code	32
SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::__objc2_dealloc, 0):
	push	{r4, r5, r7, lr}
	add	r7, sp, #8
	sub	sp, sp, #8
	mov	r4, r1
	mov	r5, r0
	bl	_object_getClass
	movw	r1, :lower16:(L_anon.[ID].14-(LPC7_0+8))
	mov	r2, #4
	movt	r1, :upper16:(L_anon.[ID].14-(LPC7_0+8))
	movw	r3, :lower16:(l_anon.[ID].2-(LPC7_1+8))
	movt	r3, :upper16:(l_anon.[ID].2-(LPC7_1+8))
LPC7_0:
	add	r1, pc, r1
LPC7_1:
	add	r3, pc, r3
	bl	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	ldr	r0, [r5, r0]
	cmp	r0, #0
	beq	LBB7_2
	bl	_objc_release
LBB7_2:
	movw	r0, :lower16:(LL_OBJC_CLASSLIST_REFERENCES_$_NSObject$non_lazy_ptr-(LPC7_2+8))
	mov	r1, r4
	movt	r0, :upper16:(LL_OBJC_CLASSLIST_REFERENCES_$_NSObject$non_lazy_ptr-(LPC7_2+8))
	str	r5, [sp]
LPC7_2:
	ldr	r0, [pc, r0]
	ldr	r0, [r0]
	str	r0, [sp, #4]
	mov	r0, sp
	bl	_objc_msgSendSuper
	sub	sp, r7, #8
	pop	{r4, r5, r7, pc}

	.globl	_init
	.p2align	2
	.code	32
_init:
	push	{r4, r7, lr}
	add	r7, sp, #4
	sub	sp, sp, #8
	movw	r1, :lower16:(LL_OBJC_SELECTOR_REFERENCES_init$non_lazy_ptr-(LPC8_0+8))
	movt	r1, :upper16:(LL_OBJC_SELECTOR_REFERENCES_init$non_lazy_ptr-(LPC8_0+8))
	movw	r2, :lower16:(LL_OBJC_CLASSLIST_REFERENCES_$_NSObject$non_lazy_ptr-(LPC8_1+8))
LPC8_0:
	ldr	r1, [pc, r1]
	movt	r2, :upper16:(LL_OBJC_CLASSLIST_REFERENCES_$_NSObject$non_lazy_ptr-(LPC8_1+8))
LPC8_1:
	ldr	r2, [pc, r2]
	ldr	r1, [r1]
	ldr	r2, [r2]
	stm	sp, {r0, r2}
	mov	r0, sp
	bl	_objc_msgSendSuper
	mov	r4, r0
	cmp	r0, #0
	beq	LBB8_2
	mov	r0, r4
	bl	_object_getClass
	movw	r1, :lower16:(L_anon.[ID].12-(LPC8_2+8))
	mov	r2, #4
	movt	r1, :upper16:(L_anon.[ID].12-(LPC8_2+8))
	movw	r3, :lower16:(l_anon.[ID].13-(LPC8_3+8))
	movt	r3, :upper16:(l_anon.[ID].13-(LPC8_3+8))
LPC8_2:
	add	r1, pc, r1
LPC8_3:
	add	r3, pc, r3
	bl	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	mov	r1, #42
	strb	r1, [r4, r0]
	mov	r0, r4
	bl	_object_getClass
	movw	r1, :lower16:(L_anon.[ID].14-(LPC8_4+8))
	mov	r2, #4
	movt	r1, :upper16:(L_anon.[ID].14-(LPC8_4+8))
	movw	r3, :lower16:(l_anon.[ID].2-(LPC8_5+8))
	movt	r3, :upper16:(l_anon.[ID].2-(LPC8_5+8))
LPC8_4:
	add	r1, pc, r1
LPC8_5:
	add	r3, pc, r3
	bl	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	mov	r1, #0
	str	r1, [r4, r0]
LBB8_2:
	mov	r0, r4
	sub	sp, r7, #4
	pop	{r4, r7, pc}

	.globl	_class_method
	.p2align	2
	.code	32
_class_method:
	bx	lr

	.globl	_method
	.p2align	2
	.code	32
_method:
	bx	lr

	.globl	_method_bool
	.p2align	2
	.code	32
_method_bool:
	clz	r0, r2
	lsr	r0, r0, #5
	bx	lr

	.globl	_method_id
	.p2align	2
	.code	32
_method_id:
	push	{r4, r7, lr}
	add	r7, sp, #4
	mov	r4, r0
	bl	_object_getClass
	movw	r1, :lower16:(L_anon.[ID].14-(LPC12_0+8))
	mov	r2, #4
	movt	r1, :upper16:(L_anon.[ID].14-(LPC12_0+8))
	movw	r3, :lower16:(l_anon.[ID].2-(LPC12_1+8))
	movt	r3, :upper16:(l_anon.[ID].2-(LPC12_1+8))
LPC12_0:
	add	r1, pc, r1
LPC12_1:
	add	r3, pc, r3
	bl	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	ldr	r0, [r4, r0]
	cmp	r0, #0
	beq	LBB12_2
	bl	_objc_retain
	bl	_objc_autoreleaseReturnValue
	pop	{r4, r7, pc}
LBB12_2:
	mov	r0, #0
	bl	_objc_autoreleaseReturnValue
	pop	{r4, r7, pc}

	.globl	_method_id_with_param
	.p2align	2
	.code	32
_method_id_with_param:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	mov	r6, r2
	mov	r5, r0
	bl	SYM(objc2::runtime::nsobject::NSObject::new::GENERATED_ID, 0)
	mov	r4, r0
	cmp	r6, #0
	beq	LBB13_3
	mov	r0, r5
	bl	_object_getClass
	movw	r1, :lower16:(L_anon.[ID].14-(LPC13_0+8))
	mov	r2, #4
	movt	r1, :upper16:(L_anon.[ID].14-(LPC13_0+8))
	movw	r3, :lower16:(l_anon.[ID].2-(LPC13_1+8))
	movt	r3, :upper16:(l_anon.[ID].2-(LPC13_1+8))
LPC13_0:
	add	r1, pc, r1
LPC13_1:
	add	r3, pc, r3
	bl	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	ldr	r0, [r5, r0]
	cmp	r0, #0
	beq	LBB13_4
	bl	_objc_retain
	mov	r5, r0
	b	LBB13_5
LBB13_3:
	mov	r5, r4
	mov	r0, r5
	bl	_objc_autoreleaseReturnValue
	pop	{r4, r5, r6, r7, pc}
LBB13_4:
	mov	r5, #0
LBB13_5:
	mov	r0, r4
	bl	_objc_release
	mov	r0, r5
	bl	_objc_autoreleaseReturnValue
	pop	{r4, r5, r6, r7, pc}

	.globl	_copyWithZone
	.p2align	2
	.code	32
_copyWithZone:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	push	{r8, r10}
	mov	r5, r0
	bl	_get_obj
	mov	r4, r0
	cmp	r0, #0
	beq	LBB14_5
	mov	r0, r5
	bl	_object_getClass
	movw	r10, :lower16:(L_anon.[ID].12-(LPC14_0+8))
	mov	r2, #4
	movt	r10, :upper16:(L_anon.[ID].12-(LPC14_0+8))
	movw	r8, :lower16:(l_anon.[ID].13-(LPC14_1+8))
	movt	r8, :upper16:(l_anon.[ID].13-(LPC14_1+8))
LPC14_0:
	add	r10, pc, r10
LPC14_1:
	add	r8, pc, r8
	mov	r1, r10
	mov	r3, r8
	bl	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	ldrb	r6, [r5, r0]
	mov	r0, r4
	bl	_object_getClass
	mov	r1, r10
	mov	r2, #4
	mov	r3, r8
	bl	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	strb	r6, [r4, r0]
	mov	r0, r5
	bl	_object_getClass
	movw	r1, :lower16:(L_anon.[ID].14-(LPC14_2+8))
	mov	r2, #4
	movt	r1, :upper16:(L_anon.[ID].14-(LPC14_2+8))
	movw	r3, :lower16:(l_anon.[ID].2-(LPC14_3+8))
	movt	r3, :upper16:(l_anon.[ID].2-(LPC14_3+8))
LPC14_2:
	add	r1, pc, r1
LPC14_3:
	add	r3, pc, r3
	bl	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	ldr	r0, [r5, r0]
	cmp	r0, #0
	beq	LBB14_3
	bl	_objc_retain
	mov	r5, r0
	b	LBB14_4
LBB14_3:
	mov	r5, #0
LBB14_4:
	mov	r0, r4
	bl	_object_getClass
	movw	r1, :lower16:(L_anon.[ID].14-(LPC14_4+8))
	mov	r2, #4
	movt	r1, :upper16:(L_anon.[ID].14-(LPC14_4+8))
	movw	r3, :lower16:(l_anon.[ID].2-(LPC14_5+8))
	movt	r3, :upper16:(l_anon.[ID].2-(LPC14_5+8))
LPC14_4:
	add	r1, pc, r1
LPC14_5:
	add	r3, pc, r3
	bl	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	str	r5, [r4, r0]
LBB14_5:
	mov	r0, r4
	pop	{r8, r10}
	pop	{r4, r5, r6, r7, pc}

	.section	__DATA,__const
	.p2align	2, 0x0
l_anon.[ID].0:
	.long	SYM(core[CRATE_ID]::ptr::drop_in_place::<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}>, 0)
	.asciz	"\004\000\000\000\004\000\000"
	.long	SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0)
	.long	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)

	.section	__TEXT,__const
	.p2align	2, 0x0
l_anon.[ID].1:
	.byte	0

	.p2align	2, 0x0
l_anon.[ID].2:
	.byte	19
	.space	19

	.p2align	2, 0x0
l_anon.[ID].3:
	.byte	17
	.space	19

	.p2align	2, 0x0
l_anon.[ID].4:
	.space	1
	.space	19

l_anon.[ID].5:
	.ascii	"_NSZone"

	.section	__DATA,__const
	.p2align	2, 0x0
l_anon.[ID].6:
	.byte	28
	.space	3
	.long	l_anon.[ID].5
	.asciz	"\007\000\000"
	.long	l_anon.[ID].1
	.space	4

	.p2align	2, 0x0
l_anon.[ID].7:
	.byte	25
	.space	3
	.long	l_anon.[ID].6
	.space	12

	.section	__TEXT,__const
l_anon.[ID].8:
	.ascii	"called `Option::unwrap()` on a `None` value"

l_anon.[ID].9:
	.ascii	"$RUSTC/library/std/src/sync/once.rs"

	.section	__DATA,__const
	.p2align	2, 0x0
l_anon.[ID].10:
	.long	l_anon.[ID].9
	.asciz	"p\000\000\000\225\000\000\0002\000\000"

	.section	__TEXT,__const
l_anon.[ID].11:
	.ascii	"CustomClassName"

	.section	__TEXT,__literal4,4byte_literals
L_anon.[ID].12:
	.ascii	"_foo"

	.section	__TEXT,__const
	.p2align	2, 0x0
l_anon.[ID].13:
	.byte	5
	.space	19

	.section	__TEXT,__literal4,4byte_literals
L_anon.[ID].14:
	.ascii	"_obj"

	.section	__TEXT,__const
l_anon.[ID].15:
	.ascii	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	2, 0x0
l_anon.[ID].16:
	.long	l_anon.[ID].15
	.asciz	"5\000\000\000\f\000\000\000\001\000\000"

.zerofill __DATA,__bss,SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0),4,2
	.section	__TEXT,__const
l_anon.[ID].17:
	.ascii	"NSCopying"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_d874ee9262978be2
L_OBJC_METH_VAR_NAME_d874ee9262978be2:
	.asciz	"classMethod"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_d874ee9262978be2
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_d874ee9262978be2:
	.long	L_OBJC_METH_VAR_NAME_d874ee9262978be2

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_d874ee9262978be2
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_d874ee9262978be2:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_4539fd1dbda0cddc
L_OBJC_METH_VAR_NAME_4539fd1dbda0cddc:
	.asciz	"method"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_4539fd1dbda0cddc
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_4539fd1dbda0cddc:
	.long	L_OBJC_METH_VAR_NAME_4539fd1dbda0cddc

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_4539fd1dbda0cddc
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_4539fd1dbda0cddc:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_2b1b3a94e0ece2e5
L_OBJC_METH_VAR_NAME_2b1b3a94e0ece2e5:
	.asciz	"methodBool:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_2b1b3a94e0ece2e5
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_2b1b3a94e0ece2e5:
	.long	L_OBJC_METH_VAR_NAME_2b1b3a94e0ece2e5

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_2b1b3a94e0ece2e5
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_2b1b3a94e0ece2e5:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_f7f521670860b0ce
L_OBJC_METH_VAR_NAME_f7f521670860b0ce:
	.asciz	"methodId"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_f7f521670860b0ce
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_f7f521670860b0ce:
	.long	L_OBJC_METH_VAR_NAME_f7f521670860b0ce

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_f7f521670860b0ce
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_f7f521670860b0ce:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_6addfcf634c6232f
L_OBJC_METH_VAR_NAME_6addfcf634c6232f:
	.asciz	"methodIdWithParam:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_6addfcf634c6232f
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_6addfcf634c6232f:
	.long	L_OBJC_METH_VAR_NAME_6addfcf634c6232f

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_6addfcf634c6232f
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_6addfcf634c6232f:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_4a8c690dbc9d8166
L_OBJC_METH_VAR_NAME_4a8c690dbc9d8166:
	.asciz	"copyWithZone:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_4a8c690dbc9d8166
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_4a8c690dbc9d8166:
	.long	L_OBJC_METH_VAR_NAME_4a8c690dbc9d8166

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_4a8c690dbc9d8166
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_4a8c690dbc9d8166:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__nl_symbol_ptr,non_lazy_symbol_pointers
	.p2align	2, 0x0
LL_OBJC_CLASSLIST_REFERENCES_$_NSObject$non_lazy_ptr:
	.indirect_symbol	L_OBJC_CLASSLIST_REFERENCES_$_NSObject
	.long	0
LL_OBJC_SELECTOR_REFERENCES_dealloc$non_lazy_ptr:
	.indirect_symbol	L_OBJC_SELECTOR_REFERENCES_dealloc
	.long	0
LL_OBJC_SELECTOR_REFERENCES_init$non_lazy_ptr:
	.indirect_symbol	L_OBJC_SELECTOR_REFERENCES_init
	.long	0
LL_OBJC_SELECTOR_REFERENCES_new$non_lazy_ptr:
	.indirect_symbol	L_OBJC_SELECTOR_REFERENCES_new
	.long	0

.subsections_via_symbols
