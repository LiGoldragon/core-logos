//! A function declaration and its single-tail-expression body.

use crate::attribute::Attribute;
use crate::expression::Expression;
use crate::generics::Generics;
use crate::type_reference::TypeReference;
use crate::visibility::Visibility;
use name_table::Identifier;

/// A function: `<attrs> <vis> fn <name><generics>(<receiver>, <parameters>) -> <return> <body>`.
/// A method inside an impl block and a free function are the same node; the impl
/// block owns the `Self` context. The witnessed Tier-1 body is a single tail
/// expression (no statements).
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Function {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub name: Identifier,
    pub generics: Generics,
    pub receiver: Option<Receiver>,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<TypeReference>,
    pub body: Block,
}

/// The receiver of a method, dispatched by kind rather than a nullable-flag pair.
/// `self` by value and `&self` shared borrow are the witnessed Tier-1 forms; a
/// `&mut self` exclusive borrow and a `mut self` binding are out of vocabulary (no
/// witnessed Tier-1 body needs them), so the reader rejects them loudly.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum Receiver {
    /// `self` — the receiver taken by value.
    Value,
    /// `&self` — the receiver taken by shared reference.
    Reference,
}

/// One typed parameter: a stored name and its type. The name is always present as a
/// stored `Identifier`; the type may be a signature type (`impl Into<String>`,
/// `&String`) as well as a plain path or application.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Parameter {
    pub name: Identifier,
    pub type_reference: TypeReference,
}

/// A function body: a single tail expression. The `{ }` block delimiter is a
/// projection concern. Statements (`let`, early `return`) are out of the Tier-1
/// vocabulary — the witnessed Tier-1 bodies are exactly one tail expression — so the
/// reader rejects a multi-statement body loudly.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Block {
    pub tail_expression: Expression,
}
