//! Attributes as a plain ordered vector of data — never computed at projection.

use crate::path::PathNode;
use name_table::Identifier;

/// One attribute node. The golden preamble is three of these in a fixed order:
/// a `ToolPath` (`#[rustfmt::skip]`), a `Configuration` wrapping a `Derive` (the
/// feature-gated NOTA derive group), and a plain `Derive` (the rkyv group). Both
/// derive groups are just entries in the ordered vector — there is no "two derive
/// groups" concept, and nothing is computed at projection: the whole derive set,
/// including any conditional members, is stored data transcribed in order.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum Attribute {
    /// `#[derive(<paths>)]`.
    Derive(DeriveGroup),
    /// `#[cfg_attr(<predicate>, <inner>)]`.
    Configuration(ConfigurationAttribute),
    /// `#[cfg(<predicate>)]` — a plain configuration gate on the whole item, e.g.
    /// `#[cfg(feature = "nota-text")]` above the NOTA import. Distinct from
    /// `Configuration`: that conditionally applies an *inner* attribute, this gates
    /// the item's compilation. Reuses `ConfigurationPredicate`, the one predicate
    /// vocabulary shared with `cfg_attr`.
    Cfg(ConfigurationPredicate),
    /// A bare dotted tool attribute, e.g. `#[rustfmt::skip]` as `PathNode[rustfmt, skip]`.
    /// Reuses `PathNode` (not opaque text) so the `::` materializes at projection.
    ToolPath(PathNode),
    /// A namespaced helper attribute deriving traits on a generated type, e.g.
    /// `#[rkyv(derive(PartialOrd, Ord))]`.
    HelperDerive(HelperDerive),
}

/// A derive group: an ordered vector of paths. `#[derive(<paths joined by ", ">)]`.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct DeriveGroup {
    pub paths: Vec<PathNode>,
}

/// A conditional attribute wrapping an inner attribute under a predicate. Recursive
/// through `inner`, so the rkyv bounds are stated explicitly with `omit_bounds` on
/// the recursive field.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
#[rkyv(
    serialize_bounds(__S: rkyv::ser::Writer + rkyv::ser::Allocator, __S::Error: rkyv::rancor::Source),
    deserialize_bounds(__D::Error: rkyv::rancor::Source),
    bytecheck(bounds(__C: rkyv::validation::ArchiveContext, __C::Error: rkyv::rancor::Source)),
)]
pub struct ConfigurationAttribute {
    pub predicate: ConfigurationPredicate,
    #[rkyv(omit_bounds)]
    pub inner: Box<Attribute>,
}

/// A configuration predicate. `Feature` resolves an interned feature name
/// (`nota-text`) — a stored identifier, never a projection-time string.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum ConfigurationPredicate {
    Feature(Identifier),
}

/// A namespaced helper attribute: `#[<path>(derive(<derived>))]`.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct HelperDerive {
    pub path: PathNode,
    pub derived: DeriveGroup,
}
