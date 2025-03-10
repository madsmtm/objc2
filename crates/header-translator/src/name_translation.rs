//! # Name translation algorithms
//!
//! See <https://github.com/swiftlang/swift/blob/swift-6.0.3-RELEASE/docs/CToSwiftNameTranslation.md>.
//!
//! Kinda ugly and under-tested, may not work for all cases.

use std::iter::FusedIterator;

/// Split a string according to Swift's word boundary algorithm.
///
/// The docs says:
///
/// > The word-splitting algorithm used by the Swift compiler is as follows:
/// > there is a boundary after
/// > 1. An underscore ("_").
/// > 2. A series of two or more uppercase ASCII characters and the suffix
/// >    "s", "es", or "ies" (e.g. "URLs", "VAXes")...unless the last
/// >    uppercase letter is "I" and the suffix is "s", in which case it's
/// >    just as likely to be an acronym followed by "Is" (i.e. "URLIs" is
/// >    treated as "URL Is").
/// > 3. A series of two or more uppercase ASCII characters followed by an
/// >    uppercase ASCII character and then a lowercase ASCII character
/// >    ("XMLReader" becomes "XML Reader").
/// > 4. A series of two or more uppercase ASCII characters followed by a
/// >    non-ASCII-alphabetic character. ("UTF8" becomes "UTF 8")
/// > 5. A series of two or more uppercase ASCII characters at the end of the
/// >    string.
/// > 6. An uppercase ASCII character and any number of non-ASCII-uppercase,
/// >    non-underscore characters ("ContrivedExample" becomes
/// >    "Contrived Example").
/// > 7. Any number of non-ASCII-uppercase, non-underscore characters
/// >    ("lowercase_example" becomes "lowercase _ example").
///
/// <https://github.com/swiftlang/swift/blob/swift-6.0.3-RELEASE/docs/CToSwiftNameTranslation.md#word-boundaries>
pub(crate) fn split_words(s: &str) -> impl Iterator<Item = &str> + '_ {
    Iter { remaining: s }
}

struct Iter<'a> {
    remaining: &'a str,
}

fn is_plural_suffix(word: &str) -> bool {
    matches!(word, "s" | "es" | "ies")
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        // Implementation adapted from:
        // https://github.com/swiftlang/swift/blob/swift-6.0.3-RELEASE/lib/Basic/StringExtras.cpp#L122

        // If there are no more characters remaining, we're done iterating.
        if self.remaining.is_empty() {
            return None;
        }

        let remaining = |idx| {
            let (_, remaining) = self.remaining.split_at(idx);
            remaining
        };

        fn skip(s: &str, f: impl Fn(char) -> bool) -> usize {
            s.find(|c: char| !f(c)).unwrap_or(s.len())
        }

        // Treat _ as a word on its own. Don't coalesce.
        if self.remaining.starts_with("_") {
            let (current, remaining) = self.remaining.split_at(1);
            self.remaining = remaining;
            return Some(current);
        }

        let mut idx = 0;

        // Skip over any uppercase letters at the beginning of the word.
        idx += skip(remaining(idx), |c| c.is_ascii_uppercase());

        // If there was more than one uppercase letter, this is an acronym.
        if idx > 1 {
            // Collect the lowercase letters up to the next word.
            let next = skip(remaining(idx), |c| c.is_ascii_lowercase());

            if remaining(idx).is_empty() {
                // If we hit the end of the string, that's it.
            } else if is_plural_suffix(remaining(idx).split_at(next).0)
                && remaining(idx - 1).split_at(next + 1).0 != "Is"
            {
                // If the next word is a plural suffix, add it on.
                idx += next;
            } else if remaining(idx).as_bytes()[0].is_ascii_lowercase() {
                // If the next word is alphabetic, assume the next letter is
                // the start of that word.
                idx -= 1;
            }

            let (current, remaining) = self.remaining.split_at(idx);
            self.remaining = remaining;
            return Some(current);
        }

        // Skip non-uppercase letters.
        idx += skip(remaining(idx), |c| !c.is_ascii_uppercase() && c != '_');

        let (current, remaining) = self.remaining.split_at(idx);
        self.remaining = remaining;
        Some(current)
    }
}

impl FusedIterator for Iter<'_> {}

