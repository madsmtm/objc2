	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_get_sel
	.p2align	2
	.code	32
_get_sel:
	movw	r0, :lower16:(__ZN15test_static_sel7get_sel5do_it5VALUE17hd5cef0cedc2e7124E-(LPC0_0+8))
	movt	r0, :upper16:(__ZN15test_static_sel7get_sel5do_it5VALUE17hd5cef0cedc2e7124E-(LPC0_0+8))
LPC0_0:
	add	r0, pc, r0
	bx	lr

	.globl	_get_same_sel
	.p2align	2
	.code	32
_get_same_sel:
	movw	r0, :lower16:(__ZN15test_static_sel12get_same_sel5do_it5VALUE17hd4d7a88675d577ceE-(LPC1_0+8))
	movt	r0, :upper16:(__ZN15test_static_sel12get_same_sel5do_it5VALUE17hd4d7a88675d577ceE-(LPC1_0+8))
LPC1_0:
	add	r0, pc, r0
	bx	lr

	.globl	_get_common
	.p2align	2
	.code	32
_get_common:
	movw	r0, :lower16:(__ZN15test_static_sel10get_common5do_it5VALUE17h8f318ccc63cbbef4E-(LPC2_0+8))
	movt	r0, :upper16:(__ZN15test_static_sel10get_common5do_it5VALUE17h8f318ccc63cbbef4E-(LPC2_0+8))
LPC2_0:
	add	r0, pc, r0
	bx	lr

	.globl	_get_different_sel
	.p2align	2
	.code	32
_get_different_sel:
	movw	r0, :lower16:(__ZN15test_static_sel17get_different_sel5do_it5VALUE17h00902267a93f7400E-(LPC3_0+8))
	movt	r0, :upper16:(__ZN15test_static_sel17get_different_sel5do_it5VALUE17h00902267a93f7400E-(LPC3_0+8))
LPC3_0:
	add	r0, pc, r0
	bx	lr

	.globl	_unused_sel
	.p2align	2
	.code	32
_unused_sel:
	bx	lr

	.globl	_use_fns
	.p2align	2
	.code	32
_use_fns:
	movw	r9, :lower16:(__ZN15test_static_sel7use_fns5do_it5VALUE17h145936052e081cd8E-(LPC5_0+8))
	movt	r9, :upper16:(__ZN15test_static_sel7use_fns5do_it5VALUE17h145936052e081cd8E-(LPC5_0+8))
	movw	r2, :lower16:(__ZN15test_static_sel17get_different_sel5do_it5VALUE17h00902267a93f7400E-(LPC5_1+8))
	movt	r2, :upper16:(__ZN15test_static_sel17get_different_sel5do_it5VALUE17h00902267a93f7400E-(LPC5_1+8))
	movw	r3, :lower16:(__ZN15test_static_sel7get_sel5do_it5VALUE17hd5cef0cedc2e7124E-(LPC5_2+8))
	movt	r3, :upper16:(__ZN15test_static_sel7get_sel5do_it5VALUE17hd5cef0cedc2e7124E-(LPC5_2+8))
	movw	r1, :lower16:(__ZN15test_static_sel12get_same_sel5do_it5VALUE17hd4d7a88675d577ceE-(LPC5_3+8))
	movt	r1, :upper16:(__ZN15test_static_sel12get_same_sel5do_it5VALUE17hd4d7a88675d577ceE-(LPC5_3+8))
LPC5_2:
	add	r3, pc, r3
LPC5_3:
	add	r1, pc, r1
LPC5_0:
	add	r9, pc, r9
LPC5_1:
	add	r2, pc, r2
	str	r3, [r0]
	stmib	r0, {r1, r2, r9}
	bx	lr

	.globl	_use_same_twice
	.p2align	2
	.code	32
_use_same_twice:
	movw	r1, :lower16:(__ZN15test_static_sel7get_sel5do_it5VALUE17hd5cef0cedc2e7124E-(LPC6_0+8))
	movt	r1, :upper16:(__ZN15test_static_sel7get_sel5do_it5VALUE17hd5cef0cedc2e7124E-(LPC6_0+8))
LPC6_0:
	add	r1, pc, r1
	str	r1, [r0]
	str	r1, [r0, #4]
	bx	lr

	.globl	_use_in_loop
	.p2align	2
	.code	32
_use_in_loop:
	bx	lr

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel7get_sel5do_it5VALUE17hd5cef0cedc2e7124E:
	.asciz	"simple"

__ZN15test_static_sel12get_same_sel5do_it5VALUE17hd4d7a88675d577ceE:
	.asciz	"simple"

__ZN15test_static_sel10get_common5do_it5VALUE17h8f318ccc63cbbef4E:
	.asciz	"alloc"

__ZN15test_static_sel17get_different_sel5do_it5VALUE17h00902267a93f7400E:
	.asciz	"i:am:different:"

__ZN15test_static_sel7use_fns5do_it5VALUE17h145936052e081cd8E:
	.asciz	"fourthSel"

.subsections_via_symbols
