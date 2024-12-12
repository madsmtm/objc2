use std::collections::HashMap;
use std::ops;

use clang::Entity;

use crate::config::Config;
use crate::ItemIdentifier;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MacroLocation {
    file_id: Option<(u64, u64, u64)>,
    line: u32,
    column: u32,
    offset: u32,
}

impl MacroLocation {
    pub fn from_location(location: &clang::source::SourceLocation<'_>) -> Self {
        let clang::source::Location {
            file,
            line,
            column,
            offset,
        } = location.get_expansion_location();
        Self {
            file_id: file.map(|f| f.get_id()),
            line,
            column,
            offset,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MacroEntity {
    pub(crate) id: ItemIdentifier,
    pub(crate) is_function_like: bool,
}

impl MacroEntity {
    pub fn from_entity(entity: &Entity<'_>, context: &Context<'_>) -> Self {
        Self {
            id: ItemIdentifier::new(entity, context),
            is_function_like: entity.is_function_like_macro(),
        }
    }
}

pub struct Context<'config> {
    config: &'config Config,
    pub macro_invocations: HashMap<MacroLocation, MacroEntity>,
}

impl<'config> Context<'config> {
    pub fn new(config: &'config Config) -> Self {
        Self {
            config,
            macro_invocations: Default::default(),
        }
    }
}

impl ops::Deref for Context<'_> {
    type Target = Config;

    fn deref(&self) -> &Self::Target {
        self.config
    }
}
