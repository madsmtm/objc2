use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fmt::{self, Write as _};

use apple_doc::{
    BlobStore, Content, Doc, DocKind, Kind, PrimaryContentSection, Reference, ReferenceVariant,
    ReferenceVariantTrait, SqliteDb,
};
use clang::documentation::{
    BlockCommand, CommentChild, HtmlStartTag, InlineCommand, InlineCommandStyle, ParamCommand,
    TParamCommand,
};
use clang::{Entity, EntityKind};

use crate::display_helper::FormatterFn;
use crate::{Context, ItemIdentifier};

pub type TxtMap<'a> = BTreeMap<(&'a str, Kind), Vec<&'a str>>;

pub struct DocState<'data> {
    txts: &'data TxtMap<'data>,
    sqlite_db: &'data SqliteDb,
    blobs: &'data RefCell<BlobStore>,
}

impl<'data> DocState<'data> {
    pub fn new(
        txts: &'data TxtMap<'data>,
        sqlite_db: &'data SqliteDb,
        blobs: &'data RefCell<BlobStore>,
    ) -> Self {
        Self {
            txts,
            sqlite_db,
            blobs,
        }
    }

    pub fn get<'r: 'data, 's: 'r, 'n: 'r>(
        &'s self,
        name: &'n str,
        kind: Kind,
    ) -> impl Iterator<Item = Doc> + 'r {
        let ids = self
            .txts
            .get(&(name, kind))
            .map(|ids| &**ids)
            .unwrap_or(&[]);
        ids.into_iter().filter_map(move |id| {
            // Some entries in `*.txt` don't have a documentation entry.
            let r = self.sqlite_db.get_ref(id).unwrap()?;
            let mut blobs = self.blobs.borrow_mut();
            Some(blobs.parse_doc(&r).unwrap())
        })
    }

    #[track_caller]
    pub fn one_doc(&self, name: &str, kind: Kind, context: &Context<'_>) -> Option<Doc> {
        let mut current = None;

        let mut iter = self.get(name, kind).enumerate().peekable();

        while let Some((i, doc)) = iter.next() {
            // HACK: Use item when there's only one, regardless of the module.
            if iter.peek().is_none() && i == 0 {
                return Some(doc);
            }

            // Remove documentation entries for items in other modules than
            // the current module.
            //
            // TODO: Is there a better way to do this?
            if doc.metadata.modules.is_empty()
                || doc
                    .metadata
                    .modules
                    .iter()
                    .any(|module| module.name.as_deref() == Some(context.current_library_title))
            {
                if current.is_some() {
                    error!(
                        name,
                        ?kind,
                        ?current,
                        ?doc,
                        "must have only one matching doc item"
                    );
                }
                current = Some(doc);
            }
        }

        current
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Documentation {
    first: Option<String>,
    from_header: Vec<CommentChild>,
    extras: Vec<String>,
    alias: Option<String>,
    apple: Option<Doc>,
}

impl Documentation {
    pub fn empty() -> Self {
        Self {
            first: None,
            from_header: vec![],
            extras: vec![],
            alias: None,
            apple: None,
        }
    }

    /// Construct from an entity, possible one that has been renamed (such
    /// that we'll want a doc alias to the entity's actual name).
    pub fn from_entity(entity: &Entity<'_>, context: &Context<'_>) -> Self {
        let from_header = if let Some(comment) = entity.get_comment() {
            if let Some(parsed) = entity.get_parsed_comment() {
                parsed.get_children()
            } else {
                warn!(?entity, comment, "had comment, but not parsed comment");
                Vec::new()
            }
        } else {
            Vec::new()
        };

        let library = context.library(ItemIdentifier::new_optional(entity, context));
        let alias = if let Some(renamed) = &library.get(entity).renamed {
            let name = entity.get_name().expect("renamed entity must have name");
            if *renamed != name {
                Some(name)
            } else {
                None
            }
        } else {
            None
        };

        let txt_kind = match entity.get_kind() {
            EntityKind::ObjCInterfaceDecl => Some(Kind::Class),
            EntityKind::ObjCCategoryDecl => None,
            EntityKind::ObjCProtocolDecl => Some(Kind::Protocol),
            EntityKind::TypedefDecl => Some(Kind::Typedef),
            EntityKind::StructDecl => Some(Kind::Struct),
            EntityKind::UnionDecl => Some(Kind::Union),
            EntityKind::EnumDecl => Some(Kind::Enum),
            EntityKind::VarDecl => Some(Kind::GlobalVariable),
            EntityKind::FunctionDecl => Some(Kind::Function),
            EntityKind::ObjCInstanceMethodDecl => None, // TODO
            EntityKind::ObjCPropertyDecl => None,       // TODO
            EntityKind::ObjCClassMethodDecl => None,    // TODO
            EntityKind::EnumConstantDecl => Some(Kind::EnumCase),
            EntityKind::FieldDecl => None,                    // TODO
            EntityKind::MacroDefinition => Some(Kind::Macro), // TODO
            EntityKind::UnexposedDecl => None,
            _ => {
                warn!(?entity, "unknown entity being documented");
                None
            }
        };

        let apple = txt_kind.and_then(|txt_kind| {
            entity
                .get_name()
                .and_then(|c_name| context.doc.one_doc(&c_name, txt_kind, context))
        });

        Self {
            first: None,
            from_header,
            extras: vec![],
            alias,
            apple,
        }
    }

    pub fn set_first(&mut self, doc: impl Into<String>) {
        self.first = Some(doc.into());
    }

    pub fn add(&mut self, doc: impl Into<String>) {
        self.extras.push(doc.into());
    }

    pub fn set_alias(&mut self, alias: String) {
        self.alias = Some(alias);
    }

    pub fn set_apple(&mut self, apple: Option<Doc>) {
        self.apple = apple;
    }

    pub fn fmt<'a>(&'a self) -> impl fmt::Display + 'a {
        FormatterFn(move |f| {
            let apple = if let Some(apple) = &self.apple {
                format!("{}", apple_to_md(apple))
            } else {
                String::new()
            };

            let mut from_header = String::new();

            for child in &self.from_header {
                write!(&mut from_header, "{}", format_child(child))?;
            }

            let from_header = fix_code_blocks(&from_header).trim().replace("\t", "    ");

            let content = if apple.is_empty() && from_header.is_empty() {
                None
            } else if apple == from_header {
                // TODO: Merge these instead, by comparing nodes
                Some(apple)
            } else {
                Some(format!("{apple}{from_header}"))
            };

            let groups = self
                .first
                .iter()
                .chain(content.iter())
                .chain(self.extras.iter());

            for (i, group) in groups.enumerate() {
                if i != 0 {
                    // Intersperse extra newline between groups.
                    writeln!(f, "///")?;
                }
                for line in group.lines() {
                    if line.is_empty() {
                        writeln!(f, "///")?;
                    } else {
                        writeln!(f, "/// {line}")?;
                    }
                }
            }

            if let Some(alias) = &self.alias {
                writeln!(f, "#[doc(alias = {alias:?})]")?;
            }

            Ok(())
        })
    }
}

