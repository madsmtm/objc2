	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_fn1_get_class
	.p2align	4
_fn1_get_class:
	push	ebp
	mov	ebp, esp
	sub	esp, 8
	call	L0$pb
L0$pb:
	pop	ecx
	mov	eax, dword ptr [ecx + SYM(test_dynamic_class[CRATE_ID]::get_class::CACHED_CLASS, 0)-L0$pb]
	test	eax, eax
	je	LBB0_1
	add	esp, 8
	pop	ebp
	ret
LBB0_1:
	sub	esp, 4
	lea	eax, [ecx + l_anon.[ID].2-L0$pb]
	lea	edx, [ecx + L_anon.[ID].0-L0$pb]
	lea	ecx, [ecx + SYM(test_dynamic_class[CRATE_ID]::get_class::CACHED_CLASS, 0)-L0$pb]
	push	eax
	push	edx
	push	ecx
	call	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	add	esp, 24
	pop	ebp
	ret

	.globl	_fn1_get_same_class
	.p2align	4
_fn1_get_same_class:
	push	ebp
	mov	ebp, esp
	sub	esp, 8
	call	L1$pb
L1$pb:
	pop	ecx
	mov	eax, dword ptr [ecx + SYM(test_dynamic_class[CRATE_ID]::get_same_class::CACHED_CLASS, 0)-L1$pb]
	test	eax, eax
	je	LBB1_1
	add	esp, 8
	pop	ebp
	ret
LBB1_1:
	sub	esp, 4
	lea	eax, [ecx + l_anon.[ID].3-L1$pb]
	lea	edx, [ecx + L_anon.[ID].0-L1$pb]
	lea	ecx, [ecx + SYM(test_dynamic_class[CRATE_ID]::get_same_class::CACHED_CLASS, 0)-L1$pb]
	push	eax
	push	edx
	push	ecx
	call	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	add	esp, 24
	pop	ebp
	ret

	.globl	_fn3_get_different_class
	.p2align	4
_fn3_get_different_class:
	push	ebp
	mov	ebp, esp
	sub	esp, 8
	call	L2$pb
L2$pb:
	pop	ecx
	mov	eax, dword ptr [ecx + SYM(test_dynamic_class[CRATE_ID]::get_different_class::CACHED_CLASS, 0)-L2$pb]
	test	eax, eax
	je	LBB2_1
	add	esp, 8
	pop	ebp
	ret
LBB2_1:
	sub	esp, 4
	lea	eax, [ecx + l_anon.[ID].5-L2$pb]
	lea	edx, [ecx + L_anon.[ID].4-L2$pb]
	lea	ecx, [ecx + SYM(test_dynamic_class[CRATE_ID]::get_different_class::CACHED_CLASS, 0)-L2$pb]
	push	eax
	push	edx
	push	ecx
	call	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	add	esp, 24
	pop	ebp
	ret

	.globl	_fn4_unused_class
	.p2align	4
_fn4_unused_class:
	push	ebp
	mov	ebp, esp
	sub	esp, 8
	call	L3$pb
L3$pb:
	pop	eax
	mov	ecx, dword ptr [eax + SYM(test_dynamic_class[CRATE_ID]::unused_class::CACHED_CLASS, 0)-L3$pb]
	test	ecx, ecx
	je	LBB3_1
	add	esp, 8
	pop	ebp
	ret
LBB3_1:
	sub	esp, 4
	lea	ecx, [eax + l_anon.[ID].7-L3$pb]
	lea	edx, [eax + L_anon.[ID].6-L3$pb]
	lea	eax, [eax + SYM(test_dynamic_class[CRATE_ID]::unused_class::CACHED_CLASS, 0)-L3$pb]
	push	ecx
	push	edx
	push	eax
	call	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	add	esp, 24
	pop	ebp
	ret

	.globl	_fn5_use_fns
	.p2align	4
_fn5_use_fns:
	push	ebp
	mov	ebp, esp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	call	L4$pb
L4$pb:
	pop	esi
	mov	edx, dword ptr [esi + SYM(test_dynamic_class[CRATE_ID]::get_class::CACHED_CLASS, 0)-L4$pb]
	test	edx, edx
	je	LBB4_1
	mov	edi, dword ptr [esi + SYM(test_dynamic_class[CRATE_ID]::get_same_class::CACHED_CLASS, 0)-L4$pb]
	test	edi, edi
	je	LBB4_3
LBB4_4:
	mov	eax, dword ptr [ebp + 8]
	mov	ebx, dword ptr [esi + SYM(test_dynamic_class[CRATE_ID]::get_different_class::CACHED_CLASS, 0)-L4$pb]
	test	ebx, ebx
	je	LBB4_5
