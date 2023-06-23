	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.p2align	2
	.code	32
SYM(core[CRATE_ID]::ptr::drop_in_place::<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::{closure#0}>::{closure#0}>, 0):
	bx	lr

	.p2align	2
	.code	32
SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::{closure#0}>::{closure#0}, 0):
	push	{r4, r5, r7, lr}
	add	r7, sp, #8
	push	{r8, r10, r11}
	sub	sp, sp, #40
	ldr	r0, [r0]
	mov	r2, #0
	ldrb	r1, [r0]
	strb	r2, [r0]
	cmp	r1, #0
	beq	LBB1_3
	bl	SYM(<objc2::runtime::nsobject::NSObject as objc2::class_type::ClassType>::class::objc_static_workaround::GENERATED_ID, 0)
	mov	r2, r0
	movw	r0, :lower16:(l_anon.[ID].11-(LPC1_0+8))
	movt	r0, :upper16:(l_anon.[ID].11-(LPC1_0+8))
	mov	r1, #15
LPC1_0:
	add	r0, pc, r0
	bl	SYM(objc2::declare::ClassBuilder::new::GENERATED_ID, 0)
	cmp	r0, #0
	beq	LBB1_4
	movw	r1, :lower16:(L_anon.[ID].5-(LPC1_1+8))
	mov	r2, #0
	movt	r1, :upper16:(L_anon.[ID].5-(LPC1_1+8))
	movw	r3, :lower16:(l_anon.[ID].6-(LPC1_2+8))
	movt	r3, :upper16:(l_anon.[ID].6-(LPC1_2+8))
	add	r4, sp, #8
LPC1_2:
	add	r3, pc, r3
	str	r0, [sp, #8]
	strd	r2, r3, [sp]
LPC1_1:
	add	r1, pc, r1
	mov	r0, r4
	mov	r2, #4
	mov	r3, #1
	bl	SYM(objc2::declare::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
	movw	r8, :lower16:(l_anon.[ID].8-(LPC1_3+8))
	mov	r0, #2
	movt	r8, :upper16:(l_anon.[ID].8-(LPC1_3+8))
	movw	r1, :lower16:(L_anon.[ID].7-(LPC1_4+8))
	movt	r1, :upper16:(L_anon.[ID].7-(LPC1_4+8))
LPC1_3:
	add	r8, pc, r8
	stm	sp, {r0, r8}
LPC1_4:
	add	r1, pc, r1
	mov	r0, r4
	mov	r2, #4
	mov	r3, #4
	bl	SYM(objc2::declare::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
	movw	r11, :lower16:(SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::{closure#0}::__objc2_dealloc, 0)-(LPC1_5+8))
	mov	r0, r4
	movt	r11, :upper16:(SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::{closure#0}::__objc2_dealloc, 0)-(LPC1_5+8))
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_694e247a0bc88869-(LPC1_6+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_694e247a0bc88869-(LPC1_6+8))
LPC1_5:
	add	r11, pc, r11
LPC1_6:
	ldr	r1, [pc, r1]
	movw	r10, :lower16:(l_anon.[ID].12-(LPC1_7+8))
	movt	r10, :upper16:(l_anon.[ID].12-(LPC1_7+8))
	movw	r5, :lower16:(l_anon.[ID].1-(LPC1_8+8))
	movt	r5, :upper16:(l_anon.[ID].1-(LPC1_8+8))
LPC1_7:
	add	r10, pc, r10
LPC1_8:
	add	r5, pc, r5
	mov	r3, #0
	mov	r2, r5
	strd	r10, r11, [sp]
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	movw	r0, :lower16:(LL_OBJC_SELECTOR_REFERENCES_init$non_lazy_ptr-(LPC1_9+8))
	mov	r2, r5
	movt	r0, :upper16:(LL_OBJC_SELECTOR_REFERENCES_init$non_lazy_ptr-(LPC1_9+8))
	mov	r3, #0
LPC1_9:
	ldr	r0, [pc, r0]
	ldr	r1, [r0]
	movw	r9, :lower16:(_init-(LPC1_10+8))
	movt	r9, :upper16:(_init-(LPC1_10+8))
	mov	r0, r4
LPC1_10:
	add	r9, pc, r9
	strd	r8, r9, [sp]
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	movw	r11, :lower16:(_class_method-(LPC1_11+8))
	mov	r0, r4
	movt	r11, :upper16:(_class_method-(LPC1_11+8))
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_8dd788dbcc16b9bc-(LPC1_12+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_8dd788dbcc16b9bc-(LPC1_12+8))
LPC1_11:
	add	r11, pc, r11
LPC1_12:
	ldr	r1, [pc, r1]
	mov	r2, r5
	mov	r3, #0
	strd	r10, r11, [sp]
	bl	SYM(objc2::declare::ClassBuilder::add_class_method_inner::GENERATED_ID, 0)
	movw	r11, :lower16:(_method-(LPC1_13+8))
	mov	r0, r4
	movt	r11, :upper16:(_method-(LPC1_13+8))
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_450db9db0953dff5-(LPC1_14+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_450db9db0953dff5-(LPC1_14+8))
LPC1_13:
	add	r11, pc, r11
LPC1_14:
	ldr	r1, [pc, r1]
	mov	r2, r5
	mov	r3, #0
	strd	r10, r11, [sp]
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	movw	r3, :lower16:(_method_bool-(LPC1_15+8))
	mov	r0, r4
	movt	r3, :upper16:(_method_bool-(LPC1_15+8))
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_783b35bc45c6e4a6-(LPC1_16+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_783b35bc45c6e4a6-(LPC1_16+8))
LPC1_15:
	add	r3, pc, r3
LPC1_16:
	ldr	r1, [pc, r1]
	movw	r10, :lower16:(l_anon.[ID].13-(LPC1_17+8))
	movt	r10, :upper16:(l_anon.[ID].13-(LPC1_17+8))
LPC1_17:
	add	r10, pc, r10
	str	r10, [sp]
	mov	r2, r10
	str	r3, [sp, #4]
	mov	r3, #1
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	movw	r9, :lower16:(_method_id-(LPC1_18+8))
	mov	r0, r4
	movt	r9, :upper16:(_method_id-(LPC1_18+8))
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_828e9fbc6d0b4498-(LPC1_19+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_828e9fbc6d0b4498-(LPC1_19+8))
LPC1_18:
	add	r9, pc, r9
LPC1_19:
	ldr	r1, [pc, r1]
	mov	r2, r5
	mov	r3, #0
	strd	r8, r9, [sp]
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	movw	r9, :lower16:(_method_id_with_param-(LPC1_20+8))
	mov	r0, r4
	movt	r9, :upper16:(_method_id_with_param-(LPC1_20+8))
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_788cc14ba6a28eb8-(LPC1_21+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_788cc14ba6a28eb8-(LPC1_21+8))
LPC1_20:
	add	r9, pc, r9
LPC1_21:
	ldr	r1, [pc, r1]
	mov	r2, r10
	mov	r3, #1
	strd	r8, r9, [sp]
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	movw	r0, :lower16:(l_anon.[ID].14-(LPC1_22+8))
	mov	r1, #9
	movt	r0, :upper16:(l_anon.[ID].14-(LPC1_22+8))
LPC1_22:
	add	r0, pc, r0
	bl	SYM(objc2::runtime::AnyProtocol::get::GENERATED_ID, 0)
	mov	r1, r0
	mov	r0, r4
	bl	SYM(objc2::__macro_helpers::<impl objc2::declare::ClassBuilder>::__add_protocol_methods::GENERATED_ID, 0)
	movw	r9, :lower16:(_copy_with_zone-(LPC1_23+8))
	mov	r3, #1
	movt	r9, :upper16:(_copy_with_zone-(LPC1_23+8))
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_f058a81939de2cb9-(LPC1_24+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_f058a81939de2cb9-(LPC1_24+8))
LPC1_23:
	add	r9, pc, r9
LPC1_24:
	ldr	r1, [pc, r1]
	movw	r2, :lower16:(l_anon.[ID].17-(LPC1_25+8))
	movt	r2, :upper16:(l_anon.[ID].17-(LPC1_25+8))
	strd	r8, r9, [sp]
LPC1_25:
	add	r2, pc, r2
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	ldr	r0, [sp, #8]
	bl	SYM(objc2::declare::ClassBuilder::register::GENERATED_ID, 0)
	sub	sp, r7, #20
	pop	{r8, r10, r11}
	pop	{r4, r5, r7, pc}
LBB1_3:
	movw	r0, :lower16:(l_anon.[ID].2-(LPC1_26+8))
	mov	r1, #43
	movt	r0, :upper16:(l_anon.[ID].2-(LPC1_26+8))
	movw	r2, :lower16:(l_anon.[ID].4-(LPC1_27+8))
	movt	r2, :upper16:(l_anon.[ID].4-(LPC1_27+8))
LPC1_26:
	add	r0, pc, r0
LPC1_27:
	add	r2, pc, r2
	mov	lr, pc
	b	SYM(core::panicking::panic::GENERATED_ID, 0)
LBB1_4:
	movw	r0, :lower16:(l_anon.[ID].20-(LPC1_28+8))
	mov	r5, #0
	movt	r0, :upper16:(l_anon.[ID].20-(LPC1_28+8))
	movw	r2, :lower16:(l_anon.[ID].21-(LPC1_29+8))
	movt	r2, :upper16:(l_anon.[ID].21-(LPC1_29+8))
	movw	r1, :lower16:(l_anon.[ID].10-(LPC1_30+8))
	movt	r1, :upper16:(l_anon.[ID].10-(LPC1_30+8))
	movw	r3, :lower16:(SYM(<&str as core[CRATE_ID]::fmt::Display>::fmt, 0)-(LPC1_31+8))
	movt	r3, :upper16:(SYM(<&str as core[CRATE_ID]::fmt::Display>::fmt, 0)-(LPC1_31+8))
	str	r5, [sp, #24]
	mov	r5, #2
LPC1_28:
	add	r0, pc, r0
	str	r5, [sp, #12]
	mov	r5, #1
LPC1_29:
	add	r2, pc, r2
LPC1_31:
	add	r3, pc, r3
	str	r5, [sp, #20]
	sub	r5, r7, #28
	str	r0, [sp, #8]
LPC1_30:
	add	r1, pc, r1
	add	r0, sp, #8
	str	r5, [sp, #16]
	str	r3, [r7, #-24]
	str	r2, [r7, #-28]
	mov	lr, pc
	b	SYM(core::panicking::panic_fmt::GENERATED_ID, 0)

	.p2align	2
	.code	32
SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0):
	push	{r7, lr}
	mov	r7, sp
	sub	sp, sp, #4
	ldr	r0, [r0]
	str	r0, [sp]
	mov	r0, sp
	bl	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::{closure#0}>::{closure#0}, 0)
	mov	sp, r7
	pop	{r7, pc}

	.p2align	2
	.code	32
SYM(<&str as core[CRATE_ID]::fmt::Display>::fmt, 0):
	push	{r7, lr}
	mov	r7, sp
	mov	r2, r1
	ldrd	r0, r1, [r0]
	bl	SYM(<str as core::fmt::Display>::fmt::GENERATED_ID, 0)
	pop	{r7, pc}

	.globl	_get_class
	.p2align	2
	.code	32
_get_class:
	push	{r7, lr}
	mov	r7, sp
	sub	sp, sp, #12
	movw	r0, :lower16:(SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::REGISTER_CLASS, 0)-(LPC4_0+8))
	movt	r0, :upper16:(SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::REGISTER_CLASS, 0)-(LPC4_0+8))
LPC4_0:
	add	r0, pc, r0
	ldr	r0, [r0]
	dmb	ish
	cmp	r0, #3
	bne	LBB4_3
LBB4_1:
	movw	r0, :lower16:(l_anon.[ID].11-(LPC4_4+8))
	mov	r1, #15
	movt	r0, :upper16:(l_anon.[ID].11-(LPC4_4+8))
LPC4_4:
	add	r0, pc, r0
	bl	SYM(objc2::runtime::AnyClass::get::GENERATED_ID, 0)
	cmp	r0, #0
	movne	sp, r7
	popne	{r7, pc}
LBB4_2:
	movw	r0, :lower16:(l_anon.[ID].2-(LPC4_5+8))
	mov	r1, #43
	movt	r0, :upper16:(l_anon.[ID].2-(LPC4_5+8))
	movw	r2, :lower16:(l_anon.[ID].10-(LPC4_6+8))
	movt	r2, :upper16:(l_anon.[ID].10-(LPC4_6+8))
LPC4_5:
	add	r0, pc, r0
LPC4_6:
	add	r2, pc, r2
	mov	lr, pc
	b	SYM(core::panicking::panic::GENERATED_ID, 0)
LBB4_3:
	movw	r0, :lower16:(SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::REGISTER_CLASS, 0)-(LPC4_1+8))
	mov	r2, #1
	movt	r0, :upper16:(SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::REGISTER_CLASS, 0)-(LPC4_1+8))
	movw	r3, :lower16:(l_anon.[ID].0-(LPC4_2+8))
	movt	r3, :upper16:(l_anon.[ID].0-(LPC4_2+8))
	movw	r1, :lower16:(l_anon.[ID].10-(LPC4_3+8))
	movt	r1, :upper16:(l_anon.[ID].10-(LPC4_3+8))
	strb	r2, [r7, #-5]
LPC4_3:
	add	r1, pc, r1
	sub	r2, r7, #5
	str	r2, [r7, #-4]
LPC4_1:
	add	r0, pc, r0
	str	r1, [sp]
LPC4_2:
	add	r3, pc, r3
	sub	r2, r7, #4
	mov	r1, #0
	bl	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	b	LBB4_1

	.globl	_get_obj
	.p2align	2
	.code	32
_get_obj:
	push	{r4, r7, lr}
	add	r7, sp, #4
	movw	r0, :lower16:(LL_OBJC_SELECTOR_REFERENCES_new$non_lazy_ptr-(LPC5_0+8))
	movt	r0, :upper16:(LL_OBJC_SELECTOR_REFERENCES_new$non_lazy_ptr-(LPC5_0+8))
LPC5_0:
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
	bl	SYM(objc2::runtime::AnyObject::class::GENERATED_ID, 0)
	movw	r1, :lower16:(L_anon.[ID].5-(LPC6_0+8))
	mov	r2, #4
	movt	r1, :upper16:(L_anon.[ID].5-(LPC6_0+8))
	movw	r3, :lower16:(l_anon.[ID].6-(LPC6_1+8))
	movt	r3, :upper16:(l_anon.[ID].6-(LPC6_1+8))
LPC6_0:
	add	r1, pc, r1
LPC6_1:
	add	r3, pc, r3
	bl	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	ldrb	r5, [r4, r0]
	mov	r0, r4
	bl	SYM(objc2::runtime::AnyObject::class::GENERATED_ID, 0)
	movw	r1, :lower16:(L_anon.[ID].7-(LPC6_2+8))
	mov	r2, #4
	movt	r1, :upper16:(L_anon.[ID].7-(LPC6_2+8))
	movw	r3, :lower16:(l_anon.[ID].8-(LPC6_3+8))
	movt	r3, :upper16:(l_anon.[ID].8-(LPC6_3+8))
LPC6_2:
	add	r1, pc, r1
LPC6_3:
	add	r3, pc, r3
	bl	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	ldr	r6, [r4, r0]
	mov	r0, r4
	bl	_objc_release
	mov	r0, r5
	mov	r1, r6
	pop	{r4, r5, r6, r7, pc}

	.globl	SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class, 0)
	.p2align	2
	.code	32
SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class, 0):
	push	{r7, lr}
	mov	r7, sp
	sub	sp, sp, #12
	movw	r0, :lower16:(SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::REGISTER_CLASS, 0)-(LPC7_0+8))
	movt	r0, :upper16:(SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::REGISTER_CLASS, 0)-(LPC7_0+8))
LPC7_0:
	add	r0, pc, r0
	ldr	r0, [r0]
	dmb	ish
	cmp	r0, #3
	bne	LBB7_3
LBB7_1:
	movw	r0, :lower16:(l_anon.[ID].11-(LPC7_4+8))
	mov	r1, #15
	movt	r0, :upper16:(l_anon.[ID].11-(LPC7_4+8))
LPC7_4:
	add	r0, pc, r0
	bl	SYM(objc2::runtime::AnyClass::get::GENERATED_ID, 0)
	cmp	r0, #0
	movne	sp, r7
	popne	{r7, pc}
LBB7_2:
	movw	r0, :lower16:(l_anon.[ID].2-(LPC7_5+8))
	mov	r1, #43
	movt	r0, :upper16:(l_anon.[ID].2-(LPC7_5+8))
	movw	r2, :lower16:(l_anon.[ID].10-(LPC7_6+8))
	movt	r2, :upper16:(l_anon.[ID].10-(LPC7_6+8))
LPC7_5:
	add	r0, pc, r0
LPC7_6:
	add	r2, pc, r2
	mov	lr, pc
	b	SYM(core::panicking::panic::GENERATED_ID, 0)
LBB7_3:
	movw	r0, :lower16:(SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::REGISTER_CLASS, 0)-(LPC7_1+8))
	mov	r2, #1
	movt	r0, :upper16:(SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::REGISTER_CLASS, 0)-(LPC7_1+8))
	movw	r3, :lower16:(l_anon.[ID].0-(LPC7_2+8))
	movt	r3, :upper16:(l_anon.[ID].0-(LPC7_2+8))
	movw	r1, :lower16:(l_anon.[ID].10-(LPC7_3+8))
	movt	r1, :upper16:(l_anon.[ID].10-(LPC7_3+8))
	strb	r2, [r7, #-5]
LPC7_3:
	add	r1, pc, r1
	sub	r2, r7, #5
	str	r2, [r7, #-4]
LPC7_1:
	add	r0, pc, r0
	str	r1, [sp]
LPC7_2:
	add	r3, pc, r3
	sub	r2, r7, #4
	mov	r1, #0
	bl	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	b	LBB7_1

	.p2align	2
	.code	32
SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::{closure#0}::__objc2_dealloc, 0):
	push	{r4, r5, r7, lr}
	add	r7, sp, #8
	sub	sp, sp, #8
	mov	r4, r1
	mov	r5, r0
	bl	SYM(objc2::runtime::AnyObject::class::GENERATED_ID, 0)
	movw	r1, :lower16:(L_anon.[ID].7-(LPC8_0+8))
	mov	r2, #4
	movt	r1, :upper16:(L_anon.[ID].7-(LPC8_0+8))
	movw	r3, :lower16:(l_anon.[ID].8-(LPC8_1+8))
	movt	r3, :upper16:(l_anon.[ID].8-(LPC8_1+8))
LPC8_0:
	add	r1, pc, r1
LPC8_1:
	add	r3, pc, r3
	bl	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	ldr	r0, [r5, r0]
	cmp	r0, #0
	beq	LBB8_2
	bl	_objc_release
LBB8_2:
	bl	SYM(<objc2::runtime::nsobject::NSObject as objc2::class_type::ClassType>::class::objc_static_workaround::GENERATED_ID, 0)
	str	r0, [sp, #4]
	mov	r0, sp
	mov	r1, r4
	str	r5, [sp]
	bl	_objc_msgSendSuper
	sub	sp, r7, #8
	pop	{r4, r5, r7, pc}

	.globl	_init
	.p2align	2
	.code	32
_init:
	push	{r4, r5, r7, lr}
	add	r7, sp, #8
	sub	sp, sp, #8
	mov	r4, r0
	movw	r0, :lower16:(LL_OBJC_SELECTOR_REFERENCES_init$non_lazy_ptr-(LPC9_0+8))
	movt	r0, :upper16:(LL_OBJC_SELECTOR_REFERENCES_init$non_lazy_ptr-(LPC9_0+8))
LPC9_0:
	ldr	r0, [pc, r0]
	ldr	r5, [r0]
	bl	SYM(<objc2::runtime::nsobject::NSObject as objc2::class_type::ClassType>::class::objc_static_workaround::GENERATED_ID, 0)
	str	r0, [sp, #4]
	mov	r0, sp
	mov	r1, r5
	str	r4, [sp]
	bl	_objc_msgSendSuper
	mov	r4, r0
	cmp	r0, #0
	beq	LBB9_2
	mov	r0, r4
	bl	SYM(objc2::runtime::AnyObject::class::GENERATED_ID, 0)
	movw	r1, :lower16:(L_anon.[ID].5-(LPC9_1+8))
	mov	r2, #4
	movt	r1, :upper16:(L_anon.[ID].5-(LPC9_1+8))
	movw	r3, :lower16:(l_anon.[ID].6-(LPC9_2+8))
	movt	r3, :upper16:(l_anon.[ID].6-(LPC9_2+8))
LPC9_1:
	add	r1, pc, r1
LPC9_2:
	add	r3, pc, r3
	bl	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	mov	r1, #42
	strb	r1, [r4, r0]
	mov	r0, r4
	bl	SYM(objc2::runtime::AnyObject::class::GENERATED_ID, 0)
	movw	r1, :lower16:(L_anon.[ID].7-(LPC9_3+8))
	mov	r2, #4
	movt	r1, :upper16:(L_anon.[ID].7-(LPC9_3+8))
	movw	r3, :lower16:(l_anon.[ID].8-(LPC9_4+8))
	movt	r3, :upper16:(l_anon.[ID].8-(LPC9_4+8))
LPC9_3:
	add	r1, pc, r1
LPC9_4:
	add	r3, pc, r3
	bl	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	mov	r1, #0
	str	r1, [r4, r0]
LBB9_2:
	mov	r0, r4
	sub	sp, r7, #8
	pop	{r4, r5, r7, pc}

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
	bl	SYM(objc2::runtime::AnyObject::class::GENERATED_ID, 0)
	movw	r1, :lower16:(L_anon.[ID].7-(LPC13_0+8))
	mov	r2, #4
	movt	r1, :upper16:(L_anon.[ID].7-(LPC13_0+8))
	movw	r3, :lower16:(l_anon.[ID].8-(LPC13_1+8))
	movt	r3, :upper16:(l_anon.[ID].8-(LPC13_1+8))
LPC13_0:
	add	r1, pc, r1
LPC13_1:
	add	r3, pc, r3
	bl	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	ldr	r0, [r4, r0]
	cmp	r0, #0
	beq	LBB13_2
	bl	_objc_retain
	bl	_objc_autoreleaseReturnValue
	pop	{r4, r7, pc}
LBB13_2:
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
	beq	LBB14_3
	mov	r0, r5
	bl	SYM(objc2::runtime::AnyObject::class::GENERATED_ID, 0)
	movw	r1, :lower16:(L_anon.[ID].7-(LPC14_0+8))
	mov	r2, #4
	movt	r1, :upper16:(L_anon.[ID].7-(LPC14_0+8))
	movw	r3, :lower16:(l_anon.[ID].8-(LPC14_1+8))
	movt	r3, :upper16:(l_anon.[ID].8-(LPC14_1+8))
LPC14_0:
	add	r1, pc, r1
LPC14_1:
	add	r3, pc, r3
	bl	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	ldr	r0, [r5, r0]
	cmp	r0, #0
	beq	LBB14_4
	bl	_objc_retain
	mov	r5, r0
	b	LBB14_5
LBB14_3:
	mov	r5, r4
	mov	r0, r5
	bl	_objc_autoreleaseReturnValue
	pop	{r4, r5, r6, r7, pc}
LBB14_4:
	mov	r5, #0
LBB14_5:
	mov	r0, r4
	bl	_objc_release
	mov	r0, r5
	bl	_objc_autoreleaseReturnValue
	pop	{r4, r5, r6, r7, pc}

	.globl	_copy_with_zone
	.p2align	2
	.code	32
_copy_with_zone:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	push	{r8, r10}
	mov	r5, r0
	bl	_get_obj
	mov	r4, r0
	cmp	r0, #0
	beq	LBB15_5
	mov	r0, r5
	bl	SYM(objc2::runtime::AnyObject::class::GENERATED_ID, 0)
	movw	r10, :lower16:(L_anon.[ID].5-(LPC15_0+8))
	mov	r2, #4
	movt	r10, :upper16:(L_anon.[ID].5-(LPC15_0+8))
	movw	r8, :lower16:(l_anon.[ID].6-(LPC15_1+8))
	movt	r8, :upper16:(l_anon.[ID].6-(LPC15_1+8))
LPC15_0:
	add	r10, pc, r10
LPC15_1:
	add	r8, pc, r8
	mov	r1, r10
	mov	r3, r8
	bl	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	ldrb	r6, [r5, r0]
	mov	r0, r4
	bl	SYM(objc2::runtime::AnyObject::class::GENERATED_ID, 0)
	mov	r1, r10
	mov	r2, #4
	mov	r3, r8
	bl	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	strb	r6, [r4, r0]
	mov	r0, r5
	bl	SYM(objc2::runtime::AnyObject::class::GENERATED_ID, 0)
	movw	r1, :lower16:(L_anon.[ID].7-(LPC15_2+8))
	mov	r2, #4
	movt	r1, :upper16:(L_anon.[ID].7-(LPC15_2+8))
	movw	r3, :lower16:(l_anon.[ID].8-(LPC15_3+8))
	movt	r3, :upper16:(l_anon.[ID].8-(LPC15_3+8))
LPC15_2:
	add	r1, pc, r1
LPC15_3:
	add	r3, pc, r3
	bl	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	ldr	r0, [r5, r0]
	cmp	r0, #0
	beq	LBB15_3
	bl	_objc_retain
	mov	r5, r0
	b	LBB15_4
LBB15_3:
	mov	r5, #0
LBB15_4:
	mov	r0, r4
	bl	SYM(objc2::runtime::AnyObject::class::GENERATED_ID, 0)
	movw	r1, :lower16:(L_anon.[ID].7-(LPC15_4+8))
	mov	r2, #4
	movt	r1, :upper16:(L_anon.[ID].7-(LPC15_4+8))
	movw	r3, :lower16:(l_anon.[ID].8-(LPC15_5+8))
	movt	r3, :upper16:(l_anon.[ID].8-(LPC15_5+8))
LPC15_4:
	add	r1, pc, r1
LPC15_5:
	add	r3, pc, r3
	bl	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	str	r5, [r4, r0]
LBB15_5:
	mov	r0, r4
	pop	{r8, r10}
	pop	{r4, r5, r6, r7, pc}

	.section	__DATA,__const
	.p2align	2, 0x0
l_anon.[ID].0:
	.long	SYM(core[CRATE_ID]::ptr::drop_in_place::<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::{closure#0}>::{closure#0}>, 0)
	.asciz	"\004\000\000\000\004\000\000"
	.long	SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0)
	.long	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::{closure#0}>::{closure#0}, 0)

	.section	__TEXT,__const
	.p2align	2, 0x0
l_anon.[ID].1:
	.byte	0

l_anon.[ID].2:
	.ascii	"called `Option::unwrap()` on a `None` value"

l_anon.[ID].3:
	.ascii	"/Users/madsmarquart/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/std/src/sync/once.rs"

	.section	__DATA,__const
	.p2align	2, 0x0
l_anon.[ID].4:
	.long	l_anon.[ID].3
	.asciz	"t\000\000\000\225\000\000\0002\000\000"

	.section	__TEXT,__literal4,4byte_literals
L_anon.[ID].5:
	.ascii	"_foo"

	.section	__TEXT,__const
	.p2align	2, 0x0
l_anon.[ID].6:
	.byte	5
	.space	19

	.section	__TEXT,__literal4,4byte_literals
L_anon.[ID].7:
	.ascii	"_obj"

	.section	__TEXT,__const
	.p2align	2, 0x0
l_anon.[ID].8:
	.byte	19
	.space	19

l_anon.[ID].9:
	.ascii	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	2, 0x0
l_anon.[ID].10:
	.long	l_anon.[ID].9
	.asciz	"5\000\000\000\013\000\000\000\001\000\000"

	.section	__TEXT,__const
l_anon.[ID].11:
	.ascii	"CustomClassName"

.zerofill __DATA,__bss,SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::REGISTER_CLASS, 0),4,2
	.p2align	2, 0x0
l_anon.[ID].12:
	.byte	17
	.space	19

	.p2align	2, 0x0
l_anon.[ID].13:
	.space	1
	.space	19

l_anon.[ID].14:
	.ascii	"NSCopying"

l_anon.[ID].15:
	.ascii	"_NSZone"

	.section	__DATA,__const
	.p2align	2, 0x0
l_anon.[ID].16:
	.byte	28
	.space	3
	.long	l_anon.[ID].15
	.asciz	"\007\000\000"
	.long	l_anon.[ID].1
	.space	4

	.p2align	2, 0x0
l_anon.[ID].17:
	.byte	25
	.space	3
	.long	l_anon.[ID].16
	.space	12

	.section	__TEXT,__const
l_anon.[ID].18:
	.ascii	"could not create new class "

l_anon.[ID].19:
	.ascii	". Perhaps a class with that name already exists?"

	.section	__DATA,__const
	.p2align	2, 0x0
l_anon.[ID].20:
	.long	l_anon.[ID].18
	.asciz	"\033\000\000"
	.long	l_anon.[ID].19
	.asciz	"0\000\000"

	.p2align	2, 0x0
l_anon.[ID].21:
	.long	l_anon.[ID].11
	.asciz	"\017\000\000"

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_694e247a0bc88869
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_694e247a0bc88869:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_694e247a0bc88869
L_OBJC_METH_VAR_NAME_694e247a0bc88869:
	.asciz	"dealloc"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_694e247a0bc88869
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_694e247a0bc88869:
	.long	L_OBJC_METH_VAR_NAME_694e247a0bc88869

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_8dd788dbcc16b9bc
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_8dd788dbcc16b9bc:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_8dd788dbcc16b9bc
L_OBJC_METH_VAR_NAME_8dd788dbcc16b9bc:
	.asciz	"classMethod"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_8dd788dbcc16b9bc
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_8dd788dbcc16b9bc:
	.long	L_OBJC_METH_VAR_NAME_8dd788dbcc16b9bc

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_450db9db0953dff5
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_450db9db0953dff5:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_450db9db0953dff5
L_OBJC_METH_VAR_NAME_450db9db0953dff5:
	.asciz	"method"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_450db9db0953dff5
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_450db9db0953dff5:
	.long	L_OBJC_METH_VAR_NAME_450db9db0953dff5

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_783b35bc45c6e4a6
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_783b35bc45c6e4a6:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_783b35bc45c6e4a6
L_OBJC_METH_VAR_NAME_783b35bc45c6e4a6:
	.asciz	"methodBool:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_783b35bc45c6e4a6
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_783b35bc45c6e4a6:
	.long	L_OBJC_METH_VAR_NAME_783b35bc45c6e4a6

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_828e9fbc6d0b4498
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_828e9fbc6d0b4498:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_828e9fbc6d0b4498
L_OBJC_METH_VAR_NAME_828e9fbc6d0b4498:
	.asciz	"methodId"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_828e9fbc6d0b4498
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_828e9fbc6d0b4498:
	.long	L_OBJC_METH_VAR_NAME_828e9fbc6d0b4498

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_788cc14ba6a28eb8
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_788cc14ba6a28eb8:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_788cc14ba6a28eb8
L_OBJC_METH_VAR_NAME_788cc14ba6a28eb8:
	.asciz	"methodIdWithParam:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_788cc14ba6a28eb8
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_788cc14ba6a28eb8:
	.long	L_OBJC_METH_VAR_NAME_788cc14ba6a28eb8

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_f058a81939de2cb9
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_f058a81939de2cb9:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_f058a81939de2cb9
L_OBJC_METH_VAR_NAME_f058a81939de2cb9:
	.asciz	"copyWithZone:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_f058a81939de2cb9
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_f058a81939de2cb9:
	.long	L_OBJC_METH_VAR_NAME_f058a81939de2cb9

	.section	__DATA,__nl_symbol_ptr,non_lazy_symbol_pointers
	.p2align	2, 0x0
LL_OBJC_SELECTOR_REFERENCES_init$non_lazy_ptr:
	.indirect_symbol	L_OBJC_SELECTOR_REFERENCES_init
	.long	0
LL_OBJC_SELECTOR_REFERENCES_new$non_lazy_ptr:
	.indirect_symbol	L_OBJC_SELECTOR_REFERENCES_new
	.long	0

.subsections_via_symbols
