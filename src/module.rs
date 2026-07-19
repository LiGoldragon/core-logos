//! An inline module declaration and the items it encloses.

use crate::attribute::Attribute;
use crate::item::EncodedItem;
use crate::visibility::Visibility;
use name_table::Identifier;

/// An inline module: `<attrs> <vis> mod <name> { <items> }`. The witnessed shape is
/// the `short_header` const module (`pub mod short_header { pub const INPUT_RECORD:
/// u64 = 0x…; … }`): a named module whose items are consts. A bare `mod name;` file
/// module has no inline items and is out of the modeled shape. Recursive through
/// `items` (a module holds `EncodedItem`s, one of which may itself be a module), so the
/// rkyv self-referential bounds are stated with `omit_bounds` on the item vector.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
#[rkyv(
    serialize_bounds(__S: rkyv::ser::Writer + rkyv::ser::Allocator, __S::Error: rkyv::rancor::Source),
    deserialize_bounds(__D::Error: rkyv::rancor::Source),
    bytecheck(bounds(__C: rkyv::validation::ArchiveContext, __C::Error: rkyv::rancor::Source)),
)]
pub struct Module {
    pub visibility: Visibility,
    pub attributes: Vec<Attribute>,
    pub name: Identifier,
    #[rkyv(omit_bounds)]
    pub items: Vec<EncodedItem>,
}
