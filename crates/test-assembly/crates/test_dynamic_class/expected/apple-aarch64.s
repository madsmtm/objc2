	.section	__TEXT,__text,regular,pure_instructions
	.globl	_get_class
	.p2align	2
_get_class:
Lloh0:
	adrp	x8, __MergedGlobals@PAGE
Lloh1:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF]
	cbz	x0, LBB0_2
	ret
LBB0_2:
Lloh2:
	adrp	x0, __MergedGlobals@PAGE
Lloh3:
	add	x0, x0, __MergedGlobals@PAGEOFF
Lloh4:
	adrp	x1, l_anon.[ID].0@PAGE
Lloh5:
	add	x1, x1, l_anon.[ID].0@PAGEOFF
Lloh6:
	adrp	x2, l_anon.[ID].2@PAGE
Lloh7:
	add	x2, x2, l_anon.[ID].2@PAGEOFF
	b	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	.loh AdrpLdr	Lloh0, Lloh1
	.loh AdrpAdd	Lloh6, Lloh7
	.loh AdrpAdd	Lloh4, Lloh5
	.loh AdrpAdd	Lloh2, Lloh3

	.globl	_get_same_class
	.p2align	2
_get_same_class:
Lloh8:
	adrp	x8, __MergedGlobals@PAGE+8
Lloh9:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF+8]
	cbz	x0, LBB1_2
	ret
LBB1_2:
Lloh10:
	adrp	x0, __MergedGlobals@PAGE+8
Lloh11:
	add	x0, x0, __MergedGlobals@PAGEOFF+8
Lloh12:
	adrp	x1, l_anon.[ID].0@PAGE
Lloh13:
	add	x1, x1, l_anon.[ID].0@PAGEOFF
Lloh14:
	adrp	x2, l_anon.[ID].3@PAGE
Lloh15:
	add	x2, x2, l_anon.[ID].3@PAGEOFF
	b	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	.loh AdrpLdr	Lloh8, Lloh9
	.loh AdrpAdd	Lloh14, Lloh15
	.loh AdrpAdd	Lloh12, Lloh13
	.loh AdrpAdd	Lloh10, Lloh11

	.globl	_get_different_class
	.p2align	2
_get_different_class:
Lloh16:
	adrp	x8, __MergedGlobals@PAGE+16
Lloh17:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF+16]
	cbz	x0, LBB2_2
	ret
LBB2_2:
Lloh18:
	adrp	x0, __MergedGlobals@PAGE+16
Lloh19:
	add	x0, x0, __MergedGlobals@PAGEOFF+16
Lloh20:
	adrp	x1, l_anon.[ID].4@PAGE
Lloh21:
	add	x1, x1, l_anon.[ID].4@PAGEOFF
Lloh22:
	adrp	x2, l_anon.[ID].5@PAGE
Lloh23:
	add	x2, x2, l_anon.[ID].5@PAGEOFF
	b	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	.loh AdrpLdr	Lloh16, Lloh17
	.loh AdrpAdd	Lloh22, Lloh23
	.loh AdrpAdd	Lloh20, Lloh21
	.loh AdrpAdd	Lloh18, Lloh19

	.globl	_unused_class
	.p2align	2
_unused_class:
Lloh24:
	adrp	x8, SYM(test_dynamic_class[CRATE_ID]::unused_class::CACHED_CLASS, 0)@PAGE
Lloh25:
	ldr	x8, [x8, SYM(test_dynamic_class[CRATE_ID]::unused_class::CACHED_CLASS, 0)@PAGEOFF]
	cbz	x8, LBB3_2
	ret
LBB3_2:
Lloh26:
	adrp	x0, SYM(test_dynamic_class[CRATE_ID]::unused_class::CACHED_CLASS, 0)@PAGE
Lloh27:
	add	x0, x0, SYM(test_dynamic_class[CRATE_ID]::unused_class::CACHED_CLASS, 0)@PAGEOFF
Lloh28:
	adrp	x1, l_anon.[ID].6@PAGE
