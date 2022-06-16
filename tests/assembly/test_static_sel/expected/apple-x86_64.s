	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_get_sel
	.p2align	4, 0x90
_get_sel:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + __ZN15test_static_sel7get_sel5do_it3REF17h3382eb6577a19884E]
	pop	rbp
	ret

	.globl	_get_same_sel
	.p2align	4, 0x90
_get_same_sel:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + __ZN15test_static_sel12get_same_sel5do_it3REF17hc31d191dc2fd749eE]
	pop	rbp
	ret

	.globl	_get_common
	.p2align	4, 0x90
_get_common:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + __ZN15test_static_sel10get_common5do_it3REF17hdadb6ccb121002e4E]
	pop	rbp
	ret

	.globl	_get_different_sel
	.p2align	4, 0x90
_get_different_sel:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + __ZN15test_static_sel17get_different_sel5do_it3REF17he5023edc2d5cf13dE]
	pop	rbp
	ret

	.globl	_unused_sel
	.p2align	4, 0x90
_unused_sel:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + __ZN15test_static_sel10unused_sel5do_it3REF17h24fb2858983de1caE]
	pop	rbp
	ret

	.globl	_use_fns
	.p2align	4, 0x90
_use_fns:
	push	rbp
	mov	rbp, rsp
	mov	rax, rdi
	mov	rcx, qword ptr [rip + __ZN15test_static_sel7get_sel5do_it3REF17h3382eb6577a19884E]
	mov	rdx, qword ptr [rip + __ZN15test_static_sel12get_same_sel5do_it3REF17hc31d191dc2fd749eE]
	mov	rsi, qword ptr [rip + __ZN15test_static_sel17get_different_sel5do_it3REF17he5023edc2d5cf13dE]
	mov	rdi, qword ptr [rip + __ZN15test_static_sel7use_fns5do_it3REF17h57c91cfc799dcb29E]
	mov	qword ptr [rax], rcx
	mov	qword ptr [rax + 8], rdx
	mov	qword ptr [rax + 16], rsi
	mov	qword ptr [rax + 24], rdi
	pop	rbp
	ret

	.globl	_use_same_twice
	.p2align	4, 0x90
_use_same_twice:
	push	rbp
	mov	rbp, rsp
	mov	rax, rdi
	mov	rcx, qword ptr [rip + __ZN15test_static_sel7get_sel5do_it3REF17h3382eb6577a19884E]
	mov	rdx, qword ptr [rip + __ZN15test_static_sel7get_sel5do_it3REF17h3382eb6577a19884E]
	mov	qword ptr [rdi], rcx
	mov	qword ptr [rdi + 8], rdx
	pop	rbp
	ret

	.globl	_use_in_loop
	.p2align	4, 0x90
_use_in_loop:
	push	rbp
	mov	rbp, rsp
	test	rdi, rdi
	je	LBB7_6
	lea	rcx, [rdi - 1]
	mov	eax, edi
	and	eax, 7
	cmp	rcx, 7
	jb	LBB7_4
	and	rdi, -8
	.p2align	4, 0x90
LBB7_3:
	mov	rcx, qword ptr [rip + __ZN15test_static_sel11use_in_loop5do_it3REF17hbcc9fc4e6297cac5E]
	mov	rcx, qword ptr [rip + __ZN15test_static_sel11use_in_loop5do_it3REF17hbcc9fc4e6297cac5E]
	mov	rcx, qword ptr [rip + __ZN15test_static_sel11use_in_loop5do_it3REF17hbcc9fc4e6297cac5E]
	mov	rcx, qword ptr [rip + __ZN15test_static_sel11use_in_loop5do_it3REF17hbcc9fc4e6297cac5E]
	mov	rcx, qword ptr [rip + __ZN15test_static_sel11use_in_loop5do_it3REF17hbcc9fc4e6297cac5E]
	mov	rcx, qword ptr [rip + __ZN15test_static_sel11use_in_loop5do_it3REF17hbcc9fc4e6297cac5E]
	mov	rcx, qword ptr [rip + __ZN15test_static_sel11use_in_loop5do_it3REF17hbcc9fc4e6297cac5E]
	mov	rcx, qword ptr [rip + __ZN15test_static_sel11use_in_loop5do_it3REF17hbcc9fc4e6297cac5E]
	add	rdi, -8
	jne	LBB7_3
LBB7_4:
	test	rax, rax
	je	LBB7_6
	.p2align	4, 0x90
LBB7_5:
	mov	rcx, qword ptr [rip + __ZN15test_static_sel11use_in_loop5do_it3REF17hbcc9fc4e6297cac5E]
	dec	rax
	jne	LBB7_5
LBB7_6:
	pop	rbp
	ret

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel7get_sel5do_it5VALUE17h338b867de999a683E:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3
__ZN15test_static_sel7get_sel5do_it3REF17h3382eb6577a19884E:
	.quad	__ZN15test_static_sel7get_sel5do_it5VALUE17h338b867de999a683E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel12get_same_sel5do_it5VALUE17hfda78dbf2d31d676E:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3
__ZN15test_static_sel12get_same_sel5do_it3REF17hc31d191dc2fd749eE:
	.quad	__ZN15test_static_sel12get_same_sel5do_it5VALUE17hfda78dbf2d31d676E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel10get_common5do_it5VALUE17h6913f03b163cd283E:
	.asciz	"alloc"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3
__ZN15test_static_sel10get_common5do_it3REF17hdadb6ccb121002e4E:
	.quad	__ZN15test_static_sel10get_common5do_it5VALUE17h6913f03b163cd283E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel17get_different_sel5do_it5VALUE17h88a6ccd34e9782e8E:
	.asciz	"i:am:different:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3
__ZN15test_static_sel17get_different_sel5do_it3REF17he5023edc2d5cf13dE:
	.quad	__ZN15test_static_sel17get_different_sel5do_it5VALUE17h88a6ccd34e9782e8E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel10unused_sel5do_it5VALUE17hef3d043ebac21292E:
	.asciz	"unused"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3
__ZN15test_static_sel10unused_sel5do_it3REF17h24fb2858983de1caE:
	.quad	__ZN15test_static_sel10unused_sel5do_it5VALUE17hef3d043ebac21292E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel7use_fns5do_it5VALUE17h3053c06c1c5263daE:
	.asciz	"fourthSel"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3
__ZN15test_static_sel7use_fns5do_it3REF17h57c91cfc799dcb29E:
	.quad	__ZN15test_static_sel7use_fns5do_it5VALUE17h3053c06c1c5263daE

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel11use_in_loop5do_it5VALUE17h24465c3f4d5d6e1eE:
	.asciz	"loopedSelector"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3
__ZN15test_static_sel11use_in_loop5do_it3REF17hbcc9fc4e6297cac5E:
	.quad	__ZN15test_static_sel11use_in_loop5do_it5VALUE17h24465c3f4d5d6e1eE

.subsections_via_symbols