/// Find the common prefix of a number of names, based on word boundaries.
pub(crate) fn common_prefix<'a>(items: impl IntoIterator<Item = &'a str>) -> &'a str {
    // Algorithm adapted from https://stackoverflow.com/a/6718435.
    let mut items = items.into_iter();

    let Some(first) = items.next() else {
        return "";
    };

    let mut min = first;
    let mut max = first;
    for item in items {
        // Lexiographical ordering. Assume that this is the same as
        // split_words(item).lt(split_words(min))
        if item < min {
            min = item;
        }
        // split_words(max).lt(split_words(item))
        if max < item {
            max = item;
        }
    }

    let mut min_it = Iter { remaining: min };
    let mut max_it = split_words(max);

    while let Some(min_word) = min_it.next() {
        if let Some(max_word) = max_it.next() {
            if min_word != max_word {
                return min
                    .split_at(min.len() - min_it.remaining.len() - min_word.len())
                    .0;
            }
        }
    }

    min
}

/// Find the common prefix of the enum.
///
/// According to the following algorithm:
/// <https://github.com/swiftlang/swift/blob/swift-6.0.3-RELEASE/docs/CToSwiftNameTranslation.md#enum-style-prefix-stripping>
///
/// Should be given all available, non-deprecated enum cases without custom
/// names. If there are no such cases, then all cases without custom names,
/// whether available or not.
pub(crate) fn enum_prefix<'a>(
    enum_name: &'a str,
    relevant_enum_cases: impl IntoIterator<Item = &'a str> + Clone,
) -> &'a str {
    // 2. Find the common word-boundary prefix CP of these cases.
    let mut cp = common_prefix(relevant_enum_cases.clone());

    // 3. If CP starts with "k" ...
    let mut had_k = false;
    let original_cp = cp;
    if let Some(cp_without_k) = cp.strip_prefix("k") {
        // ... followed by an uppercase letter ...
        if cp_without_k.starts_with(|c: char| c.is_ascii_uppercase()) {
            // treat that as meaning "constant" and ignore it
            cp = cp_without_k;
            had_k = true;
        }
        // ... or if it's just "k" and none of the cases have a
        // non-identifier-start character immediately after the 'k'
        if cp_without_k.is_empty()
            && relevant_enum_cases.into_iter().all(|case| {
                case.strip_prefix("k")
                    .unwrap()
                    .starts_with(|c: char| !c.is_ascii_digit())
            })
        {
            // treat that as meaning "constant" and ignore it
            cp = cp_without_k;
            had_k = true;
        }
    }

    // 4. Find the common word-boundary prefix EP of CP and the type's
    // original C name (rather than its Swift name).
    let mut ep = common_prefix([cp, enum_name]);

    // 5. If the next word of CP after EP is ...
    let next_word_enum = split_words(enum_name.strip_prefix(ep).unwrap())
        .next()
        .unwrap_or("");
    let next_word_cp = split_words(cp.strip_prefix(ep).unwrap())
        .next()
        .unwrap_or("");

    // ...
    // - the next word of the type's original C name minus "s" ("URL" vs. "URLs")
    // - the next word of the type's original C name minus "es" ("Address" vs. "Addresses")
    // - the next word of the type's original C name with "ies" replaced by "y" ("Property" vs. "Properties")
    // ...
    if next_word_enum.strip_suffix("s") == Some(next_word_cp)
        || next_word_enum.strip_suffix("es") == Some(next_word_cp)
        || (next_word_enum.strip_suffix("ies").is_some()
            && next_word_enum.strip_suffix("ies") == next_word_cp.strip_suffix("y"))
    {
        // ... add the next word of CP to EP.
        ep = cp.split_at(ep.len() + next_word_cp.len()).0;
    }

    // 6. If the next word of CP after EP is an underscore
    // ("MyEnum_FirstCase" vs. "MyEnum") ...
    if let Some("_") = split_words(cp.strip_prefix(ep).unwrap()).next() {
        // ... add the underscore to EP.
        ep = cp.split_at(ep.len() + 1).0;
    }

    // 7. If "k" was dropped from CP in step 3, add it back to the front of EP.
    if had_k {
        ep = original_cp.split_at(ep.len() + 1).0;
    }

    ep
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_words() {
        #[track_caller]
        fn check(word: &str, expected: &[&str]) {
            let result: Vec<&str> = split_words(word).collect();
            assert_eq!(result, expected);
        }

        // Rule 2
        check("URLs", &["URLs"]);
        check("URLes", &["URLes"]);
        check("URLies", &["URLies"]);
        check("URLIs", &["URL", "Is"]);
        check("URLis", &["UR", "Lis"]);

        // Rule 3
        check("XMLReader", &["XML", "Reader"]);

        // Rule 4
        check("UTF8", &["UTF", "8"]);

        // Rule 6
        check("ContrivedExample", &["Contrived", "Example"]);

        // Rule 7
        check("lowercase_example", &["lowercase", "_", "example"]);

        // Various
        check("URL_Loader", &["URL", "_", "Loader"]);
        check("URLLoader", &["URL", "Loader"]);
        check("URL_loader", &["URL", "_", "loader"]);
        check("URLs_loader", &["URLs", "_", "loader"]);
        check("URLloader", &["UR", "Lloader"]);
        check("UTF_8", &["UTF", "_", "8"]);
        check(
            "UIImagePickerControllerQualityType640x480",
            &[
                "UI",
                "Image",
                "Picker",
                "Controller",
                "Quality",
                "Type640x480",
            ],
        );
    }

    #[test]
    fn test_common_prefix() {
        assert_eq!(common_prefix(["a", "b", "c"]), "");
        assert_eq!(common_prefix(["aa", "aa", "aa"]), "aa");
        assert_eq!(common_prefix(["aa", "ab", "ac"]), "");
        assert_eq!(common_prefix(["One", "On"]), "");
        assert_eq!(common_prefix(["OneTwo", "OneTwoThree"]), "OneTwo");
        assert_eq!(common_prefix(["OneTwo", "OneThree"]), "One");
        assert_eq!(common_prefix(["AOneThree", "AOxTwo", "AOneTwo"]), "A");

        assert_eq!(common_prefix([]), "");
    }

    #[test]
    fn test_enum_prefix() {
        #[track_caller]
        fn check<const N: usize>(enum_name: &str, cases: [&str; N], expected: &str) {
            assert_eq!(enum_prefix(enum_name, cases), expected);
        }

        // Contrived
        check(
            "MyEnum",
            ["MyEnumCaseOne", "MyEnumCaseTwo", "MyEnumCaseThree"],
            "MyEnum",
        );
        check(
            "MyOptions",
            ["MyOptionCaseOne", "MyOptionCaseTwo", "MyOptionCaseThree"],
            "MyOption",
        );
        check("NoCases", [], "");

        // Real-world
        check(
            "NSWindowSharingType",
            ["NSWindowSharingNone", "NSWindowSharingReadOnly"],
            "NSWindowSharing",
        );
        check(
            "NSApplicationPresentationOptions",
            [
                "NSApplicationPresentationDefault",
                "NSApplicationPresentationAutoHideDock",
                "NSApplicationPresentationHideDock",
                "NSApplicationPresentationAutoHideMenuBar",
            ],
            "NSApplicationPresentation",
        );
        check(
            "NSRequestUserAttentionType",
            ["NSCriticalRequest", "NSInformationalRequest"],
            "NS",
        );
        check(
            "MTLFeatureSet",
            [
                "MTLFeatureSet_iOS_GPUFamily1_v1",
                "MTLFeatureSet_macOS_GPUFamily1_v1",
            ],
            "MTLFeatureSet_",
        );
        check(
            "CGError",
            [
                "kCGErrorSuccess",
                "kCGErrorFailure",
                "kCGErrorIllegalArgument",
            ],
            "kCGError",
        );
        check(
            "MTLSparsePageSize",
            [
                "MTLSparsePageSize16",
                "MTLSparsePageSize64",
                "MTLSparsePageSize256",
            ],
            "MTLSparsePage",
        );
        check(
            "UIImagePickerControllerQualityType",
            [
                "UIImagePickerControllerQualityTypeHigh",
                "UIImagePickerControllerQualityTypeMedium",
                "UIImagePickerControllerQualityTypeLow",
                "UIImagePickerControllerQualityType640x480",
                "UIImagePickerControllerQualityTypeIFrame1280x720",
                "UIImagePickerControllerQualityTypeIFrame960x540",
            ],
            "UIImagePickerControllerQuality",
        );

        // BUG (also somewhat present in Swift's translation)
        check(
            "NEHotspotConfigurationEAPTLSVersion",
            [
                "NEHotspotConfigurationEAPTLSVersion_1_0",
                "NEHotspotConfigurationEAPTLSVersion_1_1",
                "NEHotspotConfigurationEAPTLSVersion_1_2",
            ],
            "NEHotspotConfigurationEAPTLSVersion_",
        );
    }
}
