	.text
	.intel_syntax noprefix
	.section	.text.get_ascii,"ax",@progbits
	.globl	get_ascii
	.p2align	4, 0x90
	.type	get_ascii,@function
get_ascii:
	push	ebx
	sub	esp, 8
	call	.L0$pb
.L0$pb:
	pop	ebx
.Ltmp0:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp0-.L0$pb)
	mov	eax, dword ptr [ebx + SYM(test_ns_string[CRATE_ID]::get_ascii::CACHED_NSSTRING, 0).0@GOTOFF]
	test	eax, eax
	je	.LBB0_1
	add	esp, 8
	pop	ebx
	ret
.LBB0_1:
	sub	esp, 8
	lea	eax, [ebx + .Lanon.[ID].0@GOTOFF]
	push	3
	push	eax
	call	SYM(icrate::Foundation::additions::string::<impl icrate::Foundation::generated::__NSString::NSString>::from_str::GENERATED_ID, 0)@PLT
	add	esp, 16
	mov	ecx, eax
	xchg	dword ptr [ebx + SYM(test_ns_string[CRATE_ID]::get_ascii::CACHED_NSSTRING, 0).0@GOTOFF], ecx
	add	esp, 8
	pop	ebx
	ret
.Lfunc_end0:
	.size	get_ascii, .Lfunc_end0-get_ascii

	.section	.text.get_utf16,"ax",@progbits
	.globl	get_utf16
	.p2align	4, 0x90
	.type	get_utf16,@function
get_utf16:
	push	ebx
	sub	esp, 8
	call	.L1$pb
.L1$pb:
	pop	ebx
.Ltmp1:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp1-.L1$pb)
	mov	eax, dword ptr [ebx + SYM(test_ns_string[CRATE_ID]::get_utf16::CACHED_NSSTRING, 0).0@GOTOFF]
	test	eax, eax
	je	.LBB1_1
	add	esp, 8
	pop	ebx
	ret
.LBB1_1:
	sub	esp, 8
	lea	eax, [ebx + .Lanon.[ID].1@GOTOFF]
	push	5
	push	eax
	call	SYM(icrate::Foundation::additions::string::<impl icrate::Foundation::generated::__NSString::NSString>::from_str::GENERATED_ID, 0)@PLT
	add	esp, 16
	mov	ecx, eax
	xchg	dword ptr [ebx + SYM(test_ns_string[CRATE_ID]::get_utf16::CACHED_NSSTRING, 0).0@GOTOFF], ecx
	add	esp, 8
	pop	ebx
	ret
.Lfunc_end1:
	.size	get_utf16, .Lfunc_end1-get_utf16

	.section	.text.get_with_nul,"ax",@progbits
	.globl	get_with_nul
	.p2align	4, 0x90
	.type	get_with_nul,@function
get_with_nul:
	push	ebx
	sub	esp, 8
	call	.L2$pb
.L2$pb:
	pop	ebx
.Ltmp2:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp2-.L2$pb)
	mov	eax, dword ptr [ebx + SYM(test_ns_string[CRATE_ID]::get_with_nul::CACHED_NSSTRING, 0).0@GOTOFF]
	test	eax, eax
	je	.LBB2_1
	add	esp, 8
	pop	ebx
	ret
.LBB2_1:
	sub	esp, 8
	lea	eax, [ebx + .Lanon.[ID].2@GOTOFF]
	push	6
	push	eax
	call	SYM(icrate::Foundation::additions::string::<impl icrate::Foundation::generated::__NSString::NSString>::from_str::GENERATED_ID, 0)@PLT
	add	esp, 16
	mov	ecx, eax
	xchg	dword ptr [ebx + SYM(test_ns_string[CRATE_ID]::get_with_nul::CACHED_NSSTRING, 0).0@GOTOFF], ecx
	add	esp, 8
	pop	ebx
	ret
.Lfunc_end2:
	.size	get_with_nul, .Lfunc_end2-get_with_nul

	.type	SYM(test_ns_string[CRATE_ID]::get_ascii::CACHED_NSSTRING, 0).0,@object
	.section	.bss.SYM(test_ns_string[CRATE_ID]::get_ascii::CACHED_NSSTRING, 0).0,"aw",@nobits
	.p2align	2, 0x0
SYM(test_ns_string[CRATE_ID]::get_ascii::CACHED_NSSTRING, 0).0:
	.long	0
	.size	SYM(test_ns_string[CRATE_ID]::get_ascii::CACHED_NSSTRING, 0).0, 4

	.type	.Lanon.[ID].0,@object
	.section	.rodata..Lanon.[ID].0,"a",@progbits
.Lanon.[ID].0:
	.ascii	"abc"
	.size	.Lanon.[ID].0, 3

	.type	SYM(test_ns_string[CRATE_ID]::get_utf16::CACHED_NSSTRING, 0).0,@object
	.section	.bss.SYM(test_ns_string[CRATE_ID]::get_utf16::CACHED_NSSTRING, 0).0,"aw",@nobits
	.p2align	2, 0x0
SYM(test_ns_string[CRATE_ID]::get_utf16::CACHED_NSSTRING, 0).0:
	.long	0
	.size	SYM(test_ns_string[CRATE_ID]::get_utf16::CACHED_NSSTRING, 0).0, 4

	.type	.Lanon.[ID].1,@object
	.section	.rodata..Lanon.[ID].1,"a",@progbits
.Lanon.[ID].1:
	.ascii	"\303\241b\304\207"
	.size	.Lanon.[ID].1, 5

	.type	SYM(test_ns_string[CRATE_ID]::get_with_nul::CACHED_NSSTRING, 0).0,@object
	.section	.bss.SYM(test_ns_string[CRATE_ID]::get_with_nul::CACHED_NSSTRING, 0).0,"aw",@nobits
	.p2align	2, 0x0
SYM(test_ns_string[CRATE_ID]::get_with_nul::CACHED_NSSTRING, 0).0:
	.long	0
	.size	SYM(test_ns_string[CRATE_ID]::get_with_nul::CACHED_NSSTRING, 0).0, 4

	.type	.Lanon.[ID].2,@object
	.section	.rodata..Lanon.[ID].2,"a",@progbits
.Lanon.[ID].2:
	.asciz	"a\000b\000c"
	.size	.Lanon.[ID].2, 6

	.section	".note.GNU-stack","",@progbits
