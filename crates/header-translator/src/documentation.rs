use std::fmt::{self, Write as _};

use clang::documentation::{
    BlockCommand, CommentChild, HtmlStartTag, InlineCommand, InlineCommandStyle, ParamCommand,
    TParamCommand,
};
use clang::Entity;

use crate::display_helper::FormatterFn;
use crate::ItemIdentifier;

#[derive(Debug, Clone, PartialEq)]
pub struct Documentation {
    children: Vec<CommentChild>,
}

impl Documentation {
    pub fn from_entity(entity: &Entity<'_>) -> Self {
        if let Some(comment) = entity.get_comment() {
            if let Some(parsed) = entity.get_parsed_comment() {
                Self {
                    children: parsed.get_children(),
                }
            } else {
                warn!(?entity, comment, "had comment, but not parsed comment");
                Self {
                    children: Vec::new(),
                }
            }
        } else {
            Self {
                children: Vec::new(),
            }
        }
    }

    pub fn property_setter(getter_sel: &str) -> Self {
        // Emit setter docs to link to getter (otherwise we'd have to
        // duplicate the documentation across getter and setter).
        let text = format!("Setter for [`{getter_sel}`][Self::{getter_sel}].");
        Self {
            children: vec![CommentChild::Paragraph(vec![CommentChild::Text(text)])],
        }
    }

    pub fn fmt<'a>(&'a self, doc_id: Option<&'a ItemIdentifier>) -> impl fmt::Display + 'a {
        FormatterFn(move |f| {
            let mut s = String::new();

            for child in &self.children {
                write!(&mut s, "{}", format_child(child))?;
            }

            let s = fix_code_blocks(&s)
                .trim()
                .replace("\n", "\n/// ")
                .replace("/// \n", "///\n")
                .replace("\t", "    ");

            if !s.is_empty() {
                writeln!(f, "/// {s}")?;
            }

            // Generate a markdown link to Apple's documentation.
            //
            // This is best effort only, and doesn't work for functions and
            // methods, and possibly some renamed classes and traits.
            //
            // Additionally, the link may redirect.
            if let Some(id) = doc_id {
                if s.is_empty() {
                    write!(f, "/// ")?;
                } else {
                    writeln!(f, "///")?;
                    write!(f, "/// See also ")?;
                }
                writeln!(
                    f,
                    "[Apple's documentation](https://developer.apple.com/documentation/{}/{}?language=objc)",
                    id.library_name().to_lowercase(),
                    id.name.to_lowercase()
                )?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[track_caller]
    fn check(children: &[CommentChild], expected: &str) {
        let actual = Documentation {
            children: children.to_vec(),
        }
        .fmt(None)
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
