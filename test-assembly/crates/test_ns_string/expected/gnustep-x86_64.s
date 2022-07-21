	.text
	.intel_syntax noprefix
	.section	.text.get_ascii,"ax",@progbits
	.globl	get_ascii
	.p2align	4, 0x90
	.type	get_ascii,@function
get_ascii:
	push	rax
	lea	rdi, [rip + .L__unnamed_1]
	mov	esi, 3
	call	qword ptr [rip + SYM(objc2_foundation::__string_macro::is_ascii::GENERATED_ID, 0)@GOTPCREL]
	test	al, al
	je	.LBB0_1
	lea	rdi, [rip + SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 0)]
	pop	rax
	jmp	qword ptr [rip + SYM(objc2_foundation::__string_macro::CFStringAscii::as_nsstring::GENERATED_ID, 0)@GOTPCREL]
.LBB0_1:
	lea	rdi, [rip + SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 1)]
	pop	rax
	jmp	qword ptr [rip + SYM(objc2_foundation::__string_macro::CFStringUtf16::as_nsstring::GENERATED_ID, 0)@GOTPCREL]
.Lfunc_end0:
	.size	get_ascii, .Lfunc_end0-get_ascii

	.section	.text.get_utf16,"ax",@progbits
	.globl	get_utf16
	.p2align	4, 0x90
	.type	get_utf16,@function
get_utf16:
	push	rax
	lea	rdi, [rip + .L__unnamed_2]
	mov	esi, 5
	call	qword ptr [rip + SYM(objc2_foundation::__string_macro::is_ascii::GENERATED_ID, 0)@GOTPCREL]
	test	al, al
	je	.LBB1_1
	lea	rdi, [rip + SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 0)]
	pop	rax
	jmp	qword ptr [rip + SYM(objc2_foundation::__string_macro::CFStringAscii::as_nsstring::GENERATED_ID, 0)@GOTPCREL]
.LBB1_1:
	lea	rdi, [rip + SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 1)]
	pop	rax
	jmp	qword ptr [rip + SYM(objc2_foundation::__string_macro::CFStringUtf16::as_nsstring::GENERATED_ID, 0)@GOTPCREL]
.Lfunc_end1:
	.size	get_utf16, .Lfunc_end1-get_utf16

	.section	.text.get_with_nul,"ax",@progbits
	.globl	get_with_nul
	.p2align	4, 0x90
	.type	get_with_nul,@function
get_with_nul:
	push	rax
	lea	rdi, [rip + .L__unnamed_3]
	mov	esi, 6
	call	qword ptr [rip + SYM(objc2_foundation::__string_macro::is_ascii::GENERATED_ID, 0)@GOTPCREL]
	test	al, al
	je	.LBB2_1
	lea	rdi, [rip + SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 0)]
	pop	rax
	jmp	qword ptr [rip + SYM(objc2_foundation::__string_macro::CFStringAscii::as_nsstring::GENERATED_ID, 0)@GOTPCREL]
.LBB2_1:
	lea	rdi, [rip + SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 1)]
	pop	rax
	jmp	qword ptr [rip + SYM(objc2_foundation::__string_macro::CFStringUtf16::as_nsstring::GENERATED_ID, 0)@GOTPCREL]
.Lfunc_end2:
	.size	get_with_nul, .Lfunc_end2-get_with_nul

	.type	EMPTY,@object
	.section	.data.rel.ro.EMPTY,"aw",@progbits
	.globl	EMPTY
	.p2align	3
EMPTY:
	.quad	SYM(test_ns_string[CRATE_ID]::EMPTY::CFSTRING, 0)
	.size	EMPTY, 8

	.type	XYZ,@object
	.section	.data.rel.ro.XYZ,"aw",@progbits
	.globl	XYZ
	.p2align	3
XYZ:
	.quad	SYM(test_ns_string[CRATE_ID]::XYZ::CFSTRING, 0)
	.size	XYZ, 8

	.type	.L__unnamed_1,@object
	.section	.rodata..L__unnamed_1,"a",@progbits
