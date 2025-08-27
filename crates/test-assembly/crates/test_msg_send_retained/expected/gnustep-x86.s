	.intel_syntax noprefix
	.section	.text.fn01_handle_new,"ax",@progbits
	.globl	fn01_handle_new
	.p2align	4
	.type	fn01_handle_new,@function
fn01_handle_new:
	push	ebx
	push	edi
	push	esi
	mov	esi, dword ptr [esp + 16]
	mov	edi, dword ptr [esp + 20]
	call	.L0$pb
.L0$pb:
	pop	ebx
.Ltmp0:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp0-.L0$pb)
	sub	esp, 8
	push	edi
	push	esi
	call	objc_msg_lookup@PLT
	add	esp, 8
	push	edi
	push	esi
	call	eax
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.Lfunc_end0:
	.size	fn01_handle_new, .Lfunc_end0-fn01_handle_new

	.section	.text.fn02_handle_new_fallible,"ax",@progbits
	.globl	fn02_handle_new_fallible
	.p2align	4
	.type	fn02_handle_new_fallible,@function
fn02_handle_new_fallible:
	push	ebx
	push	edi
	push	esi
	mov	edi, dword ptr [esp + 20]
	mov	esi, dword ptr [esp + 16]
	call	.L1$pb
.L1$pb:
	pop	ebx
.Ltmp1:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp1-.L1$pb)
	sub	esp, 8
	push	edi
	push	esi
	call	objc_msg_lookup@PLT
	add	esp, 8
	push	edi
	push	esi
	call	eax
	add	esp, 16
	test	eax, eax
	je	.LBB1_2
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB1_2:
	sub	esp, 4
	lea	eax, [ebx + .Lanon.[ID].1@GOTOFF]
	push	eax
	push	edi
	push	esi
	call	SYM(objc2::__macros::retain_semantics::new_fail::GENERATED_ID, 0)@PLT
.Lfunc_end1:
	.size	fn02_handle_new_fallible, .Lfunc_end1-fn02_handle_new_fallible

	.section	.text.fn03_handle_alloc,"ax",@progbits
	.globl	fn03_handle_alloc
	.p2align	4
	.type	fn03_handle_alloc,@function
fn03_handle_alloc:
	push	ebx
	push	edi
	push	esi
	mov	esi, dword ptr [esp + 16]
	mov	edi, dword ptr [esp + 20]
	call	.L2$pb
.L2$pb:
	pop	ebx
.Ltmp2:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp2-.L2$pb)
	sub	esp, 8
	push	edi
	push	esi
	call	objc_msg_lookup@PLT
	add	esp, 8
	push	edi
	push	esi
	call	eax
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.Lfunc_end2:
	.size	fn03_handle_alloc, .Lfunc_end2-fn03_handle_alloc

	.section	.text.fn04_handle_init,"ax",@progbits
	.globl	fn04_handle_init
	.p2align	4
	.type	fn04_handle_init,@function
fn04_handle_init:
	push	ebx
	push	edi
	push	esi
	mov	esi, dword ptr [esp + 16]
	call	.L3$pb
.L3$pb:
	pop	ebx
.Ltmp3:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp3-.L3$pb)
	test	esi, esi
	je	.LBB3_2
	mov	edi, dword ptr [esp + 20]
	sub	esp, 8
	push	edi
	push	esi
	call	objc_msg_lookup@PLT
	add	esp, 8
	push	edi
	push	esi
	call	eax
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB3_2:
	xor	eax, eax
	pop	esi
	pop	edi
	pop	ebx
	ret
.Lfunc_end3:
	.size	fn04_handle_init, .Lfunc_end3-fn04_handle_init

	.section	.text.fn05_handle_init_fallible,"ax",@progbits
	.globl	fn05_handle_init_fallible
	.p2align	4
	.type	fn05_handle_init_fallible,@function
fn05_handle_init_fallible:
	push	ebx
	push	edi
	push	esi
	mov	esi, dword ptr [esp + 16]
	mov	edi, dword ptr [esp + 20]
	call	.L4$pb
.L4$pb:
	pop	ebx
