use std::iter;

use clang::{Entity, EntityKind};

use crate::{immediate_children, Context, ItemIdentifier};

/// Parse the directly referenced protocols of a declaration.
pub(crate) fn parse_direct_protocols<'clang>(
    entity: &Entity<'clang>,
    _context: &Context<'_>,
) -> Vec<Entity<'clang>> {
    let mut protocols = Vec::new();

    #[allow(clippy::single_match)]
    immediate_children(entity, |child, _span| match child.get_kind() {
        EntityKind::ObjCProtocolRef => {
            let child = child
                .get_reference()
                .expect("ObjCProtocolRef to reference entity");
            if child == *entity {
                error!(?entity, "recursive protocol");
            } else {
                protocols.push(child);
            }
        }
        _ => {}
    });

    protocols
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProtocolRef {
    pub(crate) id: ItemIdentifier,
    pub(crate) super_protocols: Vec<ProtocolRef>,
}

impl ProtocolRef {
    pub(crate) fn super_protocols(entity: &Entity<'_>, context: &Context<'_>) -> Vec<Self> {
        let mut super_protocols = Vec::new();

        for entity in parse_direct_protocols(entity, context) {
            super_protocols.push(ProtocolRef::from_entity(&entity, context));
        }

        super_protocols
    }

    pub(crate) fn from_entity(entity: &Entity<'_>, context: &Context<'_>) -> Self {
        let mut super_protocols = Vec::new();

        for entity in parse_direct_protocols(entity, context) {
            super_protocols.push(ProtocolRef::from_entity(&entity, context));
        }

        Self {
            id: context.replace_protocol_name(ItemIdentifier::new(&entity, context)),
            super_protocols,
        }
    }

    pub(crate) fn required_items(&self) -> Vec<ItemIdentifier> {
        self.super_protocols
            .iter()
            .flat_map(|super_protocol| super_protocol.required_items())
            .chain(iter::once(self.id.clone()))
            .chain(iter::once(ItemIdentifier::objc("__macros__")))
            .collect()
    }
}