fn fix_code_blocks(s: &str) -> String {
    let mut ret = String::with_capacity(s.len());
    let mut last_end = 0;
    for (i, (start, part)) in s.match_indices("```").enumerate() {
        if i % 2 == 1 {
            continue;
        }
        if &s[start..start + 4] != "```\n" {
            continue;
        }
        ret.push_str(&s[last_end..start]);
        ret.push_str("```text");
        last_end = start + part.len();
    }
    ret.push_str(&s[last_end..s.len()]);
    ret
}

fn format_child(child: &CommentChild) -> impl fmt::Display + '_ {
    FormatterFn(move |f| {
        match child {
            CommentChild::BlockCommand(BlockCommand {
                command,
                arguments,
                children,
            }) => {
                // See:
                // - <https://doxygen.nl/manual/commands.html>
                // - <https://developer.apple.com/library/archive/documentation/DeveloperTools/Conceptual/HeaderDoc/tags/tags.html>
                match &**command {
                    // @abstract is an alternate name for @brief
                    "brief" | "abstract" => {}
                    // @description and @details are alternate names for @discussion
                    "discussion" | "description" | "details" => {}
                    "remark" | "remarks" => {}
                    "see" => write!(f, "See: ")?,
                    "seealso" | "sa" => write!(f, "See also: ")?,
                    "note" => write!(f, "Note: ")?,
                    "warning" => write!(f, "Warning: ")?,
                    "dependency" => write!(f, "Dependencies: ")?,
                    "result" | "return" | "returns" => write!(f, "Returns: ")?,
                    // TODO: Convert to # Panic section?
                    "throws" => write!(f, "Throws a ")?,
                    "performance" => write!(f, "Performance: ")?,
                    // For some odd reason, @host is parsed to post here?
                    "post" => write!(f, "@host")?,
                    // Ignore
                    "superclass" => return Ok(()),
                    // This is just the name of the thing we're parsing, so ignore.
                    "defined" => return Ok(()),
                    // Ignore for now, though in the future we should perhaps
                    // integrate this with `Availability`.
                    "deprecated" => return Ok(()),
                    // List
                    "li" => {}
                    _ => warn!(?child, "unknown documentation command"),
                }

                if !arguments.is_empty() {
                    error!(?child, "BlockCommand had arguments");
                }

                for child in children {
                    write!(f, "{}", format_child(child))?;
                }

                writeln!(f)?;
            }
            CommentChild::InlineCommand(InlineCommand {
                command,
                arguments,
                style,
            }) => {
                #[allow(clippy::needless_borrowed_reference)] // False positive
                match (&**command, &**arguments) {
                    // Styles gotten via InlineCommandStyle
                    ("a" | "c" | "p" | "e" | "em", &[ref argument]) => {
                        let argument = argument.trim();
                        match style {
                            Some(InlineCommandStyle::Bold) => write!(f, "**{argument}**")?,
                            Some(InlineCommandStyle::Emphasized) => write!(f, "_{argument}_")?,
                            Some(InlineCommandStyle::Monospace) => write!(f, "`{argument}`")?,
                            None => write!(f, "{argument}")?,
                        }
                    }
                    // TODO: Use this somehow
                    ("ref", &[ref argument]) => {
                        let argument = argument.trim();
                        write!(f, "{argument}")?
                    }
                    // Unintentionally parsed as Sphinx
                    ("r", &[]) => write!(f, "\\r")?,
                    ("n", &[]) => write!(f, "\\n")?,
                    ("t", &[]) => write!(f, "\\t")?,
                    ("in", &[]) => write!(f, "\\in ")?,
                    ("autoreleasepool", &[]) => write!(f, "objc2::rc::autoreleasepool")?,
                    ("selector", &[]) => write!(f, "sel!")?,
                    ("MainActor", &[]) => write!(f, "MainThreadOnly")?,
                    // Boolean values (Written as @YES and @NO).
                    ("YES", &[]) => write!(f, "`true`")?,
                    ("NO", &[]) => write!(f, "`false`")?,
                    // Grouping
                    ("group" | "memberof", _) => {}
                    // Alternative for @param that isn't parsed into ParamCommand (?)
                    ("pparam" | "parameter", &[]) => write!(f, "Parameter ")?,
                    ("parameters", &[]) => write!(f, "Parameters ")?,
                    // Not parsed into a block
                    ("field", &[]) => write!(f, "Field: ")?,
                    ("super", _) => write!(f, "Super: ")?,
                    ("availability", _) => write!(f, "Availability: ")?,
                    // This is just the name of the thing we're parsing, so ignore.
                    ("define" | "defined" | "key" | "options", _) => {}
                    // Shouldn't actually be hit, but is for some reason?
                    ("header", _) => {}
                    ("description", _) => {}
                    // ImageCaptureCore uses enums with things like `@ICMediaPresentation`.
                    (ic, _) if ic.starts_with("IC") => {}
                    _ => warn!(?child, "unknown documentation command"),
                }
            }
            CommentChild::HtmlStartTag(HtmlStartTag {
                name,
                attributes,
                closing,
            }) => {
                write!(f, "<{}", name.trim())?;

                for (key, val) in attributes {
                    write!(f, " {key}={val:?}")?;
                }

                writeln!(f, "{}>", if *closing { "/" } else { "" })?;
            }
            CommentChild::HtmlEndTag(name) => {
                writeln!(f, "</{}>", name.trim())?;
            }
            CommentChild::Paragraph(children) => {
                for child in children {
                    write!(f, "{}", format_child(child))?;
                }

                writeln!(f)?;
            }
            CommentChild::ParamCommand(ParamCommand {
                index: _,
                parameter,
                direction: _,
                children,
            })
            | CommentChild::TParamCommand(TParamCommand {
                position: _,
                parameter,
                children,
            }) => {
                write!(f, "Parameter `{parameter}`: ")?;

                for child in children {
                    write!(f, "{}", format_child(child))?;
                }

                writeln!(f)?;
            }
            CommentChild::Text(text) => {
                let text = text.trim();
                if !text.is_empty() {
                    writeln!(f, "{text}")?;
                }
            }
            CommentChild::VerbatimCommand(verbatim) => {
                writeln!(f, "```text")?;
                for text in verbatim {
                    writeln!(f, "{text}")?;
                }
                writeln!(f, "```")?;
                writeln!(f)?;
            }
            CommentChild::VerbatimLineCommand(_) => {
                // Often comes from @member or similar that just name the item
                // again.
                // writeln!(f, "{}", verbatim_line.trim())?;
            }
        }

        Ok(())
    })
}

