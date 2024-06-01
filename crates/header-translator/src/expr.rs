use std::collections::BTreeMap;
use std::fmt;

use clang::token::TokenKind;
use clang::{Entity, EntityKind, EntityVisitResult, EvaluationResult};

use crate::rust_type::Ty;
use crate::stmt::{enum_constant_name, new_enum_id};
use crate::{Context, ItemIdentifier};

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Punctuation(String),
    Literal(String),
    Expr(Expr),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Signed(i64),
    Unsigned(u64),
    Float(f64),
    MacroInvocation {
        id: ItemIdentifier,
        evaluated: Option<Box<Expr>>,
    },
    Enum {
        id: ItemIdentifier,
        variant: String,
        // TODO: Type
    },
    Const(ItemIdentifier), // TODO: Type
    Var {
        id: ItemIdentifier,
        ty: Ty,
    },
    Tokens(Vec<Token>),
}

impl Expr {
    fn from_evaluated(entity: &Entity<'_>) -> Self {
        let res = entity
            .evaluate()
            .expect("must be able to evaluate result of macro in expression");
        match res {
            EvaluationResult::SignedInteger(n) => Expr::Signed(n),
            EvaluationResult::UnsignedInteger(n) => Expr::Unsigned(n),
            EvaluationResult::Float(n) => Self::Float(n),
            res => panic!("unexpected evaluation result {res:?}"),
        }
    }

    pub fn parse_enum_constant(entity: &Entity<'_>, context: &Context<'_>) -> Self {
        Self::parse(entity, context)
    }

    pub fn parse_var(entity: &Entity<'_>, context: &Context<'_>) -> Self {
        Self::parse(entity, context)
    }

    fn parse(entity: &Entity<'_>, context: &Context<'_>) -> Self {
        match (entity.get_kind(), &*entity.get_children()) {
            (EntityKind::ParenExpr, [_]) => {
                // TODO: Remove unnecessary top-level parentheses
                // Self::parse(child, context)
                Self::parse_from_tokens(entity, context)
            }
            (EntityKind::DeclRefExpr, []) => Self::parse_from_decl_ref(entity, context),
            // We can't really use the information in here for much, since the
            // kind of operator is not exposed. So we fall back to parsing raw
            // tokens instead.
            (EntityKind::UnaryOperator, [_]) => Self::parse_from_tokens(entity, context),
            (EntityKind::BinaryOperator, [_, _]) => Self::parse_from_tokens(entity, context),
            (EntityKind::IntegerLiteral, []) => Self::parse_from_tokens(entity, context),
            (EntityKind::FloatingLiteral, []) => Self::parse_from_tokens(entity, context),
            // Remove unnecessary cast
            (EntityKind::CStyleCastExpr, [_type, child]) => Self::parse(child, context),
            (EntityKind::UnexposedExpr, _) => Self::parse_from_tokens(entity, context),
            (_, children) => panic!("unknown expr: {entity:?}, {children:#?}"),
        }
    }

    fn parse_from_tokens(entity: &Entity<'_>, context: &Context<'_>) -> Self {
        let mut declaration_references = BTreeMap::new();

        entity.visit_children(|entity, _parent| {
            if let EntityKind::DeclRefExpr = entity.get_kind() {
                let name = entity.get_name().expect("DeclRefExpr name");
                declaration_references.insert(name, Self::parse_from_decl_ref(&entity, context));
            }
            EntityVisitResult::Recurse
        });

        let range = entity.get_range().expect("expr range");
        let tokens = range.tokenize();

        if tokens.is_empty() {
            let location = entity.get_location().expect("expr location");
            if let Some(macro_invocation) = context
                .macro_invocations
                .get(&location.get_spelling_location())
            {
                return Expr::MacroInvocation {
                    id: ItemIdentifier::new(macro_invocation, context),
                    evaluated: Some(Box::new(Self::from_evaluated(entity))),
                };
            } else {
                return Self::from_evaluated(entity);
            }
        }

        let mut res = vec![];

        for token in &tokens {
            res.push(match (token.get_kind(), token.get_spelling()) {
                (TokenKind::Identifier, ident) => {
                    Token::Expr(if let Some(expr) = declaration_references.get(&ident) {
                        expr.clone()
                    } else {
                        let macro_invocation = context
                            .macro_invocations
                            .get(&token.get_location().get_spelling_location())
                            .expect("expr macro invocation");
                        Expr::MacroInvocation {
                            id: ItemIdentifier::new(macro_invocation, context),
                            evaluated: None,
                        }
                    })
                }
                (TokenKind::Literal, lit) => {
                    let lit = lit
                        .trim_end_matches("UL")
                        .trim_end_matches('L')
                        .trim_end_matches('u')
                        .trim_end_matches('U');
                    let lit = lit.replace("0X", "0x");
                    if let Some(lit) = lit.strip_prefix('\'') {
                        // Byte-character literal
                        let char = lit
                            .strip_suffix('\'')
                            .expect("start quote to have end quote");
                        Token::Literal(format!("b'{char}' as _"))
                    } else {
                        Token::Literal(lit)
                    }
                }
                (TokenKind::Punctuation, punct) => {
                    match &*punct {
                        // These have the same semantics in C and Rust
                        "(" | ")" | "<<" | "-" | "+" | "|" | "&" | "^" => Token::Punctuation(punct),
                        // Bitwise not
                        "~" => Token::Punctuation("!".to_string()),
                        punct => panic!("unknown expr punctuation {punct}"),
                    }
                }
                (kind, spelling) => panic!("unknown expr token {kind:?}/{spelling}"),
            });
        }

        // Trim unnecessary parentheses
        if res.first() == Some(&Token::Punctuation("(".to_string()))
            && res.last() == Some(&Token::Punctuation(")".to_string()))
        {
            res.remove(0);
            res.pop();
        }

        Self::Tokens(res)
    }