.Ltmp4:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp4-.L4$pb)
	test	esi, esi
	je	.LBB4_3
	sub	esp, 8
	push	edi
	push	esi
	call	objc_msg_lookup@PLT
	add	esp, 8
	push	edi
	push	esi
	call	eax
	add	esp, 16
	test	eax, eax
	je	.LBB4_3
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB4_3:
	sub	esp, 4
	lea	eax, [ebx + .Lanon.[ID].2@GOTOFF]
	push	eax
	push	edi
	push	esi
	call	SYM(objc2::__macros::retain_semantics::init_fail::GENERATED_ID, 0)@PLT
.Lfunc_end4:
	.size	fn05_handle_init_fallible, .Lfunc_end4-fn05_handle_init_fallible

	.section	.text.fn06_handle_alloc_init,"ax",@progbits
	.globl	fn06_handle_alloc_init
	.p2align	4
	.type	fn06_handle_alloc_init,@function
fn06_handle_alloc_init:
	push	ebx
	push	edi
	push	esi
	mov	esi, dword ptr [esp + 16]
	mov	edi, dword ptr [esp + 20]
	call	.L5$pb
.L5$pb:
	pop	ebx
.Ltmp5:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp5-.L5$pb)
	sub	esp, 8
	push	edi
	push	esi
	call	objc_msg_lookup@PLT
	add	esp, 8
	push	edi
	push	esi
	call	eax
	add	esp, 16
	test	eax, eax
	je	.LBB5_2
	mov	esi, dword ptr [esp + 24]
	sub	esp, 8
	push	esi
	push	eax
	mov	edi, eax
	call	objc_msg_lookup@PLT
	add	esp, 8
	push	esi
	push	edi
	call	eax
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB5_2:
	xor	eax, eax
	pop	esi
	pop	edi
	pop	ebx
	ret
.Lfunc_end5:
	.size	fn06_handle_alloc_init, .Lfunc_end5-fn06_handle_alloc_init

	.section	.text.fn07_handle_alloc_release,"ax",@progbits
	.globl	fn07_handle_alloc_release
	.p2align	4
	.type	fn07_handle_alloc_release,@function
fn07_handle_alloc_release:
	push	ebx
	push	edi
	push	esi
	sub	esp, 16
	mov	esi, dword ptr [esp + 32]
	mov	edi, dword ptr [esp + 36]
	call	.L6$pb
.L6$pb:
	pop	ebx
.Ltmp6:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp6-.L6$pb)
	mov	dword ptr [esp + 4], edi
	mov	dword ptr [esp], esi
	call	objc_msg_lookup@PLT
	mov	dword ptr [esp + 4], edi
	mov	dword ptr [esp], esi
	call	eax
	mov	dword ptr [esp], eax
	call	objc_release@PLT
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.Lfunc_end6:
	.size	fn07_handle_alloc_release, .Lfunc_end6-fn07_handle_alloc_release

	.section	.text.fn08_handle_alloc_init_release,"ax",@progbits
	.globl	fn08_handle_alloc_init_release
	.p2align	4
	.type	fn08_handle_alloc_init_release,@function
fn08_handle_alloc_init_release:
	push	ebp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	mov	esi, dword ptr [esp + 32]
	mov	ebp, dword ptr [esp + 36]
	mov	edi, dword ptr [esp + 40]
	call	.L7$pb
.L7$pb:
	pop	ebx
.Ltmp7:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp7-.L7$pb)
	mov	dword ptr [esp + 4], ebp
	mov	dword ptr [esp], esi
	call	objc_msg_lookup@PLT
	mov	dword ptr [esp + 4], ebp
	mov	dword ptr [esp], esi
	call	eax
	mov	esi, eax
	mov	dword ptr [esp + 4], edi
	mov	dword ptr [esp], eax
	call	objc_msg_lookup@PLT
	mov	dword ptr [esp + 4], edi
	mov	dword ptr [esp], esi
	call	eax
	mov	dword ptr [esp], eax
	call	objc_release@PLT
	add	esp, 12
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
.Lfunc_end7:
	.size	fn08_handle_alloc_init_release, .Lfunc_end7-fn08_handle_alloc_init_release

	.section	.text.fn09_handle_copy,"ax",@progbits
	.globl	fn09_handle_copy
	.p2align	4
	.type	fn09_handle_copy,@function
fn09_handle_copy:
	push	ebx
	push	edi
	push	esi
	mov	esi, dword ptr [esp + 16]
	mov	edi, dword ptr [esp + 20]
	call	.L8$pb
