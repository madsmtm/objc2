//! # Bindings to the `Metal` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/metal/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
//!
//! Metal has tools for validating that you're using it correctly, using these
//! is highly recommended! See [Apple's documentation on it][apple-doc], or
//! run `man MetalValidation` to get information on environment variables.
//!
//! [apple-doc]: https://developer.apple.com/documentation/xcode/validating-your-apps-metal-api-usage/.
//!
//! NOTE: To use [`MTLCreateSystemDefaultDevice`] you need to link to
//! `CoreGraphics`, this can be done by using `objc2-core-graphics`, or by
//! doing:
//! ```rust
//! #[link(name = "CoreGraphics", kind = "framework")]
//! extern "C" {}
//! ```
//!
#![cfg_attr(
    not(feature = "MTLDevice"),
    doc = "[`MTLCreateSystemDefaultDevice`]: #needs-MTLDevice-feature"
)]
//!
//! # Precompiling shaders
//!
//! When you're testing things out, you can usually get by with using
//! [`MTLDevice::newLibraryWithSource_options_error`]. However, when you
//! actually want to ship your application to users, you should strongly
//! consider pre-compiling and bundling your shaders with your application.
//!
//! This can be done using something like:
//!
//! ```sh
//! xcrun -sdk macosx metal -c shaders.metal -o shaders.air
//! xcrun -sdk macosx metallib shaders.air -o shaders.metallib
//! ```
//!
//! TODO: Expand on this further.
//!
#![cfg_attr(
    not(feature = "MTLDevice"),
    doc = "[`MTLDevice::newLibraryWithSource_options_error`]: #needs-MTLDevice-feature"
)]
//!
//! # Metal 4 Support
//!
//! This crate includes comprehensive support for Metal 4, introduced at WWDC 2025.
//! Metal 4 provides significant new features including:
//!
//! - **Shader Direct Inference**: ML inference directly in Metal shaders
//! - **Explicit Memory Management**: Direct control via [`MTL4CommandAllocator`]
//! - **New Core APIs**: [`MTL4CommandQueue`], [`MTL4CommandBuffer`], [`MTL4Compiler`], etc.
//!
#![cfg_attr(
    not(feature = "std"),
    doc = "[`MTL4CommandAllocator`]: #needs-std-feature"
)]
//!
//! ## Version Requirements
//!
//! Metal 4 requires specific OS versions:
//!
//! - macOS 26.0+ (Tahoe)
//! - iOS 26.0+
//! - iPadOS 26.0+
//! - tvOS 26.0+
//! - visionOS 26.0+
//!
//! ## Availability Detection
//!
//! Always check for Metal 4 availability at runtime, as not all devices support it:
#![cfg_attr(
    feature = "std",
    doc = r##"
//! ```no_run
//! use objc2_metal::metal4_detection;
//!
//! if metal4_detection::is_metal4_available() {
//!     // Use Metal 4 features
//! } else {
//!     // Fall back to earlier Metal versions
//! }
//! ```
//! "##
)]
//!
#![cfg_attr(
    not(feature = "std"),
    doc = "//! ```ignore\n//! // Metal 4 detection requires the `std` feature\n//! ```\n"
)]
//!
//! Or use the [`available!`] macro for more granular control:
//!
//! ```no_run
//! use objc2::available;
//!
//! if available!(macos = 26.0, ios = 26.0, tvos = 26.0, visionos = 26.0) {
//!     // Use Metal 4 features
//! } else {
//!     // Fall back to earlier Metal versions
//! }
//! ```
//!
#![cfg_attr(
    not(feature = "std"),
    doc = "[`available!`]: https://docs.rs/objc2/latest/objc2/macro.available.html"
)]
//!
#![cfg_attr(
    feature = "std",
    doc = r##"//! [`available!`]: https://docs.rs/objc2/latest/objc2/macro.available.html
//! "##
)]
//!
//! ## Metal 4 Types
//!
//! The following Metal 4 types are available when the corresponding features are enabled:
//!
//! - **Command Pipeline**: [`MTL4CommandQueue`], [`MTL4CommandBuffer`], [`MTL4CommandAllocator`], [`MTL4CommandEncoder`]
//! - **Rendering**: [`MTL4RenderCommandEncoder`], [`MTL4RenderPass`], [`MTL4RenderPipeline`]
//! - **Compute**: [`MTL4ComputeCommandEncoder`], [`MTL4ComputePipeline`]
//! - **Compilation**: [`MTL4Compiler`], [`MTL4CompilerTask`], [`MTL4FunctionDescriptor`]
//! - **Resources**: [`MTL4ArgumentTable`], [`MTL4BufferRange`], [`MTL4AccelerationStructure`]
//! - **Machine Learning**: [`MTL4MachineLearningCommandEncoder`], [`MTL4MachineLearningPipeline`]
//!
#![cfg_attr(
    not(any(
        feature = "MTL4CommandQueue",
        feature = "MTL4CommandBuffer",
        feature = "MTL4CommandAllocator",
        feature = "MTL4Compiler"
    )),
    doc = "//! [`MTL4CommandQueue`]: #needs-MTL4CommandQueue-feature\n"
)]
//!
#![cfg_attr(
    not(feature = "std"),
    doc = r##"//! See [`metal4_detection`] module for runtime detection utilities.
