use std::collections::BTreeSet;
use std::fmt;

use clang::{Entity, EntityKind};

use crate::{stmt::parse_superclasses, Context, ItemIdentifier};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Feature<'a>(&'a ItemIdentifier);

impl<'a> Feature<'a> {
    pub fn new(item: &'a ItemIdentifier) -> Self {
        Self(item)
    }

    pub fn name(&self) -> Option<String> {
        if self.0.is_system() {
            None
        } else {
            Some(format!("{}_{}", self.0.library, self.0.name))
        }
    }
}

// Use a set to deduplicate features, and to have them in
// a consistent order.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Features(BTreeSet<String>);

impl Features {
    pub fn new() -> Self {
        Self(BTreeSet::new())
    }

    pub fn add_item(&mut self, item: &ItemIdentifier) {
        if let Some(name) = Feature::new(item).name() {
            self.0.insert(name);
        }
    }

    pub fn remove_item(&mut self, item: &ItemIdentifier) {
        if let Some(name) = Feature::new(item).name() {
            self.0.remove(&name);
        }
    }

    pub fn merge(&mut self, features: Features) {
        self.0.extend(features.0);
    }

    pub fn remove(&mut self, features: &Features) {
        for name in &features.0 {
            self.0.remove(name);
        }
    }

    pub fn cfg_gate_ln(&self) -> impl fmt::Display + '_ {
        struct Inner<'a>(&'a BTreeSet<String>);

        impl fmt::Display for Inner<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self.0.len() {
                    0 => Ok(()),
                    1 => {
                        let feature = self.0.first().unwrap();
                        writeln!(f, "#[cfg(feature = \"{feature}\")]")
                    }
                    _ => {
                        write!(f, "#[cfg(all(")?;

                        for (i, feature) in self.0.iter().enumerate() {
                            if i != 0 {
                                write!(f, ", ")?;
                            }
                            write!(f, "feature = \"{feature}\"")?;
                        }

                        write!(f, "))]")?;
                        writeln!(f)?;

                        Ok(())
                    }
                }
            }
        }

        Inner(&self.0)
    }

    /// Get the features required for a given declaration to be enabled.
    pub(crate) fn required_by_decl(entity: &Entity<'_>, context: &Context<'_>) -> Self {
        match entity.get_kind() {
            EntityKind::ObjCInterfaceDecl => {
                let mut features = Self::new();
                features.add_item(&ItemIdentifier::new(entity, context));
                features
            }
            EntityKind::ObjCProtocolDecl => Self::new(),
            _ => panic!("invalid required_by_decl kind {entity:?}"),
        }
    }

    /// Get the features implied enabled by having a given declaration.
    pub(crate) fn implied_by_decl(entity: &Entity<'_>, context: &Context<'_>) -> Self {
        let mut features = Self::required_by_decl(entity, context);
        match entity.get_kind() {
            EntityKind::ObjCInterfaceDecl => {
                // The feature is enabled if superclass' features are.
                for (id, _, _) in parse_superclasses(entity, context) {
                    features.add_item(&id);
                }
                features
            }
            EntityKind::ObjCProtocolDecl => features,
            _ => panic!("invalid implied_by_decl kind {entity:?}"),
        }
    }
}
