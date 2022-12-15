use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub struct Library {
    files: BTreeMap<String, File>,
}



#[derive(Debug, PartialEq)]
pub struct Libraries {
    libraries: BTreeMap<String, Library>,
}
