//! A tuple newtype declaration.

use crate::attribute::Attribute;
use crate::type_reference::TypeReference;
use crate::visibility::Visibility;
use name_table::Identifier;

/// A tuple newtype: `<attrs> <vis> struct <name>(<wrapped_visibility> <wrapped>);`.
/// The dominant wire shape (`CommitSequence(Integer)`). Visibility and the whole
/// attribute preamble are stored data; the name is a stored `Identifier`.
///
/// The single tuple field carries its own `wrapped_visibility`, stored data exactly
/// as at the item level and the named-field level ([`crate::field::Field`]): a
/// `pub`-field tuple struct (`TraceEvent(pub ObjectName)`) stores `Public`, the
/// bare form (`CommitSequence(Integer)`) stores `Private`, and `Private` projects
/// to the empty token stream — so the "no `pub` on the field" special case
/// dissolves into the normal case, never a projection-time guess.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Newtype {
    pub visibility: Visibility,
    pub attributes: Vec<Attribute>,
    pub name: Identifier,
    pub wrapped_visibility: Visibility,
    pub wrapped: TypeReference,
}
