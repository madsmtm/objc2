	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.p2align	2
	.code	32
SYM(<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::initialize::<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::get_or_init<objc2[CRATE_ID]::__macro_helpers::os_version::apple::lookup_version>::{closure#0}, !>, 0):
	ldr	r0, LCPI0_0
LPC0_0:
	ldr	r0, [pc, r0]
	ldr	r1, [r0]
	dmb	ish
	cmp	r1, #3
	bxeq	lr
LBB0_1:
	push	{r2, r3, r4, r5, r6, r7, lr}
	add	r7, sp, #20
	add	r1, r0, #4
	str	r1, [sp, #4]
	sub	r1, r7, #1
	str	r1, [sp, #8]
	add	r1, sp, #4
	str	r1, [r7, #-8]
	ldr	r1, LCPI0_1
LPC0_1:
	add	r1, pc, r1
	ldr	r3, LCPI0_2
LPC0_2:
	add	r3, pc, r3
	str	r1, [sp]
	sub	r2, r7, #8
	mov	r1, #1
	bl	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	mov	sp, r7
	pop	{r7, lr}
	bx	lr
	.p2align	2
	.data_region
LCPI0_0:
	.long	LSYM(objc2::__macro_helpers::os_version::apple::current_version::CURRENT_VERSION::GENERATED_ID, 0)$non_lazy_ptr-(LPC0_0+8)
LCPI0_1:
	.long	l_anon.[ID].2-(LPC0_1+8)
LCPI0_2:
	.long	l_anon.[ID].0-(LPC0_2+8)
	.end_data_region

	.p2align	2
	.code	32
SYM(<std[CRATE_ID]::sync::once::Once>::call_once_force::<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::initialize<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::get_or_init<objc2[CRATE_ID]::__macro_helpers::os_version::apple::lookup_version>::{closure#0}, !>::{closure#0}>::{closure#0}, 0):
	push	{r4, r7, lr}
	add	r7, sp, #4
	ldr	r0, [r0]
	ldr	r4, [r0]
	mov	r1, #0
	str	r1, [r0]
	cmp	r4, #0
	beq	LBB1_2
	bl	SYM(objc2::__macro_helpers::os_version::apple::lookup_version::GENERATED_ID, 0)
	str	r0, [r4]
	pop	{r4, r7, pc}
LBB1_2:
	movw	r0, :lower16:(l_anon.[ID].3-(LPC1_0+8))
	movt	r0, :upper16:(l_anon.[ID].3-(LPC1_0+8))
LPC1_0:
	add	r0, pc, r0
	mov	lr, pc
	b	SYM(core::option::unwrap_failed::GENERATED_ID, 0)

	.p2align	2
	.code	32
SYM(<<std[CRATE_ID]::sync::once::Once>::call_once_force<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::initialize<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::get_or_init<objc2[CRATE_ID]::__macro_helpers::os_version::apple::lookup_version>::{closure#0}, !>::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0):
	push	{r4, r7, lr}
	add	r7, sp, #4
	ldr	r0, [r0]
	ldr	r4, [r0]
	mov	r1, #0
	str	r1, [r0]
	cmp	r4, #0
	beq	LBB2_2
	bl	SYM(objc2::__macro_helpers::os_version::apple::lookup_version::GENERATED_ID, 0)
	str	r0, [r4]
	pop	{r4, r7, pc}
LBB2_2:
	movw	r0, :lower16:(l_anon.[ID].3-(LPC2_0+8))
	movt	r0, :upper16:(l_anon.[ID].3-(LPC2_0+8))
LPC2_0:
	add	r0, pc, r0
	mov	lr, pc
	b	SYM(core::option::unwrap_failed::GENERATED_ID, 0)

	.globl	_always
	.p2align	2
	.code	32
_always:
	mov	r0, #1
	bx	lr

	.globl	_never
	.p2align	2
	.code	32
_never:
	mov	r0, #0
	bx	lr

	.globl	_low
	.p2align	2
	.code	32
_low:
	mov	r0, #1
	bx	lr

	.globl	_high
	.p2align	2
	.code	32
_high:
	push	{r4, r7, lr}
	add	r7, sp, #4
	movw	r4, :lower16:(LSYM(objc2::__macro_helpers::os_version::apple::current_version::CURRENT_VERSION::GENERATED_ID, 0)$non_lazy_ptr-(LPC6_0+8))
	movt	r4, :upper16:(LSYM(objc2::__macro_helpers::os_version::apple::current_version::CURRENT_VERSION::GENERATED_ID, 0)$non_lazy_ptr-(LPC6_0+8))
LPC6_0:
	ldr	r4, [pc, r4]
	ldr	r0, [r4]
	dmb	ish
	cmp	r0, #3
	bne	LBB6_2
LBB6_1:
	ldrh	r1, [r4, #6]
	mov	r0, #0
	cmp	r1, #17
	movwhi	r0, #1
	pop	{r4, r7, pc}
LBB6_2:
	bl	SYM(<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::initialize::<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::get_or_init<objc2[CRATE_ID]::__macro_helpers::os_version::apple::lookup_version>::{closure#0}, !>, 0)
	b	LBB6_1

	.globl	_only_ios
	.p2align	2
	.code	32
_only_ios:
	mov	r0, #1
	bx	lr

	.globl	_two_checks
	.p2align	2
	.code	32
_two_checks:
	push	{r4, r5, r7, lr}
	add	r7, sp, #8
	movw	r4, :lower16:(LSYM(objc2::__macro_helpers::os_version::apple::current_version::CURRENT_VERSION::GENERATED_ID, 0)$non_lazy_ptr-(LPC8_0+8))
	movt	r4, :upper16:(LSYM(objc2::__macro_helpers::os_version::apple::current_version::CURRENT_VERSION::GENERATED_ID, 0)$non_lazy_ptr-(LPC8_0+8))
LPC8_0:
	ldr	r4, [pc, r4]
	ldr	r0, [r4]
	dmb	ish
	cmp	r0, #3
	bne	LBB8_3
LBB8_1:
	ldrh	r5, [r4, #6]
	ldr	r0, [r4]
	dmb	ish
	cmp	r0, #3
	bne	LBB8_4
LBB8_2:
	mov	r0, #0
	cmp	r5, #16
	mov	r1, #0
	movwhi	r1, #1
	ldrh	r2, [r4, #6]
	cmp	r2, #17
	movwhi	r0, #1
	and	r0, r1, r0
	pop	{r4, r5, r7, pc}
LBB8_3:
	bl	SYM(<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::initialize::<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::get_or_init<objc2[CRATE_ID]::__macro_helpers::os_version::apple::lookup_version>::{closure#0}, !>, 0)
	b	LBB8_1
LBB8_4:
	bl	SYM(<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::initialize::<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::get_or_init<objc2[CRATE_ID]::__macro_helpers::os_version::apple::lookup_version>::{closure#0}, !>, 0)
	b	LBB8_2

	.section	__DATA,__const
	.p2align	2, 0x0
l_anon.[ID].0:
	.asciz	"\000\000\000\000\004\000\000\000\004\000\000"
	.long	SYM(<<std[CRATE_ID]::sync::once::Once>::call_once_force<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::initialize<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::get_or_init<objc2[CRATE_ID]::__macro_helpers::os_version::apple::lookup_version>::{closure#0}, !>::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0)
	.long	SYM(<std[CRATE_ID]::sync::once::Once>::call_once_force::<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::initialize<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::get_or_init<objc2[CRATE_ID]::__macro_helpers::os_version::apple::lookup_version>::{closure#0}, !>::{closure#0}>::{closure#0}, 0)

	.section	__TEXT,__const
l_anon.[ID].1:
	.ascii	"$RUSTC/library/std/src/sync/once.rs"

	.section	__DATA,__const
	.p2align	2, 0x0
l_anon.[ID].2:
	.long	l_anon.[ID].1
	.asciz	"p\000\000\000\331\000\000\000\024\000\000"

	.p2align	2, 0x0
l_anon.[ID].3:
	.long	l_anon.[ID].1
	.asciz	"p\000\000\000\331\000\000\0001\000\000"

	.section	__DATA,__nl_symbol_ptr,non_lazy_symbol_pointers
	.p2align	2, 0x0
LSYM(objc2::__macro_helpers::os_version::apple::current_version::CURRENT_VERSION::GENERATED_ID, 0)$non_lazy_ptr:
	.indirect_symbol	SYM(objc2::__macro_helpers::os_version::apple::current_version::CURRENT_VERSION::GENERATED_ID, 0)
	.long	0

.subsections_via_symbols
