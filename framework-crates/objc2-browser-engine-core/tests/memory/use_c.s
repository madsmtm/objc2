	.section	__TEXT,__text,regular,pure_instructions
	.globl	_rw                             ; -- Begin function rw
	.p2align	2
_rw:                                    ; @rw
	.cfi_startproc
; %bb.0:
	stp	x29, x30, [sp, #-16]!           ; 16-byte Folded Spill
	.cfi_def_cfa_offset 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	; InlineAsm Start
	adr	x0, Ltmp0
	mov	x1, #18772                      ; =0x4954
	movk	x1, #74, lsl #16
	bl	_be_memory_inline_jit_restrict_rwx_to_rw_with_witness_impl
Ltmp0:
	nop

	; InlineAsm End
	ldp	x29, x30, [sp], #16             ; 16-byte Folded Reload
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_rx                             ; -- Begin function rx
	.p2align	2
_rx:                                    ; @rx
	.cfi_startproc
; %bb.0:
	stp	x29, x30, [sp, #-16]!           ; 16-byte Folded Spill
	.cfi_def_cfa_offset 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	; InlineAsm Start
	bl	_be_memory_inline_jit_restrict_rwx_to_rx_with_witness_impl
Ltmp1:
	nop

	; InlineAsm End
	ldp	x29, x30, [sp], #16             ; 16-byte Folded Reload
	ret
	.cfi_endproc
                                        ; -- End function
.subsections_via_symbols
