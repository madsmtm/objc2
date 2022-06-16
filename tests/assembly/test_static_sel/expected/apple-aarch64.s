	.section	__TEXT,__text,regular,pure_instructions
	.globl	_get_sel
	.p2align	2
_get_sel:
Lloh0:
	adrp	x0, __ZN15test_static_sel7get_sel5do_it5VALUE17hbe84d58afa6acbd3E@PAGE
Lloh1:
	add	x0, x0, __ZN15test_static_sel7get_sel5do_it5VALUE17hbe84d58afa6acbd3E@PAGEOFF
	ret
	.loh AdrpAdd	Lloh0, Lloh1

	.globl	_get_same_sel
	.p2align	2
_get_same_sel:
Lloh2:
	adrp	x0, __ZN15test_static_sel12get_same_sel5do_it5VALUE17heb382f8971b53283E@PAGE
Lloh3:
	add	x0, x0, __ZN15test_static_sel12get_same_sel5do_it5VALUE17heb382f8971b53283E@PAGEOFF
	ret
	.loh AdrpAdd	Lloh2, Lloh3

	.globl	_get_common
	.p2align	2
_get_common:
Lloh4:
	adrp	x0, __ZN15test_static_sel10get_common5do_it5VALUE17hd149c186017d657dE@PAGE
Lloh5:
	add	x0, x0, __ZN15test_static_sel10get_common5do_it5VALUE17hd149c186017d657dE@PAGEOFF
	ret
	.loh AdrpAdd	Lloh4, Lloh5

	.globl	_get_different_sel
	.p2align	2
_get_different_sel:
Lloh6:
	adrp	x0, __ZN15test_static_sel17get_different_sel5do_it5VALUE17hf787bc78eaf65504E@PAGE
Lloh7:
	add	x0, x0, __ZN15test_static_sel17get_different_sel5do_it5VALUE17hf787bc78eaf65504E@PAGEOFF
	ret
	.loh AdrpAdd	Lloh6, Lloh7

	.globl	_unused_sel
	.p2align	2
_unused_sel:
	ret

	.globl	_use_fns
	.p2align	2
_use_fns:
Lloh8:
	adrp	x9, __ZN15test_static_sel7get_sel5do_it5VALUE17hbe84d58afa6acbd3E@PAGE
Lloh9:
	add	x9, x9, __ZN15test_static_sel7get_sel5do_it5VALUE17hbe84d58afa6acbd3E@PAGEOFF
Lloh10:
	adrp	x10, __ZN15test_static_sel12get_same_sel5do_it5VALUE17heb382f8971b53283E@PAGE
Lloh11:
	add	x10, x10, __ZN15test_static_sel12get_same_sel5do_it5VALUE17heb382f8971b53283E@PAGEOFF
	stp	x9, x10, [x8]
Lloh12:
	adrp	x9, __ZN15test_static_sel17get_different_sel5do_it5VALUE17hf787bc78eaf65504E@PAGE
Lloh13:
	add	x9, x9, __ZN15test_static_sel17get_different_sel5do_it5VALUE17hf787bc78eaf65504E@PAGEOFF
Lloh14:
	adrp	x10, __ZN15test_static_sel7use_fns5do_it5VALUE17hd8d355acf71c10e1E@PAGE
Lloh15:
	add	x10, x10, __ZN15test_static_sel7use_fns5do_it5VALUE17hd8d355acf71c10e1E@PAGEOFF
	stp	x9, x10, [x8, #16]
	ret
	.loh AdrpAdd	Lloh14, Lloh15
	.loh AdrpAdd	Lloh12, Lloh13
	.loh AdrpAdd	Lloh10, Lloh11
	.loh AdrpAdd	Lloh8, Lloh9

	.globl	_use_same_twice
	.p2align	2
_use_same_twice:
Lloh16:
	adrp	x9, __ZN15test_static_sel7get_sel5do_it5VALUE17hbe84d58afa6acbd3E@PAGE
Lloh17:
	add	x9, x9, __ZN15test_static_sel7get_sel5do_it5VALUE17hbe84d58afa6acbd3E@PAGEOFF
	stp	x9, x9, [x8]
	ret
	.loh AdrpAdd	Lloh16, Lloh17

	.globl	_use_in_loop
	.p2align	2
_use_in_loop:
	ret

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel7get_sel5do_it5VALUE17hbe84d58afa6acbd3E:
	.asciz	"simple"

__ZN15test_static_sel12get_same_sel5do_it5VALUE17heb382f8971b53283E:
	.asciz	"simple"

__ZN15test_static_sel10get_common5do_it5VALUE17hd149c186017d657dE:
	.asciz	"alloc"

__ZN15test_static_sel17get_different_sel5do_it5VALUE17hf787bc78eaf65504E:
	.asciz	"i:am:different:"

__ZN15test_static_sel7use_fns5do_it5VALUE17hd8d355acf71c10e1E:
	.asciz	"fourthSel"

.subsections_via_symbols