.L8$pb:
	pop	ebx
.Ltmp8:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp8-.L8$pb)
	sub	esp, 8
	push	edi
	push	esi
	call	objc_msg_lookup@PLT
	add	esp, 8
	push	edi
	push	esi
	call	eax
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.Lfunc_end8:
	.size	fn09_handle_copy, .Lfunc_end8-fn09_handle_copy

	.section	.text.fn10_handle_copy_fallible,"ax",@progbits
	.globl	fn10_handle_copy_fallible
	.p2align	4
	.type	fn10_handle_copy_fallible,@function
fn10_handle_copy_fallible:
	push	ebx
	push	edi
	push	esi
	sub	esp, 16
	mov	esi, dword ptr [esp + 32]
	mov	edi, dword ptr [esp + 36]
	call	.L9$pb
.L9$pb:
	pop	ebx
.Ltmp9:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp9-.L9$pb)
	mov	dword ptr [esp + 4], edi
	mov	dword ptr [esp], esi
	call	objc_msg_lookup@PLT
	mov	dword ptr [esp + 4], edi
	mov	dword ptr [esp], esi
	call	eax
	test	eax, eax
	je	.LBB9_2
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB9_2:
	lea	eax, [ebx + .Lanon.[ID].3@GOTOFF]
	mov	dword ptr [esp], eax
	call	SYM(objc2::__macros::retain_semantics::copy_fail::GENERATED_ID, 0)@PLT
.Lfunc_end9:
	.size	fn10_handle_copy_fallible, .Lfunc_end9-fn10_handle_copy_fallible

	.section	.text.fn11_handle_mutable_copy,"ax",@progbits
	.globl	fn11_handle_mutable_copy
	.p2align	4
	.type	fn11_handle_mutable_copy,@function
fn11_handle_mutable_copy:
	push	ebx
	push	edi
	push	esi
	mov	esi, dword ptr [esp + 16]
	mov	edi, dword ptr [esp + 20]
	call	.L10$pb
.L10$pb:
	pop	ebx
.Ltmp10:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp10-.L10$pb)
	sub	esp, 8
	push	edi
	push	esi
	call	objc_msg_lookup@PLT
	add	esp, 8
	push	edi
	push	esi
	call	eax
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.Lfunc_end10:
	.size	fn11_handle_mutable_copy, .Lfunc_end10-fn11_handle_mutable_copy

	.section	.text.fn12_handle_mutable_copy_fallible,"ax",@progbits
	.globl	fn12_handle_mutable_copy_fallible
	.p2align	4
	.type	fn12_handle_mutable_copy_fallible,@function
fn12_handle_mutable_copy_fallible:
	push	ebx
	push	edi
	push	esi
	sub	esp, 16
	mov	esi, dword ptr [esp + 32]
	mov	edi, dword ptr [esp + 36]
	call	.L11$pb
.L11$pb:
	pop	ebx
.Ltmp11:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp11-.L11$pb)
	mov	dword ptr [esp + 4], edi
	mov	dword ptr [esp], esi
	call	objc_msg_lookup@PLT
	mov	dword ptr [esp + 4], edi
	mov	dword ptr [esp], esi
	call	eax
	test	eax, eax
	je	.LBB11_2
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB11_2:
	lea	eax, [ebx + .Lanon.[ID].4@GOTOFF]
	mov	dword ptr [esp], eax
	call	SYM(objc2::__macros::retain_semantics::mutable_copy_fail::GENERATED_ID, 0)@PLT
.Lfunc_end11:
	.size	fn12_handle_mutable_copy_fallible, .Lfunc_end11-fn12_handle_mutable_copy_fallible

	.section	.text.fn13_handle_autoreleased,"ax",@progbits
	.globl	fn13_handle_autoreleased
	.p2align	4
	.type	fn13_handle_autoreleased,@function
fn13_handle_autoreleased:
	push	ebx
	push	edi
	push	esi
	sub	esp, 16
	mov	esi, dword ptr [esp + 32]
	mov	edi, dword ptr [esp + 36]
	call	.L12$pb
.L12$pb:
	pop	ebx
