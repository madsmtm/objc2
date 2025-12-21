use std::path::PathBuf;

use apple_sdk::DeveloperDirectory;
use base64::{engine::general_purpose::URL_SAFE, Engine};
use serde::Deserialize;
use sha1::Digest;

#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Debug)]
pub enum Kind {
    Root,
    Article,
    Tutorial,
    ConceptsTutorial,
    Hub,
    Overview,
    Module,
    Class,
    Struct,
    Protocol,
    Enum,
    Function,
    GlobalVariable,
    Typedef,
    AssociatedType, // Swift only
    SwiftOp,
    Macro,
    Union,
    EnumCase,
    InitMethod, // Swift only
    Method,
    Property,
    Node, // "ShaderGraph Node"
    Const,
    SwiftSubscriptOp,
    TypeMethod,   // Swift only
    TypeProperty, // Swift only
    PropertyListKey,
    SampleCode,
    Endpoint, // "Web Service Endpoint"
    Object,
    Namespace,
    LangHeader,
    GroupMarker,
}

c_enum::c_enum! {
    #[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Deserialize)]
    #[repr(transparent)]
    pub enum Lang: u8 {
        Other = 0x00,
        Swift = 0x01,
        ObjectiveC = 0x02,
        Data = 0x04,
        // 0xFF = Root
    }
}

/// Get `/Applications/Xcode.app/Contents/SharedFrameworks/DNTDocumentationSupport.framework/Versions/A/Resources/external`.
pub fn external_dir() -> PathBuf {
    let developer_dir = DeveloperDirectory::from_env()
        .unwrap()
        .or_else(|| DeveloperDirectory::from_xcode_select_paths().unwrap())
        .or_else(DeveloperDirectory::default_xcode)
        .expect("could not find developer directory. Pass DEVELOPER_DIR=...");

    // TODO: This assumes the developer dir is inside Xcode, emit an error
    // when the developer dir is a commandline tools installation.
    developer_dir
        .path()
        .join("../SharedFrameworks/DNTDocumentationSupport.framework/Versions/A/Resources/external")
}

/// Convert a URL to a short "UUID".
///
/// NOTE: The return values is only probibalistically unique, not guaranteed
/// to be so universally, but that's what Apple calls it, and I guess it's
/// unique enough for the purposes in here.
///
/// # Examples
///
/// ```
/// use apple_doc::url_to_uuid;
/// assert_eq!(url_to_uuid("swift/documentation/objectivec/nsobject-swift.class"), "lsYDQ_QV9T");
/// assert_eq!(url_to_uuid("objective-c/documentation/objectivec/nsobject-swift.class"), "lcYDQ_QV9T");
/// ```
pub fn url_to_uuid(url: &str) -> String {
    let (prefix, component) = url.split_once('/').unwrap();

    let prefix = match prefix {
        "swift" => "ls",
        "objective-c" => "lc",
        "data" => "ld",
        "other" => "lo",
        prefix => panic!("unknown URL prefix: {prefix:?}"),
    };

    // Truncated 6 bytes of the SHA1 hash.
    let mut hasher = sha1::Sha1::new();
    hasher.update("/");
    hasher.update(component);
    let result = hasher.finalize();
    let result = &result[..6];

    // Base64 encode the string (with alphabet containing `-` and `_`).
    format!("{prefix}{}", URL_SAFE.encode(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid() {
        assert_eq!(
            url_to_uuid("objective-c/documentation/appkit/nsview"),
            "lcsJ_pkQS_"
        );
    }
}