/// Format Apple's documentation as markdown link.
fn apple_to_md(doc: &Doc) -> impl fmt::Display + '_ {
    FormatterFn(move |f| match &doc.kind {
        DocKind::Symbol(page) => {
            if !page.abstract_.is_empty() {
                writeln!(f, "{}", contents_to_md(doc, &page.abstract_))?;
            }

            for section in &doc.sections {
                match section {
                    _ => {
                        write!(f, "SECTION: {section:?}")?;
                    }
                }
            }

            for section in &page.primary_content_sections {
                match section {
                    PrimaryContentSection::Declarations { .. } => {}
                    PrimaryContentSection::Mentions { .. } => {}
                    PrimaryContentSection::Content { content } => {
                        writeln!(f)?;
                        writeln!(f, "{}", contents_to_md(doc, content))?;
                    }
                    PrimaryContentSection::Parameters { parameters } => {
                        writeln!(f)?;
                        writeln!(f, "Parameters:")?;
                        for parameter in parameters {
                            write!(f, "- ")?;
                            if let Some(name) = &parameter.name {
                                write!(f, "{name}: ")?;
                            }
                            write!(f, "{}", contents_to_md(doc, &parameter.content))?;
                        }
                    }
                    _ => {
                        write!(f, "PRIMSECTION: {section:?}")?;
                    }
                }
            }

            Ok(())
        }
        _ => writeln!(f, "TODO doc: {doc:?}"),
    })
}

