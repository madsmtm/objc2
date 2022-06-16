	.section	__TEXT,__text,regular,pure_instructions
	.globl	_get_sel
	.p2align	2
_get_sel:
Lloh0:
	adrp	x8, __MergedGlobals@PAGE
Lloh1:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh0, Lloh1

	.globl	_get_same_sel
	.p2align	2
_get_same_sel:
Lloh2:
	adrp	x8, __MergedGlobals@PAGE+8
Lloh3:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF+8]
	ret
	.loh AdrpLdr	Lloh2, Lloh3

	.globl	_get_common
	.p2align	2
_get_common:
Lloh4:
	adrp	x8, __ZN15test_static_sel10get_common5do_it3REF17hc0153c8b995ae069E@PAGE
Lloh5:
	ldr	x0, [x8, __ZN15test_static_sel10get_common5do_it3REF17hc0153c8b995ae069E@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh4, Lloh5

	.globl	_get_different_sel
	.p2align	2
_get_different_sel:
Lloh6:
	adrp	x8, __MergedGlobals@PAGE+16
Lloh7:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF+16]
	ret
	.loh AdrpLdr	Lloh6, Lloh7

	.globl	_unused_sel
	.p2align	2
_unused_sel:
Lloh8:
	adrp	x8, __ZN15test_static_sel10unused_sel5do_it3REF17h10d313b0038716f3E@PAGE
Lloh9:
	ldr	xzr, [x8, __ZN15test_static_sel10unused_sel5do_it3REF17h10d313b0038716f3E@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh8, Lloh9

	.globl	_use_fns
	.p2align	2
_use_fns:
Lloh10:
	adrp	x9, __MergedGlobals@PAGE
Lloh11:
	add	x9, x9, __MergedGlobals@PAGEOFF
	ldr	x10, [x9]
	ldr	x11, [x9, #8]
	ldr	x12, [x9, #16]
	ldr	x9, [x9, #24]
	stp	x10, x11, [x8]
	stp	x12, x9, [x8, #16]
	ret
	.loh AdrpAdd	Lloh10, Lloh11

	.globl	_use_same_twice
	.p2align	2
_use_same_twice:
	adrp	x9, __MergedGlobals@PAGE
	ldr	x10, [x9, __MergedGlobals@PAGEOFF]
	ldr	x9, [x9, __MergedGlobals@PAGEOFF]
	stp	x10, x9, [x8]
	ret

	.globl	_use_in_loop
	.p2align	2
_use_in_loop:
	cbz	x0, LBB7_3
	adrp	x8, __ZN15test_static_sel11use_in_loop5do_it3REF17hc74b5495e1d5fa3dE@PAGE
LBB7_2:
	ldr	xzr, [x8, __ZN15test_static_sel11use_in_loop5do_it3REF17hc74b5495e1d5fa3dE@PAGEOFF]
	subs	x0, x0, #1
	b.ne	LBB7_2
LBB7_3:
	ret

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel7get_sel5do_it5VALUE17hbe84d58afa6acbd3E:
	.asciz	"simple"

__ZN15test_static_sel12get_same_sel5do_it5VALUE17heb382f8971b53283E:
	.asciz	"simple"

__ZN15test_static_sel10get_common5do_it5VALUE17hd149c186017d657dE:
	.asciz	"alloc"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3
__ZN15test_static_sel10get_common5do_it3REF17hc0153c8b995ae069E:
	.quad	__ZN15test_static_sel10get_common5do_it5VALUE17hd149c186017d657dE

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel17get_different_sel5do_it5VALUE17hf787bc78eaf65504E:
	.asciz	"i:am:different:"

__ZN15test_static_sel10unused_sel5do_it5VALUE17h1b48bd5d3ca1c638E:
	.asciz	"unused"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3
__ZN15test_static_sel10unused_sel5do_it3REF17h10d313b0038716f3E:
	.quad	__ZN15test_static_sel10unused_sel5do_it5VALUE17h1b48bd5d3ca1c638E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel7use_fns5do_it5VALUE17hd8d355acf71c10e1E:
	.asciz	"fourthSel"

__ZN15test_static_sel11use_in_loop5do_it5VALUE17he5797d44142247f6E:
	.asciz	"loopedSelector"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3
__ZN15test_static_sel11use_in_loop5do_it3REF17hc74b5495e1d5fa3dE:
	.quad	__ZN15test_static_sel11use_in_loop5do_it5VALUE17he5797d44142247f6E

	.p2align	3
__MergedGlobals:
	.quad	__ZN15test_static_sel7get_sel5do_it5VALUE17hbe84d58afa6acbd3E
	.quad	__ZN15test_static_sel12get_same_sel5do_it5VALUE17heb382f8971b53283E
	.quad	__ZN15test_static_sel17get_different_sel5do_it5VALUE17hf787bc78eaf65504E
	.quad	__ZN15test_static_sel7use_fns5do_it5VALUE17hd8d355acf71c10e1E

.subsections_via_symbols
