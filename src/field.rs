//! A named struct field carrying its own visibility as data.

use crate::type_reference::TypeReference;
use crate::visibility::Visibility;
use name_table::Identifier;

/// A field: visibility, a stored name, and a type reference. The name is always
/// present as a stored `Identifier` — snake_case elision is a text-projection
/// concern that never reaches this crate. Visibility is data at the field level
/// exactly as at the item level, so a `pub(crate)` field stores `Crate` rather
/// than being downgraded from `pub` by a projection-time reference-graph check.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Field {
    pub visibility: Visibility,
    pub name: Identifier,
    pub type_reference: TypeReference,
}
