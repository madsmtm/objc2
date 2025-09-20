//! # Bindings to the `OpenGL` framework
//!
//! Note that this crate intentionally leaves out the cross-platform parts of
//! the OpenGL API. See crates like `gl` or `gl_generator` for that instead.
//!
//! See [the OpenGL programming guide][opengl-docs] and [the general docs on
//! framework crates][framework-crates] for more information.
//!
//! [opengl-docs]: https://developer.apple.com/library/archive/documentation/GraphicsImaging/Conceptual/OpenGL-MacProgGuide/opengl_intro/opengl_intro.html
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
#![no_std]
#![cfg_attr(feature = "unstable-darwin-objc", feature(darwin_objc))]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-opengl/0.3.1")]
#![deprecated = "The OpenGL API is deprecated by Apple"]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod generated;
#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;

// OpenGL/gltypes.h
#[allow(unused)]
pub(crate) type GLbitfield = u32;
#[allow(unused)]
pub(crate) type GLenum = u32;
#[allow(unused)]
pub(crate) type GLint = i32;
#[allow(unused)]
pub(crate) type GLsizei = i32;
#[allow(unused)]
pub(crate) type GLuint = u32;

// OpenGL/CGLContext.h
#[allow(unused)]
mod context {
    use core::cell::UnsafeCell;
    use core::marker::{PhantomData, PhantomPinned};
    use objc2::encode::{Encoding, RefEncode};

    #[repr(C)]
    #[derive(Debug)]
    #[allow(missing_copy_implementations)]
    #[allow(unreachable_pub)]
    pub struct _CGLContextObject {
        inner: [u8; 0],
        _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
    }

    unsafe impl RefEncode for _CGLContextObject {
        const ENCODING_REF: Encoding =
            Encoding::Pointer(&Encoding::Struct("_CGLContextObject", &[]));
    }
}

#[allow(unused)]
pub(crate) use self::context::_CGLContextObject;