.Ltmp12:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp12-.L12$pb)
	mov	dword ptr [esp + 4], edi
	mov	dword ptr [esp], esi
	call	objc_msg_lookup@PLT
	mov	dword ptr [esp + 4], edi
	mov	dword ptr [esp], esi
	call	eax
	mov	dword ptr [esp], eax
	call	objc_retainAutoreleasedReturnValue@PLT
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.Lfunc_end12:
	.size	fn13_handle_autoreleased, .Lfunc_end12-fn13_handle_autoreleased

	.section	.text.fn14_handle_autoreleased_with_arg,"ax",@progbits
	.globl	fn14_handle_autoreleased_with_arg
	.p2align	4
	.type	fn14_handle_autoreleased_with_arg,@function
fn14_handle_autoreleased_with_arg:
	push	ebp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	movzx	edi, byte ptr [esp + 40]
	mov	esi, dword ptr [esp + 32]
	mov	ebp, dword ptr [esp + 36]
	call	.L13$pb
.L13$pb:
	pop	ebx
.Ltmp13:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp13-.L13$pb)
	sub	esp, 8
	push	ebp
	push	esi
	call	objc_msg_lookup@PLT
	add	esp, 12
	push	edi
	push	ebp
	push	esi
	call	eax
	add	esp, 4
	push	eax
	call	objc_retainAutoreleasedReturnValue@PLT
	add	esp, 28
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
.Lfunc_end13:
	.size	fn14_handle_autoreleased_with_arg, .Lfunc_end13-fn14_handle_autoreleased_with_arg

	.section	.text.fn15_handle_autoreleased_fallible,"ax",@progbits
	.globl	fn15_handle_autoreleased_fallible
	.p2align	4
	.type	fn15_handle_autoreleased_fallible,@function
fn15_handle_autoreleased_fallible:
	push	ebx
	push	edi
	push	esi
	mov	edi, dword ptr [esp + 20]
	mov	esi, dword ptr [esp + 16]
	call	.L14$pb
.L14$pb:
	pop	ebx
.Ltmp14:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp14-.L14$pb)
	sub	esp, 8
	push	edi
	push	esi
	call	objc_msg_lookup@PLT
	add	esp, 8
	push	edi
	push	esi
	call	eax
	add	esp, 4
	push	eax
	call	objc_retainAutoreleasedReturnValue@PLT
	add	esp, 16
	test	eax, eax
	je	.LBB14_2
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB14_2:
	sub	esp, 4
	lea	eax, [ebx + .Lanon.[ID].5@GOTOFF]
	push	eax
	push	edi
	push	esi
	call	SYM(objc2::__macros::retain_semantics::none_fail::GENERATED_ID, 0)@PLT
.Lfunc_end14:
	.size	fn15_handle_autoreleased_fallible, .Lfunc_end14-fn15_handle_autoreleased_fallible

	.section	.text.fn16_handle_with_out_param,"ax",@progbits
	.globl	fn16_handle_with_out_param
	.p2align	4
	.type	fn16_handle_with_out_param,@function
fn16_handle_with_out_param:
.Lfunc_begin0:
	push	ebp
	push	ebx
	push	edi
	push	esi
	sub	esp, 28
	mov	ebp, dword ptr [esp + 56]
	mov	edi, dword ptr [esp + 52]
	mov	esi, dword ptr [esp + 48]
	call	.L15$pb
.L15$pb:
	pop	ebx
.Ltmp27:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp27-.L15$pb)
	mov	eax, dword ptr [ebp]
	mov	dword ptr [esp + 24], eax
.Ltmp15:
	mov	dword ptr [esp + 4], edi
	mov	dword ptr [esp], esi
	call	objc_msg_lookup@PLT
.Ltmp16:
.Ltmp17:
	mov	dword ptr [esp + 8], ebp
	mov	dword ptr [esp + 4], edi
	mov	dword ptr [esp], esi
	call	eax
.Ltmp18:
.Ltmp19:
	mov	dword ptr [esp], eax
	call	objc_retainAutoreleasedReturnValue@PLT
.Ltmp20:
	mov	esi, eax
	mov	eax, dword ptr [ebp]
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
	mov	eax, dword ptr [esp + 24]
	mov	dword ptr [esp], eax
	call	objc_release@PLT
	mov	eax, esi
	add	esp, 28
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
.LBB15_4:
.Ltmp21:
	mov	esi, eax
	mov	eax, dword ptr [ebp]