.L__unnamed_1:
	.ascii	"abc"
	.size	.L__unnamed_1, 3

	.type	.L__unnamed_2,@object
	.section	.rodata..L__unnamed_2,"a",@progbits
.L__unnamed_2:
	.ascii	"\303\241b\304\207"
	.size	.L__unnamed_2, 5

	.type	.L__unnamed_3,@object
	.section	.rodata..L__unnamed_3,"a",@progbits
.L__unnamed_3:
	.asciz	"a\000b\000c"
	.size	.L__unnamed_3, 6

	.type	.L__unnamed_4,@object
	.section	.rodata..L__unnamed_4,"a",@progbits
.L__unnamed_4:
	.zero	1
	.size	.L__unnamed_4, 1

	.type	SYM(test_ns_string[CRATE_ID]::EMPTY::CFSTRING, 0),@object
	.section	"__DATA,__cfstring,regular","aw",@progbits
	.globl	SYM(test_ns_string[CRATE_ID]::EMPTY::CFSTRING, 0)
	.p2align	3
SYM(test_ns_string[CRATE_ID]::EMPTY::CFSTRING, 0):
	.quad	__CFConstantStringClassReference
	.asciz	"\310\007\000\000\000\000\000"
	.quad	.L__unnamed_4
	.zero	8
	.size	SYM(test_ns_string[CRATE_ID]::EMPTY::CFSTRING, 0), 32

	.type	.L__unnamed_5,@object
	.section	.rodata..L__unnamed_5,"a",@progbits
	.p2align	1
.L__unnamed_5:
	.zero	2
	.size	.L__unnamed_5, 2

	.type	SYM(test_ns_string[CRATE_ID]::EMPTY::CFSTRING, 1),@object
	.section	"__DATA,__cfstring,regular","aw",@progbits
	.globl	SYM(test_ns_string[CRATE_ID]::EMPTY::CFSTRING, 1)
	.p2align	3
SYM(test_ns_string[CRATE_ID]::EMPTY::CFSTRING, 1):
	.quad	__CFConstantStringClassReference
	.asciz	"\320\007\000\000\000\000\000"
	.quad	.L__unnamed_5
	.zero	8
	.size	SYM(test_ns_string[CRATE_ID]::EMPTY::CFSTRING, 1), 32

	.type	.L__unnamed_6,@object
	.section	.rodata.cst4,"aM",@progbits,4
.L__unnamed_6:
	.asciz	"xyz"
	.size	.L__unnamed_6, 4

	.type	SYM(test_ns_string[CRATE_ID]::XYZ::CFSTRING, 0),@object
	.section	"__DATA,__cfstring,regular","aw",@progbits
	.globl	SYM(test_ns_string[CRATE_ID]::XYZ::CFSTRING, 0)
	.p2align	3
SYM(test_ns_string[CRATE_ID]::XYZ::CFSTRING, 0):
	.quad	__CFConstantStringClassReference
	.asciz	"\310\007\000\000\000\000\000"
	.quad	.L__unnamed_6
	.asciz	"\003\000\000\000\000\000\000"
	.size	SYM(test_ns_string[CRATE_ID]::XYZ::CFSTRING, 0), 32

	.type	.L__unnamed_7,@object
	.section	.rodata.cst8,"aM",@progbits,8
	.p2align	1
.L__unnamed_7:
	.asciz	"x\000y\000z\000\000"
	.size	.L__unnamed_7, 8

	.type	SYM(test_ns_string[CRATE_ID]::XYZ::CFSTRING, 1),@object
	.section	"__DATA,__cfstring,regular","aw",@progbits
	.globl	SYM(test_ns_string[CRATE_ID]::XYZ::CFSTRING, 1)
	.p2align	3
SYM(test_ns_string[CRATE_ID]::XYZ::CFSTRING, 1):
	.quad	__CFConstantStringClassReference
	.asciz	"\320\007\000\000\000\000\000"
	.quad	.L__unnamed_7
	.asciz	"\003\000\000\000\000\000\000"
	.size	SYM(test_ns_string[CRATE_ID]::XYZ::CFSTRING, 1), 32

	.type	.L__unnamed_8,@object
	.section	.rodata.cst4,"aM",@progbits,4
