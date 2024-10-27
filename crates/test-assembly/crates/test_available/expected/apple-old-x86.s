	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
SYM(<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::initialize::<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::get_or_init<objc2[CRATE_ID]::__macro_helpers::os_version::apple::lookup_version>::{closure#0}, !>, 0):
	push	ebp
	mov	ebp, esp
	push	esi
	sub	esp, 20
	call	L0$pb
L0$pb:
	pop	ecx
	mov	eax, dword ptr [ecx + LSYM(objc2::__macro_helpers::os_version::apple::current_version::CURRENT_VERSION::GENERATED_ID, 0)$non_lazy_ptr-L0$pb]
	mov	edx, dword ptr [eax]
	cmp	edx, 3
	jne	LBB0_1
LBB0_2:
	add	esp, 20
	pop	esi
	pop	ebp
	ret
LBB0_1:
	lea	edx, [eax + 4]
	lea	esi, [ebp - 20]
	mov	dword ptr [esi], edx
	lea	edx, [ebp - 5]
	mov	dword ptr [esi + 4], edx
	lea	edx, [ebp - 12]
	mov	dword ptr [edx], esi
	sub	esp, 12
	lea	esi, [ecx + l_anon.[ID].2-L0$pb]
	lea	ecx, [ecx + l_anon.[ID].0-L0$pb]
	push	esi
	push	ecx
	push	edx
	push	1
	push	eax
	call	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	add	esp, 32
	jmp	LBB0_2

	.p2align	4, 0x90
SYM(<std[CRATE_ID]::sync::once::Once>::call_once_force::<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::initialize<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::get_or_init<objc2[CRATE_ID]::__macro_helpers::os_version::apple::lookup_version>::{closure#0}, !>::{closure#0}>::{closure#0}, 0):
	push	ebp
	mov	ebp, esp
	push	esi
	push	eax
	call	L1$pb
L1$pb:
	pop	eax
	mov	ecx, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ecx]
	mov	esi, dword ptr [ecx]
	mov	dword ptr [ecx], 0
	test	esi, esi
	je	LBB1_2
	call	SYM(objc2::__macro_helpers::os_version::apple::lookup_version::GENERATED_ID, 0)
	mov	dword ptr [esi], eax
	add	esp, 4
	pop	esi
	pop	ebp
	ret
LBB1_2:
	lea	eax, [eax + l_anon.[ID].3-L1$pb]
	mov	dword ptr [esp], eax
	call	SYM(core::option::unwrap_failed::GENERATED_ID, 0)

	.p2align	4, 0x90
SYM(<<std[CRATE_ID]::sync::once::Once>::call_once_force<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::initialize<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::get_or_init<objc2[CRATE_ID]::__macro_helpers::os_version::apple::lookup_version>::{closure#0}, !>::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0):
	push	ebp
	mov	ebp, esp
	push	esi
	push	eax
	call	L2$pb
L2$pb:
	pop	eax
	mov	ecx, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ecx]
	mov	esi, dword ptr [ecx]
	mov	dword ptr [ecx], 0
	test	esi, esi
	je	LBB2_2
	call	SYM(objc2::__macro_helpers::os_version::apple::lookup_version::GENERATED_ID, 0)
	mov	dword ptr [esi], eax
	add	esp, 4
	pop	esi
	pop	ebp
	ret
LBB2_2:
	lea	eax, [eax + l_anon.[ID].3-L2$pb]
	mov	dword ptr [esp], eax
	call	SYM(core::option::unwrap_failed::GENERATED_ID, 0)

	.globl	_always
	.p2align	4, 0x90
_always:
	push	ebp
	mov	ebp, esp
	mov	al, 1
	pop	ebp
	ret

	.globl	_never
	.p2align	4, 0x90
_never:
	push	ebp
	mov	ebp, esp
	xor	eax, eax
	pop	ebp
	ret

	.globl	_low
	.p2align	4, 0x90
_low:
	push	ebp
	mov	ebp, esp
	mov	al, 1
	pop	ebp
	ret

	.globl	_high
	.p2align	4, 0x90
_high:
	push	ebp
	mov	ebp, esp
	push	esi
	push	eax
	call	L6$pb
L6$pb:
	pop	eax
	mov	esi, dword ptr [eax + LSYM(objc2::__macro_helpers::os_version::apple::current_version::CURRENT_VERSION::GENERATED_ID, 0)$non_lazy_ptr-L6$pb]
	mov	eax, dword ptr [esi]
	cmp	eax, 3
	jne	LBB6_1
LBB6_2:
	cmp	word ptr [esi + 6], 15
	setae	al
	add	esp, 4
	pop	esi
	pop	ebp
	ret
LBB6_1:
	call	SYM(<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::initialize::<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::get_or_init<objc2[CRATE_ID]::__macro_helpers::os_version::apple::lookup_version>::{closure#0}, !>, 0)
	jmp	LBB6_2

	.globl	_only_ios
	.p2align	4, 0x90
_only_ios:
	push	ebp
	mov	ebp, esp
	xor	eax, eax
	pop	ebp
	ret

	.globl	_two_checks
	.p2align	4, 0x90
_two_checks:
	push	ebp
	mov	ebp, esp
	push	edi
	push	esi
	call	L8$pb
L8$pb:
	pop	eax
	mov	esi, dword ptr [eax + LSYM(objc2::__macro_helpers::os_version::apple::current_version::CURRENT_VERSION::GENERATED_ID, 0)$non_lazy_ptr-L8$pb]
	mov	eax, dword ptr [esi]
	cmp	eax, 3
	jne	LBB8_1
	movzx	edi, word ptr [esi + 6]
	mov	eax, dword ptr [esi]
	cmp	eax, 3
	jne	LBB8_3
LBB8_4:
	cmp	di, 14
	setae	cl
	cmp	word ptr [esi + 6], 15
	setae	al
	and	al, cl
	pop	esi
	pop	edi
	pop	ebp
	ret
LBB8_1:
	call	SYM(<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::initialize::<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::get_or_init<objc2[CRATE_ID]::__macro_helpers::os_version::apple::lookup_version>::{closure#0}, !>, 0)
	movzx	edi, word ptr [esi + 6]
	mov	eax, dword ptr [esi]
	cmp	eax, 3
	je	LBB8_4
LBB8_3:
	call	SYM(<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::initialize::<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::get_or_init<objc2[CRATE_ID]::__macro_helpers::os_version::apple::lookup_version>::{closure#0}, !>, 0)
	jmp	LBB8_4

	.section	__DATA,__const
	.p2align	2, 0x0
l_anon.[ID].0:
	.asciz	"\000\000\000\000\004\000\000\000\004\000\000"
	.long	SYM(<<std[CRATE_ID]::sync::once::Once>::call_once_force<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::initialize<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::get_or_init<objc2[CRATE_ID]::__macro_helpers::os_version::apple::lookup_version>::{closure#0}, !>::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0)
	.long	SYM(<std[CRATE_ID]::sync::once::Once>::call_once_force::<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::initialize<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::get_or_init<objc2[CRATE_ID]::__macro_helpers::os_version::apple::lookup_version>::{closure#0}, !>::{closure#0}>::{closure#0}, 0)

	.section	__TEXT,__const
l_anon.[ID].1:
	.ascii	"$RUSTC/library/std/src/sync/once.rs"

	.section	__DATA,__const
	.p2align	2, 0x0
l_anon.[ID].2:
	.long	l_anon.[ID].1
	.asciz	"p\000\000\000\331\000\000\000\024\000\000"

	.p2align	2, 0x0
l_anon.[ID].3:
	.long	l_anon.[ID].1
	.asciz	"p\000\000\000\331\000\000\0001\000\000"

	.section	__IMPORT,__pointers,non_lazy_symbol_pointers
LSYM(objc2::__macro_helpers::os_version::apple::current_version::CURRENT_VERSION::GENERATED_ID, 0)$non_lazy_ptr:
	.indirect_symbol	SYM(objc2::__macro_helpers::os_version::apple::current_version::CURRENT_VERSION::GENERATED_ID, 0)
	.long	0

.subsections_via_symbols
