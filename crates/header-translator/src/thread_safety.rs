use clang::{Entity, EntityKind};

use crate::{
    immediate_children,
    method::MethodModifiers,
    stmt::{method_or_property_entities, parse_direct_protocols, parse_superclasses},
    unexposed_attr::UnexposedAttr,
    Context, ItemIdentifier, Mutability,
};

fn parse(entity: &Entity<'_>, context: &Context<'_>) -> (Option<bool>, bool) {
    let mut sendable = None;
    let mut mainthreadonly = false;

    immediate_children(entity, |entity, _span| {
        if let EntityKind::UnexposedAttr = entity.get_kind() {
            if let Some(attr) = UnexposedAttr::parse(&entity, context) {
                match attr {
                    UnexposedAttr::Sendable => {
                        sendable = Some(true);
                    }
                    UnexposedAttr::NonSendable => {
                        sendable = Some(false);
                    }
                    UnexposedAttr::UIActor => {
                        mainthreadonly = true;
                    }
                    _ => {}
                }
            }
        }
    });

    (sendable, mainthreadonly)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ThreadSafetyAttr {
    /// The item is only accessible from the main thread.
    ///
    /// This implies that the item is not sendable.
    MainThreadOnly,
    /// The item is sendable.
    Sendable,
    /// The item is not sendable.
    NotSendable,
}

impl ThreadSafetyAttr {
    fn search_protocols(entity: &Entity<'_>, context: &Context<'_>) -> Option<Self> {
        for protocol in parse_direct_protocols(entity, context) {
            if let Some(explicit) = Self::parse_explicit_decl(entity, context) {
                return Some(explicit);
            }
            // Recurse
            if let Some(searched) = Self::search_protocols(&protocol, context) {
                return Some(searched);
            }
        }

        None
    }

    fn parse_explicit_decl(entity: &Entity<'_>, context: &Context<'_>) -> Option<Self> {
        let mut attr = match parse(entity, context) {
            (sendable, true) => {
                if sendable == Some(true) {
                    error!("tried to make type both Sendable and MainThreadOnly");
                }
                Some(Self::MainThreadOnly)
            }
            (Some(true), false) => Some(Self::Sendable),
            (Some(false), false) => Some(Self::NotSendable),
            (None, false) => None,
        };

        match entity.get_kind() {
            EntityKind::ObjCInterfaceDecl => {
                let data = context
                    .class_data
                    .get(&ItemIdentifier::new(entity, context).name);

                if data.map(|data| data.mutability.clone()).unwrap_or_default()
                    == Mutability::MainThreadOnly
                {
                    return Some(Self::MainThreadOnly);
                }
            }
            EntityKind::ObjCProtocolDecl => {
                let data = context
                    .protocol_data
                    .get(&ItemIdentifier::new(entity, context).name);

                // Set the protocol as main thread only if all methods are
                // explicitly _marked_ (not inferred, since then we'd have to
                // recurse into types) main thread only.
                //
                // This is done to make the UI nicer when the user tries to
                // implement such traits.
                //
                // Note: This is a deviation from the headers, but I don't
                // see a way for this to be unsound? As an example, let's say
                // there is some Objective-C code that assumes it can create
                // an object which is not `MainThreadOnly`, and then sets it
                // as the application delegate.
                //
                // Rust code that later retrieves the delegate would assume
                // that the object is `MainThreadOnly`, and could use this
                // information to create `MainThreadMarker`; but they can
                // _already_ do that, since the only way to retrieve the
                // delegate in the first place would be through
                // `NSApplication`!
                let entities = method_or_property_entities(entity, |name| {
                    data.and_then(|data| data.methods.get(name))
                        .copied()
                        .unwrap_or_default()
                });
                if !entities.is_empty()
                    && entities.iter().all(|method_or_property| {
                        MethodModifiers::parse(method_or_property, context).mainthreadonly
                    })
                {
                    attr = Some(Self::MainThreadOnly);
                }

                // Overwrite with config preference
                if let Some(data) = data
                    .map(|data| data.requires_mainthreadonly)
                    .unwrap_or_default()
                {
                    if data {
                        if attr == Some(Self::MainThreadOnly) {
                            warn!("set `requires-mainthreadonly = true`, but the protocol was already marked as main thread only");
                        }
                        attr = Some(Self::MainThreadOnly);
                    } else {
                        error!("cannot set `requires-mainthreadonly = false`");
                    }
                }
            }
            kind => error!(?kind, "invalid decl for thread safety"),
        }

        attr
    }

    fn parse_inferred_decl(entity: &Entity<'_>, context: &Context<'_>) -> Self {
        match entity.get_kind() {
            EntityKind::ObjCInterfaceDecl => {
                let parsed_explicit: Vec<_> = parse_superclasses(entity, context)
                    .iter()
                    .flat_map(|(_, _, entity)| Self::parse_explicit_decl(entity, context))
                    .collect();

                for attr in &parsed_explicit {
                    // If any superclass is MainThreadOnly, then this type
                    // is as well.
                    if let Self::MainThreadOnly = attr {
                        return Self::MainThreadOnly;
                    }
                }
                // Otherwise, take thread safety from the first superclass
                if let Some(attr) = parsed_explicit.first() {
                    return *attr;
                }

                // Classes default to not being sendable
                Self::NotSendable
            }
            EntityKind::ObjCProtocolDecl => {
                // If the protocol wasn't declared main thread itself, try
                // to search inherited / super protocols instead.
                Self::search_protocols(entity, context).unwrap_or(Self::NotSendable)
            }
            kind => {
                error!(?kind, "invalid decl for thread safety");
                Self::NotSendable
            }
        }
    }
}

/// Information about thread-safety properties of a type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ThreadSafety {
    /// What the attribute was explicitly declared as.
    explicit: Option<ThreadSafetyAttr>,
    /// What the attribute was inferred to be.
    inferred: ThreadSafetyAttr,
}

impl ThreadSafety {
    pub(crate) fn from_decl(entity: &Entity<'_>, context: &Context<'_>) -> Self {
        let explicit = ThreadSafetyAttr::parse_explicit_decl(entity, context);
        let inferred =
            explicit.unwrap_or_else(|| ThreadSafetyAttr::parse_inferred_decl(entity, context));
        Self { explicit, inferred }
    }

    pub(crate) fn inferred_mainthreadonly(&self) -> bool {
        self.inferred == ThreadSafetyAttr::MainThreadOnly
    }

    pub(crate) fn explicit_mainthreadonly(&self) -> bool {
        self.explicit == Some(ThreadSafetyAttr::MainThreadOnly)
    }

    pub(crate) fn explicit_sendable(&self) -> bool {
        self.explicit == Some(ThreadSafetyAttr::Sendable)
    }
}
