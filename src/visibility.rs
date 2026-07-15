//! Visibility as stored data on general nodes.

use crate::path::PathNode;

/// Visibility is stored data — a variant on the general item and field nodes,
/// never a minted specialized type (there is no `PublicStruct` vs
/// `PrivateStruct`). It is the item's and field's actual Rust visibility carried
/// verbatim, never computed at projection. `Private` is a value whose Rust
/// projection is the empty token stream, so the special case (no `pub`) dissolves
/// into the normal case (a node that projects nothing).
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum Visibility {
    Public,
    Crate,
    Module(PathNode),
    Private,
}
