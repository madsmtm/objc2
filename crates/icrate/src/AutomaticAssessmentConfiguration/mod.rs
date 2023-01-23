#[path = "../generated/AutomaticAssessmentConfiguration/mod.rs"]
mod generated;

pub use self::generated::*;

#[link(name = "AutomaticAssessmentConfiguration", kind = "framework")]
extern "C" {}