fn contents_to_md<'a>(doc: &'a Doc, contents: &'a [Content]) -> impl fmt::Display + 'a {
    FormatterFn(move |f| {
        for content in contents {
            write!(f, "{}", content_to_md(doc, content))?;
        }
        Ok(())
    })
}

fn content_to_md<'a>(doc: &'a Doc, content: &'a Content) -> impl fmt::Display + 'a {
    FormatterFn(move |f| match content {
        Content::Text { text } => write!(f, "{text}"),
        Content::CodeVoice { code } => write!(f, "`{code}`"),
        Content::Emphasis { inline_content } => {
            write!(f, "_{}_", contents_to_md(doc, inline_content))
        }
        Content::Strong { inline_content } => {
            write!(f, "**{}**", contents_to_md(doc, inline_content))
        }
        Content::ThematicBreak {} => writeln!(f, "---"),
        Content::CodeListing {
            syntax,
            code,
            metadata: _,
        } => {
            write!(f, "```")?;
            if let Some(syntax) = syntax {
                write!(f, "{syntax}")?;
            } else {
                write!(f, "text")?;
            }
            writeln!(f)?;
            for line in code {
                writeln!(f, "{line}")?;
            }
            writeln!(f, "```")?;
            writeln!(f)?;
            Ok(())
        }
        Content::Image {
            identifier,
            metadata,
        } => {
            if let Some(Reference::Image { variants, alt, .. }) = doc.references.get(identifier) {
                let alt = metadata
                    .as_ref()
                    .and_then(|metadata| metadata.title.clone())
                    .unwrap_or_else(|| alt.clone().unwrap_or_default());
                match &**variants {
                    [] => {
                        error!(identifier, "image must have at least one variant");
                        Ok(())
                    }
                    [ReferenceVariant { url, .. }] => {
                        writeln!(f)?;
                        writeln!(f, "![{alt}]({url})")?;
                        Ok(())
                    }
                    variants => {
                        let mut light_srcset = String::new();
                        let mut dark_srcset = String::new();
                        let mut fallback = None;

                        for variant in variants {
                            let mut pixel_density = "";
                            let mut srcset = &mut light_srcset;

                            for trait_ in &variant.traits {
                                match trait_ {
                                    ReferenceVariantTrait::X1 => pixel_density = " 1x",
                                    ReferenceVariantTrait::X2 => pixel_density = " 2x",
                                    ReferenceVariantTrait::X3 => pixel_density = " 3x",
                                    ReferenceVariantTrait::Dark => srcset = &mut dark_srcset,
                                    ReferenceVariantTrait::Light => srcset = &mut light_srcset,
                                }
                            }

                            if !srcset.is_empty() {
                                write!(&mut srcset, ", ").unwrap();
                            }
                            write!(&mut srcset, "{}{pixel_density}", variant.url).unwrap();

                            fallback = Some(&variant.url);
                        }

                        writeln!(f)?;
                        writeln!(f, "<picture>")?;
                        if !dark_srcset.is_empty() {
                            writeln!(
                                f,
                                "    <source media=\"(prefers-color-scheme: dark)\" srcset=\"{dark_srcset}\" />"
                            )?;
                        }
                        if !light_srcset.is_empty() {
                            writeln!(
                                f,
                                "    <source media=\"(prefers-color-scheme: light)\" srcset=\"{light_srcset}\" />"
                            )?;
                        }
                        writeln!(f, "    <img alt={alt:?} src={:?} />", fallback.unwrap())?;
                        writeln!(f, "</picture>")?;
                        Ok(())
                    }
                }
            } else {
                error!(identifier, "could not find image reference");
                Ok(())
            }
        }
        Content::Video { identifier, .. } => {
            writeln!(f, "(TODO vid: {:?})", doc.references.get(identifier))
        }
        Content::Links { style, items } => write!(f, "(TODO links: {content:?})"),
        Content::Superscript { inline_content } => {
            writeln!(f, "<sup>{}</sup>", contents_to_md(doc, inline_content))
        }
        Content::Small { inline_content } => {
            writeln!(f, "<sub>{}</sub>", contents_to_md(doc, inline_content))
        }
        Content::Row {
            number_of_columns,
            columns,
        } => writeln!(f, "(TODO row: {content:?})"),
        Content::Table {
            header,
            extended_data,
            rows,
            alignments,
            metadata,
        } => writeln!(f, "(TODO table: {content:?})"),
        Content::TabNavigator { tabs } => writeln!(f, "(TODO tabnav: {content:?})"),
        Content::UnorderedList { items } => {
            for item in items {
                write!(f, "-")?;
                let mut iter = item.content.iter();
                if let Some(first) = iter.next() {
                    write!(f, " {}", content_to_md(doc, first))?;
                } else {
                    writeln!(f)?;
                }
                for content in iter {
                    write!(f, "  {}", content_to_md(doc, content))?;
                }
            }
            Ok(())
        }
        Content::OrderedList { start, items } => {
            let mut value = start.unwrap_or(1);
            let width = (items.len() + value as usize) / 10;
            for item in items {
                write!(f, "{value:>width$}.")?;
                let mut iter = item.content.iter();
                if let Some(first) = iter.next() {
                    write!(f, " {}", content_to_md(doc, first))?;
                } else {
                    writeln!(f)?;
                }
                for content in iter {
                    write!(f, "{:>width$} {}", "", content_to_md(doc, content))?;
                }
                value += 1;
            }
            Ok(())
        }
        Content::Topic {
            identifier,
            is_active,
        } => writeln!(f, "(TODO topic: {content:?})"),
        Content::Reference {
            identifier,
            is_active,
            overriding_title,
            overriding_title_inline_content,
        } => {
            let reference = doc
                .references
                .get(identifier)
                .expect("must have reference in doc");
            write!(
                f,
                "{}",
                reference_to_md(
                    doc,
                    reference,
                    overriding_title.as_ref(),
                    overriding_title_inline_content.as_ref()
                )
            )
        }
        Content::Paragraph { inline_content } => {
            writeln!(f, "{}", contents_to_md(doc, inline_content))?;
            writeln!(f)?;
            Ok(())
        }
        Content::Heading {
            anchor,
            level,
            text,
        } => {
            // Remove anchor if it's (likely) what `rustdoc` is going to output.
            // https://github.com/rust-lang/rust/blob/ddaf12390d3ffb7d5ba74491a48f3cd528e5d777/src/librustdoc/html/markdown.rs#L571
            //
            // TODO: Normalize the anchor (lowercase).
            let anchor = anchor.to_ascii_lowercase();
            if text.chars().filter_map(slugify).collect::<String>() != anchor {
                writeln!(f, "<a id={anchor:?}></a>")?;
            }
            for _ in 0..*level {
                write!(f, "#")?;
            }
            writeln!(f, " {text}")?;
            writeln!(f)?;
            Ok(())
        }
        Content::NewTerm { inline_content } => {
            writeln!(f, "(TODO newterm: {})", contents_to_md(doc, inline_content))?;
            Ok(())
        }
        Content::Aside {
            name,
            // Styles include "warning", "important", "note" and "tip".
            // Only "warning" is supported by Rustdoc though:
            // https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html#adding-a-warning-block
            style: _,
            content,
        } => {
            writeln!(f, "<div class=\"warning\">")?;
            writeln!(f)?;

            writeln!(f, "### {}", name.as_deref().unwrap_or("Aside"))?;

            writeln!(f, "{}", contents_to_md(doc, content))?;

            writeln!(f)?;
            writeln!(f, "</div>")?;

            Ok(())
        }
        Content::TermList { items } => {
            for term in items {
                write!(
                    f,
                    "- {}: {}",
                    contents_to_md(doc, &term.term.inline_content),
                    contents_to_md(doc, &term.definition.content)
                )?;
            }

            Ok(())
        }
    })
}

