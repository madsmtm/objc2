	.text
	.intel_syntax noprefix
	.section	.text.unlikely.SYM(objc2[CRATE_ID]::__macro_helpers::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0),"ax",@progbits
	.p2align	4, 0x90
	.type	SYM(objc2[CRATE_ID]::__macro_helpers::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0),@function
SYM(objc2[CRATE_ID]::__macro_helpers::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0):
	push	rbx
	mov	rbx, rsi
	call	qword ptr [rip + objc_retain@GOTPCREL]
	test	rax, rax
	je	.LBB0_1
	pop	rbx
	ret
.LBB0_1:
	lea	rdi, [rip + .Lanon.[ID].0]
	mov	esi, 56
	mov	rdx, rbx
	call	qword ptr [rip + SYM(core::option::expect_failed::GENERATED_ID, 0)@GOTPCREL]
	ud2
.Lfunc_end0:
	.size	SYM(objc2[CRATE_ID]::__macro_helpers::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0), .Lfunc_end0-SYM(objc2[CRATE_ID]::__macro_helpers::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0)

	.section	.text.unlikely.SYM(objc2[CRATE_ID]::runtime::message::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0),"ax",@progbits
	.p2align	4, 0x90
	.type	SYM(objc2[CRATE_ID]::runtime::message::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0),@function
SYM(objc2[CRATE_ID]::runtime::message::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0):
	push	rax
	call	qword ptr [rip + objc_retain@GOTPCREL]
	test	rax, rax
	je	.LBB1_1
	pop	rcx
	ret
.LBB1_1:
	lea	rdi, [rip + .Lanon.[ID].1]
	lea	rdx, [rip + .Lanon.[ID].3]
	mov	esi, 54
	call	qword ptr [rip + SYM(core::option::expect_failed::GENERATED_ID, 0)@GOTPCREL]
	ud2
.Lfunc_end1:
	.size	SYM(objc2[CRATE_ID]::runtime::message::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0), .Lfunc_end1-SYM(objc2[CRATE_ID]::runtime::message::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0)

	.section	.text.error_bool,"ax",@progbits
	.globl	error_bool
	.p2align	4, 0x90
	.type	error_bool,@function
error_bool:
	push	r15
	push	r14
	push	rbx
	sub	rsp, 16
	mov	ebx, edx
	mov	r14, rsi
	mov	r15, rdi
	mov	qword ptr [rsp + 8], 0
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	lea	rcx, [rsp + 8]
	mov	rdi, r15
	mov	rsi, r14
	mov	edx, ebx
	call	rax
	test	al, al
	je	.LBB2_2
	xor	eax, eax
.LBB2_3:
	add	rsp, 16
	pop	rbx
	pop	r14
	pop	r15
	ret
.LBB2_2:
	mov	rdi, qword ptr [rsp + 8]
	call	SYM(objc2[CRATE_ID]::runtime::message::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0)
	jmp	.LBB2_3
.Lfunc_end2:
	.size	error_bool, .Lfunc_end2-error_bool

	.section	.text.error_new,"ax",@progbits
	.globl	error_new
	.p2align	4, 0x90
	.type	error_new,@function
error_new:
	push	r14
	push	rbx
	push	rax
	mov	rbx, rsi
	mov	r14, rdi
	mov	qword ptr [rsp], 0
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rdx, rsp
	mov	rdi, r14
	mov	rsi, rbx
	call	rax
	test	rax, rax
	je	.LBB3_2
	mov	rdx, rax
	xor	eax, eax
	add	rsp, 8
	pop	rbx
	pop	r14
	ret
.LBB3_2:
	mov	rdi, qword ptr [rsp]
	lea	rsi, [rip + .Lanon.[ID].4]
	call	SYM(objc2[CRATE_ID]::__macro_helpers::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0)
	mov	rdx, rax
	mov	eax, 1
	add	rsp, 8
	pop	rbx
	pop	r14
	ret
.Lfunc_end3:
	.size	error_new, .Lfunc_end3-error_new

	.section	.text.error_alloc,"ax",@progbits
	.globl	error_alloc
	.p2align	4, 0x90
	.type	error_alloc,@function
error_alloc:
	push	r14
	push	rbx
	push	rax
	mov	rbx, rsi
	mov	r14, rdi
	mov	qword ptr [rsp], 0
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rdx, rsp
	mov	rdi, r14
	mov	rsi, rbx
	call	rax
	test	rax, rax
	je	.LBB4_2
	mov	rdx, rax
	xor	eax, eax
	add	rsp, 8
	pop	rbx
	pop	r14
	ret
.LBB4_2:
	mov	rdi, qword ptr [rsp]
	lea	rsi, [rip + .Lanon.[ID].5]
	call	SYM(objc2[CRATE_ID]::__macro_helpers::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0)
	mov	rdx, rax
	mov	eax, 1
	add	rsp, 8
	pop	rbx
	pop	r14
	ret
.Lfunc_end4:
	.size	error_alloc, .Lfunc_end4-error_alloc

	.section	.text.error_init,"ax",@progbits
	.globl	error_init
	.p2align	4, 0x90
	.type	error_init,@function
error_init:
	push	r14
	push	rbx
	push	rax
	mov	qword ptr [rsp], 0
	test	rdi, rdi
	je	.LBB5_1
	mov	rbx, rsi
	mov	r14, rdi
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rdx, rsp
	mov	rdi, r14
	mov	rsi, rbx
	call	rax
	test	rax, rax
	je	.LBB5_4
	mov	rdx, rax
	xor	eax, eax
	add	rsp, 8
	pop	rbx
	pop	r14
	ret
.LBB5_1:
	xor	edi, edi
	jmp	.LBB5_5