.L__unnamed_8:
	.asciz	"abc"
	.size	.L__unnamed_8, 4

	.type	SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 0),@object
	.section	"__DATA,__cfstring,regular","aw",@progbits
	.p2align	3
SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 0):
	.quad	__CFConstantStringClassReference
	.asciz	"\310\007\000\000\000\000\000"
	.quad	.L__unnamed_8
	.asciz	"\003\000\000\000\000\000\000"
	.size	SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 0), 32

	.type	.L__unnamed_9,@object
	.section	.rodata.cst8,"aM",@progbits,8
	.p2align	1
.L__unnamed_9:
	.asciz	"a\000b\000c\000\000"
	.size	.L__unnamed_9, 8

	.type	SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 1),@object
	.section	"__DATA,__cfstring,regular","aw",@progbits
	.p2align	3
SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 1):
	.quad	__CFConstantStringClassReference
	.asciz	"\320\007\000\000\000\000\000"
	.quad	.L__unnamed_9
	.asciz	"\003\000\000\000\000\000\000"
	.size	SYM(test_ns_string[CRATE_ID]::get_ascii::CFSTRING, 1), 32

	.type	.L__unnamed_10,@object
	.section	.rodata..L__unnamed_10,"a",@progbits
.L__unnamed_10:
	.asciz	"\303\241b\304\207"
	.size	.L__unnamed_10, 6

	.type	SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 0),@object
	.section	"__DATA,__cfstring,regular","aw",@progbits
	.p2align	3
SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 0):
	.quad	__CFConstantStringClassReference
	.asciz	"\310\007\000\000\000\000\000"
	.quad	.L__unnamed_10
	.asciz	"\005\000\000\000\000\000\000"
	.size	SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 0), 32

	.type	.L__unnamed_11,@object
	.section	.rodata.cst8,"aM",@progbits,8
	.p2align	1
.L__unnamed_11:
	.asciz	"\341\000b\000\007\001\000"
	.size	.L__unnamed_11, 8

	.type	SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 1),@object
	.section	"__DATA,__cfstring,regular","aw",@progbits
	.p2align	3
SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 1):
	.quad	__CFConstantStringClassReference
	.asciz	"\320\007\000\000\000\000\000"
	.quad	.L__unnamed_11
	.asciz	"\003\000\000\000\000\000\000"
	.size	SYM(test_ns_string[CRATE_ID]::get_utf16::CFSTRING, 1), 32

	.type	.L__unnamed_12,@object
	.section	.rodata..L__unnamed_12,"a",@progbits
.L__unnamed_12:
	.asciz	"a\000b\000c\000"
	.size	.L__unnamed_12, 7

	.type	SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 0),@object
	.section	"__DATA,__cfstring,regular","aw",@progbits
	.p2align	3
SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 0):
	.quad	__CFConstantStringClassReference
	.asciz	"\310\007\000\000\000\000\000"
	.quad	.L__unnamed_12
	.asciz	"\006\000\000\000\000\000\000"
	.size	SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 0), 32

	.type	.L__unnamed_13,@object
	.section	.rodata..L__unnamed_13,"a",@progbits
	.p2align	1
.L__unnamed_13:
	.asciz	"a\000\000\000b\000\000\000c\000\000\000\000"
	.size	.L__unnamed_13, 14

	.type	SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 1),@object
	.section	"__DATA,__cfstring,regular","aw",@progbits
	.p2align	3
SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 1):
	.quad	__CFConstantStringClassReference
	.asciz	"\320\007\000\000\000\000\000"
	.quad	.L__unnamed_13
	.asciz	"\006\000\000\000\000\000\000"
	.size	SYM(test_ns_string[CRATE_ID]::get_with_nul::CFSTRING, 1), 32

	.section	".note.GNU-stack","",@progbits