    fn parse_from_decl_ref(entity: &Entity<'_>, context: &Context<'_>) -> Self {
        assert_eq!(entity.get_kind(), EntityKind::DeclRefExpr);
        let definition = entity.get_definition().expect("DeclRefExpr definition");
        assert_eq!(entity.get_name(), definition.get_name());
        match definition.get_kind() {
            EntityKind::EnumConstantDecl => {
                let parent = definition
                    .get_semantic_parent()
                    .expect("EnumConstantDecl parent");
                let parent_id = new_enum_id(&parent, context);
                let name = entity.get_name().expect("EnumConstantDecl name");
                if parent_id.name.is_some() {
                    Self::Enum {
                        id: parent_id.map_name(|name| name.unwrap()),
                        variant: name,
                    }
                } else {
                    Self::Const(parent_id.map_name(|_| name))
                }
            }
            EntityKind::VarDecl => Self::Var {
                id: ItemIdentifier::new(&definition, context),
                ty: Ty::parse_static(definition.get_type().expect("var type"), context),
            },
            _ => panic!("unknown DeclRefExpr {definition:#?} in {entity:#?}"),
        }
    }

    pub(crate) fn required_items(&self) -> Vec<ItemIdentifier> {
        let mut items = Vec::new();

        match self {
            Self::Signed(_) => {}
            Self::Unsigned(_) => {}
            Self::Float(_) => {}
            Self::MacroInvocation { evaluated, id } => {
                if evaluated.is_none() {
                    items.push(id.clone());
                }
            }
            Self::Enum { id, .. } => {
                items.push(id.clone());
            }
            Self::Const(id) => {
                items.push(id.clone());
            }
            Self::Var { id, ty } => {
                items.push(id.clone());
                items.extend(ty.required_items());
            }
            Self::Tokens(tokens) => {
                for token in tokens {
                    match token {
                        Token::Punctuation(_) => {}
                        Token::Literal(_) => {}
                        Token::Expr(expr) => {
                            items.extend(expr.required_items());
                        }
                    }
                }
            }
        }

        items
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Signed(signed) => write!(f, "{signed}"),
            Self::Unsigned(unsigned) => write!(f, "{unsigned}"),
            Self::Float(n) => write!(f, "{n}"),
            Self::MacroInvocation { id, evaluated } => {
                if id.name == "NSIntegerMax" {
                    write!(f, "NSIntegerMax as _")
                } else if id.name == "NSUIntegerMax" {
                    write!(f, "NSUIntegerMax as _")
                } else if id.name == "FLT_MAX" {
                    write!(f, "c_float::MAX")
                } else if id.name == "DBL_MAX" {
                    write!(f, "c_double::MAX")
                } else if let Some(evaluated) = evaluated {
                    write!(f, "{evaluated}")
                } else {
                    write!(f, "{}", id.path())
                }
            }
            Self::Enum { id, variant } => {
                let pretty_name = enum_constant_name(&id.name, variant);
                // Note: Even if we had the enum kind available here, we would
                // not be able to avoid the `.0` here, as the expression must
                // be `const`.
                write!(f, "{}::{pretty_name}.0", id.name)
            }
            Self::Const(id) => write!(f, "{}", id.name),
            Self::Var { id, ty } => {
                if ty.is_enum_through_typedef() {
                    write!(f, "{}.0", id.name)
                } else {
                    write!(f, "{}", id.name)
                }
            }
            Self::Tokens(tokens) => {
                for token in tokens {
                    match token {
                        Token::Punctuation(punct) => write!(f, "{punct}")?,
                        Token::Literal(lit) => write!(f, "{lit}")?,
                        Token::Expr(expr) => write!(f, "{expr}")?,
                    }
                }
                Ok(())
            }
        }
    }
}
