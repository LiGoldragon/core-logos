//! A `const` declaration and its value expression.

use crate::attribute::Attribute;
use crate::expression::Expression;
use crate::type_reference::TypeReference;
use crate::visibility::Visibility;
use name_table::Identifier;

/// A const: `<attrs> <vis> const <name>: <type> = <value>;`. The one node serves
/// three positions — a top-level const (`const SIGNAL_SHORT_HEADER_BYTE_COUNT: usize
/// = 8;`), a module const (`pub const INPUT_RECORD: u64 = 0x…;`), and an associated
/// const in an impl (`const HEADS: &'static [&'static str] = &[…];`) — because they
/// are one concept. Visibility is stored data: a trait-impl associated const stores
/// `Private` (Rust forbids `pub` there) and projects the empty token stream, while a
/// module const stores `Public`.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Const {
    pub visibility: Visibility,
    pub attributes: Vec<Attribute>,
    pub name: Identifier,
    pub type_reference: TypeReference,
    pub value: Expression,
}
