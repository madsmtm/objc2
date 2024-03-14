use std::collections::BTreeSet;
use std::fmt;

use clang::{Entity, EntityKind};

use crate::file::clean_name;
use crate::id::ToOptionString;
use crate::stmt::parse_direct_protocols;
use crate::Mutability;
use crate::{stmt::parse_superclasses, Context, ItemIdentifier};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Feature<'a, N>(&'a ItemIdentifier<N>);

impl<'a, N: ToOptionString> Feature<'a, N> {
    pub fn new(item: &'a ItemIdentifier<N>) -> Self {
        Self(item)
    }

    pub fn name(&self) -> Option<String> {
        if self.0.is_system() {
            None
        } else if let Some(file_name) = &self.0.file_name {
            Some(format!("{}_{}", self.0.library, clean_name(file_name)))
        } else {
            Some(format!("Unknown-{}_{:?}", self.0.library, self.0.name))
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

    pub fn remove_item<N: ToOptionString>(&mut self, item: &ItemIdentifier<N>) {
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
                let id = ItemIdentifier::new(entity, context);
                let data = context.class_data.get(&id.name);

                let mut features = Self::new();
                features.add_item(&id);
                for (id, _, _) in parse_superclasses(entity, context) {
                    features.add_item(&id);
                }
                if let Some(Mutability::ImmutableWithMutableSubclass(subclass)) =
                    data.map(|data| &data.mutability)
                {
                    features.add_item(subclass);
                }

                features
            }
            EntityKind::ObjCProtocolDecl => {
                let mut features = Self::new();
                features.add_item(&ItemIdentifier::new(entity, context));
                for entity in parse_direct_protocols(entity, context) {
                    features.merge(Self::required_by_decl(&entity, context));
                }
                features
            }
            _ => panic!("invalid required_by_decl kind {entity:?}"),
        }
    }

    /// Get the features implied enabled by having a given declaration.
    pub(crate) fn implied_by_decl(entity: &Entity<'_>, context: &Context<'_>) -> Self {
        // No extra features are implied
        Self::required_by_decl(entity, context)
    }
}
