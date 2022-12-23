	.text
	.intel_syntax noprefix
	.section	.text.get_ascii,"ax",@progbits
	.globl	get_ascii
	.p2align	4, 0x90
	.type	get_ascii,@function
get_ascii:
	push	rax
	mov	rax, qword ptr [rip + SYM(test_ns_string[CRATE_ID]::get_ascii::CACHED_NSSTRING, 0).0]
	test	rax, rax
	je	.LBB0_1
	pop	rcx
	ret
.LBB0_1:
	lea	rdi, [rip + .Lanon.[ID].0]
	mov	esi, 3
	call	qword ptr [rip + SYM(icrate::Foundation::additions::string::NSString::from_str::GENERATED_ID, 0)@GOTPCREL]
	mov	rcx, rax
	xchg	qword ptr [rip + SYM(test_ns_string[CRATE_ID]::get_ascii::CACHED_NSSTRING, 0).0], rcx
	pop	rcx
	ret
.Lfunc_end0:
	.size	get_ascii, .Lfunc_end0-get_ascii

	.section	.text.get_utf16,"ax",@progbits
	.globl	get_utf16
	.p2align	4, 0x90
	.type	get_utf16,@function
get_utf16:
	push	rax
	mov	rax, qword ptr [rip + SYM(test_ns_string[CRATE_ID]::get_utf16::CACHED_NSSTRING, 0).0]
	test	rax, rax
	je	.LBB1_1
	pop	rcx
	ret
.LBB1_1:
	lea	rdi, [rip + .Lanon.[ID].1]
	mov	esi, 5
	call	qword ptr [rip + SYM(icrate::Foundation::additions::string::NSString::from_str::GENERATED_ID, 0)@GOTPCREL]
	mov	rcx, rax
	xchg	qword ptr [rip + SYM(test_ns_string[CRATE_ID]::get_utf16::CACHED_NSSTRING, 0).0], rcx
	pop	rcx
	ret
.Lfunc_end1:
	.size	get_utf16, .Lfunc_end1-get_utf16

	.section	.text.get_with_nul,"ax",@progbits
	.globl	get_with_nul
	.p2align	4, 0x90
	.type	get_with_nul,@function
get_with_nul:
	push	rax
	mov	rax, qword ptr [rip + SYM(test_ns_string[CRATE_ID]::get_with_nul::CACHED_NSSTRING, 0).0]
	test	rax, rax
	je	.LBB2_1
	pop	rcx
	ret
.LBB2_1:
	lea	rdi, [rip + .Lanon.[ID].2]
	mov	esi, 6
	call	qword ptr [rip + SYM(icrate::Foundation::additions::string::NSString::from_str::GENERATED_ID, 0)@GOTPCREL]
	mov	rcx, rax
	xchg	qword ptr [rip + SYM(test_ns_string[CRATE_ID]::get_with_nul::CACHED_NSSTRING, 0).0], rcx
	pop	rcx
	ret
.Lfunc_end2:
	.size	get_with_nul, .Lfunc_end2-get_with_nul

	.type	.Lanon.[ID].0,@object
	.section	.rodata..Lanon.[ID].0,"a",@progbits
.Lanon.[ID].0:
	.ascii	"abc"
	.size	.Lanon.[ID].0, 3

	.type	.Lanon.[ID].1,@object
	.section	.rodata..Lanon.[ID].1,"a",@progbits
.Lanon.[ID].1:
	.ascii	"\303\241b\304\207"
	.size	.Lanon.[ID].1, 5

	.type	.Lanon.[ID].2,@object
	.section	.rodata..Lanon.[ID].2,"a",@progbits
.Lanon.[ID].2:
	.asciz	"a\000b\000c"
	.size	.Lanon.[ID].2, 6

	.type	SYM(test_ns_string[CRATE_ID]::get_ascii::CACHED_NSSTRING, 0).0,@object
	.section	.bss.SYM(test_ns_string[CRATE_ID]::get_ascii::CACHED_NSSTRING, 0).0,"aw",@nobits
	.p2align	3
SYM(test_ns_string[CRATE_ID]::get_ascii::CACHED_NSSTRING, 0).0:
	.quad	0
	.size	SYM(test_ns_string[CRATE_ID]::get_ascii::CACHED_NSSTRING, 0).0, 8

	.type	SYM(test_ns_string[CRATE_ID]::get_utf16::CACHED_NSSTRING, 0).0,@object
	.section	.bss.SYM(test_ns_string[CRATE_ID]::get_utf16::CACHED_NSSTRING, 0).0,"aw",@nobits
	.p2align	3
SYM(test_ns_string[CRATE_ID]::get_utf16::CACHED_NSSTRING, 0).0:
	.quad	0
	.size	SYM(test_ns_string[CRATE_ID]::get_utf16::CACHED_NSSTRING, 0).0, 8

	.type	SYM(test_ns_string[CRATE_ID]::get_with_nul::CACHED_NSSTRING, 0).0,@object
	.section	.bss.SYM(test_ns_string[CRATE_ID]::get_with_nul::CACHED_NSSTRING, 0).0,"aw",@nobits
	.p2align	3
SYM(test_ns_string[CRATE_ID]::get_with_nul::CACHED_NSSTRING, 0).0:
	.quad	0
	.size	SYM(test_ns_string[CRATE_ID]::get_with_nul::CACHED_NSSTRING, 0).0, 8

	.section	".note.GNU-stack","",@progbits
