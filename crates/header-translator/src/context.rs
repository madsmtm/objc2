use std::collections::HashMap;
use std::ops;

use clang::source::Location;

use crate::config::Config;

pub struct Context<'a> {
    config: &'a Config,
    pub macro_invocations: HashMap<Location<'a>, String>,
}

impl<'a> Context<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            macro_invocations: Default::default(),
        }
    }
}

impl ops::Deref for Context<'_> {
    type Target = Config;

    fn deref(&self) -> &Self::Target {
        &self.config
    }
}
