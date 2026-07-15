//! A type-alias declaration.

use crate::attribute::Attribute;
use crate::generics::Generics;
use crate::type_reference::TypeReference;
use crate::visibility::Visibility;
use name_table::Identifier;

/// A type alias: `<attrs> <vis> type <name><generics> = <target>;`. Covers the
/// scalar aliases at the top of every wire module (`pub type Integer = u64;`),
/// whose target is a plain path, and any generic alias whose target is an
/// application.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Alias {
    pub visibility: Visibility,
    pub attributes: Vec<Attribute>,
    pub name: Identifier,
    pub generics: Generics,
    pub target: TypeReference,
}
