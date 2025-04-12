//! BEMemory.h
//!
//! Refer GCC's documentation on `__asm__`:
//! <https://gcc.gnu.org/onlinedocs/gcc-4.7.2/gcc/Extended-Asm.html>
//!
//! And to LLVM's assembly documentation:
//! <https://llvm.org/docs/LangRef.html#inline-assembler-expressions>
//!
//! And of course Rust's own `asm!` macro documentation:
//! <https://doc.rust-lang.org/reference/inline-assembly.html>.
//! <https://doc.rust-lang.org/rust-by-example/unsafe/asm.html>

use core::arch::asm;
use core::ffi::c_int;

/// Returns `true` iff the inlinable version of the jit_write_protect API is available.
#[inline]
pub fn be_memory_inline_jit_restrict_with_witness_supported() -> bool {
    extern "C" {
        fn be_memory_inline_jit_restrict_with_witness_supported() -> c_int;
    }

    unsafe { be_memory_inline_jit_restrict_with_witness_supported() != 0 }
}

/// These functions that can toggle JIT R^X permissions, while enforcing
/// control flow integrity using PAC. This function is intended to be used
/// only in performance-critical sections of code. Please consult
/// [the documentation] extensively before relying on this API.
///
/// This function should always be inlined. It will sign the return lr pointer
/// as a witness using PAC, and then call into the implementation.
///
/// The direct implementation is not permitted to be inlined into app binaries
/// because the instruction sequence required may change in the future.
///
/// Instead, we use a custom calling convention. This serves two purposes:
/// 1) Performance. The effects of this call can be precisely modeled, so
///    the performance is as close as possible to the case where we can
///    inline this body directly.
/// 2) Security. This calling convention greatly reduces the chances of spilling
///    an important value to the stack. See below.
///
/// In support of this function's security goals, users of this function must
/// confirm the following:
///
/// 1) You must not emit general PAC signing gadgets.
/// 2) The critical section defined by calls to these two functions must not
///    loop on any induction variable that is spilled to the stack, or otherwise
///    spill any important values there. An attacker can control these values.
///    If any values must be spilled or loaded from the heap or stack, they should be signed
///    outside the critical section, and authenticated inside.
/// 3) Do not create a gadget that is overly general. For example, wrapping this
///    function inside a body like this is very bad:
///
///    ```c
///    NEVER_INLINE void myFunction() {
///    if (variableFromStackOrHeap)
///        be_memory_inline_jit_restrict_rwx_to_rx_with_witness();
///    else
///        be_memory_inline_jit_restrict_rwx_to_rw_with_witness();
///    }
///    ```
///
///    Each use of this function should be maximally inlined to reduce the power
///    that attackers gain by manipulating calls to it.
///
/// Please look at the WebKit source code for an example.
///
/// [the documentation]: https://developer.apple.com/documentation/browserenginekit/protecting-code-compiled-just-in-time?language=objc
#[inline(always)]
pub extern "C" fn be_memory_inline_jit_restrict_rwx_to_rw_with_witness() {
    // SAFETY: This is correctly translated from C, see comments below.
    //
    // The C code for this is as follows:
    //
    // ```c
    // // This is the globally unique diversifier used in the implementation
    // // of be_memory_inline_jit_restrict_rwx_to_rw_with_witness_impl
    // // Ensure that your implementation does not use this diversifier for anything else.
    // #define BE_JIT_WRITE_PROTECT_TAG 0x4a4954l
    //
    // #define _BE_STRINGIZE(exp) #exp
    // #define _BE_STRINGIZE_VALUE_OF(exp) _BE_STRINGIZE(exp)
    // #define _BE_SYMBOL_STRING(name) "_" #name
    //
    // __attribute__((preserve_all))
    // __attribute__((always_inline))
    // __attribute__((visibility("hidden")))
    // __attribute__((direct_call_only))
    // inline
    // void be_memory_inline_jit_restrict_rwx_to_rw_with_witness(void)
    // {
    //     __asm__ volatile (
    // #ifdef __arm64e__
    //     ".arch_extension pauth" "\n"
    // #endif
    //     "adr x0, %=f" "\n"
    //
    //     "movz x1, #" _BE_STRINGIZE_VALUE_OF(((BE_JIT_WRITE_PROTECT_TAG >>  0) & 0xFFFF)) ", lsl #0 \n"
    //     "movk x1, #" _BE_STRINGIZE_VALUE_OF(((BE_JIT_WRITE_PROTECT_TAG >> 16) & 0xFFFF)) ", lsl #16\n"
    //
    // #ifdef __arm64e__
    //     "pacib x0, x1" "\n"
    // #endif
    //
    //     "bl " _BE_SYMBOL_STRING(be_memory_inline_jit_restrict_rwx_to_rw_with_witness_impl) "\n"
    //     "%=:" "\n"
    //     "nop" "\n"
    //     : /* no output */
    //     : /* no input */
    //     : "r0", "r1", "r2", "r3", "r4", "r5", "r6", "r7", "r8", "r16", "r17", "lr", "memory", "cc"
    //     );
    // }
    // ```
    //
    // For the function attributes, we have:
    // - `preserve_all` doesn't actually affect the ABI for our purposes,
    //   just coerces the compiler to do less if the function isn't inlined.
    //   Should be irrelevant, since... \/
    // - `always_inline` forces the function to be inlined. We do the same by
    //   marking this with `#[inline(always)]`.
    // - `visibility("hidden")` makes the symbol not show up when creating
    //   dynamic libraries. This should be irrelevant, since we force the
    //   inlining, and that works differently in Rust?
    //
    //   FIXME: Should we convert this to a macro, to _ensure_ it's always
    //   inlined? Although that causes problems with the assembly.
    // - `direct_call_only` is a new Clang attribute, only used for
    //   diagnostics to help avoid mistakes when calling the function.
    unsafe {
        // The difference on `__arm64e__` means we have to duplicate the
        // entire assembly block (since we don't want to emit partial blocks).
        if cfg!(arm64e) {
            asm!(
                ".arch_extension pauth", // __arm64__
                // The C code uses `%=f` to generate a reference to an
                // anonymous label. We use numeric local labels to achieve the
                // same effect, as documented in:
                // <https://doc.rust-lang.org/rust-by-example/unsafe/asm.html#labels>
                "adr x0, 2f",
                // We inline the macro, as `const` in asm! is not in our MSRV.
                //
                // The assembler will take care of simplifying the expressions
                // to `#18772` and `#74`.
                "movz x1, #((0x4a4954l >>  0) & 0xFFFF), lsl #0",
                "movk x1, #((0x4a4954l >> 16) & 0xFFFF), lsl #16",
                "pacib x0, x1", // __arm64__
                // Again, we inline the macro call.
                "bl _be_memory_inline_jit_restrict_rwx_to_rw_with_witness_impl",
                // `%=:` marks the label we referred to above.
                "2:",
                "nop",
                // Register clobbers.
                out("x0") _,
                out("x1") _,
                out("x2") _,
                out("x3") _,
                out("x4") _,
                out("x5") _,
                out("x6") _,
                out("x7") _,
                out("x8") _,
                out("x16") _,
                out("x17") _,
                out("lr") _,
                // Other clobbers/attributes:
                // - `volatile` means we cannot add `pure`/`readonly`.
                // - `memory` clobber means we cannot add `nomem`.
                // - `cc` clobber means we cannot add `preserves_flags`.
                //
                // The lack of an `unwind` clobber also makes `extern "C"`
                // correct, as this is guaranteed to not unwind.
                options(nostack),
            )
        } else {
            asm!(
                // ".arch_extension pauth",
                "adr x0, 2f",
                "movz x1, #((0x4a4954l >>  0) & 0xFFFF), lsl #0",
                "movk x1, #((0x4a4954l >> 16) & 0xFFFF), lsl #16",
                // "pacib x0, x1",
                "bl _be_memory_inline_jit_restrict_rwx_to_rw_with_witness_impl",
                "2:",
                "nop",
                out("x0") _,
                out("x1") _,
                out("x2") _,
                out("x3") _,
                out("x4") _,
                out("x5") _,
                out("x6") _,
                out("x7") _,
                out("x8") _,
                out("x16") _,
                out("x17") _,
                out("lr") _,
                options(nostack),
            )
        }
    }
}