.Ltmp22:
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
.Ltmp23:
.Ltmp24:
	mov	eax, dword ptr [esp + 24]
	mov	dword ptr [esp], eax
	call	objc_release@PLT
.Ltmp25:
	mov	dword ptr [esp], esi
	call	_Unwind_Resume@PLT
.LBB15_7:
.Ltmp26:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@PLT
.Lfunc_end15:
	.size	fn16_handle_with_out_param, .Lfunc_end15-fn16_handle_with_out_param
	.section	.gcc_except_table.fn16_handle_with_out_param,"a",@progbits
	.p2align	2, 0x0
GCC_except_table15:
.Lexception0:
	.byte	255
	.byte	155
	.uleb128 .Lttbase0-.Lttbaseref0
.Lttbaseref0:
	.byte	1
	.uleb128 .Lcst_end0-.Lcst_begin0
.Lcst_begin0:
	.uleb128 .Ltmp15-.Lfunc_begin0
	.uleb128 .Ltmp20-.Ltmp15
	.uleb128 .Ltmp21-.Lfunc_begin0
	.byte	0
	.uleb128 .Ltmp20-.Lfunc_begin0
	.uleb128 .Ltmp22-.Ltmp20
	.byte	0
	.byte	0
	.uleb128 .Ltmp22-.Lfunc_begin0
	.uleb128 .Ltmp25-.Ltmp22
	.uleb128 .Ltmp26-.Lfunc_begin0
	.byte	1
	.uleb128 .Ltmp25-.Lfunc_begin0
	.uleb128 .Lfunc_end15-.Ltmp25
	.byte	0
	.byte	0
.Lcst_end0:
	.byte	127
	.byte	0
	.p2align	2, 0x0
.Lttbase0:
	.byte	0
	.p2align	2, 0x0

	.type	.Lanon.[ID].0,@object
	.section	.rodata.str1.1,"aMS",@progbits,1
.Lanon.[ID].0:
	.asciz	"crates/$DIR/lib.rs"
	.size	.Lanon.[ID].0, 58

	.type	.Lanon.[ID].1,@object
	.section	.data.rel.ro..Lanon.[ID].1,"aw",@progbits
	.p2align	2, 0x0
.Lanon.[ID].1:
	.long	.Lanon.[ID].0
	.asciz	"9\000\000\000\017\000\000\000\005\000\000"
	.size	.Lanon.[ID].1, 16

	.type	.Lanon.[ID].2,@object
	.section	.data.rel.ro..Lanon.[ID].2,"aw",@progbits
	.p2align	2, 0x0
.Lanon.[ID].2:
	.long	.Lanon.[ID].0
	.asciz	"9\000\000\000\036\000\000\000\005\000\000"
	.size	.Lanon.[ID].2, 16

	.type	.Lanon.[ID].3,@object
	.section	.data.rel.ro..Lanon.[ID].3,"aw",@progbits
	.p2align	2, 0x0
.Lanon.[ID].3:
	.long	.Lanon.[ID].0
	.asciz	"9\000\000\000:\000\000\000\005\000\000"
	.size	.Lanon.[ID].3, 16

	.type	.Lanon.[ID].4,@object
	.section	.data.rel.ro..Lanon.[ID].4,"aw",@progbits
	.p2align	2, 0x0
.Lanon.[ID].4:
	.long	.Lanon.[ID].0
	.asciz	"9\000\000\000D\000\000\000\005\000\000"
	.size	.Lanon.[ID].4, 16

	.type	.Lanon.[ID].5,@object
	.section	.data.rel.ro..Lanon.[ID].5,"aw",@progbits
	.p2align	2, 0x0
.Lanon.[ID].5:
	.long	.Lanon.[ID].0
	.asciz	"9\000\000\000X\000\000\000\005\000\000"
	.size	.Lanon.[ID].5, 16

	.hidden	DW.ref.rust_eh_personality
	.weak	DW.ref.rust_eh_personality
	.section	.data.DW.ref.rust_eh_personality,"awG",@progbits,DW.ref.rust_eh_personality,comdat
	.p2align	2, 0x0
	.type	DW.ref.rust_eh_personality,@object
	.size	DW.ref.rust_eh_personality, 4
DW.ref.rust_eh_personality:
	.long	rust_eh_personality
	.section	".note.GNU-stack","",@progbits
