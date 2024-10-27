	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
SYM(<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::initialize::<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::get_or_init<objc2[CRATE_ID]::__macro_helpers::os_version::apple::lookup_version>::{closure#0}, !>, 0):
	mov	rdi, qword ptr [rip + SYM(objc2::__macro_helpers::os_version::apple::current_version::CURRENT_VERSION::GENERATED_ID, 0)@GOTPCREL]
	mov	rax, qword ptr [rdi]
	cmp	rax, 3
	jne	LBB0_1
	ret
LBB0_1:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 32
	lea	rax, [rdi + 8]
	lea	rcx, [rbp - 32]
	mov	qword ptr [rcx], rax
	lea	rax, [rbp - 1]
	mov	qword ptr [rcx + 8], rax
	lea	rdx, [rbp - 16]
	mov	qword ptr [rdx], rcx
	lea	rcx, [rip + l_anon.[ID].0]
	lea	r8, [rip + l_anon.[ID].2]
	push	1
	pop	rsi
	call	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	add	rsp, 32
	pop	rbp
	ret

	.p2align	4, 0x90
SYM(<std[CRATE_ID]::sync::once::Once>::call_once_force::<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::initialize<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::get_or_init<objc2[CRATE_ID]::__macro_helpers::os_version::apple::lookup_version>::{closure#0}, !>::{closure#0}>::{closure#0}, 0):
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	rax, qword ptr [rdi]
	mov	rbx, qword ptr [rax]
	mov	qword ptr [rax], 0
	test	rbx, rbx
	je	LBB1_2
	call	SYM(objc2::__macro_helpers::os_version::apple::lookup_version::GENERATED_ID, 0)
	mov	dword ptr [rbx], eax
	add	rsp, 8
	pop	rbx
	pop	rbp
	ret
LBB1_2:
	lea	rdi, [rip + l_anon.[ID].3]
	call	SYM(core::option::unwrap_failed::GENERATED_ID, 0)

	.p2align	4, 0x90
SYM(<<std[CRATE_ID]::sync::once::Once>::call_once_force<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::initialize<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::get_or_init<objc2[CRATE_ID]::__macro_helpers::os_version::apple::lookup_version>::{closure#0}, !>::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0):
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	rax, qword ptr [rdi]
	mov	rbx, qword ptr [rax]
	mov	qword ptr [rax], 0
	test	rbx, rbx
	je	LBB2_2
	call	SYM(objc2::__macro_helpers::os_version::apple::lookup_version::GENERATED_ID, 0)
	mov	dword ptr [rbx], eax
	add	rsp, 8
	pop	rbx
	pop	rbp
	ret
LBB2_2:
	lea	rdi, [rip + l_anon.[ID].3]
	call	SYM(core::option::unwrap_failed::GENERATED_ID, 0)

	.globl	_always
	.p2align	4, 0x90
_always:
	push	rbp
	mov	rbp, rsp
	mov	al, 1
	pop	rbp
	ret

	.globl	_never
	.p2align	4, 0x90
_never:
	push	rbp
	mov	rbp, rsp
	xor	eax, eax
	pop	rbp
	ret

	.globl	_low
	.p2align	4, 0x90
_low:
	push	rbp
	mov	rbp, rsp
	mov	al, 1
	pop	rbp
	ret

	.globl	_high
	.p2align	4, 0x90
_high:
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	rbx, qword ptr [rip + SYM(objc2::__macro_helpers::os_version::apple::current_version::CURRENT_VERSION::GENERATED_ID, 0)@GOTPCREL]
	mov	rax, qword ptr [rbx]
	cmp	rax, 3
	jne	LBB6_1
LBB6_2:
	cmp	word ptr [rbx + 10], 15
	setae	al
	add	rsp, 8
	pop	rbx
	pop	rbp
	ret
LBB6_1:
	call	SYM(<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::initialize::<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::get_or_init<objc2[CRATE_ID]::__macro_helpers::os_version::apple::lookup_version>::{closure#0}, !>, 0)
	jmp	LBB6_2

	.globl	_only_ios
	.p2align	4, 0x90
_only_ios:
	push	rbp
	mov	rbp, rsp
	xor	eax, eax
	pop	rbp
	ret

	.globl	_two_checks
	.p2align	4, 0x90
_two_checks:
	push	rbp
	mov	rbp, rsp
	push	r14
	push	rbx
	mov	rbx, qword ptr [rip + SYM(objc2::__macro_helpers::os_version::apple::current_version::CURRENT_VERSION::GENERATED_ID, 0)@GOTPCREL]
	mov	rax, qword ptr [rbx]
	cmp	rax, 3
	jne	LBB8_1
	movzx	r14d, word ptr [rbx + 10]
	mov	rax, qword ptr [rbx]
	cmp	rax, 3
	jne	LBB8_3
LBB8_4:
	cmp	r14w, 14
	setae	cl
	cmp	word ptr [rbx + 10], 15
	setae	al
	and	al, cl
	pop	rbx
	pop	r14
	pop	rbp
	ret
LBB8_1:
	call	SYM(<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::initialize::<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::get_or_init<objc2[CRATE_ID]::__macro_helpers::os_version::apple::lookup_version>::{closure#0}, !>, 0)
	movzx	r14d, word ptr [rbx + 10]
	mov	rax, qword ptr [rbx]
	cmp	rax, 3
	je	LBB8_4
LBB8_3:
	call	SYM(<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::initialize::<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::get_or_init<objc2[CRATE_ID]::__macro_helpers::os_version::apple::lookup_version>::{closure#0}, !>, 0)
	jmp	LBB8_4

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].0:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	SYM(<<std[CRATE_ID]::sync::once::Once>::call_once_force<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::initialize<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::get_or_init<objc2[CRATE_ID]::__macro_helpers::os_version::apple::lookup_version>::{closure#0}, !>::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0)
	.quad	SYM(<std[CRATE_ID]::sync::once::Once>::call_once_force::<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::initialize<<std[CRATE_ID]::sync::once_lock::OnceLock<objc2[CRATE_ID]::__macro_helpers::os_version::OSVersion>>::get_or_init<objc2[CRATE_ID]::__macro_helpers::os_version::apple::lookup_version>::{closure#0}, !>::{closure#0}>::{closure#0}, 0)

	.section	__TEXT,__const
l_anon.[ID].1:
	.ascii	"$RUSTC/library/std/src/sync/once.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].2:
	.quad	l_anon.[ID].1
	.asciz	"p\000\000\000\000\000\000\000\331\000\000\000\024\000\000"

	.p2align	3, 0x0
l_anon.[ID].3:
	.quad	l_anon.[ID].1
	.asciz	"p\000\000\000\000\000\000\000\331\000\000\0001\000\000"

.subsections_via_symbols
