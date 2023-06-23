	.section	__TEXT,__text,regular,pure_instructions
	.globl	_get_sel
	.p2align	2
_get_sel:
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
	b	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	.loh AdrpLdr	Lloh0, Lloh1
	.loh AdrpAdd	Lloh4, Lloh5
	.loh AdrpAdd	Lloh2, Lloh3

	.globl	_get_same_sel
	.p2align	2
_get_same_sel:
Lloh6:
	adrp	x8, __MergedGlobals@PAGE+8
Lloh7:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF+8]
	cbz	x0, LBB1_2
	ret
LBB1_2:
Lloh8:
	adrp	x0, __MergedGlobals@PAGE+8
Lloh9:
	add	x0, x0, __MergedGlobals@PAGEOFF+8
Lloh10:
	adrp	x1, l_anon.[ID].0@PAGE
Lloh11:
	add	x1, x1, l_anon.[ID].0@PAGEOFF
	b	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	.loh AdrpLdr	Lloh6, Lloh7
	.loh AdrpAdd	Lloh10, Lloh11
	.loh AdrpAdd	Lloh8, Lloh9

	.globl	_get_common_twice
	.p2align	2
_get_common_twice:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh12:
	adrp	x20, SYM(objc2::__macro_helpers::alloc_sel::CACHED_SEL::GENERATED_ID, 0)@GOTPAGE
Lloh13:
	ldr	x20, [x20, SYM(objc2::__macro_helpers::alloc_sel::CACHED_SEL::GENERATED_ID, 0)@GOTPAGEOFF]
	ldr	x19, [x20]
	cbz	x19, LBB2_3
	ldr	x1, [x20]
	cbz	x1, LBB2_4
LBB2_2:
	mov	x0, x19
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	ret
LBB2_3:
Lloh14:
	adrp	x0, SYM(objc2::__macro_helpers::alloc_sel::CACHED_SEL::GENERATED_ID, 0)@GOTPAGE
Lloh15:
	ldr	x0, [x0, SYM(objc2::__macro_helpers::alloc_sel::CACHED_SEL::GENERATED_ID, 0)@GOTPAGEOFF]
Lloh16:
	adrp	x1, l_anon.[ID].1@PAGE
Lloh17:
	add	x1, x1, l_anon.[ID].1@PAGEOFF
	bl	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	x19, x0
	ldr	x1, [x20]
	cbnz	x1, LBB2_2
LBB2_4:
Lloh18:
	adrp	x0, SYM(objc2::__macro_helpers::alloc_sel::CACHED_SEL::GENERATED_ID, 0)@GOTPAGE
Lloh19:
	ldr	x0, [x0, SYM(objc2::__macro_helpers::alloc_sel::CACHED_SEL::GENERATED_ID, 0)@GOTPAGEOFF]
Lloh20:
	adrp	x1, l_anon.[ID].1@PAGE
Lloh21:
	add	x1, x1, l_anon.[ID].1@PAGEOFF
	bl	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	x1, x0
	mov	x0, x19
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	ret
	.loh AdrpLdrGot	Lloh12, Lloh13
	.loh AdrpAdd	Lloh16, Lloh17
	.loh AdrpLdrGot	Lloh14, Lloh15
	.loh AdrpAdd	Lloh20, Lloh21
	.loh AdrpLdrGot	Lloh18, Lloh19

	.globl	_get_different_sel
	.p2align	2
_get_different_sel:
Lloh22:
	adrp	x8, __MergedGlobals@PAGE+16
Lloh23:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF+16]
	cbz	x0, LBB3_2
	ret
LBB3_2:
Lloh24:
	adrp	x0, __MergedGlobals@PAGE+16
Lloh25:
	add	x0, x0, __MergedGlobals@PAGEOFF+16
Lloh26:
	adrp	x1, l_anon.[ID].2@PAGE
Lloh27:
	add	x1, x1, l_anon.[ID].2@PAGEOFF
	b	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	.loh AdrpLdr	Lloh22, Lloh23
	.loh AdrpAdd	Lloh26, Lloh27
	.loh AdrpAdd	Lloh24, Lloh25

	.globl	_unused_sel
	.p2align	2
_unused_sel:
Lloh28:
	adrp	x8, SYM(test_dynamic_sel[CRATE_ID]::unused_sel::CACHED_SEL, 0)@PAGE
Lloh29:
	ldr	x8, [x8, SYM(test_dynamic_sel[CRATE_ID]::unused_sel::CACHED_SEL, 0)@PAGEOFF]
	cbz	x8, LBB4_2
	ret
