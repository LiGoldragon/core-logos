//! An impl block: a set of functions attached to a self type, optionally through a
//! trait.

use crate::attribute::Attribute;
use crate::function::Function;
use crate::generics::Generics;
use crate::type_reference::TypeReference;

/// An impl block: `<attrs> impl<generics> <trait for>? <self_type> { <functions> }`.
/// An inherent impl (`impl Topic { … }`) carries no trait; a trait impl
/// (`impl From<String> for Topic { … }`) carries the implemented trait as a type
/// reference (`From<String>`). An impl block declares no name and has no
/// visibility — both are properties of the functions and the self type it attaches
/// to, not of the block.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ImplBlock {
    pub attributes: Vec<Attribute>,
    pub generics: Generics,
    pub implemented_trait: Option<TypeReference>,
    pub self_type: TypeReference,
    pub functions: Vec<Function>,
}
