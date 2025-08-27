	.section	__TEXT,__text,regular,pure_instructions
	.globl	_fn1_get_sel
	.p2align	2
_fn1_get_sel:
Lloh0:
	adrp	x8, __MergedGlobals@PAGE+16
Lloh1:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF+16]
	cbz	x0, LBB0_2
	ret
LBB0_2:
Lloh2:
	adrp	x0, __MergedGlobals@PAGE+16
Lloh3:
	add	x0, x0, __MergedGlobals@PAGEOFF+16
Lloh4:
	adrp	x1, l_anon.[ID].0@PAGE
Lloh5:
	add	x1, x1, l_anon.[ID].0@PAGEOFF
	b	SYM(objc2::__macros::sel::CachedSel::fetch::GENERATED_ID, 0)
	.loh AdrpLdr	Lloh0, Lloh1
	.loh AdrpAdd	Lloh4, Lloh5
	.loh AdrpAdd	Lloh2, Lloh3

	.globl	_fn2_get_same_sel
	.p2align	2
_fn2_get_same_sel:
Lloh6:
	adrp	x8, __MergedGlobals@PAGE
Lloh7:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF]
	cbz	x0, LBB1_2
	ret
LBB1_2:
Lloh8:
	adrp	x0, __MergedGlobals@PAGE
Lloh9:
	add	x0, x0, __MergedGlobals@PAGEOFF
Lloh10:
	adrp	x1, l_anon.[ID].0@PAGE
Lloh11:
	add	x1, x1, l_anon.[ID].0@PAGEOFF
	b	SYM(objc2::__macros::sel::CachedSel::fetch::GENERATED_ID, 0)
	.loh AdrpLdr	Lloh6, Lloh7
	.loh AdrpAdd	Lloh10, Lloh11
	.loh AdrpAdd	Lloh8, Lloh9

	.globl	_fn3_get_common_twice
	.p2align	2
_fn3_get_common_twice:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh12:
	adrp	x19, SYM(objc2::__macros::sel::alloc_sel::CACHED_SEL::GENERATED_ID, 0)@GOTPAGE
Lloh13:
	ldr	x19, [x19, SYM(objc2::__macros::sel::alloc_sel::CACHED_SEL::GENERATED_ID, 0)@GOTPAGEOFF]
	ldr	x0, [x19]
	cbz	x0, LBB2_3
	ldr	x1, [x19]
	cbz	x1, LBB2_4
LBB2_2:
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	ret
LBB2_3:
Lloh14:
	adrp	x0, SYM(objc2::__macros::sel::alloc_sel::CACHED_SEL::GENERATED_ID, 0)@GOTPAGE
Lloh15:
	ldr	x0, [x0, SYM(objc2::__macros::sel::alloc_sel::CACHED_SEL::GENERATED_ID, 0)@GOTPAGEOFF]
Lloh16:
	adrp	x1, l_anon.[ID].1@PAGE
Lloh17:
	add	x1, x1, l_anon.[ID].1@PAGEOFF
	bl	SYM(objc2::__macros::sel::CachedSel::fetch::GENERATED_ID, 0)
	ldr	x1, [x19]
	cbnz	x1, LBB2_2
LBB2_4:
Lloh18:
	adrp	x8, SYM(objc2::__macros::sel::alloc_sel::CACHED_SEL::GENERATED_ID, 0)@GOTPAGE
Lloh19:
	ldr	x8, [x8, SYM(objc2::__macros::sel::alloc_sel::CACHED_SEL::GENERATED_ID, 0)@GOTPAGEOFF]
Lloh20:
	adrp	x1, l_anon.[ID].1@PAGE
Lloh21:
	add	x1, x1, l_anon.[ID].1@PAGEOFF
	mov	x19, x0
	mov	x0, x8
	bl	SYM(objc2::__macros::sel::CachedSel::fetch::GENERATED_ID, 0)
	mov	x1, x0
	mov	x0, x19
	b	LBB2_2
	.loh AdrpLdrGot	Lloh12, Lloh13
	.loh AdrpAdd	Lloh16, Lloh17
	.loh AdrpLdrGot	Lloh14, Lloh15
	.loh AdrpAdd	Lloh20, Lloh21
	.loh AdrpLdrGot	Lloh18, Lloh19

	.globl	_fn4_get_different_sel
	.p2align	2
_fn4_get_different_sel:
Lloh22:
	adrp	x8, __MergedGlobals@PAGE+8
Lloh23:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF+8]
	cbz	x0, LBB3_2
	ret
