use std::fmt;
use std::fmt::Write;

use clang::token::TokenKind;
use clang::{Entity, EntityKind, EntityVisitResult};

use crate::unexposed_macro::UnexposedMacro;

#[derive(Clone, Debug, PartialEq)]
pub struct Expr {
    s: String,
}

impl Expr {
    pub fn from_val((signed, unsigned): (i64, u64), is_signed: bool) -> Self {
        let s = if is_signed {
            format!("{}", signed)
        } else {
            format!("{}", unsigned)
        };
        Expr { s }
    }

    pub fn parse_enum_constant(entity: &Entity<'_>) -> Option<Self> {
        let mut declaration_references = Vec::new();

        entity.visit_children(|entity, _parent| {
            if let EntityKind::DeclRefExpr = entity.get_kind() {
                let name = entity.get_name().expect("expr decl ref name");
                declaration_references.push(name);
            }
            EntityVisitResult::Recurse
        });

        let mut res = None;

        entity.visit_children(|entity, _parent| {
            match entity.get_kind() {
                EntityKind::UnexposedAttr => {
                    if let Some(macro_) = UnexposedMacro::parse(&entity) {
                        panic!("parsed macro in expr: {macro_:?}, {entity:?}");
                    }
                }
                _ => {
                    if res.is_none() {
                        res = Self::parse(&entity, &declaration_references);
                    } else {
                        panic!("found multiple expressions where one was expected");
                    }
                }
            }
            EntityVisitResult::Continue
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
                    write!(s, "{}", ident).unwrap();
                }
                (TokenKind::Literal, lit) => {
                    let lit = lit
                        .trim_end_matches("UL")
                        .trim_end_matches("L")
                        .trim_end_matches("u")
                        .trim_end_matches("U");
                    let lit = lit.replace("0X", "0x");
                    write!(s, "{}", lit).unwrap();
                }
                (TokenKind::Punctuation, punct) => {
                    match &*punct {
                        // These have the same semantics in C and Rust
                        "(" | ")" | "<<" | "-" | "+" | "|" | "&" | "^" => {
                            write!(s, "{}", punct).unwrap()
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
            s = s.trim_start_matches("(").trim_end_matches(")").to_string();
        }

        Some(Self { s })
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.s)
    }
}
