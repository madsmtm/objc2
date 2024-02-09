//! # Bindings to the `MetricKit` framework
#![allow(non_snake_case)]
pub use crate::generated::MetricKit::*;

#[cfg(feature = "MetricKit_MXMetricManager")]
mod manager;

#[cfg(feature = "MetricKit_MXMetricManager")]
pub use self::manager::*;
