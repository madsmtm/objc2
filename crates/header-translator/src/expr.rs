use std::collections::HashMap;
use std::fmt;
use std::num::Wrapping;

use cexpr::expr::EvalResult;
use clang::token::{Token, TokenKind};
use clang::{Entity, EntityKind, EntityVisitResult};

use crate::unexposed_macro::UnexposedMacro;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Parsed(EvalResult),
    DeclRef(String),
    Todo,
}

pub type Identifiers = HashMap<Vec<u8>, EvalResult>;

impl Expr {
    pub fn parse(entity: &Entity<'_>, identifiers: &Identifiers) -> Option<Self> {
        let mut res = None;

        entity.visit_children(|entity, _parent| {
            match entity.get_kind() {
                EntityKind::UnexposedAttr => {
                    if let Some(macro_) = UnexposedMacro::parse(&entity) {
                        panic!("parsed macro in expr: {macro_:?}, {entity:?}");
                    }
                }
                EntityKind::UnexposedExpr => {
                    return EntityVisitResult::Recurse;
                }
                EntityKind::UnaryOperator
                | EntityKind::IntegerLiteral
                | EntityKind::ParenExpr
                | EntityKind::BinaryOperator => {
                    if res.is_some() {
                        panic!("found multiple matching children in expr");
                    }
                    let range = entity.get_range().expect("expr range");
                    let tokens = range.tokenize();
                    let tokens: Vec<_> = tokens.into_iter().filter_map(as_cexpr_token).collect();
                    let parser = cexpr::expr::IdentifierParser::new(identifiers);
                    match parser.expr(&tokens) {
                        Ok((remaining_tokens, evaluated)) if remaining_tokens.is_empty() => {
                            res = Some(Self::Parsed(evaluated));
                        }
                        _ => res = Some(Self::Todo),
                    }
                }
                EntityKind::DeclRefExpr => {
                    if res.is_some() {
                        panic!("found multiple matching children in expr");
                    }
                    let name = entity.get_name().expect("expr decl ref");
                    res = Some(Self::DeclRef(name));
                }
                kind => panic!("unknown expr kind {kind:?}"),
            }
            EntityVisitResult::Continue
        });

        res
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parsed(evaluated) => match evaluated {
                EvalResult::Int(Wrapping(n)) => write!(f, "{n}"),
                EvalResult::Float(n) => write!(f, "{n}"),
                rest => panic!("invalid expr eval result {rest:?}"),
            },
            Self::DeclRef(s) => write!(f, "Self::{}", s),
            Self::Todo => write!(f, "todo"),
        }
    }
}

/// Converts a clang::Token to a `cexpr` token if possible.
///
/// Taken from `bindgen`.
pub fn as_cexpr_token(t: Token<'_>) -> Option<cexpr::token::Token> {
    use cexpr::token;

    let kind = match t.get_kind() {
        TokenKind::Punctuation => token::Kind::Punctuation,
        TokenKind::Literal => token::Kind::Literal,
        TokenKind::Identifier => token::Kind::Identifier,
        TokenKind::Keyword => token::Kind::Keyword,
        // NB: cexpr is not too happy about comments inside
        // expressions, so we strip them down here.
        TokenKind::Comment => return None,
    };

    let spelling: Vec<u8> = t.get_spelling().into();

    Some(token::Token {
        kind,
        raw: spelling.into_boxed_slice(),
    })
}
