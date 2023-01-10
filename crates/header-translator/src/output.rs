use std::collections::BTreeMap;

use crate::library::Library;

#[derive(Debug, PartialEq)]
pub struct Output {
    pub libraries: BTreeMap<String, Library>,
}

impl Output {
    pub fn from_libraries(libraries: impl IntoIterator<Item = impl Into<String>>) -> Self {
        let libraries = libraries
            .into_iter()
            .map(|name| (name.into(), Library::new()))
            .collect();
        Self { libraries }
    }

    pub fn compare(&self, other: &Self) {
        super::compare_btree(
            &self.libraries,
            &other.libraries,
            |libary_name, self_library, other_library| {
                let _span = debug_span!("library", libary_name).entered();
                self_library.compare(other_library);
            },
        );
    }
}