LBB4_6:
	mov	ecx, dword ptr [esi + SYM(test_dynamic_class[CRATE_ID]::use_fns::CACHED_CLASS, 0)-L4$pb]
	test	ecx, ecx
	je	LBB4_7
LBB4_8:
	mov	dword ptr [eax], edx
	mov	dword ptr [eax + 4], edi
	mov	dword ptr [eax + 8], ebx
	mov	dword ptr [eax + 12], ecx
	add	esp, 12
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret	4
LBB4_1:
	sub	esp, 4
	lea	eax, [esi + l_anon.[ID].2-L4$pb]
	lea	ecx, [esi + L_anon.[ID].0-L4$pb]
	lea	edx, [esi + SYM(test_dynamic_class[CRATE_ID]::get_class::CACHED_CLASS, 0)-L4$pb]
	push	eax
	push	ecx
	push	edx
	call	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	add	esp, 16
	mov	edx, eax
	mov	edi, dword ptr [esi + SYM(test_dynamic_class[CRATE_ID]::get_same_class::CACHED_CLASS, 0)-L4$pb]
	test	edi, edi
	jne	LBB4_4
LBB4_3:
	sub	esp, 4
	lea	eax, [esi + l_anon.[ID].3-L4$pb]
	lea	ecx, [esi + L_anon.[ID].0-L4$pb]
	mov	edi, edx
	lea	edx, [esi + SYM(test_dynamic_class[CRATE_ID]::get_same_class::CACHED_CLASS, 0)-L4$pb]
	push	eax
	push	ecx
	push	edx
	call	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	mov	edx, edi
	add	esp, 16
	mov	edi, eax
	mov	eax, dword ptr [ebp + 8]
	mov	ebx, dword ptr [esi + SYM(test_dynamic_class[CRATE_ID]::get_different_class::CACHED_CLASS, 0)-L4$pb]
	test	ebx, ebx
	jne	LBB4_6
LBB4_5:
	sub	esp, 4
	lea	eax, [esi + l_anon.[ID].5-L4$pb]
	lea	ecx, [esi + L_anon.[ID].4-L4$pb]
	mov	ebx, edx
	lea	edx, [esi + SYM(test_dynamic_class[CRATE_ID]::get_different_class::CACHED_CLASS, 0)-L4$pb]
	push	eax
	push	ecx
	push	edx
	call	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	mov	edx, ebx
	add	esp, 16
	mov	ebx, eax
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [esi + SYM(test_dynamic_class[CRATE_ID]::use_fns::CACHED_CLASS, 0)-L4$pb]
	test	ecx, ecx
	jne	LBB4_8
LBB4_7:
	sub	esp, 4
	lea	ecx, [esi + l_anon.[ID].9-L4$pb]
	mov	dword ptr [ebp - 16], edx
	lea	edx, [esi + L_anon.[ID].8-L4$pb]
	lea	esi, [esi + SYM(test_dynamic_class[CRATE_ID]::use_fns::CACHED_CLASS, 0)-L4$pb]
	push	ecx
	push	edx
	push	esi
	mov	esi, eax
	call	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	mov	edx, dword ptr [ebp - 16]
	add	esp, 16
	mov	ecx, eax
	mov	eax, esi
	jmp	LBB4_8

	.globl	_fn6_use_same_twice
	.p2align	4
_fn6_use_same_twice:
	push	ebp
	mov	ebp, esp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	call	L5$pb
L5$pb:
	pop	edi
	mov	eax, dword ptr [ebp + 8]
	mov	esi, dword ptr [edi + SYM(test_dynamic_class[CRATE_ID]::get_class::CACHED_CLASS, 0)-L5$pb]
	test	esi, esi
	je	LBB5_1
	mov	ecx, dword ptr [edi + SYM(test_dynamic_class[CRATE_ID]::get_class::CACHED_CLASS, 0)-L5$pb]
	test	ecx, ecx
	je	LBB5_3
LBB5_4:
	mov	dword ptr [eax], esi
	mov	dword ptr [eax + 4], ecx
	add	esp, 12
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret	4
LBB5_1:
	sub	esp, 4
	lea	ecx, [edi + l_anon.[ID].2-L5$pb]
	lea	edx, [edi + L_anon.[ID].0-L5$pb]
	lea	esi, [edi + SYM(test_dynamic_class[CRATE_ID]::get_class::CACHED_CLASS, 0)-L5$pb]
	push	ecx
	push	edx
	push	esi
	mov	ebx, eax
	call	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	add	esp, 16
	mov	esi, eax
	mov	eax, ebx
	mov	ecx, dword ptr [edi + SYM(test_dynamic_class[CRATE_ID]::get_class::CACHED_CLASS, 0)-L5$pb]
	test	ecx, ecx
	jne	LBB5_4
