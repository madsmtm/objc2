pub use self::comparison_result::NSComparisonResult;
pub use self::geometry::{
    CGFloat, CGPoint, CGRect, CGSize, NSMaxXEdge, NSMaxYEdge, NSMinXEdge, NSMinYEdge, NSPoint,
    NSRect, NSRectEdge, NSRectEdgeMaxX, NSRectEdgeMaxY, NSRectEdgeMinX, NSRectEdgeMinY, NSSize,
};
pub use self::range::NSRange;
#[cfg(feature = "Foundation_NSThread")]
#[cfg(feature = "dispatch")]
pub use self::thread::MainThreadBound;
#[cfg(feature = "Foundation_NSThread")]
pub use self::thread::{is_main_thread, is_multi_threaded, MainThreadMarker};

#[macro_use]
mod iter;
pub mod array;
mod attributed_string;
mod bundle;
mod comparison_result;
mod data;
pub mod dictionary;
pub mod enumerator;
mod error;
mod exception;
mod geometry;
mod number;
mod process_info;
mod range;
pub mod set;
mod string;
mod thread;
mod util;
mod uuid;
mod value;