fn reference_to_md<'a>(
    doc: &'a Doc,
    reference: &'a Reference,
    overriding_title: Option<&'a String>,
    overriding_title_inline_content: Option<&'a Vec<Content>>,
) -> impl fmt::Display + 'a {
    FormatterFn(move |f| match reference {
        Reference::Link {
            identifier: _,
            url,
            title: _,
            title_inline_content,
        } => {
            let title_inline_content =
                overriding_title_inline_content.unwrap_or(title_inline_content);
            write!(f, "[{}]({url})", contents_to_md(doc, title_inline_content))
        }
        Reference::Topic {
            identifier: _,
            kind,
            title,
            name: _,
            title_style: _,
            url,
            ..
        } => {
            let title = overriding_title.unwrap_or(title);
            // TODO: Make these a doc link to the actual item.
            //
            // Probably requires using the identifier to look up the item.
            if kind == "symbol" {
                write!(f, "[`{title}`](https://developer.apple.com{url})")
            } else {
                write!(f, "[{title}](https://developer.apple.com{url})")
            }
        }
        _ => write!(f, "REFERENCE TODO: {reference:?}"),
    })
}

fn slugify(c: char) -> Option<char> {
    if c.is_alphanumeric() || c == '-' || c == '_' {
        if c.is_ascii() {
            Some(c.to_ascii_lowercase())
        } else {
            Some(c)
        }
    } else if c.is_whitespace() && c.is_ascii() {
        Some('-')
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[track_caller]
    fn check(children: &[CommentChild], expected: &str) {
        let actual = Documentation {
            first: None,
            from_header: children.to_vec(),
            extras: vec![],
            alias: None,
            apple: None,
        }
        .fmt()
        .to_string();

        assert_eq!(actual, expected, "{children:?} was not");
    }

    #[test]
    fn format_simple() {
        let children = [CommentChild::Paragraph(vec![CommentChild::Text(
            " xyz.".into(),
        )])];
        check(&children, "/// xyz.\n");

        let children = [CommentChild::Paragraph(vec![
            CommentChild::Text(" abc.".into()),
            CommentChild::Text(" def.".into()),
        ])];
        check(&children, "/// abc.\n/// def.\n");
    }

    #[test]
    fn format_complex() {
        // @method initWithLayoutTag:
        // @abstract Initialize from a layout tag.
        // @param layoutTag
        //     The tag.
        // @discussion
        //     Returns nil if the tag is either ABC or
        //     DEF.
        let children = [
            CommentChild::Paragraph(vec![CommentChild::Text("\t".into())]),
            CommentChild::VerbatimLineCommand(" initWithLayoutTag:".into()),
            CommentChild::Paragraph(vec![CommentChild::Text("\t".into())]),
            CommentChild::BlockCommand(BlockCommand {
                command: "abstract".into(),
                arguments: vec![],
                children: vec![
                    CommentChild::Text(" Initialize from a layout tag.".into()),
                    CommentChild::Text("\t".into()),
                ],
            }),
            CommentChild::ParamCommand(ParamCommand {
                index: Some(0),
                parameter: "layoutTag".into(),
                direction: None,
                children: vec![
                    CommentChild::Text("\t\tThe tag.".into()),
                    CommentChild::Text("\t".into()),
                ],
            }),
            CommentChild::BlockCommand(BlockCommand {
                command: "discussion".into(),
                arguments: vec![],
                children: vec![
                    CommentChild::Text("\t\tReturns nil if the tag is either ABC or".into()),
                    CommentChild::Text("\t\tDEF.".into()),
                ],
            }),
        ];
        let expected = "/// Initialize from a layout tag.
///
/// Parameter `layoutTag`: The tag.
///
/// Returns nil if the tag is either ABC or
/// DEF.
";
        check(&children, expected);

        // @property lengthInBeats
        // @abstract XYZ
        // @discussion
        //     A
        //     B
        //     C.
        //
        //     D.
        let children = [
            CommentChild::Paragraph(vec![CommentChild::Text(" ".into())]),
            CommentChild::VerbatimLineCommand(" lengthInBeats".into()),
            CommentChild::Paragraph(vec![CommentChild::Text("\t".into())]),
            CommentChild::BlockCommand(BlockCommand {
                command: "abstract".into(),
                arguments: vec![],
                children: vec![
                    CommentChild::Text(" XYZ".into()),
                    CommentChild::Text("\t".into()),
                ],
            }),
            CommentChild::BlockCommand(BlockCommand {
                command: "discussion".into(),
                arguments: vec![],
                children: vec![
                    CommentChild::Text("\t\tA".into()),
                    CommentChild::Text("\t\tB".into()),
                    CommentChild::Text("\t\tC.".into()),
                ],
            }),
            CommentChild::Paragraph(vec![CommentChild::Text("\t\tD.".into())]),
        ];
        let expected = "/// XYZ
///
/// A
/// B
/// C.
///
/// D.
";
        check(&children, expected);

        // @method serializeToURL:error:
        // @abstract XYZ.
        // @discussion A.
        // B.
        // C.
        // @param url E.
        // @param error F.
        // @return G.
        let children = [
            CommentChild::Paragraph(vec![CommentChild::Text(" ".into())]),
            CommentChild::VerbatimLineCommand(" serializeToURL:error:".into()),
            CommentChild::Paragraph(vec![CommentChild::Text(" ".into())]),
            CommentChild::BlockCommand(BlockCommand {
                command: "abstract".into(),
                arguments: vec![],
                children: vec![
                    CommentChild::Text(" XYZ.".into()),
                    CommentChild::Text(" ".into()),
                ],
            }),
            CommentChild::BlockCommand(BlockCommand {
                command: "discussion".into(),
                arguments: vec![],
                children: vec![
                    CommentChild::Text(" A.".into()),
                    CommentChild::Text(" B.".into()),
                    CommentChild::Text(" C.".into()),
                    CommentChild::Text(" ".into()),
                ],
            }),
            CommentChild::ParamCommand(ParamCommand {
                index: Some(0),
                parameter: "url".into(),
                direction: None,
                children: vec![
                    CommentChild::Text(" E.".into()),
                    CommentChild::Text(" ".into()),
                ],
            }),
            CommentChild::ParamCommand(ParamCommand {
                index: Some(1),
                parameter: "error".into(),
                direction: None,
                children: vec![
                    CommentChild::Text(" F.".into()),
                    CommentChild::Text(" ".into()),
                ],
            }),
            CommentChild::BlockCommand(BlockCommand {
                command: "return".into(),
                arguments: vec![],
                children: vec![CommentChild::Text(" G.".into())],
            }),
        ];
        let expected = "/// XYZ.
///
/// A.
/// B.
/// C.
///
/// Parameter `url`: E.
///
/// Parameter `error`: F.
///
/// Returns: G.
";
        check(&children, expected);
    }

    #[test]
    fn multiline_with_internal_tabs() {
        let children = [
            CommentChild::Paragraph(vec![
                CommentChild::Text("\tA".into()),
                CommentChild::Text("\tB.".into()),
            ]),
            CommentChild::Paragraph(vec![CommentChild::Text("\tC.".into())]),
        ];
        let expected = "/// A\n/// B.\n///\n/// C.\n";
        check(&children, expected);
    }
}