LBB5_3:
	sub	esp, 4
	lea	ecx, [edi + l_anon.[ID].2-L5$pb]
	lea	edx, [edi + L_anon.[ID].0-L5$pb]
	lea	edi, [edi + SYM(test_dynamic_class[CRATE_ID]::get_class::CACHED_CLASS, 0)-L5$pb]
	push	ecx
	push	edx
	push	edi
	mov	edi, eax
	call	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	add	esp, 16
	mov	ecx, eax
	mov	eax, edi
	jmp	LBB5_4

	.globl	_fn7_use_in_loop
	.p2align	4
_fn7_use_in_loop:
	push	ebp
	mov	ebp, esp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	call	L6$pb
L6$pb:
	pop	esi
	mov	edi, dword ptr [ebp + 8]
	test	edi, edi
	je	LBB6_5
	lea	ecx, [esi + l_anon.[ID].11-L6$pb]
	lea	edx, [esi + L_anon.[ID].10-L6$pb]
	lea	ebx, [esi + SYM(test_dynamic_class[CRATE_ID]::use_in_loop::CACHED_CLASS, 0)-L6$pb]
	.p2align	4
LBB6_2:
	mov	eax, dword ptr [esi + SYM(test_dynamic_class[CRATE_ID]::use_in_loop::CACHED_CLASS, 0)-L6$pb]
	test	eax, eax
	je	LBB6_3
	dec	edi
	jne	LBB6_2
	jmp	LBB6_5
LBB6_3:
	sub	esp, 4
	push	ecx
	push	edx
	push	ebx
	call	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	lea	edx, [esi + L_anon.[ID].10-L6$pb]
	lea	ecx, [esi + l_anon.[ID].11-L6$pb]
	add	esp, 16
	dec	edi
	jne	LBB6_2
LBB6_5:
	add	esp, 12
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret

	.section	__TEXT,__cstring,cstring_literals
L_anon.[ID].0:
	.asciz	"NSObject"

	.section	__TEXT,__const
l_anon.[ID].1:
	.ascii	"crates/$DIR/../test_static_class/lib.rs"

	.section	__DATA,__const
	.p2align	2, 0x0
l_anon.[ID].2:
	.long	l_anon.[ID].1
	.asciz	"J\000\000\000\007\000\000\000\005\000\000"

	.p2align	2, 0x0
l_anon.[ID].3:
	.long	l_anon.[ID].1
	.asciz	"J\000\000\000\f\000\000\000\005\000\000"

	.section	__TEXT,__cstring,cstring_literals
L_anon.[ID].4:
	.asciz	"NSString"

	.section	__DATA,__const
	.p2align	2, 0x0
l_anon.[ID].5:
	.long	l_anon.[ID].1
	.asciz	"J\000\000\000\021\000\000\000\005\000\000"

	.section	__TEXT,__cstring,cstring_literals
L_anon.[ID].6:
	.asciz	"NSData"

	.section	__DATA,__const
	.p2align	2, 0x0
l_anon.[ID].7:
	.long	l_anon.[ID].1
	.asciz	"J\000\000\000\026\000\000\000\r\000\000"

	.section	__TEXT,__cstring,cstring_literals
L_anon.[ID].8:
	.asciz	"NSException"

	.section	__DATA,__const
	.p2align	2, 0x0
l_anon.[ID].9:
	.long	l_anon.[ID].1
	.asciz	"J\000\000\000\036\000\000\000\016\000\000"

	.section	__TEXT,__cstring,cstring_literals
L_anon.[ID].10:
	.asciz	"NSLock"

	.section	__DATA,__const
	.p2align	2, 0x0
l_anon.[ID].11:
	.long	l_anon.[ID].1
	.asciz	"J\000\000\000,\000\000\000\021\000\000"

.zerofill __DATA,__bss,SYM(test_dynamic_class[CRATE_ID]::get_class::CACHED_CLASS, 0),4,2
.zerofill __DATA,__bss,SYM(test_dynamic_class[CRATE_ID]::get_same_class::CACHED_CLASS, 0),4,2
.zerofill __DATA,__bss,SYM(test_dynamic_class[CRATE_ID]::get_different_class::CACHED_CLASS, 0),4,2
.zerofill __DATA,__bss,SYM(test_dynamic_class[CRATE_ID]::unused_class::CACHED_CLASS, 0),4,2
.zerofill __DATA,__bss,SYM(test_dynamic_class[CRATE_ID]::use_fns::CACHED_CLASS, 0),4,2
.zerofill __DATA,__bss,SYM(test_dynamic_class[CRATE_ID]::use_in_loop::CACHED_CLASS, 0),4,2
.subsections_via_symbols
