//! The structure of the values in `fs`.
//!
//! These are JSON values.
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Doc {
    pub identifier: DocIdentifier,
    pub metadata: Metadata,
    pub references: BTreeMap<String, Reference>,
    pub sections: Vec<Section>,

    pub schema_version: SchemaVersion,

    #[serde(default)]
    pub variants: Vec<Variant>,

    #[serde(flatten)]
    pub kind: DocKind,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(rename_all_fields = "camelCase")]
#[serde(tag = "kind")]
pub enum DocKind {
    Root { hierarchy: Hierarchy },
    Technologies { hierarchy: Hierarchy },
    Overview,
    Project,
    Symbol(Page),
    Article(Page),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Page {
    #[serde(default)]
    #[serde(rename = "abstract")]
    pub abstract_: Vec<Content>,
    #[serde(default)]
    pub primary_content_sections: Vec<PrimaryContentSection>,
    #[serde(default)]
    pub topic_sections: Vec<TopicSection>,
    pub topic_sections_style: Option<String>,
    #[serde(default)]
    pub relationships_sections: Vec<TopicSection>, // symbol
    #[serde(default)]
    pub default_implementations_sections: Vec<TopicSection>, // symbol
    #[serde(default)]
    pub see_also_sections: Vec<TopicSection>,
    #[serde(default)]
    pub deprecation_summary: Vec<Content>, // symbol
    pub sample_code_download: Option<SampleDownload>, // article
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DocIdentifier {
    pub url: String,
    pub interface_language: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct SchemaVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Variant {
    pub paths: Vec<String>,
    pub traits: Vec<VariantTrait>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct VariantTrait {
    pub interface_language: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Hierarchy {
    pub homepage_navigation: Vec<NavigationItem>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct NavigationItem {
    pub reference: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
// TODO: #[serde(deny_unknown_fields)]
pub struct SampleDownload {}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(rename_all_fields = "camelCase")]
#[serde(tag = "type")]
#[serde(deny_unknown_fields)]
#[allow(clippy::large_enum_variant)]
pub enum Reference {
    Image {
        identifier: String,
        variants: Vec<ReferenceVariant>,
        alt: Option<String>,
    },

    Video {
        identifier: String,
        poster: Option<String>,
        variants: Vec<ReferenceVariant>,
        alt: Option<String>,
    },

    Link {
        identifier: String,
        url: String,
        title: String,
        title_inline_content: Vec<Content>,
    },

    File {
        identifier: String,
        syntax: String,
        content: Vec<String>,
        file_name: String,
        file_type: String,
        highlights: Option<Vec<serde_json::Value>>,
    },

    Download {
        identifier: String,
        url: String,
        checksum: String,
    },

    Unresolvable {
        identifier: String,
        title: String,
    },

    XcodeRequirement {
        identifier: String,
        title: String,
        url: String,
    },

    Section {
        identifier: String,
        kind: String,
        title: String,
        url: String,
        #[serde(rename = "abstract")]
        abstract_: Vec<Content>,
        role: Option<String>,
    },

    Topic {
        identifier: String,
        kind: String,
        title: String,
        name: Option<String>,
        title_style: Option<String>,
        url: String,
        #[serde(rename = "abstract")]
        abstract_: Vec<Content>,

        role: Option<String>,

        #[serde(default)]
        images: Vec<TopicImage>,
        #[serde(default)]
        deprecated: bool,
        #[serde(default)]
        beta: bool,
        #[serde(default)]
        required: bool,

        estimated_time: Option<String>,
        conformance: Option<Conformance>,
        fragments: Option<serde_json::Value>,
        #[serde(default)]
        default_implementations: u32,
        ide_title: Option<String>,
    },
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ReferenceVariant {
    pub url: String,
    pub traits: Vec<String>,

    #[serde(rename = "svgID")]
    pub svg_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct TopicSection {
    #[serde(rename = "abstract")]
    pub abstract_: Option<Vec<Content>>,
    pub discussion: Option<PrimaryContentSection>,
    pub title: Option<String>,
    pub anchor: Option<String>,
    pub identifiers: Vec<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub kind: Option<String>, // only on relationship sections
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
// TODO: #[serde(deny_unknown_fields)]
pub struct Image {}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Metadata {
    pub images: Option<Vec<Image>>,
    pub category: Option<String>,
    pub estimated_time: Option<String>,
    pub category_path_component: Option<String>,
    pub color: Option<Color>,
    pub default_suggested_tags: Option<Vec<String>>,
    pub custom_metadata: Option<BTreeMap<String, String>>,
    pub parent: Option<Parent>,
    #[serde(default)]
    pub required: bool,
    pub role: String,
    pub title: String,
    pub extended_module: Option<String>,
    #[serde(rename = "externalID")]
    pub external_id: Option<String>,
    #[serde(default)]
    pub modules: Vec<Module>,
    pub platforms: Option<Vec<PlatformData>>,
    pub role_heading: Option<String>,
    pub symbol_kind: Option<String>,
    pub conformance: Option<Conformance>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Color {
    pub standard_color_identifier: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Conformance {
    pub availability_prefix: Option<Vec<Content>>,
    pub conformance_prefix: Option<Vec<Content>>,
    pub constraints: Option<Vec<Content>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Parent {
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct TopicImage {
    pub identifier: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(rename_all_fields = "camelCase")]
#[serde(tag = "kind")]
#[allow(clippy::large_enum_variant)]
pub enum Section {
    Hero {
        content: Vec<Content>,
        project_files: Option<String>,
        xcode_requirement: Option<String>,
        chapter: Option<String>,
        estimated_time: Option<String>,
        estimated_time_in_minutes: Option<u32>,
        image: Option<String>,
        background_image: Option<String>,
        video: Option<String>,
        title: String,
        action: Option<Content>,
    },

    Technologies {
        groups: Vec<TechnologyGroup>,
    },

    Section {},

    Resources {},

    Volume {},

    Assessments {},

    Tasks {},

    CallToAction {},

    HomepageResources {},

    ArticleBody {},
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct TechnologyGroup {
    pub name: Option<String>,
    pub technologies: Vec<Technology>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Technology {
    pub title: String,
    pub content: Vec<Content>,
    pub destination: Content,
    pub languages: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(rename_all_fields = "camelCase")]
#[serde(tag = "type")]
#[serde(deny_unknown_fields)]
pub enum Content {
    Image {
        identifier: String,
        metadata: Option<ContentMetadata>,
    },
    Video {
        identifier: String,
        metadata: Option<ContentMetadata>,
    },
    Links {
        style: String,
        items: Vec<String>,
    },
    Superscript {
        inline_content: Vec<Content>,
    },
    ThematicBreak {},
    Row {
        number_of_columns: u32,
        columns: Vec<Column>,
    },
    Table {
        header: String,
        extended_data: Option<serde_json::Value>,
        rows: Vec<Vec<Vec<Content>>>,
        alignments: Option<Vec<String>>,
        metadata: Option<ContentMetadata>,
    },
    TabNavigator {
        tabs: Vec<TabItem>,
    },
    UnorderedList {
        items: Vec<Item>,
    },
    OrderedList {
        start: Option<u32>,
        items: Vec<Item>,
    },
    Topic {
        identifier: String,
        is_active: bool,
    },
    Reference {
        identifier: String,
        is_active: bool,
        overriding_title: Option<String>,
        overriding_title_inline_content: Option<Vec<Content>>,
    },
    Text {
        text: String,
    },
    CodeVoice {
        code: String,
    },
    Paragraph {
        inline_content: Vec<Content>,
    },
    Heading {
        anchor: String,
        level: u32,
        text: String,
    },
    CodeListing {
        syntax: Option<String>,
        code: Vec<String>,
        metadata: Option<ContentMetadata>,
    },
    NewTerm {
        inline_content: Vec<Content>,
    },
    Aside {
        name: Option<String>,
        style: String,
        content: Vec<Content>,
    },
    TermList {
        items: Vec<Term>,
    },
    Emphasis {
        inline_content: Vec<Content>,
    },
    Strong {
        inline_content: Vec<Content>,
    },
    Small {
        inline_content: Vec<Content>,
    },
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct TabItem {
    pub title: String,
    pub content: Vec<Content>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ContentMetadata {
    #[serde(rename = "abstract")]
    pub abstract_: Option<Vec<Content>>,
    pub device_frame: Option<String>,
    pub anchor: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Column {
    pub content: Vec<Content>,
    pub size: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Term {
    pub definition: TermContent,
    pub term: TermParagraph,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Item {
    pub content: Vec<Content>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct TermContent {
    pub content: Vec<Content>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct TermParagraph {
    pub inline_content: Vec<Content>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Module {
    pub name: Option<String>,
    pub related_modules: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct PlatformData {
    pub name: Option<String>,
    #[serde(default)]
    pub beta: bool,
    #[serde(default)]
    pub deprecated: bool,
    #[serde(default)]
    pub unavailable: bool,
    pub message: Option<String>,
    pub renamed: Option<String>,
    pub introduced_at: Option<String>,
    pub deprecated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(rename_all_fields = "camelCase")]
// TODO: #[serde(deny_unknown_fields)]
#[serde(tag = "kind")]
pub enum PrimaryContentSection {
    Declarations {
        declarations: Vec<Declaration>,
    },

    Parameters {
        parameters: Vec<Parameter>,
    },

    Properties {},

    Content {
        content: Vec<Content>,
    },

    Mentions {
        mentions: Vec<String>,
    },

    RestEndpoint {
        title: String,
        // TODO: tokens: Vec<Token>,
    },

    PossibleValues {},

    Attributes {},

    RestParameters {
        title: String,
        source: Option<String>,
        // TODO: tokens: Vec<Token>,
    },

    RestBody {},

    RestResponses {},

    Details {
        title: String,
        details: Details,
    },
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Declaration {
    pub languages: Vec<String>,
    pub platforms: Vec<String>,
    pub tokens: Vec<Token>,
    pub other_declarations: Option<OtherDeclarations>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct OtherDeclarations {
    pub declarations: Vec<OtherDeclaration>,
    pub display_index: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct OtherDeclaration {
    pub identifier: String,
    pub tokens: Vec<Token>,
    pub conformance: Option<Conformance>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Details {
    pub name: String,
    pub platforms: Vec<String>,
    pub title_style: String,
    pub ide_title: Option<String>,
    pub value: Vec<DetailsValue>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
// TODO: #[serde(deny_unknown_fields)]
pub struct DetailsValue {}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Token {
    // TODO: Tagged enum?
    pub kind: String,
    pub text: String,
    pub identifier: Option<String>,
    pub highlight: Option<String>,
    pub other_declarations: Option<String>,
    pub precise_identifier: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Parameter {
    pub name: Option<String>,
    pub content: Vec<Content>,
}
