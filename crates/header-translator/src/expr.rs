use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt;

use clang::source::SourceRange;
use clang::token::TokenKind;
use clang::{Entity, EntityKind, EntityVisitResult, EvaluationResult};
use four_char_code::FourCharCode;

use crate::availability::Availability;
use crate::context::MacroLocation;
use crate::id::ItemTree;
use crate::name_translation::enum_prefix;
use crate::rust_type::{Primitive, Ty};
use crate::unexposed_attr::UnexposedAttr;
use crate::{immediate_children, Context, ItemIdentifier, Location};

#[derive(Clone, Debug, PartialEq)]
#[allow(clippy::upper_case_acronyms)]
pub enum Token {
    Punctuation(String),
    Literal(String),
    UnknownIdent(String),
    ByteChar(u8),
    FourChar(FourCharCode),
    CStr(String),
    CFStringBegin,
    NSStringBegin,
    CFUUID(String),
    Expr(Expr),
    Cast { to: String },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Signed(i64),
    Unsigned(u64),
    Float(f64),
    MacroInvocation {
        id: ItemIdentifier,
        is_function_like: bool,
        evaluated: Option<Box<Expr>>,
    },
    Enum {
        id: ItemIdentifier,
        variant: String,
        ty: Ty,
        attrs: HashSet<UnexposedAttr>,
    },
    Const {
        id: ItemIdentifier,
        ty: Ty,
    },
    Var {
        id: ItemIdentifier,
        ty: Ty,
    },
    Tokens(Vec<Token>),
}

