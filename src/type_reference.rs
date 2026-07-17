//! Type references and generic type application — dispatched by kind, never by
//! a head string.

use crate::path::PathNode;
use name_table::Identifier;

/// A reference to a type. Dispatch is on the node's kind, never on a string match
/// of the head — the "generics by kind" ruling. `Path` and `Application` are the
/// owned wire-data shapes (a bare path or a generic application). `Reference`,
/// `ImplTrait`, and `Slice` are the function-signature and const-type shapes the
/// vocabulary needs (`&String`, `&mut std::fmt::Formatter<'_>`, `&'static str`,
/// `impl Into<String>`, `[&'static str]`); they are legitimate in a function's
/// parameter/return positions and const-type position but never in wire-data field
/// position, a distinction the TextualRust reader enforces by position. `Lifetime`
/// is legitimate only inside a generic-argument list (`Formatter<'_>`), never as a
/// standalone type.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum TypeReference {
    Path(PathNode),
    Application(TypeApplication),
    Reference(ReferenceType),
    ImplTrait(ImplTraitType),
    /// A slice type `[<element>]`: `[&'static str]`. The `[…]` delimiter is a
    /// projection concern; the element is a stored type.
    Slice(SliceType),
    /// A lifetime in a generic-argument list: the `'_` of `Formatter<'_>` or an
    /// explicit `'static`. Stored as the interned lifetime name (`_`, `static`),
    /// projected as `'<name>`. Legitimate only as a generic argument.
    Lifetime(Identifier),
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

/// A reference type: `&<referent>`, `&<lifetime> <referent>`, or
/// `&mut <referent>` (`&String`, `&'static str`, `&mut std::fmt::Formatter<'_>`).
/// The lifetime is a stored identifier when present (`static`), absent for an
/// elided borrow. Mutability is a stored [`ReferenceMutability`] kind rather than a
/// projection-time flag: the witnessed `Display::fmt` signature borrows the
/// formatter mutably. Recursive through `referent`, so the rkyv bounds are stated
/// with `omit_bounds` on the recursive field, the standard self-referential-archive
/// fix.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
#[rkyv(
    serialize_bounds(__S: rkyv::ser::Writer + rkyv::ser::Allocator, __S::Error: rkyv::rancor::Source),
    deserialize_bounds(__D::Error: rkyv::rancor::Source),
    bytecheck(bounds(__C: rkyv::validation::ArchiveContext, __C::Error: rkyv::rancor::Source)),
)]
pub struct ReferenceType {
    pub lifetime: Option<Identifier>,
    pub mutability: ReferenceMutability,
    #[rkyv(omit_bounds)]
    pub referent: Box<TypeReference>,
}

/// Whether a reference borrows shared (`&`) or mutable (`&mut`). A closed kind, not
/// a boolean flag, so the variant set lives in the type.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum ReferenceMutability {
    /// A shared borrow `&`.
    Shared,
    /// An exclusive borrow `&mut`.
    Mutable,
}

/// A slice type `[<element>]`: `[&'static str]`. Recursive through `element`, so the
/// rkyv bounds are stated with `omit_bounds` on the recursive field.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
#[rkyv(
    serialize_bounds(__S: rkyv::ser::Writer + rkyv::ser::Allocator, __S::Error: rkyv::rancor::Source),
    deserialize_bounds(__D::Error: rkyv::rancor::Source),
    bytecheck(bounds(__C: rkyv::validation::ArchiveContext, __C::Error: rkyv::rancor::Source)),
)]
pub struct SliceType {
    #[rkyv(omit_bounds)]
    pub element: Box<TypeReference>,
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