LBB3_2:
Lloh24:
	adrp	x0, __MergedGlobals@PAGE+8
Lloh25:
	add	x0, x0, __MergedGlobals@PAGEOFF+8
Lloh26:
	adrp	x1, l_anon.[ID].2@PAGE
Lloh27:
	add	x1, x1, l_anon.[ID].2@PAGEOFF
	b	SYM(objc2::__macros::sel::CachedSel::fetch::GENERATED_ID, 0)
	.loh AdrpLdr	Lloh22, Lloh23
	.loh AdrpAdd	Lloh26, Lloh27
	.loh AdrpAdd	Lloh24, Lloh25

	.globl	_fn5_unused_sel
	.p2align	2
_fn5_unused_sel:
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
	b	SYM(objc2::__macros::sel::CachedSel::fetch::GENERATED_ID, 0)
	.loh AdrpLdr	Lloh28, Lloh29
	.loh AdrpAdd	Lloh32, Lloh33
	.loh AdrpAdd	Lloh30, Lloh31

	.globl	_fn6_use_fns
	.p2align	2
_fn6_use_fns:
	stp	x22, x21, [sp, #-48]!
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
Lloh34:
	adrp	x9, __MergedGlobals@PAGE+16
Lloh35:
	ldr	x19, [x9, __MergedGlobals@PAGEOFF+16]
	cbz	x19, LBB5_5
Lloh36:
	adrp	x9, __MergedGlobals@PAGE
Lloh37:
	ldr	x20, [x9, __MergedGlobals@PAGEOFF]
	cbz	x20, LBB5_6
LBB5_2:
Lloh38:
	adrp	x9, __MergedGlobals@PAGE+8
Lloh39:
	ldr	x21, [x9, __MergedGlobals@PAGEOFF+8]
	cbz	x21, LBB5_7
LBB5_3:
Lloh40:
	adrp	x9, __MergedGlobals@PAGE+24
Lloh41:
	ldr	x0, [x9, __MergedGlobals@PAGEOFF+24]
	cbz	x0, LBB5_8
LBB5_4:
	stp	x19, x20, [x8]
	stp	x21, x0, [x8, #16]
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	ldp	x22, x21, [sp], #48
	ret
LBB5_5:
Lloh42:
	adrp	x0, __MergedGlobals@PAGE+16
Lloh43:
	add	x0, x0, __MergedGlobals@PAGEOFF+16
Lloh44:
	adrp	x1, l_anon.[ID].0@PAGE
Lloh45:
	add	x1, x1, l_anon.[ID].0@PAGEOFF
	mov	x19, x8
	bl	SYM(objc2::__macros::sel::CachedSel::fetch::GENERATED_ID, 0)
	mov	x8, x19
	mov	x19, x0
Lloh46:
	adrp	x9, __MergedGlobals@PAGE
Lloh47:
	ldr	x20, [x9, __MergedGlobals@PAGEOFF]
	cbnz	x20, LBB5_2
LBB5_6:
Lloh48:
	adrp	x0, __MergedGlobals@PAGE
Lloh49:
	add	x0, x0, __MergedGlobals@PAGEOFF
Lloh50:
	adrp	x1, l_anon.[ID].0@PAGE
Lloh51:
	add	x1, x1, l_anon.[ID].0@PAGEOFF
	mov	x20, x8
	bl	SYM(objc2::__macros::sel::CachedSel::fetch::GENERATED_ID, 0)
	mov	x8, x20
	mov	x20, x0
Lloh52:
	adrp	x9, __MergedGlobals@PAGE+8
Lloh53:
	ldr	x21, [x9, __MergedGlobals@PAGEOFF+8]
	cbnz	x21, LBB5_3
LBB5_7:
Lloh54:
	adrp	x0, __MergedGlobals@PAGE+8
Lloh55:
	add	x0, x0, __MergedGlobals@PAGEOFF+8
Lloh56:
	adrp	x1, l_anon.[ID].2@PAGE
Lloh57:
	add	x1, x1, l_anon.[ID].2@PAGEOFF
	mov	x21, x8
	bl	SYM(objc2::__macros::sel::CachedSel::fetch::GENERATED_ID, 0)
	mov	x8, x21
	mov	x21, x0
Lloh58:
	adrp	x9, __MergedGlobals@PAGE+24
Lloh59:
	ldr	x0, [x9, __MergedGlobals@PAGEOFF+24]
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
	mov	x22, x8
	bl	SYM(objc2::__macros::sel::CachedSel::fetch::GENERATED_ID, 0)
	mov	x8, x22
	b	LBB5_4
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

	.globl	_fn7_use_same_twice
	.p2align	2
_fn7_use_same_twice:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	adrp	x20, __MergedGlobals@PAGE+16
	ldr	x19, [x20, __MergedGlobals@PAGEOFF+16]
	cbz	x19, LBB6_3
	ldr	x0, [x20, __MergedGlobals@PAGEOFF+16]
	cbz	x0, LBB6_4
LBB6_2:
	stp	x19, x0, [x8]
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	ret
LBB6_3:
Lloh64:
	adrp	x0, __MergedGlobals@PAGE+16
Lloh65:
	add	x0, x0, __MergedGlobals@PAGEOFF+16
Lloh66:
	adrp	x1, l_anon.[ID].0@PAGE
Lloh67:
	add	x1, x1, l_anon.[ID].0@PAGEOFF
	mov	x19, x8
	bl	SYM(objc2::__macros::sel::CachedSel::fetch::GENERATED_ID, 0)
	mov	x8, x19
	mov	x19, x0
	ldr	x0, [x20, __MergedGlobals@PAGEOFF+16]
	cbnz	x0, LBB6_2
LBB6_4:
Lloh68:
	adrp	x0, __MergedGlobals@PAGE+16
Lloh69:
	add	x0, x0, __MergedGlobals@PAGEOFF+16
Lloh70:
	adrp	x1, l_anon.[ID].0@PAGE
Lloh71:
	add	x1, x1, l_anon.[ID].0@PAGEOFF
	mov	x20, x8
	bl	SYM(objc2::__macros::sel::CachedSel::fetch::GENERATED_ID, 0)
	mov	x8, x20
	b	LBB6_2
	.loh AdrpAdd	Lloh66, Lloh67
	.loh AdrpAdd	Lloh64, Lloh65
	.loh AdrpAdd	Lloh70, Lloh71
	.loh AdrpAdd	Lloh68, Lloh69

	.globl	_fn8_use_in_loop
	.p2align	2
_fn8_use_in_loop:
	cbz	x0, LBB7_6
	stp	x22, x21, [sp, #-48]!
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	adrp	x21, SYM(test_dynamic_sel[CRATE_ID]::use_in_loop::CACHED_SEL, 0)@PAGE
Lloh72:
	adrp	x19, SYM(test_dynamic_sel[CRATE_ID]::use_in_loop::CACHED_SEL, 0)@PAGE
Lloh73:
	add	x19, x19, SYM(test_dynamic_sel[CRATE_ID]::use_in_loop::CACHED_SEL, 0)@PAGEOFF
Lloh74:
	adrp	x20, l_anon.[ID].5@PAGE
Lloh75:
	add	x20, x20, l_anon.[ID].5@PAGEOFF
LBB7_2:
	ldr	x8, [x21, SYM(test_dynamic_sel[CRATE_ID]::use_in_loop::CACHED_SEL, 0)@PAGEOFF]
	cbz	x8, LBB7_4
	subs	x0, x0, #1
	b.ne	LBB7_2
	b	LBB7_5
LBB7_4:
	mov	x22, x0
	mov	x0, x19
	mov	x1, x20
	bl	SYM(objc2::__macros::sel::CachedSel::fetch::GENERATED_ID, 0)
	subs	x0, x22, #1
	b.ne	LBB7_2
LBB7_5:
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	ldp	x22, x21, [sp], #48
LBB7_6:
	ret
	.loh AdrpAdd	Lloh74, Lloh75
	.loh AdrpAdd	Lloh72, Lloh73

.zerofill __DATA,__bss,SYM(test_dynamic_sel[CRATE_ID]::unused_sel::CACHED_SEL, 0),8,3
.zerofill __DATA,__bss,SYM(test_dynamic_sel[CRATE_ID]::use_in_loop::CACHED_SEL, 0),8,3
	.section	__TEXT,__cstring,cstring_literals
l_anon.[ID].0:
	.asciz	"simple"

l_anon.[ID].1:
	.asciz	"alloc"

l_anon.[ID].2:
	.asciz	"i:am:different:"

l_anon.[ID].3:
	.asciz	"unused"

l_anon.[ID].4:
	.asciz	"fourthSel"

l_anon.[ID].5:
	.asciz	"loopedSelector"

.zerofill __DATA,__bss,__MergedGlobals,32,3
.subsections_via_symbols
