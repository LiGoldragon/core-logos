//! The closed Tier-1 pattern vocabulary of match arms.
//!
//! A Tier-1 match arm binds a variant pattern — either a unit-like path
//! (`InputRoute::Record`), a tuple-variant with per-element bindings
//! (`Self::Record(_)`, `Self::Input(route)`), or the wildcard `_` catch-all that an
//! open-scrutinee match (matching a `u64` header) needs. The set stays closed: no
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
    /// The wildcard arm `_` — the catch-all of an otherwise-exhaustive match, as in
    /// the `_ => Err(SignalFrameError::UnknownHeader(header))` arm of
    /// `route_from_short_header`, where the header space is open (`u64`) and the
    /// enumerated arms cannot be exhaustive.
    Wildcard,
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
