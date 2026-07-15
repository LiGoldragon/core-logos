//! A tuple newtype declaration.

use crate::attribute::Attribute;
use crate::type_reference::TypeReference;
use crate::visibility::Visibility;
use name_table::Identifier;

/// A tuple newtype: `<attrs> <vis> struct <name>(<wrapped>);`. The dominant wire
/// shape (`CommitSequence(Integer)`). Visibility and the whole attribute preamble
/// are stored data; the name is a stored `Identifier`.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Newtype {
    pub visibility: Visibility,
    pub attributes: Vec<Attribute>,
    pub name: Identifier,
    pub wrapped: TypeReference,
}