//! "##
)]
//!
#![cfg_attr(
    feature = "std",
    doc = r##"//! See [`metal4_detection`] module for comprehensive runtime detection utilities.
//! "##
)]
//!
//! # Safety considerations
//!
//! Metal allows running arbitrary code on the GPU. We treat memory safety
//! issues on the GPU as just as unsafe as that which applies to the CPU. A
//! few notes on this below.
//!
//! ## Shaders
//!
//! Shaders are (often) written in an unsafe C-like language.
//!
//! Loading them (via `MTLLibrary`, function stitching etc.) is perfectly
//! safe, it is similar to dynamic linking. The restrictions that e.g.
//! `libloading::Library::new` labours under do not apply, since there are no
//! ctors in [the Metal Shading Language][msl-spec] (see section 4.2).
//!
//! Similarly, getting individual shaders (`MTLFunction`) is safe, we can
//! model this as the same as calling `dlsym` (which just returns a pointer).
//!
//! _Calling_ functions though, is not safe. Even though they can have their
//! parameter and return types checked at runtime, they may have additional
//! restrictions not present in the signature (e.g. `__builtin_unreachable()`
//! is possible in MSL, so is out-of-bounds accesses). If you view
//! `MTLFunction` as essentially just an `unsafe fn()` pointer, this should be
//! apparent.
//!
//! [msl-spec]: https://developer.apple.com/metal/Metal-Shading-Language-Specification.pdf
//!
//! ## Bounds checks
//!
//! It is yet unclear whether Metal APIs are bounds-checked on the CPU side or
//! not, so APIs that take offsets / lengths are often unsafe.
//!
//! ## Synchronization
//!
//! `MTLResource` subclasses such as `MTLBuffer` and `MTLTexture` require
//! synchronization between the CPU and the GPU, or between different threads
//! on the GPU itself, so APIs taking these are often unsafe.
//!
//! ## Memory management and lifetimes
//!
//! Resources used in `MTL4CommandBuffer`s or command buffers with created
//! with one of:
//! - `MTLCommandBufferDescriptor::setRetainedReferences(false)`.
//! - `MTLCommandQueue::commandBufferWithUnretainedReferences()`.
//!
//! Must be kept alive for as long as they're used.
//!
//! ## Type safety
//!
//! `MTLBuffer` is untyped (in a similar manner as a `[u8]` slice), you must
//! ensure that any usage of it is done with valid types.
#![recursion_limit = "256"]
#![allow(non_snake_case)]
#![no_std]
// Disabled so `--all-features` works on stable Rust.
// #![cfg_attr(feature = "unstable-darwin-objc", feature(darwin_objc))]
#![cfg_attr(docsrs, feature(doc_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-metal/0.3.2")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "MTLAccelerationStructureTypes")]
mod acceleration_structure_types;
#[cfg(feature = "MTLCaptureManager")]
mod capture;
#[cfg(feature = "MTLCounters")]
mod counters;
#[cfg(feature = "MTLDevice")]
mod device;
mod generated;

// Metal 4 detection utilities (always available when std feature is enabled)
#[cfg(feature = "std")]
pub mod metal4_detection;
#[cfg(feature = "unstable-private")]
mod private;
#[cfg(feature = "MTLRasterizationRate")]
mod rasterization_rate;
#[cfg(feature = "MTLResource")]
mod resource;
mod slice;
#[cfg(feature = "MTLTexture")]
mod texture;
#[cfg(feature = "MTLTypes")]
mod types;

#[cfg(feature = "MTLAccelerationStructureTypes")]
pub use self::acceleration_structure_types::MTLPackedFloat3;
#[cfg(feature = "MTLCounters")]
pub use self::counters::*;
#[cfg(feature = "MTLDevice")]
pub use self::device::*;

// Re-export Metal 4 detection utilities
#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;
#[cfg(feature = "std")]
pub use self::metal4_detection::*;
#[cfg(feature = "unstable-private")]
pub use self::private::MTLDevicePrivate;
#[cfg(feature = "MTLResource")]
pub use self::resource::*;
#[cfg(all(feature = "MTLRenderCommandEncoder", feature = "MTLCommandEncoder"))]
pub use self::slice::MTLRenderCommandEncoderSliceExt;
#[cfg(feature = "MTLTexture")]
pub use self::texture::*;
#[cfg(feature = "MTLTypes")]
pub use self::types::MTLResourceID;

// mach/mach_types.h
#[allow(dead_code, non_camel_case_types)]
#[cfg(feature = "libc")]
pub(crate) type task_id_token_t = libc::mach_port_t;
