//! Generic parameters, defined by kind.

use crate::path::PathNode;
use name_table::Identifier;

/// The generic parameter list on a declaration. Empty for the common
/// non-generic case; `Generics::none()` names that case.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Generics {
    pub parameters: Vec<GenericParameter>,
}

impl Generics {
    /// The empty parameter list — the dominant declaration case.
    pub fn none() -> Self {
        Self {
            parameters: Vec::new(),
        }
    }
}

/// A generic parameter, dispatched on kind. Lowering dispatches on the variant,
/// never on a string name. Const parameters are a by-design growth point — the
/// surveyed goldens do not exercise them, so the closed enum does not yet carry
/// that variant.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum GenericParameter {
    Type(TypeParameter),
    Lifetime(LifetimeParameter),
}

/// A type parameter with optional trait bounds. Bounds are paths
/// (`Engine: NexusEngine`); the witnessed wire declarations carry none.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct TypeParameter {
    pub name: Identifier,
    pub bounds: Vec<PathNode>,
}

/// A lifetime parameter (`'engine`).
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct LifetimeParameter {
    pub name: Identifier,
}
