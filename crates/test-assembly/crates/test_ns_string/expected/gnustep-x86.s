	.intel_syntax noprefix
	.section	.text.fn1_get_ascii,"ax",@progbits
	.globl	fn1_get_ascii
	.p2align	4
	.type	fn1_get_ascii,@function
fn1_get_ascii:
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
	lea	eax, [ebx + .Lanon.[ID].1@GOTOFF]
	push	3
	push	eax
	call	SYM(<objc2_foundation[CRATE_ID]::generated::__NSString::NSString>::from_str, 0)@PLT
	add	esp, 16
	mov	ecx, eax
	xchg	dword ptr [ebx + SYM(test_ns_string[CRATE_ID]::get_ascii::CACHED_NSSTRING, 0).0@GOTOFF], ecx
	add	esp, 8
	pop	ebx
	ret
.Lfunc_end0:
	.size	fn1_get_ascii, .Lfunc_end0-fn1_get_ascii

	.section	.text.fn2_get_utf16,"ax",@progbits
	.globl	fn2_get_utf16
	.p2align	4
	.type	fn2_get_utf16,@function
fn2_get_utf16:
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
	lea	eax, [ebx + .Lanon.[ID].2@GOTOFF]
	push	5
	push	eax
	call	SYM(<objc2_foundation[CRATE_ID]::generated::__NSString::NSString>::from_str, 0)@PLT
	add	esp, 16
	mov	ecx, eax
	xchg	dword ptr [ebx + SYM(test_ns_string[CRATE_ID]::get_utf16::CACHED_NSSTRING, 0).0@GOTOFF], ecx
	add	esp, 8
	pop	ebx
	ret
.Lfunc_end1:
	.size	fn2_get_utf16, .Lfunc_end1-fn2_get_utf16

	.section	.text.fn3_get_with_nul,"ax",@progbits
	.globl	fn3_get_with_nul
	.p2align	4
	.type	fn3_get_with_nul,@function
fn3_get_with_nul:
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
	lea	eax, [ebx + .Lanon.[ID].0@GOTOFF]
	push	6
	push	eax
	call	SYM(<objc2_foundation[CRATE_ID]::generated::__NSString::NSString>::from_str, 0)@PLT
	add	esp, 16
	mov	ecx, eax
	xchg	dword ptr [ebx + SYM(test_ns_string[CRATE_ID]::get_with_nul::CACHED_NSSTRING, 0).0@GOTOFF], ecx
	add	esp, 8
	pop	ebx
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
	.p2align	2, 0x0
SYM(test_ns_string[CRATE_ID]::get_with_nul::CACHED_NSSTRING, 0).0:
	.long	0
	.size	SYM(test_ns_string[CRATE_ID]::get_with_nul::CACHED_NSSTRING, 0).0, 4

	.type	SYM(test_ns_string[CRATE_ID]::get_ascii::CACHED_NSSTRING, 0).0,@object
	.section	.bss.SYM(test_ns_string[CRATE_ID]::get_ascii::CACHED_NSSTRING, 0).0,"aw",@nobits
	.p2align	2, 0x0
SYM(test_ns_string[CRATE_ID]::get_ascii::CACHED_NSSTRING, 0).0:
	.long	0
	.size	SYM(test_ns_string[CRATE_ID]::get_ascii::CACHED_NSSTRING, 0).0, 4

	.type	SYM(test_ns_string[CRATE_ID]::get_utf16::CACHED_NSSTRING, 0).0,@object
	.section	.bss.SYM(test_ns_string[CRATE_ID]::get_utf16::CACHED_NSSTRING, 0).0,"aw",@nobits
	.p2align	2, 0x0
SYM(test_ns_string[CRATE_ID]::get_utf16::CACHED_NSSTRING, 0).0:
	.long	0
	.size	SYM(test_ns_string[CRATE_ID]::get_utf16::CACHED_NSSTRING, 0).0, 4

	.section	".note.GNU-stack","",@progbits