LBB4_2:
Lloh30:
	adrp	x0, SYM(test_dynamic_sel[CRATE_ID]::unused_sel::CACHED_SEL, 0)@PAGE
Lloh31:
	add	x0, x0, SYM(test_dynamic_sel[CRATE_ID]::unused_sel::CACHED_SEL, 0)@PAGEOFF
Lloh32:
	adrp	x1, l_anon.[ID].3@PAGE
Lloh33:
	add	x1, x1, l_anon.[ID].3@PAGEOFF
	b	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	.loh AdrpLdr	Lloh28, Lloh29
	.loh AdrpAdd	Lloh32, Lloh33
	.loh AdrpAdd	Lloh30, Lloh31

	.globl	_use_fns
	.p2align	2
_use_fns:
	stp	x22, x21, [sp, #-48]!
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	mov	x19, x8
Lloh34:
	adrp	x8, __MergedGlobals@PAGE
Lloh35:
	ldr	x20, [x8, __MergedGlobals@PAGEOFF]
	cbz	x20, LBB5_5
Lloh36:
	adrp	x8, __MergedGlobals@PAGE+8
Lloh37:
	ldr	x21, [x8, __MergedGlobals@PAGEOFF+8]
	cbz	x21, LBB5_6
LBB5_2:
Lloh38:
	adrp	x8, __MergedGlobals@PAGE+16
Lloh39:
	ldr	x22, [x8, __MergedGlobals@PAGEOFF+16]
	cbz	x22, LBB5_7
LBB5_3:
Lloh40:
	adrp	x8, __MergedGlobals@PAGE+24
Lloh41:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF+24]
	cbz	x0, LBB5_8
LBB5_4:
	stp	x20, x21, [x19]
	stp	x22, x0, [x19, #16]
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	ldp	x22, x21, [sp], #48
	ret
LBB5_5:
Lloh42:
	adrp	x0, __MergedGlobals@PAGE
Lloh43:
	add	x0, x0, __MergedGlobals@PAGEOFF
Lloh44:
	adrp	x1, l_anon.[ID].0@PAGE
Lloh45:
	add	x1, x1, l_anon.[ID].0@PAGEOFF
	bl	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	x20, x0
Lloh46:
	adrp	x8, __MergedGlobals@PAGE+8
Lloh47:
	ldr	x21, [x8, __MergedGlobals@PAGEOFF+8]
	cbnz	x21, LBB5_2
LBB5_6:
Lloh48:
	adrp	x0, __MergedGlobals@PAGE+8
Lloh49:
	add	x0, x0, __MergedGlobals@PAGEOFF+8
Lloh50:
	adrp	x1, l_anon.[ID].0@PAGE
Lloh51:
	add	x1, x1, l_anon.[ID].0@PAGEOFF
	bl	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	x21, x0
Lloh52:
	adrp	x8, __MergedGlobals@PAGE+16
Lloh53:
	ldr	x22, [x8, __MergedGlobals@PAGEOFF+16]
	cbnz	x22, LBB5_3
LBB5_7:
Lloh54:
	adrp	x0, __MergedGlobals@PAGE+16
Lloh55:
	add	x0, x0, __MergedGlobals@PAGEOFF+16
Lloh56:
	adrp	x1, l_anon.[ID].2@PAGE
Lloh57:
	add	x1, x1, l_anon.[ID].2@PAGEOFF
	bl	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	x22, x0
Lloh58:
	adrp	x8, __MergedGlobals@PAGE+24
Lloh59:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF+24]
	cbnz	x0, LBB5_4
LBB5_8:
Lloh60:
	adrp	x0, __MergedGlobals@PAGE+24
Lloh61:
	add	x0, x0, __MergedGlobals@PAGEOFF+24
Lloh62:
	adrp	x1, l_anon.[ID].4@PAGE
