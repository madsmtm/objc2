	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.p2align	4, 0x90
SYM(core[CRATE_ID]::ptr::drop_in_place::<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::{closure#0}>::{closure#0}>, 0):
	push	ebp
	mov	ebp, esp
	pop	ebp
	ret

	.p2align	4, 0x90
SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::{closure#0}>::{closure#0}, 0):
	push	ebp
	mov	ebp, esp
	push	ebx
	push	edi
	push	esi
	sub	esp, 44
	call	L1$pb
L1$pb:
	pop	esi
	mov	eax, dword ptr [ebp + 8]
	mov	eax, dword ptr [eax]
	cmp	byte ptr [eax], 0
	mov	byte ptr [eax], 0
	je	LBB1_3
	mov	eax, dword ptr [esi + LL_OBJC_CLASSLIST_REFERENCES_$_NSObject$non_lazy_ptr-L1$pb]
	sub	esp, 4
	lea	ecx, [esi + l_anon.[ID].16-L1$pb]
	push	dword ptr [eax]
	push	15
	push	ecx
	call	SYM(objc2::declare::ClassBuilder::new::GENERATED_ID, 0)
	add	esp, 16
	test	eax, eax
	je	LBB1_4
	mov	dword ptr [ebp - 36], eax
	sub	esp, 8
	lea	eax, [esi + l_anon.[ID].12-L1$pb]
	lea	ecx, [esi + L_anon.[ID].11-L1$pb]
	lea	ebx, [ebp - 36]
	push	eax
	push	0
	push	1
	push	4
	push	ecx
	push	ebx
	call	SYM(objc2::declare::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
	add	esp, 24
	lea	ecx, [esi + l_anon.[ID].2-L1$pb]
	lea	eax, [esi + L_anon.[ID].13-L1$pb]
	push	ecx
	push	2
	push	4
	push	4
	push	eax
	push	ebx
	call	SYM(objc2::declare::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
	add	esp, 32
	mov	eax, dword ptr [esi + LL_OBJC_SELECTOR_REFERENCES_dealloc$non_lazy_ptr-L1$pb]
	sub	esp, 8
	lea	ecx, [esi + SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::{closure#0}::__objc2_dealloc, 0)-L1$pb]
	lea	edi, [esi + l_anon.[ID].3-L1$pb]
	lea	edx, [esi + l_anon.[ID].1-L1$pb]
	push	ecx
	push	edi
	push	0
	push	edx
	mov	edi, edx
	push	dword ptr [eax]
	push	ebx
	call	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	add	esp, 32
	mov	eax, dword ptr [esi + LL_OBJC_SELECTOR_REFERENCES_init$non_lazy_ptr-L1$pb]
	sub	esp, 8
	lea	ecx, [esi + _init-L1$pb]
	push	ecx
	lea	ecx, [esi + l_anon.[ID].2-L1$pb]
	push	ecx
	push	0
	push	edi
	push	dword ptr [eax]
	push	ebx
	call	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	add	esp, 24
	lea	eax, [esi + _class_method-L1$pb]
	push	eax
	lea	eax, [esi + l_anon.[ID].3-L1$pb]
	push	eax
	push	0
	push	edi
	push	dword ptr [esi + L_OBJC_SELECTOR_REFERENCES_d874ee9262978be2-L1$pb]
	push	ebx
	call	SYM(objc2::declare::ClassBuilder::add_class_method_inner::GENERATED_ID, 0)
	add	esp, 24
	lea	eax, [esi + _method-L1$pb]
	push	eax
	lea	eax, [esi + l_anon.[ID].3-L1$pb]
	push	eax
	push	0
	push	edi
	push	dword ptr [esi + L_OBJC_SELECTOR_REFERENCES_4539fd1dbda0cddc-L1$pb]
	push	ebx
	call	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	add	esp, 24
	lea	eax, [esi + _method_bool-L1$pb]
	lea	ecx, [esi + l_anon.[ID].4-L1$pb]
	push	eax
	push	ecx
	push	1
	push	ecx
	push	dword ptr [esi + L_OBJC_SELECTOR_REFERENCES_2b1b3a94e0ece2e5-L1$pb]
	push	ebx
	call	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	add	esp, 24
	lea	eax, [esi + _method_id-L1$pb]
	push	eax
	lea	edi, [esi + l_anon.[ID].2-L1$pb]
	push	edi
	push	0
	lea	eax, [esi + l_anon.[ID].1-L1$pb]
	push	eax
	push	dword ptr [esi + L_OBJC_SELECTOR_REFERENCES_f7f521670860b0ce-L1$pb]
	push	ebx
	call	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	add	esp, 24
	lea	eax, [esi + _method_id_with_param-L1$pb]
	push	eax
	push	edi
	push	1
	lea	eax, [esi + l_anon.[ID].4-L1$pb]
	push	eax
	push	dword ptr [esi + L_OBJC_SELECTOR_REFERENCES_6addfcf634c6232f-L1$pb]
	push	ebx
	call	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	add	esp, 24
	lea	eax, [esi + l_anon.[ID].17-L1$pb]
	push	9
	push	eax
	call	SYM(objc2::runtime::AnyProtocol::get::GENERATED_ID, 0)
	add	esp, 8
	push	eax
	push	ebx
	call	SYM(objc2::__macro_helpers::<impl objc2::declare::ClassBuilder>::__add_protocol_methods::GENERATED_ID, 0)
	add	esp, 8
	lea	ecx, [esi + _copyWithZone-L1$pb]
	lea	edx, [esi + l_anon.[ID].7-L1$pb]
	push	ecx
	push	edi
	push	1
	push	edx
	push	dword ptr [esi + L_OBJC_SELECTOR_REFERENCES_4a8c690dbc9d8166-L1$pb]
	push	eax
	call	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	add	esp, 20
	push	dword ptr [ebp - 36]
	call	SYM(objc2::declare::ClassBuilder::register::GENERATED_ID, 0)
	add	esp, 60
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
LBB1_3:
	sub	esp, 4
	lea	eax, [esi + l_anon.[ID].10-L1$pb]
	lea	ecx, [esi + l_anon.[ID].8-L1$pb]
	push	eax
	push	43
	push	ecx
	call	SYM(core::panicking::panic::GENERATED_ID, 0)
LBB1_4:
	lea	eax, [esi + l_anon.[ID].21-L1$pb]
	mov	dword ptr [ebp - 44], eax
	lea	eax, [esi + SYM(<&str as core[CRATE_ID]::fmt::Display>::fmt, 0)-L1$pb]
	mov	dword ptr [ebp - 40], eax
	lea	eax, [esi + l_anon.[ID].20-L1$pb]
	mov	dword ptr [ebp - 36], eax
	mov	dword ptr [ebp - 32], 2
	mov	dword ptr [ebp - 20], 0
	lea	eax, [ebp - 44]
	mov	dword ptr [ebp - 28], eax
	mov	dword ptr [ebp - 24], 1
	sub	esp, 8
	lea	eax, [esi + l_anon.[ID].15-L1$pb]
	lea	ecx, [ebp - 36]
	push	eax
	push	ecx
	call	SYM(core::panicking::panic_fmt::GENERATED_ID, 0)

	.p2align	4, 0x90
SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0):
	push	ebp
	mov	ebp, esp
	sub	esp, 24
	mov	eax, dword ptr [ebp + 8]
	mov	eax, dword ptr [eax]
	mov	dword ptr [ebp - 4], eax
	lea	eax, [ebp - 4]
	mov	dword ptr [esp], eax
	call	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::{closure#0}>::{closure#0}, 0)
	add	esp, 24
	pop	ebp
	ret

	.p2align	4, 0x90
SYM(<&str as core[CRATE_ID]::fmt::Display>::fmt, 0):
	push	ebp
	mov	ebp, esp
	sub	esp, 8
	mov	eax, dword ptr [ebp + 8]
	sub	esp, 4
	push	dword ptr [ebp + 12]
	push	dword ptr [eax + 4]
	push	dword ptr [eax]
	call	SYM(<str as core::fmt::Display>::fmt::GENERATED_ID, 0)
	add	esp, 24
	pop	ebp
	ret

	.globl	_get_class
	.p2align	4, 0x90
_get_class:
	push	ebp
	mov	ebp, esp
	push	edi
	push	esi
	sub	esp, 16
	call	L4$pb
L4$pb:
	pop	esi
	mov	eax, dword ptr [esi + SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::REGISTER_CLASS, 0)-L4$pb]
	cmp	eax, 3
	jne	LBB4_1
LBB4_2:
	sub	esp, 8
	lea	eax, [esi + l_anon.[ID].16-L4$pb]
	push	15
	push	eax
	call	SYM(objc2::runtime::AnyClass::get::GENERATED_ID, 0)
	add	esp, 16
	test	eax, eax
	je	LBB4_4
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebp
	ret
LBB4_1:
	mov	byte ptr [ebp - 9], 1
	lea	eax, [ebp - 9]
	mov	dword ptr [ebp - 16], eax
	sub	esp, 12
	lea	eax, [esi + l_anon.[ID].15-L4$pb]
	lea	ecx, [esi + l_anon.[ID].0-L4$pb]
	lea	edx, [ebp - 16]
	lea	edi, [esi + SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::REGISTER_CLASS, 0)-L4$pb]
	push	eax
	push	ecx
	push	edx
	push	0
	push	edi
	call	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	add	esp, 32
	jmp	LBB4_2
LBB4_4:
	sub	esp, 4
	lea	eax, [esi + l_anon.[ID].15-L4$pb]
	lea	ecx, [esi + l_anon.[ID].8-L4$pb]
	push	eax
	push	43
	push	ecx
	call	SYM(core::panicking::panic::GENERATED_ID, 0)

	.globl	_get_obj
	.p2align	4, 0x90
_get_obj:
	push	ebp
	mov	ebp, esp
	push	esi
	push	eax
	call	L5$pb
L5$pb:
	pop	eax
	mov	eax, dword ptr [eax + LL_OBJC_SELECTOR_REFERENCES_new$non_lazy_ptr-L5$pb]
	mov	esi, dword ptr [eax]
	call	_get_class
	sub	esp, 8
	push	esi
	push	eax
	call	_objc_msgSend
	add	esp, 20
	pop	esi
	pop	ebp
	ret

	.globl	_access_ivars
	.p2align	4, 0x90
_access_ivars:
	push	ebp
	mov	ebp, esp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	call	L6$pb
L6$pb:
	pop	edi
	call	_get_obj
	mov	esi, eax
	sub	esp, 12
	push	eax
	call	SYM(objc2::runtime::AnyObject::class::GENERATED_ID, 0)
	add	esp, 16
	lea	ecx, [edi + l_anon.[ID].12-L6$pb]
	lea	edx, [edi + L_anon.[ID].11-L6$pb]
	push	ecx
	push	4
	push	edx
	push	eax
	call	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	add	esp, 16
	movzx	ebx, byte ptr [esi + eax]
	sub	esp, 12
	push	esi
	call	SYM(objc2::runtime::AnyObject::class::GENERATED_ID, 0)
	add	esp, 16
	lea	ecx, [edi + l_anon.[ID].2-L6$pb]
	lea	edx, [edi + L_anon.[ID].13-L6$pb]
	push	ecx
	push	4
	push	edx
	push	eax
	call	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	add	esp, 16
	mov	edi, dword ptr [esi + eax]
	sub	esp, 12
	push	esi
	call	_objc_release
	add	esp, 16
	mov	eax, ebx
	mov	edx, edi
	add	esp, 12
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret

	.globl	SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class, 0)
	.p2align	4, 0x90
SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class, 0):
	push	ebp
	mov	ebp, esp
	push	edi
	push	esi
	sub	esp, 16
	call	L7$pb
L7$pb:
	pop	esi
	mov	eax, dword ptr [esi + SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::REGISTER_CLASS, 0)-L7$pb]
	cmp	eax, 3
	jne	LBB7_1
LBB7_2:
	sub	esp, 8
	lea	eax, [esi + l_anon.[ID].16-L7$pb]
	push	15
	push	eax
	call	SYM(objc2::runtime::AnyClass::get::GENERATED_ID, 0)
	add	esp, 16
	test	eax, eax
	je	LBB7_4
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebp
	ret
LBB7_1:
	mov	byte ptr [ebp - 9], 1
	lea	eax, [ebp - 9]
	mov	dword ptr [ebp - 16], eax
	sub	esp, 12
	lea	eax, [esi + l_anon.[ID].15-L7$pb]
	lea	ecx, [esi + l_anon.[ID].0-L7$pb]
	lea	edx, [ebp - 16]
	lea	edi, [esi + SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::REGISTER_CLASS, 0)-L7$pb]
	push	eax
	push	ecx
	push	edx
	push	0
	push	edi
	call	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	add	esp, 32
	jmp	LBB7_2
LBB7_4:
	sub	esp, 4
	lea	eax, [esi + l_anon.[ID].15-L7$pb]
	lea	ecx, [esi + l_anon.[ID].8-L7$pb]
	push	eax
	push	43
	push	ecx
	call	SYM(core::panicking::panic::GENERATED_ID, 0)

	.p2align	4, 0x90
SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::{closure#0}::__objc2_dealloc, 0):
	push	ebp
	mov	ebp, esp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	call	L8$pb
L8$pb:
	pop	ebx
	mov	esi, dword ptr [ebp + 12]
	mov	edi, dword ptr [ebp + 8]
	sub	esp, 12
	push	edi
	call	SYM(objc2::runtime::AnyObject::class::GENERATED_ID, 0)
	add	esp, 16
	lea	ecx, [ebx + l_anon.[ID].2-L8$pb]
	lea	edx, [ebx + L_anon.[ID].13-L8$pb]
	push	ecx
	push	4
	push	edx
	push	eax
	call	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	add	esp, 16
	mov	eax, dword ptr [edi + eax]
	test	eax, eax
	je	LBB8_2
	sub	esp, 12
	push	eax
	call	_objc_release
	add	esp, 16
LBB8_2:
	mov	eax, dword ptr [ebx + LL_OBJC_CLASSLIST_REFERENCES_$_NSObject$non_lazy_ptr-L8$pb]
	mov	eax, dword ptr [eax]
	mov	dword ptr [ebp - 20], edi
	mov	dword ptr [ebp - 16], eax
	sub	esp, 8
	lea	eax, [ebp - 20]
	push	esi
	push	eax
	call	_objc_msgSendSuper
	add	esp, 28
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret

	.globl	_init
	.p2align	4, 0x90
_init:
	push	ebp
	mov	ebp, esp
	push	edi
	push	esi
	sub	esp, 16
	call	L9$pb
L9$pb:
	pop	edi
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [edi + LL_OBJC_SELECTOR_REFERENCES_init$non_lazy_ptr-L9$pb]
	mov	ecx, dword ptr [ecx]
	mov	edx, dword ptr [edi + LL_OBJC_CLASSLIST_REFERENCES_$_NSObject$non_lazy_ptr-L9$pb]
	mov	edx, dword ptr [edx]
	mov	dword ptr [ebp - 16], eax
	mov	dword ptr [ebp - 12], edx
	sub	esp, 8
	lea	eax, [ebp - 16]
	push	ecx
	push	eax
	call	_objc_msgSendSuper
	add	esp, 16
	mov	esi, eax
	test	eax, eax
	je	LBB9_2
	sub	esp, 12
	push	esi
	call	SYM(objc2::runtime::AnyObject::class::GENERATED_ID, 0)
	add	esp, 16
	lea	ecx, [edi + l_anon.[ID].12-L9$pb]
	lea	edx, [edi + L_anon.[ID].11-L9$pb]
	push	ecx
	push	4
	push	edx
	push	eax
	call	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	add	esp, 16
	mov	byte ptr [esi + eax], 42
	sub	esp, 12
	push	esi
	call	SYM(objc2::runtime::AnyObject::class::GENERATED_ID, 0)
	add	esp, 16
	lea	ecx, [edi + l_anon.[ID].2-L9$pb]
	lea	edx, [edi + L_anon.[ID].13-L9$pb]
	push	ecx
	push	4
	push	edx
	push	eax
	call	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	add	esp, 16
	mov	dword ptr [esi + eax], 0
LBB9_2:
	mov	eax, esi
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebp
	ret

	.globl	_class_method
	.p2align	4, 0x90
_class_method:
	push	ebp
	mov	ebp, esp
	pop	ebp
	ret

	.globl	_method
	.p2align	4, 0x90
_method:
	push	ebp
	mov	ebp, esp
	pop	ebp
	ret

	.globl	_method_bool
	.p2align	4, 0x90
_method_bool:
	push	ebp
	mov	ebp, esp
	xor	eax, eax
	cmp	byte ptr [ebp + 16], 0
	sete	al
	pop	ebp
	ret

	.globl	_method_id
	.p2align	4, 0x90
_method_id:
	push	ebp
	mov	ebp, esp
	push	edi
	push	esi
	call	L13$pb
L13$pb:
	pop	edi
	mov	esi, dword ptr [ebp + 8]
	sub	esp, 12
	push	esi
	call	SYM(objc2::runtime::AnyObject::class::GENERATED_ID, 0)
	add	esp, 16
	lea	ecx, [edi + l_anon.[ID].2-L13$pb]
	lea	edx, [edi + L_anon.[ID].13-L13$pb]
	push	ecx
	push	4
	push	edx
	push	eax
	call	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	add	esp, 16
	mov	eax, dword ptr [esi + eax]
	test	eax, eax
	je	LBB13_1
	sub	esp, 12
	push	eax
	call	_objc_retain
	add	esp, 16
	jmp	LBB13_3
LBB13_1:
	xor	eax, eax
LBB13_3:
	sub	esp, 12
	push	eax
	call	_objc_autoreleaseReturnValue
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebp
	ret

	.globl	_method_id_with_param
	.p2align	4, 0x90
_method_id_with_param:
	push	ebp
	mov	ebp, esp
	push	ebx
	push	edi
	push	esi
	sub	esp, 28
	call	L14$pb
L14$pb:
	pop	edi
	call	SYM(objc2::runtime::nsobject::NSObject::new::GENERATED_ID, 0)
	mov	esi, eax
	cmp	byte ptr [ebp + 16], 0
	je	LBB14_5
	mov	ebx, dword ptr [ebp + 8]
	mov	dword ptr [esp], ebx
	call	SYM(objc2::runtime::AnyObject::class::GENERATED_ID, 0)
	lea	ecx, [edi + l_anon.[ID].2-L14$pb]
	mov	dword ptr [esp + 12], ecx
	lea	ecx, [edi + L_anon.[ID].13-L14$pb]
	mov	dword ptr [esp + 4], ecx
	mov	dword ptr [esp], eax
	mov	dword ptr [esp + 8], 4
	call	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	mov	eax, dword ptr [ebx + eax]
	test	eax, eax
	je	LBB14_2
	mov	dword ptr [esp], eax
	call	_objc_retain
	mov	edi, eax
	jmp	LBB14_4
LBB14_2:
	xor	edi, edi
LBB14_4:
	mov	dword ptr [esp], esi
	call	_objc_release
	mov	esi, edi
LBB14_5:
	mov	dword ptr [esp], esi
	call	_objc_autoreleaseReturnValue
	add	esp, 28
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret

	.globl	_copyWithZone
	.p2align	4, 0x90
_copyWithZone:
	push	ebp
	mov	ebp, esp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	call	L15$pb
L15$pb:
	pop	edi
	call	_get_obj
	mov	esi, eax
	test	eax, eax
	je	LBB15_5
	mov	eax, dword ptr [ebp + 8]
	sub	esp, 12
	push	eax
	mov	ebx, eax
	call	SYM(objc2::runtime::AnyObject::class::GENERATED_ID, 0)
	add	esp, 16
	lea	edx, [edi + l_anon.[ID].12-L15$pb]
	lea	ecx, [edi + L_anon.[ID].11-L15$pb]
	push	edx
	push	4
	push	ecx
	push	eax
	call	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	add	esp, 16
	movzx	ebx, byte ptr [ebx + eax]
	sub	esp, 12
	push	esi
	call	SYM(objc2::runtime::AnyObject::class::GENERATED_ID, 0)
	add	esp, 16
	lea	ecx, [edi + l_anon.[ID].12-L15$pb]
	push	ecx
	push	4
	lea	ecx, [edi + L_anon.[ID].11-L15$pb]
	push	ecx
	push	eax
	call	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	add	esp, 16
	mov	byte ptr [esi + eax], bl
	sub	esp, 12
	push	dword ptr [ebp + 8]
	call	SYM(objc2::runtime::AnyObject::class::GENERATED_ID, 0)
	add	esp, 16
	lea	ecx, [edi + l_anon.[ID].2-L15$pb]
	lea	ebx, [edi + L_anon.[ID].13-L15$pb]
	mov	dword ptr [ebp - 16], ecx
	push	ecx
	push	4
	push	ebx
	push	eax
	call	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	add	esp, 16
	mov	ecx, dword ptr [ebp + 8]
	mov	eax, dword ptr [ecx + eax]
	test	eax, eax
	je	LBB15_2
	sub	esp, 12
	push	eax
	call	_objc_retain
	add	esp, 16
	mov	edi, eax
	jmp	LBB15_4
LBB15_2:
	xor	edi, edi
LBB15_4:
	sub	esp, 12
	push	esi
	call	SYM(objc2::runtime::AnyObject::class::GENERATED_ID, 0)
	add	esp, 16
	push	dword ptr [ebp - 16]
	push	4
	push	ebx
	push	eax
	call	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	add	esp, 16
	mov	dword ptr [esi + eax], edi
LBB15_5:
	mov	eax, esi
	add	esp, 12
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret

	.section	__DATA,__const
	.p2align	2, 0x0
l_anon.[ID].0:
	.long	SYM(core[CRATE_ID]::ptr::drop_in_place::<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::{closure#0}>::{closure#0}>, 0)
	.asciz	"\004\000\000\000\004\000\000"
	.long	SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0)
	.long	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::{closure#0}>::{closure#0}, 0)

	.section	__TEXT,__const
	.p2align	2, 0x0
l_anon.[ID].1:
	.byte	0

	.p2align	2, 0x0
l_anon.[ID].2:
	.byte	19
	.space	19

	.p2align	2, 0x0
l_anon.[ID].3:
	.byte	17
	.space	19

	.p2align	2, 0x0
l_anon.[ID].4:
	.space	1
	.space	19

l_anon.[ID].5:
	.ascii	"_NSZone"

	.section	__DATA,__const
	.p2align	2, 0x0
l_anon.[ID].6:
	.byte	28
	.space	3
	.long	l_anon.[ID].5
	.asciz	"\007\000\000"
	.long	l_anon.[ID].1
	.space	4

	.p2align	2, 0x0
l_anon.[ID].7:
	.byte	25
	.space	3
	.long	l_anon.[ID].6
	.space	12

	.section	__TEXT,__const
l_anon.[ID].8:
	.ascii	"called `Option::unwrap()` on a `None` value"

l_anon.[ID].9:
	.ascii	"$RUSTC/library/std/src/sync/once.rs"

	.section	__DATA,__const
	.p2align	2, 0x0
l_anon.[ID].10:
	.long	l_anon.[ID].9
	.asciz	"p\000\000\000\225\000\000\0002\000\000"

	.section	__TEXT,__literal4,4byte_literals
L_anon.[ID].11:
	.ascii	"_foo"

	.section	__TEXT,__const
	.p2align	2, 0x0
l_anon.[ID].12:
	.byte	5
	.space	19

	.section	__TEXT,__literal4,4byte_literals
L_anon.[ID].13:
	.ascii	"_obj"

	.section	__TEXT,__const
l_anon.[ID].14:
	.ascii	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	2, 0x0
l_anon.[ID].15:
	.long	l_anon.[ID].14
	.asciz	"5\000\000\000\f\000\000\000\001\000\000"

	.section	__TEXT,__const
l_anon.[ID].16:
	.ascii	"CustomClassName"

.zerofill __DATA,__bss,SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::REGISTER_CLASS, 0),4,2
l_anon.[ID].17:
	.ascii	"NSCopying"

l_anon.[ID].18:
	.ascii	"could not create new class "

l_anon.[ID].19:
	.ascii	". Perhaps a class with that name already exists?"

	.section	__DATA,__const
	.p2align	2, 0x0
l_anon.[ID].20:
	.long	l_anon.[ID].18
	.asciz	"\033\000\000"
	.long	l_anon.[ID].19
	.asciz	"0\000\000"

	.p2align	2, 0x0
l_anon.[ID].21:
	.long	l_anon.[ID].16
	.asciz	"\017\000\000"

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_d874ee9262978be2
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_d874ee9262978be2:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_d874ee9262978be2
L_OBJC_METH_VAR_NAME_d874ee9262978be2:
	.asciz	"classMethod"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_d874ee9262978be2
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_d874ee9262978be2:
	.long	L_OBJC_METH_VAR_NAME_d874ee9262978be2

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_4539fd1dbda0cddc
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_4539fd1dbda0cddc:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_4539fd1dbda0cddc
L_OBJC_METH_VAR_NAME_4539fd1dbda0cddc:
	.asciz	"method"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_4539fd1dbda0cddc
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_4539fd1dbda0cddc:
	.long	L_OBJC_METH_VAR_NAME_4539fd1dbda0cddc

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_2b1b3a94e0ece2e5
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_2b1b3a94e0ece2e5:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_2b1b3a94e0ece2e5
L_OBJC_METH_VAR_NAME_2b1b3a94e0ece2e5:
	.asciz	"methodBool:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_2b1b3a94e0ece2e5
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_2b1b3a94e0ece2e5:
	.long	L_OBJC_METH_VAR_NAME_2b1b3a94e0ece2e5

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_f7f521670860b0ce
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_f7f521670860b0ce:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_f7f521670860b0ce
L_OBJC_METH_VAR_NAME_f7f521670860b0ce:
	.asciz	"methodId"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_f7f521670860b0ce
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_f7f521670860b0ce:
	.long	L_OBJC_METH_VAR_NAME_f7f521670860b0ce

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_6addfcf634c6232f
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_6addfcf634c6232f:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_6addfcf634c6232f
L_OBJC_METH_VAR_NAME_6addfcf634c6232f:
	.asciz	"methodIdWithParam:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_6addfcf634c6232f
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_6addfcf634c6232f:
	.long	L_OBJC_METH_VAR_NAME_6addfcf634c6232f

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_4a8c690dbc9d8166
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_4a8c690dbc9d8166:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_4a8c690dbc9d8166
L_OBJC_METH_VAR_NAME_4a8c690dbc9d8166:
	.asciz	"copyWithZone:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_4a8c690dbc9d8166
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_4a8c690dbc9d8166:
	.long	L_OBJC_METH_VAR_NAME_4a8c690dbc9d8166

	.section	__IMPORT,__pointers,non_lazy_symbol_pointers
LL_OBJC_CLASSLIST_REFERENCES_$_NSObject$non_lazy_ptr:
	.indirect_symbol	L_OBJC_CLASSLIST_REFERENCES_$_NSObject
	.long	0
LL_OBJC_SELECTOR_REFERENCES_dealloc$non_lazy_ptr:
	.indirect_symbol	L_OBJC_SELECTOR_REFERENCES_dealloc
	.long	0
LL_OBJC_SELECTOR_REFERENCES_init$non_lazy_ptr:
	.indirect_symbol	L_OBJC_SELECTOR_REFERENCES_init
	.long	0
LL_OBJC_SELECTOR_REFERENCES_new$non_lazy_ptr:
	.indirect_symbol	L_OBJC_SELECTOR_REFERENCES_new
	.long	0

.subsections_via_symbols
