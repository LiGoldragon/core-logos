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

/// A function body: an ordered run of statements followed by a tail expression. The
/// `{ }` block delimiter and the statement `;` separators are projection concerns.
/// A body with no statements is the single-tail-expression form the class-A/kin
/// bodies use; the codec bodies (`encode_signal_frame` / `decode_signal_frame`) carry
/// `let` bindings ahead of the tail. Recursion through the statements and the tail
/// carries the rkyv self-referential bounds.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
#[rkyv(
    serialize_bounds(__S: rkyv::ser::Writer + rkyv::ser::Allocator, __S::Error: rkyv::rancor::Source),
    deserialize_bounds(__D::Error: rkyv::rancor::Source),
    bytecheck(bounds(__C: rkyv::validation::ArchiveContext, __C::Error: rkyv::rancor::Source)),
)]
pub struct Block {
    #[rkyv(omit_bounds)]
    pub statements: Vec<Statement>,
    #[rkyv(omit_bounds)]
    pub tail_expression: Expression,
}

/// One statement of a block, dispatched by kind. A `let` binding introduces a local;
/// an expression statement evaluates an expression for its effect
/// (`frame.extend_from_slice(&archive);`). Recursion carries the rkyv
/// self-referential bounds.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
#[rkyv(
    serialize_bounds(__S: rkyv::ser::Writer + rkyv::ser::Allocator, __S::Error: rkyv::rancor::Source),
    deserialize_bounds(__D::Error: rkyv::rancor::Source),
    bytecheck(bounds(__C: rkyv::validation::ArchiveContext, __C::Error: rkyv::rancor::Source)),
)]
pub enum Statement {
    /// A `let <binding> <name> = <value>;` local binding.
    Let(LetStatement),
    /// An expression evaluated for effect, terminated by `;`.
    Expression(#[rkyv(omit_bounds)] Expression),
}

/// A `let` binding: `let archive = …;`, `let mut frame = …;`. The mutability is a
/// closed [`LetBinding`] kind rather than a boolean flag; the bound name is a stored
/// identifier and the value is one expression. Recursion through `value` carries the
/// rkyv self-referential bounds.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
#[rkyv(
    serialize_bounds(__S: rkyv::ser::Writer + rkyv::ser::Allocator, __S::Error: rkyv::rancor::Source),
    deserialize_bounds(__D::Error: rkyv::rancor::Source),
    bytecheck(bounds(__C: rkyv::validation::ArchiveContext, __C::Error: rkyv::rancor::Source)),
)]
pub struct LetStatement {
    pub binding: LetBinding,
    pub name: Identifier,
    #[rkyv(omit_bounds)]
    pub value: Expression,
}

/// The mutability of a `let` binding, named rather than carried as a boolean flag:
/// `let name` (immutable) versus `let mut name` (mutable).
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum LetBinding {
    /// `let <name>` — an immutable binding.
    Immutable,
    /// `let mut <name>` — a mutable binding.
    Mutable,
}
