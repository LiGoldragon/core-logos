//! An impl block: a set of associated items attached to a self type, optionally
//! through a trait.

use crate::attribute::Attribute;
use crate::const_item::Const;
use crate::function::Function;
use crate::generics::Generics;
use crate::type_reference::TypeReference;
use name_table::Identifier;

/// An impl block: `<attrs> impl<generics> <trait for>? <self_type> { <items> }`.
/// An inherent impl (`impl Topic { … }`) carries no trait; a trait impl
/// (`impl From<String> for Topic { … }`) carries the implemented trait as a type
/// reference (`From<String>`). An impl block declares no name and has no
/// visibility — both are properties of the items and the self type it attaches
/// to, not of the block. Its `items` are the ordered heterogeneous member set —
/// methods, associated types, and associated consts — in source order, so a
/// `type Err = …;` that precedes its method round-trips in place.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ImplBlock {
    pub attributes: Vec<Attribute>,
    pub generics: Generics,
    pub implemented_trait: Option<TypeReference>,
    pub self_type: TypeReference,
    pub items: Vec<ImplItem>,
}

/// One associated item of an impl block, dispatched by kind rather than split across
/// parallel vectors, so source order is preserved and a new member kind is a compile
/// error until handled. A method is the dominant member; an associated type
/// (`type Err = NotaDecodeError;`) and an associated const
/// (`const HEADS: &'static [&'static str] = …;`) are the trait-impl members the
/// class-B and class-C goldens carry.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum ImplItem {
    /// An associated method or function — the same [`Function`] node as a free
    /// function; the impl block owns the `Self` context.
    Method(Function),
    /// An associated type binding: `type Err = NotaDecodeError;`.
    AssociatedType(AssociatedType),
    /// An associated const: `const HEADS: &'static [&'static str] = &[…];`. The same
    /// [`Const`] node as a top-level or module const.
    AssociatedConst(Const),
}

/// An associated type binding in an impl block: `type <name> = <value>;`. Bounds,
/// generics, and where clauses are out of the modeled vocabulary (the witnessed
/// binding is a plain equality), so the reader rejects them loudly.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AssociatedType {
    pub name: Identifier,
    pub value: TypeReference,
}