impl Expr {
    pub fn from_evaluated(entity: &Entity<'_>) -> Self {
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

    pub fn guess_type(&self, location: &Location) -> Ty {
        let fallback = Ty::Primitive(Primitive::UInt);

        match self {
            Self::Signed(_) => Ty::Primitive(Primitive::Int),
            Self::Unsigned(_) => Ty::Primitive(Primitive::UInt),
            Self::Float(_) => Ty::Primitive(Primitive::Float),
            Self::MacroInvocation { evaluated, .. } => {
                if let Some(evaluated) = evaluated {
                    evaluated.guess_type(location)
                } else {
                    fallback
                }
            }
            Self::Enum { ty, .. } | Self::Var { ty, .. } | Self::Const { ty, .. } => ty.clone(),
            Self::Tokens(tokens) => match &tokens[..] {
                [Token::CFStringBegin, ..] => Ty::const_cf_string_ref(),
                [Token::NSStringBegin, ..] => Ty::const_ns_string_ref(),
                [Token::CFUUID(_)] => Ty::const_cf_uuid_ref(),
                [Token::CStr(_)] => Ty::const_cstr_ref(),
                [Token::Literal(lit)] if lit.contains(".") => Ty::Primitive(Primitive::Float),
                [.., Token::Cast { to }] => {
                    Ty::TypeDef {
                        id: ItemIdentifier::from_raw(to.clone(), location.clone()),
                        // Unknown at this point what the casted type actually is.
                        to: Box::new(Ty::Primitive(Primitive::Void)),
                    }
                }
                [Token::Expr(expr)] => expr.guess_type(location),
                [Token::Punctuation(punct), ..] if punct == "-" => Ty::Primitive(Primitive::Int),
                _ => fallback,
            },
        }
    }

    pub fn update_idents(&mut self, ident_mapping: &HashMap<String, Expr>) -> bool {
        let Self::Tokens(tokens) = self else {
            return false;
        };
        let mut has_unknowns = false;
        for token in tokens {
            match token {
                Token::UnknownIdent(ident) => {
                    if let Some(expr) = ident_mapping.get(ident) {
                        debug!(?expr, "updated expression");
                        *token = Token::Expr(expr.clone());
                    } else {
                        has_unknowns = true;
                    }
                }
                Token::Expr(expr) => {
                    has_unknowns |= expr.update_idents(ident_mapping);
                }
                _ => {}
            }
        }
        has_unknowns
    }

    pub fn parse_enum_constant(entity: &Entity<'_>, context: &Context<'_>) -> Self {
        Self::parse(entity, context)
    }

    pub fn parse_var(entity: &Entity<'_>, context: &Context<'_>) -> Self {
        Self::parse(entity, context)
    }

    pub fn parse_macro_definition(entity: &Entity<'_>, context: &Context<'_>) -> Option<Self> {
        let declaration_references = BTreeMap::new();

        let [name_range] = entity.get_name_ranges()[..] else {
            error!(?entity, "got multiple name ranges");
            return None;
        };
        let range = entity.get_range().expect("macro def range");

        // Remove the macro definition name from the source range.
        let range = SourceRange::new(name_range.get_end(), range.get_end());

        if range.get_start() == range.get_end() {
            // Empty #define (like in header double-include guards).
            return None;
        }

        let tokens = range.tokenize();

        // Ignore macros that redefine themselves, like #define XYZ XYZ
        if let [token] = &*tokens {
            let name = entity.get_name().expect("macro def name");
            if token.get_kind() == TokenKind::Identifier && token.get_spelling() == name {
                return None;
            }
        }

        Self::from_tokens(&tokens, &declaration_references, context)
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
            // Remove unnecessary casts
            (EntityKind::CStyleCastExpr, [_type, child]) => Self::parse(child, context),
            (EntityKind::CStyleCastExpr, [child]) => Self::parse_from_tokens(child, context),
            (EntityKind::UnexposedExpr, _) => Self::parse_from_tokens(entity, context),
            (EntityKind::CharacterLiteral, []) => Self::parse_from_tokens(entity, context),
            (EntityKind::InitListExpr, _) => Self::parse_from_tokens(entity, context),
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

        // Get actual source range
        let location = entity.get_location().expect("expr location");
        let file = location
            .get_file_location()
            .file
            .expect("expr location file");
        let range = entity.get_range().expect("expr range");
        let start = file.get_offset_location(range.get_start().get_file_location().offset);
        let end = file.get_offset_location(range.get_end().get_file_location().offset);
        let range = SourceRange::new(start, end);

        let tokens = range.tokenize();

        if tokens.is_empty() {
            if let Some(macro_invocation) = context
                .macro_invocations
                .get(&MacroLocation::from_location(&location))
            {
                return Expr::MacroInvocation {
                    id: macro_invocation.id.clone(),
                    is_function_like: macro_invocation.is_function_like,
                    evaluated: Some(Box::new(Self::from_evaluated(entity))),
                };
            } else {
                return Self::from_evaluated(entity);
            }
        }

        Self::from_tokens(&tokens, &declaration_references, context)
            .unwrap_or_else(|| panic!("failed parsing tokens: {tokens:#?}"))
    }

    fn from_tokens(
        tokens: &[clang::token::Token<'_>],
        declaration_references: &BTreeMap<String, Expr>,
        context: &Context<'_>,
    ) -> Option<Self> {
        let mut res = vec![];

        let mut i = 0;
        while let Some(token) = tokens.get(i) {
            res.push(match (token.get_kind(), token.get_spelling()) {
                (TokenKind::Identifier, ident) => {
                    if ident == "CFSTR" {
                        Token::CFStringBegin
                    } else if ident == "CFUUIDGetConstantUUIDWithBytes" {
                        i = tokens.len() - i - 1;
                        Token::CFUUID("todo".into())
                    } else if let Some(expr) = declaration_references.get(&ident) {
                        Token::Expr(expr.clone())
                    } else if let Some(macro_invocation) = context
                        .macro_invocations
                        .get(&MacroLocation::from_location(&token.get_location()))
                    {
                        Token::Expr(Expr::MacroInvocation {
                            id: macro_invocation.id.clone(),
                            is_function_like: macro_invocation.is_function_like,
                            evaluated: macro_invocation.value.clone(),
                        })
                    } else if tokens
                        .get(i + 1)
                        .map(|token| {
                            token.get_kind() == TokenKind::Punctuation
                                && token.get_spelling() == "("
                        })
                        .unwrap_or(false)
                    {
                        // Guess that this is a macro invocation
                        Token::Expr(Expr::MacroInvocation {
                            id: ItemIdentifier::builtin(ident),
                            is_function_like: true,
                            evaluated: None,
                        })
                    } else {
                        Token::UnknownIdent(ident)
                    }
                }
                (TokenKind::Literal, lit) => {
                    if let Some(lit) = lit.strip_prefix('\'') {
                        let chars = lit
                            .strip_suffix('\'')
                            .expect("start quote to have end quote");

                        match chars.len() {
                            // Byte-character literal
                            1 => Token::ByteChar(chars.as_bytes()[0]),
                            // Four character codes
                            4 => {
                                let fcc = FourCharCode::from_str(chars)
                                    .expect("invalid four character code");

                                Token::FourChar(fcc)
                            }
                            _ => {
                                error!(?chars, "unknown length of single-quoted string");
                                Token::Literal("UNSUPPORTED".into())
                            }
                        }
                    } else if let Some(lit) = lit.strip_prefix('"') {
                        let s = lit
                            .strip_suffix('"')
                            .expect("start quote to have end quote")
                            .replace("\\p", "\\\\p");

                        Token::CStr(s.to_string())
                    } else {
                        // UL, ull, f, etc.
                        let lit = lit
                            .trim_end_matches('l')
                            .trim_end_matches('L')
                            .trim_end_matches('u')
                            .trim_end_matches('U');
                        let mut lit = lit.replace("0X", "0x");
                        if lit.contains('.') {
                            // If float
                            lit = lit.trim_end_matches('f').to_string();
                        }
                        Token::Literal(lit)
                    }
                }
                (TokenKind::Punctuation, punct) => {
                    match &*punct {
                        "(" => {
                            // Try to parse type-cast expression.
                            if let Some(token) = tokens.get(i + 1) {
                                if token.get_kind() == TokenKind::Identifier {
                                    let to = token.get_spelling();
                                    if let Some(token) = tokens.get(i + 2) {
                                        if token.get_kind() == TokenKind::Punctuation
                                            && token.get_spelling() == ")"
                                            && tokens.get(i + 3).is_some()
                                        {
                                            res.push(Token::Cast { to });
                                            i += 3;
                                            continue;
                                        }
                                    }
                                }
                            }
                            // Otherwise, treat it as a normal parenthesis.
                            Token::Punctuation(punct)
                        }
                        // These have the same semantics in C and Rust (bar overflow)
                        ")" | "<<" | ">>" | "-" | "+" | "|" | "&" | "^" | "*" | "/" | "," => {
                            Token::Punctuation(punct)
                        }
                        // Bitwise not
                        "~" => Token::Punctuation("!".to_string()),
                        // Binary/boolean not
                        "!" => Token::Punctuation("!".to_string()),
                        "@" => Token::NSStringBegin,
                        punct => {
                            error!("unknown expr punctuation {punct}");
                            Token::Punctuation(punct.to_string())
                        }
                    }
                }
                (TokenKind::Comment, spelling) => Token::Literal(spelling.to_string()),
                (TokenKind::Keyword, spelling) if spelling == "extern" => {
                    // Cannot handle `extern` in expression.
                    return None;
                }
                (TokenKind::Keyword, spelling) if spelling == "__attribute__" => {
                    // `__attribute__` is usually not a macro we want to map as an expression.
                    return None;
                }
                (kind, spelling) => {
                    error!("unknown expr token {kind:?}/{spelling}");
                    Token::Literal(spelling.to_string())
                }
            });
            i += 1;
        }

        fn trim_parens(res: &mut Vec<Token>) {
            // Trim unnecessary parentheses
            let is_left_paren = |token: &Token| matches!(token, Token::Punctuation(p) if p == "(");
            let is_right_paren = |token: &Token| matches!(token, Token::Punctuation(p) if p == ")");
            if res.first().map(is_left_paren).unwrap_or(false)
                && res.last().map(is_right_paren).unwrap_or(false)
                && res[1..res.len() - 1]
                    .iter()
                    .find(|token| is_left_paren(token) || is_right_paren(token))
                    .map(is_left_paren)
                    .unwrap_or(true)
            {
                res.remove(0);
                res.pop();
            }
        }

        trim_parens(&mut res);

        // HACK: Move cast to the end of the expression.
        let cast = if let Some(Token::Cast { .. }) = res.first() {
            Some(res.remove(0))
        } else {
            None
        };

        trim_parens(&mut res);

        // Trim leading `+`
        if matches!(res.first(), Some(Token::Punctuation(p)) if p == "+") {
            res.remove(0);
        }

        if let Some(cast) = cast {
            res.push(cast);
        }

        Some(Self::Tokens(res))
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
                assert_eq!(parent.get_kind(), EntityKind::EnumDecl);
                let parent_id = ItemIdentifier::new_optional(&parent, context);
                let variant = entity.get_name().expect("EnumConstantDecl name");

                let ty = Ty::parse_enum(
                    parent.get_enum_underlying_type().expect("enum type"),
                    context,
                );

                if parent_id.name.is_some() {
                    let parent_id = parent_id.map_name(|name| name.unwrap());

                    let mut attrs = HashSet::new();
                    let mut variants = vec![];
                    immediate_children(&parent, |entity, _span| match entity.get_kind() {
                        EntityKind::UnexposedAttr => {
                            if let Some(attr) = UnexposedAttr::parse(&entity, context) {
                                attrs.insert(attr);
                            }
                        }
                        EntityKind::EnumConstantDecl => {
                            let name = entity.get_name().expect("enum constant name");
                            let availability = Availability::parse(&entity, context);
                            variants.push((name, availability));
                        }
                        _ => {}
                    });

                    let mut relevant_enum_cases = variants
                        .iter()
                        .filter(|(_, availability)| availability.is_available_non_deprecated())
                        .map(|(name, _)| &**name)
                        .peekable();
                    let prefix = if relevant_enum_cases.peek().is_some() {
                        enum_prefix(&parent_id.name, relevant_enum_cases)
                    } else {
                        enum_prefix(&parent_id.name, variants.iter().map(|(name, _)| &**name))
                    };
                    let variant = variant.strip_prefix(prefix).unwrap_or(&variant).to_string();

                    Self::Enum {
                        id: parent_id,
                        variant,
                        ty,
                        attrs,
                    }
                } else {
                    Self::Const {
                        id: parent_id.map_name(|_| variant),
                        ty,
                    }
                }
            }
            EntityKind::VarDecl => Self::Var {
                id: ItemIdentifier::new(&definition, context),
                ty: Ty::parse_static(definition.get_type().expect("var type"), context),
            },
            _ => panic!("unknown DeclRefExpr {definition:#?} in {entity:#?}"),
        }
    }