.LBB5_4:
	mov	rdi, qword ptr [rsp]
.LBB5_5:
	lea	rsi, [rip + .Lanon.[ID].6]
	call	SYM(objc2[CRATE_ID]::__macro_helpers::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0)
	mov	rdx, rax
	mov	eax, 1
	add	rsp, 8
	pop	rbx
	pop	r14
	ret
.Lfunc_end5:
	.size	error_init, .Lfunc_end5-error_init

	.section	.text.error_copy,"ax",@progbits
	.globl	error_copy
	.p2align	4, 0x90
	.type	error_copy,@function
error_copy:
	push	r14
	push	rbx
	push	rax
	mov	rbx, rsi
	mov	r14, rdi
	mov	qword ptr [rsp], 0
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rdx, rsp
	mov	rdi, r14
	mov	rsi, rbx
	call	rax
	test	rax, rax
	je	.LBB6_2
	mov	rdx, rax
	xor	eax, eax
	add	rsp, 8
	pop	rbx
	pop	r14
	ret
.LBB6_2:
	mov	rdi, qword ptr [rsp]
	lea	rsi, [rip + .Lanon.[ID].7]
	call	SYM(objc2[CRATE_ID]::__macro_helpers::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0)
	mov	rdx, rax
	mov	eax, 1
	add	rsp, 8
	pop	rbx
	pop	r14
	ret
.Lfunc_end6:
	.size	error_copy, .Lfunc_end6-error_copy

	.section	.text.error_autoreleased,"ax",@progbits
	.globl	error_autoreleased
	.p2align	4, 0x90
	.type	error_autoreleased,@function
error_autoreleased:
	push	r14
	push	rbx
	push	rax
	mov	rbx, rsi
	mov	r14, rdi
	mov	qword ptr [rsp], 0
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rdx, rsp
	mov	rdi, r14
	mov	rsi, rbx
	call	rax
	mov	rdi, rax
	call	qword ptr [rip + objc_retainAutoreleasedReturnValue@GOTPCREL]
	test	rax, rax
	je	.LBB7_2
	mov	rdx, rax
	xor	eax, eax
	add	rsp, 8
	pop	rbx
	pop	r14
	ret
.LBB7_2:
	mov	rdi, qword ptr [rsp]
	lea	rsi, [rip + .Lanon.[ID].8]
	call	SYM(objc2[CRATE_ID]::__macro_helpers::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0)
	mov	rdx, rax
	mov	eax, 1
	add	rsp, 8
	pop	rbx
	pop	r14
	ret
.Lfunc_end7:
	.size	error_autoreleased, .Lfunc_end7-error_autoreleased

	.type	.Lanon.[ID].0,@object
	.section	.rodata..Lanon.[ID].0,"a",@progbits
.Lanon.[ID].0:
	.ascii	"error parameter should be set if the method returns NULL"
	.size	.Lanon.[ID].0, 56

	.type	.Lanon.[ID].1,@object
	.section	.rodata..Lanon.[ID].1,"a",@progbits
.Lanon.[ID].1:
	.ascii	"error parameter should be set if the method returns NO"
	.size	.Lanon.[ID].1, 54

	.type	.Lanon.[ID].2,@object
	.section	.rodata..Lanon.[ID].2,"a",@progbits
.Lanon.[ID].2:
	.ascii	"crates/$DIR/lib.rs"
	.size	.Lanon.[ID].2, 54

	.type	.Lanon.[ID].3,@object
	.section	.data.rel.ro..Lanon.[ID].3,"aw",@progbits
	.p2align	3, 0x0
.Lanon.[ID].3:
	.quad	.Lanon.[ID].2
	.asciz	"6\000\000\000\000\000\000\000\n\000\000\000\005\000\000"
	.size	.Lanon.[ID].3, 24

	.type	.Lanon.[ID].4,@object
	.section	.data.rel.ro..Lanon.[ID].4,"aw",@progbits
	.p2align	3, 0x0
.Lanon.[ID].4:
	.quad	.Lanon.[ID].2
	.asciz	"6\000\000\000\000\000\000\000\017\000\000\000\005\000\000"
	.size	.Lanon.[ID].4, 24

	.type	.Lanon.[ID].5,@object
	.section	.data.rel.ro..Lanon.[ID].5,"aw",@progbits
	.p2align	3, 0x0
.Lanon.[ID].5:
	.quad	.Lanon.[ID].2
	.asciz	"6\000\000\000\000\000\000\000\024\000\000\000\005\000\000"
	.size	.Lanon.[ID].5, 24

	.type	.Lanon.[ID].6,@object
	.section	.data.rel.ro..Lanon.[ID].6,"aw",@progbits
	.p2align	3, 0x0
.Lanon.[ID].6:
	.quad	.Lanon.[ID].2
	.asciz	"6\000\000\000\000\000\000\000\031\000\000\000\005\000\000"
	.size	.Lanon.[ID].6, 24

	.type	.Lanon.[ID].7,@object
	.section	.data.rel.ro..Lanon.[ID].7,"aw",@progbits
	.p2align	3, 0x0
.Lanon.[ID].7:
	.quad	.Lanon.[ID].2
	.asciz	"6\000\000\000\000\000\000\000\036\000\000\000\005\000\000"
	.size	.Lanon.[ID].7, 24

	.type	.Lanon.[ID].8,@object
	.section	.data.rel.ro..Lanon.[ID].8,"aw",@progbits
	.p2align	3, 0x0
.Lanon.[ID].8:
	.quad	.Lanon.[ID].2
	.asciz	"6\000\000\000\000\000\000\000#\000\000\000\005\000\000"
	.size	.Lanon.[ID].8, 24

	.section	".note.GNU-stack","",@progbits
