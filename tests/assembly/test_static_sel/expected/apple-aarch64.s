	.section	__TEXT,__text,regular,pure_instructions
	.globl	_get_sel
	.p2align	2
_get_sel:
	b	__ZN15test_static_sel7get_sel22objc_static_workaround17h13c8afaaf6336561E

	.globl	_get_same_sel
	.p2align	2
_get_same_sel:
	b	__ZN15test_static_sel12get_same_sel22objc_static_workaround17hcc291441d7e8a59fE

	.globl	_get_common
	.p2align	2
_get_common:
	b	__ZN15test_static_sel10get_common22objc_static_workaround17hce555de02adf8700E

	.globl	_get_different_sel
	.p2align	2
_get_different_sel:
	b	__ZN15test_static_sel17get_different_sel22objc_static_workaround17hfa539aa3b979256cE

	.globl	_unused_sel
	.p2align	2
_unused_sel:
	b	__ZN15test_static_sel10unused_sel22objc_static_workaround17h0a7026e7e912a1c5E

	.globl	_use_fns
	.p2align	2
_use_fns:
	stp	x22, x21, [sp, #-48]!
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	mov	x19, x8
	bl	__ZN15test_static_sel7get_sel22objc_static_workaround17h13c8afaaf6336561E
	mov	x20, x0
	bl	__ZN15test_static_sel12get_same_sel22objc_static_workaround17hcc291441d7e8a59fE
	mov	x21, x0
	bl	__ZN15test_static_sel17get_different_sel22objc_static_workaround17hfa539aa3b979256cE
	mov	x22, x0
	bl	__ZN15test_static_sel7use_fns22objc_static_workaround17h5f244955651662d0E
	stp	x20, x21, [x19]
	stp	x22, x0, [x19, #16]
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	ldp	x22, x21, [sp], #48
	ret

	.globl	_use_same_twice
	.p2align	2
_use_same_twice:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x19, x8
	bl	__ZN15test_static_sel7get_sel22objc_static_workaround17h13c8afaaf6336561E
	mov	x20, x0
	bl	__ZN15test_static_sel7get_sel22objc_static_workaround17h13c8afaaf6336561E
	stp	x20, x0, [x19]
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	ret

	.globl	_use_in_loop
	.p2align	2
_use_in_loop:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	cbz	x0, LBB7_3
	mov	x19, x0
LBB7_2:
	bl	__ZN15test_static_sel11use_in_loop22objc_static_workaround17h835e149ff2fe6f9bE
	subs	x19, x19, #1
	b.ne	LBB7_2
LBB7_3:
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	ret

	.p2align	2
__ZN15test_static_sel7get_sel22objc_static_workaround17h13c8afaaf6336561E:
Lloh0:
	adrp	x8, __ZN15test_static_sel7get_sel22objc_static_workaround3REF17hb6b95604312c057fE@PAGE
Lloh1:
	ldr	x0, [x8, __ZN15test_static_sel7get_sel22objc_static_workaround3REF17hb6b95604312c057fE@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh0, Lloh1

	.p2align	2
__ZN15test_static_sel12get_same_sel22objc_static_workaround17hcc291441d7e8a59fE:
Lloh2:
	adrp	x8, __ZN15test_static_sel12get_same_sel22objc_static_workaround3REF17hf2d20958868b8e47E@PAGE
Lloh3:
	ldr	x0, [x8, __ZN15test_static_sel12get_same_sel22objc_static_workaround3REF17hf2d20958868b8e47E@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh2, Lloh3

	.p2align	2
__ZN15test_static_sel10get_common22objc_static_workaround17hce555de02adf8700E:
Lloh4:
	adrp	x8, __ZN15test_static_sel10get_common22objc_static_workaround3REF17hbc6679a9cff6619cE@PAGE
Lloh5:
	ldr	x0, [x8, __ZN15test_static_sel10get_common22objc_static_workaround3REF17hbc6679a9cff6619cE@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh4, Lloh5

	.p2align	2
__ZN15test_static_sel17get_different_sel22objc_static_workaround17hfa539aa3b979256cE:
Lloh6:
	adrp	x8, __ZN15test_static_sel17get_different_sel22objc_static_workaround3REF17hdb6933336b563c79E@PAGE
Lloh7:
	ldr	x0, [x8, __ZN15test_static_sel17get_different_sel22objc_static_workaround3REF17hdb6933336b563c79E@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh6, Lloh7

	.p2align	2
__ZN15test_static_sel10unused_sel22objc_static_workaround17h0a7026e7e912a1c5E:
Lloh8:
	adrp	x8, __ZN15test_static_sel10unused_sel22objc_static_workaround3REF17h40777f2046a98be5E@PAGE
Lloh9:
	ldr	xzr, [x8, __ZN15test_static_sel10unused_sel22objc_static_workaround3REF17h40777f2046a98be5E@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh8, Lloh9

	.p2align	2
__ZN15test_static_sel7use_fns22objc_static_workaround17h5f244955651662d0E:
Lloh10:
	adrp	x8, __ZN15test_static_sel7use_fns22objc_static_workaround3REF17hdaf81138d104b376E@PAGE
Lloh11:
	ldr	x0, [x8, __ZN15test_static_sel7use_fns22objc_static_workaround3REF17hdaf81138d104b376E@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh10, Lloh11

	.p2align	2
__ZN15test_static_sel11use_in_loop22objc_static_workaround17h835e149ff2fe6f9bE:
Lloh12:
	adrp	x8, __ZN15test_static_sel11use_in_loop22objc_static_workaround3REF17h31ccfb6b27d8c440E@PAGE
Lloh13:
	ldr	xzr, [x8, __ZN15test_static_sel11use_in_loop22objc_static_workaround3REF17h31ccfb6b27d8c440E@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh12, Lloh13

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel7get_sel22objc_static_workaround4NAME17hdc8765740ecc3d11E:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	3
__ZN15test_static_sel7get_sel22objc_static_workaround3REF17hb6b95604312c057fE:
	.quad	__ZN15test_static_sel7get_sel22objc_static_workaround4NAME17hdc8765740ecc3d11E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel12get_same_sel22objc_static_workaround4NAME17hf316d81d46a5ef66E:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	3
__ZN15test_static_sel12get_same_sel22objc_static_workaround3REF17hf2d20958868b8e47E:
	.quad	__ZN15test_static_sel12get_same_sel22objc_static_workaround4NAME17hf316d81d46a5ef66E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel10get_common22objc_static_workaround4NAME17h455f306cf0f80e51E:
	.asciz	"alloc"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	3
__ZN15test_static_sel10get_common22objc_static_workaround3REF17hbc6679a9cff6619cE:
	.quad	__ZN15test_static_sel10get_common22objc_static_workaround4NAME17h455f306cf0f80e51E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel17get_different_sel22objc_static_workaround4NAME17hbce34864cd8eb8ffE:
	.asciz	"i:am:different:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	3
__ZN15test_static_sel17get_different_sel22objc_static_workaround3REF17hdb6933336b563c79E:
	.quad	__ZN15test_static_sel17get_different_sel22objc_static_workaround4NAME17hbce34864cd8eb8ffE

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel10unused_sel22objc_static_workaround4NAME17hfc0e0c35212550d0E:
	.asciz	"unused"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	3
__ZN15test_static_sel10unused_sel22objc_static_workaround3REF17h40777f2046a98be5E:
	.quad	__ZN15test_static_sel10unused_sel22objc_static_workaround4NAME17hfc0e0c35212550d0E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel7use_fns22objc_static_workaround4NAME17h33067d740a4050feE:
	.asciz	"fourthSel"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	3
__ZN15test_static_sel7use_fns22objc_static_workaround3REF17hdaf81138d104b376E:
	.quad	__ZN15test_static_sel7use_fns22objc_static_workaround4NAME17h33067d740a4050feE

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel11use_in_loop22objc_static_workaround4NAME17h7946000586832262E:
	.asciz	"loopedSelector"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	3
__ZN15test_static_sel11use_in_loop22objc_static_workaround3REF17h31ccfb6b27d8c440E:
	.quad	__ZN15test_static_sel11use_in_loop22objc_static_workaround4NAME17h7946000586832262E

.subsections_via_symbols
