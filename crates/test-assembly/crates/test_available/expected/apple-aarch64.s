	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
SYM(<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::initialize::<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::get_or_init<objc2[CRATE_ID]::__macro_helpers::os_version::apple::lookup_version>::{closure#0}, !>, 0):
Lloh0:
	adrp	x0, SYM(objc2::__macro_helpers::os_version::apple::current_version::CURRENT_VERSION::GENERATED_ID, 0)@GOTPAGE
Lloh1:
	ldr	x0, [x0, SYM(objc2::__macro_helpers::os_version::apple::current_version::CURRENT_VERSION::GENERATED_ID, 0)@GOTPAGEOFF]
	ldapr	x8, [x0]
	cmp	x8, #3
	b.ne	LBB0_2
	ret
LBB0_2:
	sub	sp, sp, #48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	add	x8, x0, #8
	sub	x9, x29, #1
	stp	x8, x9, [sp]
	mov	x8, sp
	str	x8, [sp, #16]
Lloh2:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh3:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh4:
	adrp	x4, l_anon.[ID].2@PAGE
Lloh5:
	add	x4, x4, l_anon.[ID].2@PAGEOFF
	add	x2, sp, #16
	mov	w1, #1
	bl	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	ret
	.loh AdrpLdrGot	Lloh0, Lloh1
	.loh AdrpAdd	Lloh4, Lloh5
	.loh AdrpAdd	Lloh2, Lloh3

	.p2align	2
SYM(<std[CRATE_ID]::sync::once::Once>::call_once_force::<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::initialize<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::get_or_init<objc2[CRATE_ID]::__macro_helpers::os_version::apple::lookup_version>::{closure#0}, !>::{closure#0}>::{closure#0}, 0):
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	ldr	x8, [x0]
	ldr	x19, [x8]
	str	xzr, [x8]
	cbz	x19, LBB1_2
	bl	SYM(objc2::__macro_helpers::os_version::apple::lookup_version::GENERATED_ID, 0)
	str	w0, [x19]
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	ret
LBB1_2:
Lloh6:
	adrp	x0, l_anon.[ID].3@PAGE
Lloh7:
	add	x0, x0, l_anon.[ID].3@PAGEOFF
	bl	SYM(core::option::unwrap_failed::GENERATED_ID, 0)
	.loh AdrpAdd	Lloh6, Lloh7

	.p2align	2
SYM(<<std[CRATE_ID]::sync::once::Once>::call_once_force<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::initialize<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::get_or_init<objc2[CRATE_ID]::__macro_helpers::os_version::apple::lookup_version>::{closure#0}, !>::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0):
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	ldr	x8, [x0]
	ldr	x19, [x8]
	str	xzr, [x8]
	cbz	x19, LBB2_2
	bl	SYM(objc2::__macro_helpers::os_version::apple::lookup_version::GENERATED_ID, 0)
	str	w0, [x19]
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	ret
LBB2_2:
Lloh8:
	adrp	x0, l_anon.[ID].3@PAGE
Lloh9:
	add	x0, x0, l_anon.[ID].3@PAGEOFF
	bl	SYM(core::option::unwrap_failed::GENERATED_ID, 0)
	.loh AdrpAdd	Lloh8, Lloh9

	.globl	_always
	.p2align	2
_always:
	mov	w0, #1
	ret

	.globl	_never
	.p2align	2
_never:
	mov	w0, #0
	ret

	.globl	_low
	.p2align	2
_low:
	mov	w0, #1
	ret

	.globl	_high
	.p2align	2
_high:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh10:
	adrp	x19, SYM(objc2::__macro_helpers::os_version::apple::current_version::CURRENT_VERSION::GENERATED_ID, 0)@GOTPAGE
Lloh11:
	ldr	x19, [x19, SYM(objc2::__macro_helpers::os_version::apple::current_version::CURRENT_VERSION::GENERATED_ID, 0)@GOTPAGEOFF]
	ldapr	x8, [x19]
	cmp	x8, #3
	b.ne	LBB6_2
LBB6_1:
	ldrh	w8, [x19, #10]
	cmp	w8, #14
	cset	w0, hi
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	ret
LBB6_2:
	bl	SYM(<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::initialize::<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::get_or_init<objc2[CRATE_ID]::__macro_helpers::os_version::apple::lookup_version>::{closure#0}, !>, 0)
	b	LBB6_1
	.loh AdrpLdrGot	Lloh10, Lloh11

	.globl	_only_ios
	.p2align	2
_only_ios:
	mov	w0, #0
	ret

	.globl	_two_checks
	.p2align	2
_two_checks:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh12:
	adrp	x19, SYM(objc2::__macro_helpers::os_version::apple::current_version::CURRENT_VERSION::GENERATED_ID, 0)@GOTPAGE
Lloh13:
	ldr	x19, [x19, SYM(objc2::__macro_helpers::os_version::apple::current_version::CURRENT_VERSION::GENERATED_ID, 0)@GOTPAGEOFF]
	ldapr	x8, [x19]
	cmp	x8, #3
	b.ne	LBB8_3
	ldrh	w20, [x19, #10]
	ldapr	x8, [x19]
	cmp	x8, #3
	b.ne	LBB8_4
LBB8_2:
	cmp	w20, #13
	ldrh	w8, [x19, #10]
	ccmp	w8, #14, #0, hi
	cset	w0, hi
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	ret
LBB8_3:
	bl	SYM(<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::initialize::<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::get_or_init<objc2[CRATE_ID]::__macro_helpers::os_version::apple::lookup_version>::{closure#0}, !>, 0)
	ldrh	w20, [x19, #10]
	ldapr	x8, [x19]
	cmp	x8, #3
	b.eq	LBB8_2
LBB8_4:
	bl	SYM(<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::initialize::<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::get_or_init<objc2[CRATE_ID]::__macro_helpers::os_version::apple::lookup_version>::{closure#0}, !>, 0)
	b	LBB8_2
	.loh AdrpLdrGot	Lloh12, Lloh13

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].0:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	SYM(<<std[CRATE_ID]::sync::once::Once>::call_once_force<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::initialize<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::get_or_init<objc2[CRATE_ID]::__macro_helpers::os_version::apple::lookup_version>::{closure#0}, !>::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0)
	.quad	SYM(<std[CRATE_ID]::sync::once::Once>::call_once_force::<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::initialize<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::get_or_init<objc2[CRATE_ID]::__macro_helpers::os_version::apple::lookup_version>::{closure#0}, !>::{closure#0}>::{closure#0}, 0)

	.section	__TEXT,__const
l_anon.[ID].1:
	.ascii	"$RUSTC/library/std/src/sync/once.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].2:
	.quad	l_anon.[ID].1
	.asciz	"p\000\000\000\000\000\000\000\331\000\000\000\024\000\000"

	.p2align	3, 0x0
l_anon.[ID].3:
	.quad	l_anon.[ID].1
	.asciz	"p\000\000\000\000\000\000\000\331\000\000\0001\000\000"

.subsections_via_symbols
