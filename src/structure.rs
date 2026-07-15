//! A named-field struct declaration.

use crate::attribute::Attribute;
use crate::field::Field;
use crate::generics::Generics;
use crate::visibility::Visibility;
use name_table::Identifier;

/// A named-field struct: `<attrs> <vis> struct <name><generics> { <fields> }`.
/// Each field carries its own visibility and stored name as data.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Struct {
    pub visibility: Visibility,
    pub attributes: Vec<Attribute>,
    pub name: Identifier,
    pub generics: Generics,
    pub fields: Vec<Field>,
}
