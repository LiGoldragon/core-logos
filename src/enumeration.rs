//! An enum declaration and its variants.

use crate::attribute::Attribute;
use crate::field::Field;
use crate::generics::Generics;
use crate::type_reference::TypeReference;
use crate::visibility::Visibility;
use name_table::Identifier;

/// An enum: `<attrs> <vis> enum <name><generics> { <variants> }`. Covers the
/// unit-only atoms, the tuple-payload operation enums, and struct-payload error
/// enums the wire goldens exercise.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Enumeration {
    pub visibility: Visibility,
    pub attributes: Vec<Attribute>,
    pub name: Identifier,
    pub generics: Generics,
    pub variants: Vec<Variant>,
}

/// One variant: a stored name and a payload shape.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Variant {
    pub name: Identifier,
    pub payload: VariantPayload,
}

/// A variant payload, dispatched by kind rather than a nullable-field flag:
/// a unit variant, a tuple of positional types, or a struct of named fields.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum VariantPayload {
    Unit,
    Tuple(Vec<TypeReference>),
    Struct(Vec<Field>),
}
