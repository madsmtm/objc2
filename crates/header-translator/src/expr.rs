use std::fmt;
use std::fmt::Write;

use clang::token::TokenKind;
use clang::{Entity, EntityKind, EntityVisitResult};

use crate::context::Context;
use crate::immediate_children;
use crate::unexposed_attr::UnexposedAttr;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    NSUIntegerMax,
    NSIntegerMax,
    Signed(i64),
    Unsigned(u64),
    String(String),
}

impl Expr {
    pub fn from_val((signed, unsigned): (i64, u64), is_signed: bool, pointer_width: usize) -> Self {
        let (signed_max, unsigned_max) = match pointer_width {
            64 => (i64::MAX, u64::MAX),
            32 => (i32::MAX as i64, u32::MAX as u64),
            16 => (i16::MAX as i64, u16::MAX as u64),
            pw => panic!("unhandled pointer width {pw}"),
        };

        if unsigned == unsigned_max {
            Expr::NSUIntegerMax
        } else if signed == signed_max {
            Expr::NSIntegerMax
        } else if is_signed {
            Expr::Signed(signed)
        } else {
            Expr::Unsigned(unsigned)
        }
    }

    pub fn parse_enum_constant(entity: &Entity<'_>, context: &Context<'_>) -> Option<Self> {
        let mut declaration_references = Vec::new();

        entity.visit_children(|entity, _parent| {
            if let EntityKind::DeclRefExpr = entity.get_kind() {
                let name = entity.get_name().expect("expr decl ref name");
                declaration_references.push(name);
            }
            EntityVisitResult::Recurse
        });

        let mut res = None;

        immediate_children(entity, |entity, _span| match entity.get_kind() {
            EntityKind::UnexposedAttr => {
                if let Some(attr) = UnexposedAttr::parse(&entity, context) {
                    error!(?attr, "unknown attribute");
                }
            }
            _ => {
                if res.is_none() {
                    res = Self::parse(&entity, &declaration_references);
                } else {
                    panic!("found multiple expressions where one was expected");
                }
            }
        });

        res
    }

    pub fn parse_var(entity: &Entity<'_>) -> Option<Self> {
        Self::parse(entity, &[])
    }

    fn parse(entity: &Entity<'_>, declaration_references: &[String]) -> Option<Self> {
        let range = entity.get_range().expect("expr range");
        let tokens = range.tokenize();

        if tokens.is_empty() {
            // TODO: Find a better way to parse macros
            return None;
        }

        let mut s = String::new();

        for token in &tokens {
            match (token.get_kind(), token.get_spelling()) {
                (TokenKind::Identifier, ident) => {
                    if declaration_references.contains(&ident) {
                        // TODO: Handle these specially when we need to
                    }
                    write!(s, "{ident}").unwrap();
                }
                (TokenKind::Literal, lit) => {
                    let lit = lit
                        .trim_end_matches("UL")
                        .trim_end_matches('L')
                        .trim_end_matches('u')
                        .trim_end_matches('U');
                    let lit = lit.replace("0X", "0x");
                    write!(s, "{lit}").unwrap();
                }
                (TokenKind::Punctuation, punct) => {
                    match &*punct {
                        // These have the same semantics in C and Rust
                        "(" | ")" | "<<" | "-" | "+" | "|" | "&" | "^" => {
                            write!(s, "{punct}").unwrap()
                        }
                        // Bitwise not
                        "~" => write!(s, "!").unwrap(),
                        punct => panic!("unknown expr punctuation {punct}"),
                    }
                }
                (kind, spelling) => panic!("unknown expr token {kind:?}/{spelling}"),
            }
        }

        // Trim casts
        s = s
            .trim_start_matches("(NSBoxType)")
            .trim_start_matches("(NSBezelStyle)")
            .trim_start_matches("(NSEventSubtype)")
            .trim_start_matches("(NSWindowButton)")
            .trim_start_matches("(NSExpressionType)")
            .to_string();

        // Trim unnecessary parentheses
        if s.starts_with('(')
            && s.ends_with(')')
            && s.chars().filter(|&c| c == '(' || c == ')').count() == 2
        {
            s = s.trim_start_matches('(').trim_end_matches(')').to_string();
        }

        // Handle four character codes
        if s.len() == 6 && s.starts_with('\'') && s.ends_with('\'') {
            let fcc = four_char_code::FourCharCode::from_str(&s[1..5])
                .expect("Invalid four character code");

            return Some(Self::Unsigned(fcc.as_u32().into()));
        }

        Some(Self::String(s))
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NSUIntegerMax => write!(f, "NSUIntegerMax as _"),
            Self::NSIntegerMax => write!(f, "NSIntegerMax as _"),
            Self::Signed(signed) => write!(f, "{signed}"),
            Self::Unsigned(unsigned) => write!(f, "{unsigned}"),
            Self::String(s) => write!(f, "{s}"),
        }
    }
}
