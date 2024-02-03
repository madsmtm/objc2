use std::collections::BTreeSet;
use std::fmt;

use crate::ItemIdentifier;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Feature<'a>(&'a ItemIdentifier);

impl<'a> Feature<'a> {
    pub fn new(item: &'a ItemIdentifier) -> Self {
        Self(item)
    }

    pub fn cfg_gate_ln(&self) -> impl fmt::Display + '_ {
        struct Inner<'a>(&'a ItemIdentifier);

        impl fmt::Display for Inner<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                if self.0.is_system() {
                    Ok(())
                } else {
                    writeln!(
                        f,
                        "#[cfg(feature = \"{}_{}\")]",
                        self.0.library, self.0.name
                    )
                }
            }
        }

        Inner(self.0)
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
}