Lloh29:
	add	x1, x1, l_anon.[ID].6@PAGEOFF
Lloh30:
	adrp	x2, l_anon.[ID].7@PAGE
Lloh31:
	add	x2, x2, l_anon.[ID].7@PAGEOFF
	b	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	.loh AdrpLdr	Lloh24, Lloh25
	.loh AdrpAdd	Lloh30, Lloh31
	.loh AdrpAdd	Lloh28, Lloh29
	.loh AdrpAdd	Lloh26, Lloh27

	.globl	_use_fns
	.p2align	2
_use_fns:
	stp	x22, x21, [sp, #-48]!
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
Lloh32:
	adrp	x9, __MergedGlobals@PAGE
Lloh33:
	ldr	x19, [x9, __MergedGlobals@PAGEOFF]
	cbz	x19, LBB4_5
Lloh34:
	adrp	x9, __MergedGlobals@PAGE+8
Lloh35:
	ldr	x20, [x9, __MergedGlobals@PAGEOFF+8]
	cbz	x20, LBB4_6
LBB4_2:
Lloh36:
	adrp	x9, __MergedGlobals@PAGE+16
Lloh37:
	ldr	x21, [x9, __MergedGlobals@PAGEOFF+16]
	cbz	x21, LBB4_7
LBB4_3:
Lloh38:
	adrp	x9, __MergedGlobals@PAGE+24
Lloh39:
	ldr	x0, [x9, __MergedGlobals@PAGEOFF+24]
	cbz	x0, LBB4_8
LBB4_4:
	stp	x19, x20, [x8]
	stp	x21, x0, [x8, #16]
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	ldp	x22, x21, [sp], #48
	ret
LBB4_5:
Lloh40:
	adrp	x0, __MergedGlobals@PAGE
Lloh41:
	add	x0, x0, __MergedGlobals@PAGEOFF
Lloh42:
	adrp	x1, l_anon.[ID].0@PAGE
Lloh43:
	add	x1, x1, l_anon.[ID].0@PAGEOFF
Lloh44:
	adrp	x2, l_anon.[ID].2@PAGE
Lloh45:
	add	x2, x2, l_anon.[ID].2@PAGEOFF
	mov	x19, x8
	bl	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	mov	x8, x19
	mov	x19, x0
Lloh46:
	adrp	x9, __MergedGlobals@PAGE+8
Lloh47:
	ldr	x20, [x9, __MergedGlobals@PAGEOFF+8]
	cbnz	x20, LBB4_2
LBB4_6:
Lloh48:
	adrp	x0, __MergedGlobals@PAGE+8
Lloh49:
	add	x0, x0, __MergedGlobals@PAGEOFF+8
Lloh50:
	adrp	x1, l_anon.[ID].0@PAGE
Lloh51:
	add	x1, x1, l_anon.[ID].0@PAGEOFF
Lloh52:
	adrp	x2, l_anon.[ID].3@PAGE
Lloh53:
	add	x2, x2, l_anon.[ID].3@PAGEOFF
	mov	x20, x8
	bl	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	mov	x8, x20
	mov	x20, x0
Lloh54:
	adrp	x9, __MergedGlobals@PAGE+16
Lloh55:
	ldr	x21, [x9, __MergedGlobals@PAGEOFF+16]
	cbnz	x21, LBB4_3
LBB4_7:
Lloh56:
	adrp	x0, __MergedGlobals@PAGE+16
Lloh57:
	add	x0, x0, __MergedGlobals@PAGEOFF+16
Lloh58:
	adrp	x1, l_anon.[ID].4@PAGE
Lloh59:
	add	x1, x1, l_anon.[ID].4@PAGEOFF
Lloh60:
	adrp	x2, l_anon.[ID].5@PAGE
Lloh61:
	add	x2, x2, l_anon.[ID].5@PAGEOFF
	mov	x21, x8
	bl	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	mov	x8, x21
	mov	x21, x0
Lloh62:
	adrp	x9, __MergedGlobals@PAGE+24
Lloh63:
	ldr	x0, [x9, __MergedGlobals@PAGEOFF+24]
	cbnz	x0, LBB4_4
LBB4_8:
Lloh64:
	adrp	x0, __MergedGlobals@PAGE+24
Lloh65:
	add	x0, x0, __MergedGlobals@PAGEOFF+24
Lloh66:
	adrp	x1, l_anon.[ID].8@PAGE
Lloh67:
	add	x1, x1, l_anon.[ID].8@PAGEOFF
Lloh68:
	adrp	x2, l_anon.[ID].9@PAGE
Lloh69:
	add	x2, x2, l_anon.[ID].9@PAGEOFF
	mov	x22, x8
	bl	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	stp	x19, x20, [x22]
	stp	x21, x0, [x22, #16]
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	ldp	x22, x21, [sp], #48
	ret
	.loh AdrpLdr	Lloh32, Lloh33
	.loh AdrpLdr	Lloh34, Lloh35
	.loh AdrpLdr	Lloh36, Lloh37
	.loh AdrpLdr	Lloh38, Lloh39
	.loh AdrpLdr	Lloh46, Lloh47
	.loh AdrpAdd	Lloh44, Lloh45
	.loh AdrpAdd	Lloh42, Lloh43
	.loh AdrpAdd	Lloh40, Lloh41
	.loh AdrpLdr	Lloh54, Lloh55
	.loh AdrpAdd	Lloh52, Lloh53
	.loh AdrpAdd	Lloh50, Lloh51
	.loh AdrpAdd	Lloh48, Lloh49
	.loh AdrpLdr	Lloh62, Lloh63
	.loh AdrpAdd	Lloh60, Lloh61
	.loh AdrpAdd	Lloh58, Lloh59
	.loh AdrpAdd	Lloh56, Lloh57
	.loh AdrpAdd	Lloh68, Lloh69
	.loh AdrpAdd	Lloh66, Lloh67
	.loh AdrpAdd	Lloh64, Lloh65

	.globl	_use_same_twice
	.p2align	2
_use_same_twice:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	adrp	x20, __MergedGlobals@PAGE
	ldr	x19, [x20, __MergedGlobals@PAGEOFF]
	cbz	x19, LBB5_3
	ldr	x0, [x20, __MergedGlobals@PAGEOFF]
	cbz	x0, LBB5_4
LBB5_2:
	stp	x19, x0, [x8]
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	ret
LBB5_3:
Lloh70:
	adrp	x0, __MergedGlobals@PAGE
Lloh71:
	add	x0, x0, __MergedGlobals@PAGEOFF
Lloh72:
	adrp	x1, l_anon.[ID].0@PAGE
Lloh73:
	add	x1, x1, l_anon.[ID].0@PAGEOFF
Lloh74:
	adrp	x2, l_anon.[ID].2@PAGE
Lloh75:
	add	x2, x2, l_anon.[ID].2@PAGEOFF
	mov	x19, x8
	bl	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	mov	x8, x19
	mov	x19, x0
	ldr	x0, [x20, __MergedGlobals@PAGEOFF]
	cbnz	x0, LBB5_2
LBB5_4:
Lloh76:
	adrp	x0, __MergedGlobals@PAGE
Lloh77:
	add	x0, x0, __MergedGlobals@PAGEOFF
Lloh78:
	adrp	x1, l_anon.[ID].0@PAGE
Lloh79:
	add	x1, x1, l_anon.[ID].0@PAGEOFF
Lloh80:
	adrp	x2, l_anon.[ID].2@PAGE
Lloh81:
	add	x2, x2, l_anon.[ID].2@PAGEOFF
	mov	x20, x8
	bl	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	stp	x19, x0, [x20]
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	ret
	.loh AdrpAdd	Lloh74, Lloh75
	.loh AdrpAdd	Lloh72, Lloh73
	.loh AdrpAdd	Lloh70, Lloh71
	.loh AdrpAdd	Lloh80, Lloh81
	.loh AdrpAdd	Lloh78, Lloh79
	.loh AdrpAdd	Lloh76, Lloh77

	.globl	_use_in_loop
	.p2align	2
_use_in_loop:
	cbz	x0, LBB6_6
	stp	x24, x23, [sp, #-64]!
	stp	x22, x21, [sp, #16]
	stp	x20, x19, [sp, #32]
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	mov	x19, x0
	adrp	x23, SYM(test_dynamic_class[CRATE_ID]::use_in_loop::CACHED_CLASS, 0)@PAGE
Lloh82:
	adrp	x20, SYM(test_dynamic_class[CRATE_ID]::use_in_loop::CACHED_CLASS, 0)@PAGE
Lloh83:
	add	x20, x20, SYM(test_dynamic_class[CRATE_ID]::use_in_loop::CACHED_CLASS, 0)@PAGEOFF
Lloh84:
	adrp	x21, l_anon.[ID].10@PAGE
Lloh85:
	add	x21, x21, l_anon.[ID].10@PAGEOFF
Lloh86:
	adrp	x22, l_anon.[ID].11@PAGE
Lloh87:
	add	x22, x22, l_anon.[ID].11@PAGEOFF
	b	LBB6_3
LBB6_2:
	subs	x19, x19, #1
	b.eq	LBB6_5
LBB6_3:
	ldr	x8, [x23, SYM(test_dynamic_class[CRATE_ID]::use_in_loop::CACHED_CLASS, 0)@PAGEOFF]
	cbnz	x8, LBB6_2
	mov	x0, x20
	mov	x1, x21
	mov	x2, x22
	bl	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	b	LBB6_2
LBB6_5:
	ldp	x29, x30, [sp, #48]
	ldp	x20, x19, [sp, #32]
	ldp	x22, x21, [sp, #16]
	ldp	x24, x23, [sp], #64
LBB6_6:
	ret
	.loh AdrpAdd	Lloh86, Lloh87
	.loh AdrpAdd	Lloh84, Lloh85
	.loh AdrpAdd	Lloh82, Lloh83

	.section	__TEXT,__const
l_anon.[ID].0:
	.asciz	"NSObject"

l_anon.[ID].1:
	.ascii	"crates/$DIR/../test_static_class/lib.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].2:
	.quad	l_anon.[ID].1
	.asciz	"J\000\000\000\000\000\000\000\007\000\000\000\005\000\000"

	.p2align	3, 0x0
l_anon.[ID].3:
	.quad	l_anon.[ID].1
	.asciz	"J\000\000\000\000\000\000\000\f\000\000\000\005\000\000"

	.section	__TEXT,__const
l_anon.[ID].4:
	.asciz	"NSString"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].5:
	.quad	l_anon.[ID].1
	.asciz	"J\000\000\000\000\000\000\000\021\000\000\000\005\000\000"

	.section	__TEXT,__const
l_anon.[ID].6:
	.asciz	"NSData"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].7:
	.quad	l_anon.[ID].1
	.asciz	"J\000\000\000\000\000\000\000\026\000\000\000\r\000\000"

	.section	__TEXT,__const
l_anon.[ID].8:
	.asciz	"NSException"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].9:
	.quad	l_anon.[ID].1
	.asciz	"J\000\000\000\000\000\000\000\036\000\000\000\016\000\000"

	.section	__TEXT,__const
l_anon.[ID].10:
	.asciz	"NSLock"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].11:
	.quad	l_anon.[ID].1
	.asciz	"J\000\000\000\000\000\000\000,\000\000\000\021\000\000"

.zerofill __DATA,__bss,SYM(test_dynamic_class[CRATE_ID]::unused_class::CACHED_CLASS, 0),8,3
.zerofill __DATA,__bss,SYM(test_dynamic_class[CRATE_ID]::use_in_loop::CACHED_CLASS, 0),8,3
.zerofill __DATA,__bss,__MergedGlobals,32,3
.subsections_via_symbols
