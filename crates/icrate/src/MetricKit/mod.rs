mod fixes;
#[path = "../generated/MetricKit/mod.rs"]
mod generated;

pub use self::fixes::*;
pub use self::generated::*;

#[link(name = "MetricKit", kind = "framework")]
extern "C" {}
