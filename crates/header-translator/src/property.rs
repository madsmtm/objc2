use std::fmt;

use clang::{Entity, EntityKind, EntityVisitResult, Nullability, ObjCAttributes};

use crate::availability::Availability;
use crate::config::MethodData;
use crate::method::{MemoryManagement, Method, Qualifier};
use crate::rust_type::Ty;
use crate::unexposed_macro::UnexposedMacro;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Property {
    name: String,
    getter_name: String,
    setter_name: Option<String>,
    availability: Availability,
    is_class: bool,
    is_optional_protocol: bool,
    type_in: Ty,
    type_out: Ty,
    safe: bool,
}

impl Property {
    /// Takes `EntityKind::ObjCPropertyDecl`.
    pub fn partial(entity: Entity<'_>) -> PartialProperty<'_> {
        let attributes = entity.get_objc_attributes();
        let has_setter = attributes.map(|a| !a.readonly).unwrap_or(true);

        PartialProperty {
            entity,
            name: entity.get_display_name().expect("property getter name"),
            getter_name: entity.get_objc_getter_name().expect("property getter name"),
            setter_name: has_setter.then(|| {
                entity
                    .get_objc_setter_name()
                    .expect("property setter name")
                    .trim_end_matches(|c| c == ':')
                    .to_string()
            }),
            is_class: attributes.map(|a| a.class).unwrap_or(false),
            attributes: entity.get_objc_attributes(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PartialProperty<'tu> {
    entity: Entity<'tu>,
    pub name: String,
    pub getter_name: String,
    pub setter_name: Option<String>,
    pub is_class: bool,
    attributes: Option<ObjCAttributes>,
}

impl PartialProperty<'_> {
    pub fn parse(self, data: MethodData) -> Option<Property> {
        let Self {
            entity,
            name,
            getter_name,
            setter_name,
            is_class,
            attributes,
        } = self;

        if data.skipped {
            return None;
        }

        // println!("Property {getter_name:?}/{setter_name:?}: {attributes:?}");

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

        let type_in = Ty::parse_property(
            entity.get_type().expect("property type"),
            Nullability::Unspecified,
        );
        let type_out = Ty::parse_property_return(
            entity.get_type().expect("property type"),
            default_nullability,
        );

        let mut memory_management = MemoryManagement::Normal;

        entity.visit_children(|entity, _parent| {
            match entity.get_kind() {
                EntityKind::ObjCClassRef
                | EntityKind::ObjCProtocolRef
                | EntityKind::TypeRef
                | EntityKind::ParmDecl => {
                    // Ignore
                }
                EntityKind::ObjCReturnsInnerPointer => {
                    if memory_management != MemoryManagement::Normal {
                        panic!("got unexpected ObjCReturnsInnerPointer")
                    }
                    memory_management = MemoryManagement::ReturnsInnerPointer;
                }
                EntityKind::ObjCInstanceMethodDecl => {
                    println!("method in property: {entity:?}");
                }
                EntityKind::IbOutletAttr => {
                    // TODO: What is this?
                }
                EntityKind::UnexposedAttr => {
                    if let Some(macro_) = UnexposedMacro::parse(&entity) {
                        println!("property {name}: {macro_:?}");
                    }
                }
                _ => panic!("Unknown property child: {:?}, {:?}", entity, _parent),
            };
            EntityVisitResult::Continue
        });

        let qualifier = entity.get_objc_qualifiers().map(Qualifier::parse);
        assert_eq!(qualifier, None, "properties do not support qualifiers");

        Some(Property {
            name,
            getter_name,
            setter_name,
            availability,
            is_class,
            is_optional_protocol: entity.is_objc_optional(),
            type_in,
            type_out,
            safe: !data.unsafe_,
        })
    }
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let method = Method {
            selector: self.getter_name.clone(),
            fn_name: self.getter_name.clone(),
            availability: self.availability.clone(),
            is_class: self.is_class,
            is_optional_protocol: self.is_optional_protocol,
            memory_management: MemoryManagement::Normal,
            designated_initializer: false,
            arguments: Vec::new(),
            result_type: self.type_out.clone(),
            safe: self.safe,
        };
        write!(f, "{method}")?;
        if let Some(setter_name) = &self.setter_name {
            writeln!(f, "")?;
            let method = Method {
                selector: setter_name.clone() + ":",
                fn_name: setter_name.clone(),
                availability: self.availability.clone(),
                is_class: self.is_class,
                is_optional_protocol: self.is_optional_protocol,
                memory_management: MemoryManagement::Normal,
                designated_initializer: false,
                arguments: Vec::from([(self.name.clone(), None, self.type_in.clone())]),
                result_type: Ty::VOID_RESULT,
                safe: self.safe,
            };
            write!(f, "{method}")?;
        }
        Ok(())
    }
}
