//! Paths as segment vectors of interned identifiers.

use name_table::{Identifier, Name, NameResolver, NameTableError};

/// A path is a vector of `Identifier` segments — stringless. Text forms dot the
/// segments (`rkyv.Archive`); the `::` materializes only at Rust projection, far
/// from this crate. A `PathNode` in a derive, a tool attribute, or a type
/// position is the same node type everywhere.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct PathNode {
    pub segments: Vec<Identifier>,
}

impl PathNode {
    /// Resolve every segment to its interned name. This is a projection concern —
    /// it is the only place a `PathNode` touches text, at the last moment.
    pub fn resolve<Resolver: NameResolver + ?Sized>(
        &self,
        names: &Resolver,
    ) -> Result<Vec<Name>, NameTableError> {
        self.segments
            .iter()
            .map(|segment| names.resolve(*segment).cloned())
            .collect()
    }
}