Lloh63:
	add	x1, x1, l_anon.[ID].4@PAGEOFF
	bl	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	stp	x20, x21, [x19]
	stp	x22, x0, [x19, #16]
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	ldp	x22, x21, [sp], #48
	ret
	.loh AdrpLdr	Lloh34, Lloh35
	.loh AdrpLdr	Lloh36, Lloh37
	.loh AdrpLdr	Lloh38, Lloh39
	.loh AdrpLdr	Lloh40, Lloh41
	.loh AdrpLdr	Lloh46, Lloh47
	.loh AdrpAdd	Lloh44, Lloh45
	.loh AdrpAdd	Lloh42, Lloh43
	.loh AdrpLdr	Lloh52, Lloh53
	.loh AdrpAdd	Lloh50, Lloh51
	.loh AdrpAdd	Lloh48, Lloh49
	.loh AdrpLdr	Lloh58, Lloh59
	.loh AdrpAdd	Lloh56, Lloh57
	.loh AdrpAdd	Lloh54, Lloh55
	.loh AdrpAdd	Lloh62, Lloh63
	.loh AdrpAdd	Lloh60, Lloh61

	.globl	_use_same_twice
	.p2align	2
_use_same_twice:
	stp	x22, x21, [sp, #-48]!
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	mov	x19, x8
	adrp	x21, __MergedGlobals@PAGE
	ldr	x20, [x21, __MergedGlobals@PAGEOFF]
	cbz	x20, LBB6_3
	ldr	x0, [x21, __MergedGlobals@PAGEOFF]
	cbz	x0, LBB6_4
LBB6_2:
	stp	x20, x0, [x19]
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	ldp	x22, x21, [sp], #48
	ret
LBB6_3:
Lloh64:
	adrp	x0, __MergedGlobals@PAGE
Lloh65:
	add	x0, x0, __MergedGlobals@PAGEOFF
Lloh66:
	adrp	x1, l_anon.[ID].0@PAGE
Lloh67:
	add	x1, x1, l_anon.[ID].0@PAGEOFF
	bl	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	x20, x0
	ldr	x0, [x21, __MergedGlobals@PAGEOFF]
	cbnz	x0, LBB6_2
LBB6_4:
Lloh68:
	adrp	x0, __MergedGlobals@PAGE
Lloh69:
	add	x0, x0, __MergedGlobals@PAGEOFF
Lloh70:
	adrp	x1, l_anon.[ID].0@PAGE
Lloh71:
	add	x1, x1, l_anon.[ID].0@PAGEOFF
	bl	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	stp	x20, x0, [x19]
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	ldp	x22, x21, [sp], #48
	ret
	.loh AdrpAdd	Lloh66, Lloh67
	.loh AdrpAdd	Lloh64, Lloh65
	.loh AdrpAdd	Lloh70, Lloh71
	.loh AdrpAdd	Lloh68, Lloh69

	.globl	_use_in_loop
	.p2align	2
_use_in_loop:
	stp	x22, x21, [sp, #-48]!
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	cbz	x0, LBB7_5
	mov	x19, x0
	adrp	x22, SYM(test_dynamic_sel[CRATE_ID]::use_in_loop::CACHED_SEL, 0)@PAGE
Lloh72:
	adrp	x20, SYM(test_dynamic_sel[CRATE_ID]::use_in_loop::CACHED_SEL, 0)@PAGE
Lloh73:
	add	x20, x20, SYM(test_dynamic_sel[CRATE_ID]::use_in_loop::CACHED_SEL, 0)@PAGEOFF
Lloh74:
	adrp	x21, l_anon.[ID].5@PAGE
Lloh75:
	add	x21, x21, l_anon.[ID].5@PAGEOFF
	b	LBB7_3
LBB7_2:
	subs	x19, x19, #1
	b.eq	LBB7_5
LBB7_3:
	ldr	x8, [x22, SYM(test_dynamic_sel[CRATE_ID]::use_in_loop::CACHED_SEL, 0)@PAGEOFF]
	cbnz	x8, LBB7_2
	mov	x0, x20
	mov	x1, x21
	bl	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	b	LBB7_2
LBB7_5:
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	ldp	x22, x21, [sp], #48
	ret
	.loh AdrpAdd	Lloh74, Lloh75
	.loh AdrpAdd	Lloh72, Lloh73

	.section	__TEXT,__const
l_anon.[ID].0:
	.asciz	"simple"

l_anon.[ID].1:
	.asciz	"alloc"

	.section	__TEXT,__literal16,16byte_literals
l_anon.[ID].2:
	.asciz	"i:am:different:"

	.section	__TEXT,__const
l_anon.[ID].3:
	.asciz	"unused"

l_anon.[ID].4:
	.asciz	"fourthSel"

l_anon.[ID].5:
	.asciz	"loopedSelector"

.zerofill __DATA,__bss,SYM(test_dynamic_sel[CRATE_ID]::unused_sel::CACHED_SEL, 0),8,3
.zerofill __DATA,__bss,SYM(test_dynamic_sel[CRATE_ID]::use_in_loop::CACHED_SEL, 0),8,3
.zerofill __DATA,__bss,__MergedGlobals,32,3
.subsections_via_symbols
