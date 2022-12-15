use clang::{Entity, EntityKind, Nullability, ObjCAttributes};
use tracing::span::EnteredSpan;
use tracing::{error, warn};

use crate::availability::Availability;
use crate::config::MethodData;
use crate::immediate_children;
use crate::method::{MemoryManagement, Method, Qualifier};
use crate::rust_type::Ty;
use crate::unexposed_macro::UnexposedMacro;

#[derive(Debug)]
pub struct PartialProperty<'tu> {
    pub entity: Entity<'tu>,
    pub name: String,
    pub getter_name: String,
    pub setter_name: Option<String>,
    pub is_class: bool,
    pub attributes: Option<ObjCAttributes>,
    pub _span: EnteredSpan,
}

impl PartialProperty<'_> {
    pub fn parse(
        self,
        getter_data: MethodData,
        setter_data: Option<MethodData>,
    ) -> (Option<Method>, Option<Method>) {
        let Self {
            entity,
            name,
            getter_name,
            setter_name,
            is_class,
            attributes,
            _span,
        } = self;

        let availability = Availability::parse(
            entity
                .get_platform_availability()
                .expect("method availability"),
        );

        // `@property(copy)` for some reason returns nonnull?
        //
        // Swift signifies that they use forced unwrapping here, perhaps
        // because they know that it can fail (e.g. in OOM situations), but
        // is very unlikely to?
        let default_nullability = if attributes.map(|a| a.copy).unwrap_or(false) {
            Nullability::NonNull
        } else {
            Nullability::Unspecified
        };

        let mut memory_management = MemoryManagement::Normal;

        immediate_children(&entity, |entity, _span| match entity.get_kind() {
            EntityKind::ObjCClassRef
            | EntityKind::ObjCProtocolRef
            | EntityKind::TypeRef
            | EntityKind::ParmDecl => {
                // Ignore
            }
            EntityKind::ObjCReturnsInnerPointer => {
                if memory_management != MemoryManagement::Normal {
                    error!(?memory_management, "unexpected ObjCReturnsInnerPointer")
                }
                memory_management = MemoryManagement::ReturnsInnerPointer;
            }
            EntityKind::ObjCInstanceMethodDecl => {
                warn!("method in property");
            }
            EntityKind::IbOutletAttr => {
                // TODO: What is this?
            }
            EntityKind::UnexposedAttr => {
                if let Some(macro_) = UnexposedMacro::parse(&entity) {
                    warn!(?macro_, "unknown macro");
                }
            }
            _ => warn!("unknown"),
        });

        let qualifier = entity.get_objc_qualifiers().map(Qualifier::parse);
        if qualifier.is_some() {
            error!("properties do not support qualifiers");
        }

        let getter = if !getter_data.skipped {
            let ty = Ty::parse_property_return(
                entity.get_type().expect("property type"),
                default_nullability,
            );

            Some(Method {
                selector: getter_name.clone(),
                fn_name: getter_name,
                availability: availability.clone(),
                is_class,
                is_optional_protocol: entity.is_objc_optional(),
                memory_management,
                arguments: Vec::new(),
                result_type: ty,
                safe: !getter_data.unsafe_,
                mutating: getter_data.mutating,
            })
        } else {
            None
        };

        let setter = if let Some(setter_name) = setter_name {
            let setter_data = setter_data.expect("setter_data must be present if setter_name was");
            if !setter_data.skipped {
                let ty = Ty::parse_property(
                    entity.get_type().expect("property type"),
                    Nullability::Unspecified,
                );

                Some(Method {
                    selector: setter_name.clone() + ":",
                    fn_name: setter_name,
                    availability: availability.clone(),
                    is_class,
                    is_optional_protocol: entity.is_objc_optional(),
                    memory_management,
                    arguments: vec![(name, None, ty)],
                    result_type: Ty::VOID_RESULT,
                    safe: !setter_data.unsafe_,
                    mutating: setter_data.mutating,
                })
            } else {
                None
            }
        } else {
            None
        };

        (getter, setter)
    }
}
