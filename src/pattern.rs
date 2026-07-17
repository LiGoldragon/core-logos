//! The closed Tier-1 pattern vocabulary of match arms.
//!
//! A Tier-1 match arm binds a variant pattern — either a unit-like path
//! (`InputRoute::Record`) or a tuple-variant with per-element bindings
//! (`Self::Record(_)`, `Self::Input(route)`). The set is closed: no wildcard arm, no
//! literal pattern, no struct pattern, no or-pattern. Dispatch is on the node's
//! kind, never on a head string.

use crate::path::PathNode;
use name_table::Identifier;

/// A Tier-1 match-arm pattern.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum Pattern {
    /// A path pattern matching a unit-like variant: `InputRoute::Record`.
    Path(PathNode),
    /// A tuple-variant pattern binding or ignoring positional payloads:
    /// `Self::Record(_)`, `Self::Input(route)`.
    TupleVariant(TupleVariantPattern),
}

/// A tuple-variant pattern: a variant path applied to positional element patterns.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct TupleVariantPattern {
    pub path: PathNode,
    pub elements: Vec<PatternElement>,
}

/// One positional element of a tuple-variant pattern: a wildcard `_` or an
/// identifier binding `route`.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum PatternElement {
    Wildcard,
    Binding(Identifier),
}
