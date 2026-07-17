//! Type references and generic type application — dispatched by kind, never by
//! a head string.

use crate::path::PathNode;
use name_table::Identifier;

/// A reference to a type. Dispatch is on the node's kind, never on a string match
/// of the head — the "generics by kind" ruling. `Path` and `Application` are the
/// owned wire-data shapes (a bare path or a generic application). `Reference` and
/// `ImplTrait` are the function-signature shapes the Tier-1 body vocabulary needs
/// (`&String`, `&'static str`, `impl Into<String>`); they are legitimate in a
/// function's parameter and return positions but never in wire-data field position,
/// a distinction the TextualRust reader enforces by position.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum TypeReference {
    Path(PathNode),
    Application(TypeApplication),
    Reference(ReferenceType),
    ImplTrait(ImplTraitType),
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

/// A shared-reference type: `&<referent>` or `&<lifetime> <referent>`
/// (`&String`, `&'static str`). The lifetime is a stored identifier when present
/// (`static`), absent for an elided borrow. Only the shared form is modeled: the
/// witnessed Tier-1 signatures borrow immutably, so `&mut` is out of vocabulary and
/// the reader rejects it loudly. Recursive through `referent`, so the rkyv bounds
/// are stated with `omit_bounds` on the recursive field, the standard
/// self-referential-archive fix.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
#[rkyv(
    serialize_bounds(__S: rkyv::ser::Writer + rkyv::ser::Allocator, __S::Error: rkyv::rancor::Source),
    deserialize_bounds(__D::Error: rkyv::rancor::Source),
    bytecheck(bounds(__C: rkyv::validation::ArchiveContext, __C::Error: rkyv::rancor::Source)),
)]
pub struct ReferenceType {
    pub lifetime: Option<Identifier>,
    #[rkyv(omit_bounds)]
    pub referent: Box<TypeReference>,
}

/// An `impl Trait` type in a function signature: `impl Into<String>`. The bounds are
/// trait references (a path or a generic application over paths) in stored order;
/// the witnessed Tier-1 case is a single `Into<String>` bound. Recursive through
/// `bounds`, so the rkyv bounds are stated with `omit_bounds` on the recursive
/// field.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
#[rkyv(
    serialize_bounds(__S: rkyv::ser::Writer + rkyv::ser::Allocator, __S::Error: rkyv::rancor::Source),
    deserialize_bounds(__D::Error: rkyv::rancor::Source),
    bytecheck(bounds(__C: rkyv::validation::ArchiveContext, __C::Error: rkyv::rancor::Source)),
)]
pub struct ImplTraitType {
    #[rkyv(omit_bounds)]
    pub bounds: Vec<TypeReference>,
}
