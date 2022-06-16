	.section	__TEXT,__text,regular,pure_instructions
	.globl	_get_sel
	.p2align	2
_get_sel:
	b	__ZN15test_static_sel7get_sel22objc_static_workaround17hd01b8e03bc7ee2f2E

	.globl	_get_same_sel
	.p2align	2
_get_same_sel:
	b	__ZN15test_static_sel12get_same_sel22objc_static_workaround17h6fded10733d3d13eE

	.globl	_get_common
	.p2align	2
_get_common:
	b	__ZN15test_static_sel10get_common22objc_static_workaround17h3099714314719ae4E

	.globl	_get_different_sel
	.p2align	2
_get_different_sel:
	b	__ZN15test_static_sel17get_different_sel22objc_static_workaround17h2df9cd4849db8a71E

	.globl	_unused_sel
	.p2align	2
_unused_sel:
	b	__ZN15test_static_sel10unused_sel22objc_static_workaround17h751ade9083325dbfE

	.globl	_use_fns
	.p2align	2
_use_fns:
	stp	x22, x21, [sp, #-48]!
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	mov	x19, x8
	bl	__ZN15test_static_sel7get_sel22objc_static_workaround17hd01b8e03bc7ee2f2E
	mov	x20, x0
	bl	__ZN15test_static_sel12get_same_sel22objc_static_workaround17h6fded10733d3d13eE
	mov	x21, x0
	bl	__ZN15test_static_sel17get_different_sel22objc_static_workaround17h2df9cd4849db8a71E
	mov	x22, x0
	bl	__ZN15test_static_sel7use_fns22objc_static_workaround17h0a4a70767a5df0c7E
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
	bl	__ZN15test_static_sel7get_sel22objc_static_workaround17hd01b8e03bc7ee2f2E
	mov	x20, x0
	bl	__ZN15test_static_sel7get_sel22objc_static_workaround17hd01b8e03bc7ee2f2E
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
	bl	__ZN15test_static_sel11use_in_loop22objc_static_workaround17hd571f39365fc8d75E
	subs	x19, x19, #1
	b.ne	LBB7_2
LBB7_3:
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	ret

	.p2align	2
__ZN15test_static_sel7get_sel22objc_static_workaround17hd01b8e03bc7ee2f2E:
Lloh0:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_68381ba894e318e9@PAGE
Lloh1:
	ldr	x0, [x8, L_OBJC_SELECTOR_REFERENCES_68381ba894e318e9@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh0, Lloh1

	.p2align	2
__ZN15test_static_sel12get_same_sel22objc_static_workaround17h6fded10733d3d13eE:
Lloh2:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_cd2fd6e7d2adcc35@PAGE
Lloh3:
	ldr	x0, [x8, L_OBJC_SELECTOR_REFERENCES_cd2fd6e7d2adcc35@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh2, Lloh3

	.p2align	2
__ZN15test_static_sel10get_common22objc_static_workaround17h3099714314719ae4E:
Lloh4:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_34d6c3ed70e85964@PAGE
Lloh5:
	ldr	x0, [x8, L_OBJC_SELECTOR_REFERENCES_34d6c3ed70e85964@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh4, Lloh5

	.p2align	2
__ZN15test_static_sel17get_different_sel22objc_static_workaround17h2df9cd4849db8a71E:
Lloh6:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_ab5e106a55f71e5b@PAGE
Lloh7:
	ldr	x0, [x8, L_OBJC_SELECTOR_REFERENCES_ab5e106a55f71e5b@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh6, Lloh7

	.p2align	2
__ZN15test_static_sel10unused_sel22objc_static_workaround17h751ade9083325dbfE:
Lloh8:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_6f2d5ee51a69c477@PAGE
Lloh9:
	ldr	xzr, [x8, L_OBJC_SELECTOR_REFERENCES_6f2d5ee51a69c477@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh8, Lloh9

	.p2align	2
__ZN15test_static_sel7use_fns22objc_static_workaround17h0a4a70767a5df0c7E:
Lloh10:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_1d27e854714b8860@PAGE
Lloh11:
	ldr	x0, [x8, L_OBJC_SELECTOR_REFERENCES_1d27e854714b8860@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh10, Lloh11

	.p2align	2
__ZN15test_static_sel11use_in_loop22objc_static_workaround17hd571f39365fc8d75E:
Lloh12:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_e56637a4c1a15b07@PAGE
Lloh13:
	ldr	xzr, [x8, L_OBJC_SELECTOR_REFERENCES_e56637a4c1a15b07@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh12, Lloh13

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_68381ba894e318e9
L_OBJC_METH_VAR_NAME_68381ba894e318e9:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_68381ba894e318e9
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_68381ba894e318e9:
	.quad	L_OBJC_METH_VAR_NAME_68381ba894e318e9

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_cd2fd6e7d2adcc35
L_OBJC_METH_VAR_NAME_cd2fd6e7d2adcc35:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_cd2fd6e7d2adcc35
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_cd2fd6e7d2adcc35:
	.quad	L_OBJC_METH_VAR_NAME_cd2fd6e7d2adcc35

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_34d6c3ed70e85964
L_OBJC_METH_VAR_NAME_34d6c3ed70e85964:
	.asciz	"alloc"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_34d6c3ed70e85964
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_34d6c3ed70e85964:
	.quad	L_OBJC_METH_VAR_NAME_34d6c3ed70e85964

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_ab5e106a55f71e5b
L_OBJC_METH_VAR_NAME_ab5e106a55f71e5b:
	.asciz	"i:am:different:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_ab5e106a55f71e5b
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_ab5e106a55f71e5b:
	.quad	L_OBJC_METH_VAR_NAME_ab5e106a55f71e5b

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_6f2d5ee51a69c477
L_OBJC_METH_VAR_NAME_6f2d5ee51a69c477:
	.asciz	"unused"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_6f2d5ee51a69c477
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_6f2d5ee51a69c477:
	.quad	L_OBJC_METH_VAR_NAME_6f2d5ee51a69c477

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_1d27e854714b8860
L_OBJC_METH_VAR_NAME_1d27e854714b8860:
	.asciz	"fourthSel"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_1d27e854714b8860
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_1d27e854714b8860:
	.quad	L_OBJC_METH_VAR_NAME_1d27e854714b8860

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_e56637a4c1a15b07
L_OBJC_METH_VAR_NAME_e56637a4c1a15b07:
	.asciz	"loopedSelector"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_e56637a4c1a15b07
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_e56637a4c1a15b07:
	.quad	L_OBJC_METH_VAR_NAME_e56637a4c1a15b07

.subsections_via_symbols