/// See [`be_memory_inline_jit_restrict_rwx_to_rw_with_witness`].
#[inline(always)]
pub extern "C" fn be_memory_inline_jit_restrict_rwx_to_rx_with_witness() {
    // SAFETY: See `be_memory_inline_jit_restrict_rwx_to_rw_with_witness`,
    // this works mostly the same, only simpler.
    //
    // The C code is as follows (for reference).
    //
    // ```c
    // #define _BE_SYMBOL_STRING(name) "_" #name
    //
    // __attribute__((preserve_all))
    // __attribute__((always_inline))
    // __attribute__((visibility("hidden")))
    // __attribute__((direct_call_only))
    // inline
    // void be_memory_inline_jit_restrict_rwx_to_rx_with_witness(void)
    // {
    //     __asm__ volatile (
    //     "bl " _BE_SYMBOL_STRING(be_memory_inline_jit_restrict_rwx_to_rx_with_witness_impl) "\n"
    //     "%=:" "\n"
    //     "nop" "\n"
    //     : /* no output */
    //     : /* no input */
    //     : "r0", "r1", "r2", "r3", "r4", "r5", "r6", "r7", "r8", "r16", "r17", "lr", "memory", "cc"
    //     );
    // }
    // ```
    unsafe {
        asm!(
            // "bl " _BE_SYMBOL_STRING(be_memory_inline_jit_restrict_rwx_to_rx_with_witness_impl) "\n"
            "bl _be_memory_inline_jit_restrict_rwx_to_rx_with_witness_impl",
            // "%=:" "\n"
            "2:",
            // "nop" "\n"
            "nop",
            // Clobbers
            // "r0", "r1", "r2", "r3", "r4", "r5", "r6", "r7", "r8", "r16", "r17", "lr"
            out("x0") _,
            out("x1") _,
            out("x2") _,
            out("x3") _,
            out("x4") _,
            out("x5") _,
            out("x6") _,
            out("x7") _,
            out("x8") _,
            out("x16") _,
            out("x17") _,
            out("lr") _,
            // no output
            // no input
            // "memory", "cc"
            options(nostack),
        )
    };
}
