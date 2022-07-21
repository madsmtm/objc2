	.text
	.intel_syntax noprefix
	.section	.text.get_ascii,"ax",@progbits
	.globl	get_ascii
	.p2align	4, 0x90
	.type	get_ascii,@function
get_ascii:
	call	.L0$pb
.L0$pb:
	pop	eax
.Ltmp0:
	add	eax, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp0-.L0$pb)
	lea	eax, [eax + SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 0)@GOTOFF]
	ret
.Lfunc_end0:
	.size	get_ascii, .Lfunc_end0-get_ascii

	.section	.text.get_utf16,"ax",@progbits
	.globl	get_utf16
	.p2align	4, 0x90
	.type	get_utf16,@function
get_utf16:
	call	.L1$pb
.L1$pb:
	pop	eax
.Ltmp1:
	add	eax, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp1-.L1$pb)
	lea	eax, [eax + SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 0)@GOTOFF]
	ret
.Lfunc_end1:
	.size	get_utf16, .Lfunc_end1-get_utf16

	.section	.text.get_with_nul,"ax",@progbits
	.globl	get_with_nul
	.p2align	4, 0x90
	.type	get_with_nul,@function
get_with_nul:
	call	.L2$pb
.L2$pb:
	pop	eax
.Ltmp2:
	add	eax, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp2-.L2$pb)
	lea	eax, [eax + SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 0)@GOTOFF]
	ret
.Lfunc_end2:
	.size	get_with_nul, .Lfunc_end2-get_with_nul

	.type	EMPTY,@object
	.section	.data.rel.ro.EMPTY,"aw",@progbits
	.globl	EMPTY
	.p2align	2
EMPTY:
	.long	SYM(test_ns_string[CRATE_ID]::EMPTY::CFSTRING, 0)
	.size	EMPTY, 4

	.type	XYZ,@object
	.section	.data.rel.ro.XYZ,"aw",@progbits
	.globl	XYZ
	.p2align	2
XYZ:
	.long	SYM(test_ns_string[CRATE_ID]::XYZ::CFSTRING, 0)
	.size	XYZ, 4

	.type	SYM(test_ns_string[CRATE_ID]::EMPTY::ASCII, 0),@object
	.section	"__TEXT,__cstring,cstring_literals","a",@progbits
SYM(test_ns_string[CRATE_ID]::EMPTY::ASCII, 0):
	.zero	1
	.size	SYM(test_ns_string[CRATE_ID]::EMPTY::ASCII, 0), 1

	.type	SYM(test_ns_string[CRATE_ID]::EMPTY::CFSTRING, 0),@object
	.section	"__DATA,__cfstring","aw",@progbits
	.globl	SYM(test_ns_string[CRATE_ID]::EMPTY::CFSTRING, 0)
	.p2align	2
SYM(test_ns_string[CRATE_ID]::EMPTY::CFSTRING, 0):
	.long	__CFConstantStringClassReference
	.asciz	"\310\007\000"
	.long	SYM(test_ns_string[CRATE_ID]::EMPTY::ASCII, 0)
	.zero	4
	.size	SYM(test_ns_string[CRATE_ID]::EMPTY::CFSTRING, 0), 16

	.type	SYM(test_ns_string[CRATE_ID]::XYZ::ASCII, 0),@object
	.section	"__TEXT,__cstring,cstring_literals","a",@progbits
SYM(test_ns_string[CRATE_ID]::XYZ::ASCII, 0):
	.asciz	"xyz"
	.size	SYM(test_ns_string[CRATE_ID]::XYZ::ASCII, 0), 4

	.type	SYM(test_ns_string[CRATE_ID]::XYZ::CFSTRING, 0),@object
	.section	"__DATA,__cfstring","aw",@progbits
	.globl	SYM(test_ns_string[CRATE_ID]::XYZ::CFSTRING, 0)
	.p2align	2
SYM(test_ns_string[CRATE_ID]::XYZ::CFSTRING, 0):
	.long	__CFConstantStringClassReference
	.asciz	"\310\007\000"
	.long	SYM(test_ns_string[CRATE_ID]::XYZ::ASCII, 0)
	.asciz	"\003\000\000"
	.size	SYM(test_ns_string[CRATE_ID]::XYZ::CFSTRING, 0), 16

	.type	SYM(test_ns_string[CRATE_ID]::get_ascii::ASCII, 0),@object
	.section	"__TEXT,__cstring,cstring_literals","a",@progbits
SYM(test_ns_string[CRATE_ID]::get_ascii::ASCII, 0):
	.asciz	"abc"
	.size	SYM(test_ns_string[CRATE_ID]::get_ascii::ASCII, 0), 4

	.type	SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 0),@object
	.section	"__DATA,__cfstring","aw",@progbits
	.p2align	2
SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 0):
	.long	__CFConstantStringClassReference
	.asciz	"\310\007\000"
	.long	SYM(test_ns_string[CRATE_ID]::get_ascii::ASCII, 0)
	.asciz	"\003\000\000"
	.size	SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 0), 16

	.type	SYM(test_ns_string[CRATE_ID]::get_utf16::UTF16, 0),@object
	.section	"__TEXT,__ustring","a",@progbits
	.p2align	1
SYM(test_ns_string[CRATE_ID]::get_utf16::UTF16, 0):
	.asciz	"\341\000b\000\007\001\000"
	.size	SYM(test_ns_string[CRATE_ID]::get_utf16::UTF16, 0), 8

	.type	SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 0),@object
	.section	"__DATA,__cfstring","aw",@progbits
	.p2align	2
SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 0):
	.long	__CFConstantStringClassReference
	.asciz	"\320\007\000"
	.long	SYM(test_ns_string[CRATE_ID]::get_utf16::UTF16, 0)
	.asciz	"\003\000\000"
	.size	SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 0), 16

	.type	SYM(test_ns_string[CRATE_ID]::get_with_nul::UTF16, 0),@object
	.section	"__TEXT,__ustring","a",@progbits
	.p2align	1
SYM(test_ns_string[CRATE_ID]::get_with_nul::UTF16, 0):
	.asciz	"a\000\000\000b\000\000\000c\000\000\000\000"
	.size	SYM(test_ns_string[CRATE_ID]::get_with_nul::UTF16, 0), 14

	.type	SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 0),@object
	.section	"__DATA,__cfstring","aw",@progbits
	.p2align	2
SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 0):
	.long	__CFConstantStringClassReference
	.asciz	"\320\007\000"
	.long	SYM(test_ns_string[CRATE_ID]::get_with_nul::UTF16, 0)
	.asciz	"\006\000\000"
	.size	SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 0), 16

	.section	".note.GNU-stack","",@progbits
