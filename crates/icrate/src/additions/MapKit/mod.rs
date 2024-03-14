//! # Bindings to the `MapKit` framework

pub use crate::generated::MapKit::*;

#[cfg(all(feature = "Foundation_NSObject", feature = "MapKit_MKMapItem"))]
unsafe impl crate::Foundation::NSCoding for crate::MapKit::MKMapItem {}