    pub(crate) fn required_items(&self) -> impl Iterator<Item = ItemTree> {
        let mut items = Vec::new();

        match self {
            Self::Signed(_) => {}
            Self::Unsigned(_) => {}
            Self::Float(_) => {}
            Self::MacroInvocation { evaluated, id, .. } => {
                if evaluated.is_none() {
                    items.push(ItemTree::from_id(id.clone()));
                }
            }
            Self::Enum { id, ty, .. } => {
                items.push(ItemTree::new(id.clone(), ty.required_items()));
            }
            Self::Const { id, ty, .. } => {
                items.push(ItemTree::new(id.clone(), ty.required_items()));
            }
            Self::Var { id, ty } => {
                items.push(ItemTree::new(id.clone(), ty.required_items()));
            }
            Self::Tokens(tokens) => {
                for token in tokens {
                    match token {
                        Token::CStr(_) => items.push(ItemTree::core_ffi("CStr")),
                        Token::CFStringBegin => items.push(ItemTree::cf_string_macro()),
                        Token::NSStringBegin => items.push(ItemTree::ns_string_macro()),
                        Token::Expr(expr) => {
                            items.extend(expr.required_items());
                        }
                        _ => {}
                    }
                }
            }
        }

        items.into_iter()
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Signed(signed) => write!(f, "{signed}"),
            Self::Unsigned(unsigned) => write!(f, "{unsigned}"),
            Self::Float(n) => write!(f, "{n}"),
            Self::MacroInvocation {
                id,
                is_function_like,
                evaluated,
            } => {
                if id.name == "NSIntegerMax" {
                    write!(f, "NSIntegerMax as _")
                } else if id.name == "NSUIntegerMax" {
                    write!(f, "NSUIntegerMax as _")
                } else if id.name == "UINT_MAX" {
                    write!(f, "c_uint::MAX as _")
                } else if id.name == "FLT_MIN" {
                    write!(f, "c_float::MIN")
                } else if id.name == "FLT_MAX" {
                    write!(f, "c_float::MAX")
                } else if id.name == "DBL_MAX" {
                    write!(f, "c_double::MAX")
                } else if id.name == "INT32_MIN" {
                    write!(f, "i32::MIN")
                } else if id.name == "INT32_MAX" {
                    write!(f, "i32::MAX")
                } else if let Some(evaluated) = evaluated {
                    write!(f, "{evaluated}")
                } else if *is_function_like {
                    write!(f, "{}!", id.path())
                } else {
                    write!(f, "{}", id.path())
                }
            }
            Self::Enum {
                id,
                variant,
                ty,
                attrs,
            } => {
                if attrs.contains(&UnexposedAttr::ClosedEnum) {
                    // Close enums are actual Rust `enum`s, so to get their
                    // value, we use an `as` cast.
                    write!(f, "{}::{variant} as {}", id.name, ty.enum_())
                } else {
                    // Note: Even though we have the enum kind available here,
                    // we cannot avoid the `.0` here, as the expression must
                    // be `const`.
                    write!(f, "{}::{variant}.0", id.name)
                }
            }
            Self::Const { id, .. } => write!(f, "{}", id.name),
            Self::Var { id, ty } => {
                if ty.is_enum_through_typedef() {
                    write!(f, "{}.xxx", id.name)
                } else {
                    write!(f, "{}", id.name)
                }
            }
            Self::Tokens(tokens) => {
                for token in tokens {
                    match token {
                        Token::Punctuation(punct) => write!(f, "{punct}")?,
                        Token::Literal(lit) => write!(f, "{lit}")?,
                        Token::ByteChar(char) => {
                            write!(f, "b'{}' as _", core::str::from_utf8(&[*char]).unwrap())?
                        }
                        Token::FourChar(fcc) => write!(f, "{:#010x}", fcc.as_u32())?,
                        Token::UnknownIdent(ident) => write!(f, "Unknown{ident}")?,
                        // TODO: Use c"..." once in MSRV.
                        Token::CStr(s) => write!(
                            f,
                            "unsafe {{ CStr::from_bytes_with_nul_unchecked(b\"{s}\\0\") }}"
                        )?,
                        Token::CFStringBegin => write!(f, "cf_string!")?,
                        Token::NSStringBegin => write!(f, "cf_string!")?,
                        Token::CFUUID(_) => write!(f, "cf_uuid!(todo!())")?,
                        Token::Expr(expr) => write!(f, "{expr}")?,
                        Token::Cast { .. } => write!(f, "")?,
                    }
                }
                Ok(())
            }
        }
    }
}
