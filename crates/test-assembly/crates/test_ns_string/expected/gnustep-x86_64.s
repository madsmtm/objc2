	.intel_syntax noprefix
	.section	.text.fn1_get_ascii,"ax",@progbits
	.globl	fn1_get_ascii
	.p2align	4
	.type	fn1_get_ascii,@function
fn1_get_ascii:
	mov	rax, qword ptr [rip + SYM(test_ns_string[CRATE_ID]::get_ascii::CACHED_NSSTRING, 0).0]
	test	rax, rax
	je	.LBB0_1
	ret
.LBB0_1:
	push	rax
	lea	rdi, [rip + .Lanon.[ID].1]
	mov	esi, 3
	call	qword ptr [rip + SYM(<objc2_foundation[CRATE_ID]::generated::__NSString::NSString>::from_str, 0)@GOTPCREL]
	mov	rcx, rax
	xchg	qword ptr [rip + SYM(test_ns_string[CRATE_ID]::get_ascii::CACHED_NSSTRING, 0).0], rcx
	add	rsp, 8
	ret
.Lfunc_end0:
	.size	fn1_get_ascii, .Lfunc_end0-fn1_get_ascii

	.section	.text.fn2_get_utf16,"ax",@progbits
	.globl	fn2_get_utf16
	.p2align	4
	.type	fn2_get_utf16,@function
fn2_get_utf16:
	mov	rax, qword ptr [rip + SYM(test_ns_string[CRATE_ID]::get_utf16::CACHED_NSSTRING, 0).0]
	test	rax, rax
	je	.LBB1_1
	ret
.LBB1_1:
	push	rax
	lea	rdi, [rip + .Lanon.[ID].2]
	mov	esi, 5
	call	qword ptr [rip + SYM(<objc2_foundation[CRATE_ID]::generated::__NSString::NSString>::from_str, 0)@GOTPCREL]
	mov	rcx, rax
	xchg	qword ptr [rip + SYM(test_ns_string[CRATE_ID]::get_utf16::CACHED_NSSTRING, 0).0], rcx
	add	rsp, 8
	ret
.Lfunc_end1:
	.size	fn2_get_utf16, .Lfunc_end1-fn2_get_utf16

	.section	.text.fn3_get_with_nul,"ax",@progbits
	.globl	fn3_get_with_nul
	.p2align	4
	.type	fn3_get_with_nul,@function
fn3_get_with_nul:
	mov	rax, qword ptr [rip + SYM(test_ns_string[CRATE_ID]::get_with_nul::CACHED_NSSTRING, 0).0]
	test	rax, rax
	je	.LBB2_1
	ret
.LBB2_1:
	push	rax
	lea	rdi, [rip + .Lanon.[ID].0]
	mov	esi, 6
	call	qword ptr [rip + SYM(<objc2_foundation[CRATE_ID]::generated::__NSString::NSString>::from_str, 0)@GOTPCREL]
	mov	rcx, rax
	xchg	qword ptr [rip + SYM(test_ns_string[CRATE_ID]::get_with_nul::CACHED_NSSTRING, 0).0], rcx
	add	rsp, 8
	ret
.Lfunc_end2:
	.size	fn3_get_with_nul, .Lfunc_end2-fn3_get_with_nul

	.type	.Lanon.[ID].0,@object
	.section	.rodata..Lanon.[ID].0,"a",@progbits
.Lanon.[ID].0:
	.asciz	"a\000b\000c"
	.size	.Lanon.[ID].0, 6

	.type	.Lanon.[ID].1,@object
	.section	.rodata..Lanon.[ID].1,"a",@progbits
.Lanon.[ID].1:
	.ascii	"abc"
	.size	.Lanon.[ID].1, 3

	.type	.Lanon.[ID].2,@object
	.section	.rodata..Lanon.[ID].2,"a",@progbits
.Lanon.[ID].2:
	.ascii	"\303\241b\304\207"
	.size	.Lanon.[ID].2, 5

	.type	SYM(test_ns_string[CRATE_ID]::get_with_nul::CACHED_NSSTRING, 0).0,@object
	.section	.bss.SYM(test_ns_string[CRATE_ID]::get_with_nul::CACHED_NSSTRING, 0).0,"aw",@nobits
	.p2align	3, 0x0
SYM(test_ns_string[CRATE_ID]::get_with_nul::CACHED_NSSTRING, 0).0:
	.quad	0
	.size	SYM(test_ns_string[CRATE_ID]::get_with_nul::CACHED_NSSTRING, 0).0, 8

	.type	SYM(test_ns_string[CRATE_ID]::get_ascii::CACHED_NSSTRING, 0).0,@object
	.section	.bss.SYM(test_ns_string[CRATE_ID]::get_ascii::CACHED_NSSTRING, 0).0,"aw",@nobits
	.p2align	3, 0x0
SYM(test_ns_string[CRATE_ID]::get_ascii::CACHED_NSSTRING, 0).0:
	.quad	0
	.size	SYM(test_ns_string[CRATE_ID]::get_ascii::CACHED_NSSTRING, 0).0, 8

	.type	SYM(test_ns_string[CRATE_ID]::get_utf16::CACHED_NSSTRING, 0).0,@object
	.section	.bss.SYM(test_ns_string[CRATE_ID]::get_utf16::CACHED_NSSTRING, 0).0,"aw",@nobits
	.p2align	3, 0x0
SYM(test_ns_string[CRATE_ID]::get_utf16::CACHED_NSSTRING, 0).0:
	.quad	0
	.size	SYM(test_ns_string[CRATE_ID]::get_utf16::CACHED_NSSTRING, 0).0, 8

	.section	".note.GNU-stack","",@progbits
