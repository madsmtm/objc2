use clang::{Entity, EntityVisitResult};
use tracing::span::EnteredSpan;

pub fn immediate_children<'tu>(
    entity: &Entity<'tu>,
    mut closure: impl FnMut(Entity<'tu>, EnteredSpan),
) {
    entity.visit_children(|entity, _parent| {
        let span = debug_span!(
            "child",
            kind = ?entity.get_kind(),
            dbg = entity.get_name(),
        )
        .entered();

        closure(entity, span);

        EntityVisitResult::Continue
    });
}
