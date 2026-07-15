//! Type references and generic type application — dispatched by kind, never by
//! a head string.

use crate::path::PathNode;

/// A reference to a type. Either a bare path or a generic application. Dispatch is
/// on the node's kind (`Path` vs `Application`), never on a string match of the
/// head — the "generics by kind" ruling.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum TypeReference {
    Path(PathNode),
    Application(TypeApplication),
}

/// A generic application: a head path applied to positional type arguments
/// (`Signal<Root>`, `Plane<SignalRoot, NexusRoot, SemaRoot>`). The `<>` is
/// re-sugaring materialized only at Rust projection; here it is a segment head
/// and an argument vector. Recursive through `arguments`, so the rkyv bounds are
/// stated explicitly with `omit_bounds` on the recursive field (the standard fix
/// for self-referential archives).
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
#[rkyv(
    serialize_bounds(__S: rkyv::ser::Writer + rkyv::ser::Allocator, __S::Error: rkyv::rancor::Source),
    deserialize_bounds(__D::Error: rkyv::rancor::Source),
    bytecheck(bounds(__C: rkyv::validation::ArchiveContext, __C::Error: rkyv::rancor::Source)),
)]
pub struct TypeApplication {
    pub head: PathNode,
    #[rkyv(omit_bounds)]
    pub arguments: Vec<TypeReference>,
}
