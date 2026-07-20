//! The closed Tier-1 expression vocabulary of method bodies.
//!
//! A Tier-1 function body is a single tail expression, and its expression algebra
//! is closed and small — exactly the class-A-and-kin body shapes the wire goldens
//! exercise, and nothing extensible by string. Dispatch is on the node's kind, never
//! on a head string, and every match over the algebra is exhaustive with no wildcard
//! arm, so a new expression kind is a compile error until its handling is written.
//!
//! The witnessed shapes and their nodes:
//!
//! - `self` → [`Expression::Receiver`]
//! - `payload`, `InputRoute::Record` → [`Expression::Path`]
//! - `&self.0` → [`Expression::Reference`] over a [`Expression::Field`]
//! - `self.0` → [`Expression::Field`]
//! - `Self(payload.into())`, `Self::new(payload)`, `Self::Record(payload)`,
//!   `RecordIdentifier::new(payload)`, `<Self as Trait>::method(self)` →
//!   [`Expression::Call`] (the callee is a plain or qualified path)
//! - `payload.into()`, `self.0.name()`, `NotaSource::new(source).parse::<Self>()` →
//!   [`Expression::MethodCall`] (the trailing `::<Self>` is a stored turbofish)
//! - `"SignalInputRecord"` → [`Expression::StringLiteral`]
//! - `0x0001000000000000`, `8` → [`Expression::IntegerLiteral`]
//! - `["Record", "Observe"]` → [`Expression::Array`]
//! - `match self { … }` → [`Expression::Match`]
//! - `FrameBody::Request { exchange, request }` → [`Expression::StructLiteral`]
//!
//! Every recursive slot is behind a struct that carries the rkyv self-referential
//! bounds, mirroring the leaf `TypeApplication`/`ConfigurationAttribute` pattern, so
//! the `Expression` enum itself needs no bounds attribute.

use crate::path::PathNode;
use crate::pattern::{Pattern, PatternElement};
use crate::type_reference::TypeReference;
use name_table::Identifier;

/// The closed algebra of Tier-1 body expressions. Stringless except for string
/// literals, whose content is genuine literal data (not a name): a name is interned
/// and excluded from content identity, whereas a literal's content is semantic and
/// is hashed as part of the value.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum Expression {
    /// The method receiver `self` (the lowercase value binding).
    Receiver,
    /// A path used as a value: `payload`, `InputRoute::Record`.
    Path(PathNode),
    /// A string literal: `"SignalInputRecord"`. The content is literal data hashed
    /// into content identity, never an interned name.
    StringLiteral(String),
    /// A shared reference `&<inner>`: `&self.0`.
    Reference(ReferenceExpression),
    /// A tuple-index field access `<base>.<index>`: `self.0`.
    Field(TupleFieldAccess),
    /// A call of a plain or qualified path callee with positional arguments:
    /// `Self(x)`, `Self::new(x)`, `Self::Record(x)`, `Wrapper::new(x)`,
    /// `<Self as Trait>::method(x)`.
    Call(Call),
    /// A method call `<receiver>.<method>(<arguments>)`: `payload.into()`.
    MethodCall(MethodCall),
    /// A `match <scrutinee> { <arms> }` expression.
    Match(Match),
    /// An integer literal: `0x0001000000000000`, `8`. Stringless — the value is a
    /// number and the surface form (decimal or zero-padded hexadecimal) is a closed
    /// [`IntegerRepresentation`] descriptor, never stored text. The value is literal
    /// data hashed into content identity, exactly like a string literal's content.
    IntegerLiteral(IntegerLiteral),
    /// An array literal: `["Record", "Observe"]`. The `[…]` delimiter and the `, `
    /// separators are projection concerns; the elements are stored expressions.
    Array(ArrayExpression),
    /// The `?` try operator applied to a fallible expression:
    /// `rkyv::to_bytes::<E>(self).map_err(…)?`. The postfix `?` is a projection
    /// concern; the stored data is the inner fallible expression.
    Try(TryExpression),
    /// A closure `|<parameters>| <body>`: `|_| SignalFrameError::ArchiveEncode`. The
    /// `|…|` delimiters are a projection concern; the parameters are the reused
    /// tuple-element pattern vocabulary (a wildcard or a binding) and the body is one
    /// stored expression.
    Closure(ClosureExpression),
    /// A tuple expression `(<elements>)`: `(route, value)`. The `(…)` delimiter and
    /// separators are projection concerns; the elements are stored expressions. The
    /// zero-element form is the unit value `()`.
    Tuple(TupleExpression),
    /// An index expression `<base>[<index>]`: `frame[SIGNAL_SHORT_HEADER_BYTE_COUNT..]`.
    /// The `[…]` delimiter is a projection concern; the base and the index are stored
    /// expressions (the index is commonly a [`Expression::Range`]).
    Index(IndexExpression),
    /// A half-open range `<start>..<end>` with either bound optional:
    /// `..SIGNAL_SHORT_HEADER_BYTE_COUNT` (range-to) and
    /// `SIGNAL_SHORT_HEADER_BYTE_COUNT..` (range-from). The `..` is a projection
    /// concern; the present bounds are stored expressions.
    Range(RangeExpression),
    /// A struct or struct-variant literal `<path> { <fields> }`:
    /// `FrameBody::Request { exchange, request }`. The `{ … }` delimiter and the
    /// field separators are projection concerns; the path and the ordered field
    /// initializers are stored expressions. Field shorthand is not a special case —
    /// it is a [`FieldInitializer`] whose value is `None`.
    StructLiteral(StructLiteral),
}

/// A shared-reference expression `&<referent>`. Recursion through `referent` carries
/// the rkyv self-referential bounds.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
#[rkyv(
    serialize_bounds(__S: rkyv::ser::Writer + rkyv::ser::Allocator, __S::Error: rkyv::rancor::Source),
    deserialize_bounds(__D::Error: rkyv::rancor::Source),
    bytecheck(bounds(__C: rkyv::validation::ArchiveContext, __C::Error: rkyv::rancor::Source)),
)]
pub struct ReferenceExpression {
    #[rkyv(omit_bounds)]
    pub referent: Box<Expression>,
}

/// A tuple-index field access `<base>.<index>`: `self.0`, `self.0.name()`'s
/// receiver. Only tuple-index access is modeled; named field access is out of the
/// Tier-1 vocabulary (the witnessed Tier-1 bodies index the sole tuple field).
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
#[rkyv(
    serialize_bounds(__S: rkyv::ser::Writer + rkyv::ser::Allocator, __S::Error: rkyv::rancor::Source),
    deserialize_bounds(__D::Error: rkyv::rancor::Source),
    bytecheck(bounds(__C: rkyv::validation::ArchiveContext, __C::Error: rkyv::rancor::Source)),
)]
pub struct TupleFieldAccess {
    #[rkyv(omit_bounds)]
    pub base: Box<Expression>,
    pub index: u32,
}

/// A call of a path callee with positional arguments and an optional turbofish. The
/// callee is a plain path (`Self`, `Self::new`, `Self::Record`,
/// `RecordIdentifier::new`) or a trait-qualified path (`<Self as Trait>::method`);
/// the `type_arguments` are the optional turbofish (`::<rkyv::rancor::Error>` in
/// `rkyv::to_bytes::<rkyv::rancor::Error>(self)`), an empty vector for the
/// un-turbofished call; the `()` re-sugaring is a projection concern.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
#[rkyv(
    serialize_bounds(__S: rkyv::ser::Writer + rkyv::ser::Allocator, __S::Error: rkyv::rancor::Source),
    deserialize_bounds(__D::Error: rkyv::rancor::Source),
    bytecheck(bounds(__C: rkyv::validation::ArchiveContext, __C::Error: rkyv::rancor::Source)),
)]
pub struct Call {
    pub callee: Callee,
    pub type_arguments: Vec<TypeReference>,
    #[rkyv(omit_bounds)]
    pub arguments: Vec<Expression>,
}

/// A call callee, dispatched by kind: a plain path or a trait-qualified path.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum Callee {
    Path(PathNode),
    Qualified(QualifiedPath),
}

/// A trait-qualified path used as a call callee: `<Self as Trait>::method`. The
/// `self_type` is the qualifier's type (`Self`), the `trait_path` is the `as Trait`
/// clause (`NotaEncode`), and `member` is the segment vector after `>::` (`to_nota`).
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct QualifiedPath {
    pub self_type: TypeReference,
    pub trait_path: PathNode,
    pub member: Vec<Identifier>,
}

/// A method call `<receiver>.<method><turbofish>(<arguments>)`: `payload.into()`,
/// `self.0.name()`, `source.parse::<Self>()`. The `type_arguments` are the optional
/// turbofish (`::<Self>`); an empty vector is the un-turbofished call. Recursion
/// through `receiver` and `arguments` carries the rkyv self-referential bounds.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
#[rkyv(
    serialize_bounds(__S: rkyv::ser::Writer + rkyv::ser::Allocator, __S::Error: rkyv::rancor::Source),
    deserialize_bounds(__D::Error: rkyv::rancor::Source),
    bytecheck(bounds(__C: rkyv::validation::ArchiveContext, __C::Error: rkyv::rancor::Source)),
)]
pub struct MethodCall {
    #[rkyv(omit_bounds)]
    pub receiver: Box<Expression>,
    pub method: Identifier,
    pub type_arguments: Vec<TypeReference>,
    #[rkyv(omit_bounds)]
    pub arguments: Vec<Expression>,
}

/// A `match <scrutinee> { <arms> }`. The witnessed Tier-1 scrutinee is `self` or a
/// bound local; the arms are exhaustive with no wildcard arm. Recursion carries the
/// rkyv self-referential bounds.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
#[rkyv(
    serialize_bounds(__S: rkyv::ser::Writer + rkyv::ser::Allocator, __S::Error: rkyv::rancor::Source),
    deserialize_bounds(__D::Error: rkyv::rancor::Source),
    bytecheck(bounds(__C: rkyv::validation::ArchiveContext, __C::Error: rkyv::rancor::Source)),
)]
pub struct Match {
    #[rkyv(omit_bounds)]
    pub scrutinee: Box<Expression>,
    #[rkyv(omit_bounds)]
    pub arms: Vec<MatchArm>,
}

/// One match arm: a pattern mapped to a body expression (a unit path, a string
/// literal, or a nested match in the witnessed Tier-1 cases). Recursion through
/// `body` carries the rkyv self-referential bounds.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
#[rkyv(
    serialize_bounds(__S: rkyv::ser::Writer + rkyv::ser::Allocator, __S::Error: rkyv::rancor::Source),
    deserialize_bounds(__D::Error: rkyv::rancor::Source),
    bytecheck(bounds(__C: rkyv::validation::ArchiveContext, __C::Error: rkyv::rancor::Source)),
)]
pub struct MatchArm {
    pub pattern: Pattern,
    #[rkyv(omit_bounds)]
    pub body: Expression,
}

/// An integer literal as stringless data: a numeric value and a closed surface-form
/// descriptor. The value carries the semantics (hashed into content identity like a
/// string literal's content); the representation carries the exact Rust text the
/// value projects to, so `0x0001000000000000` and `281474976710656` are the same
/// value under different [`IntegerRepresentation`]s and each round-trips byte-exact
/// without the encoded form ever holding raw token text.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct IntegerLiteral {
    pub value: u128,
    pub representation: IntegerRepresentation,
}

/// The closed surface form of an integer literal. `Decimal` is the plain form (`8`);
/// `Hexadecimal` is the `0x`-prefixed form whose `minimum_digits` records the
/// zero-padding width (`0x0001000000000000` is `minimum_digits: 16`). This is a
/// closed formatting descriptor, not stored text — the stringless boundary holds.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum IntegerRepresentation {
    /// The plain decimal form: `8`, `281474976710656`.
    Decimal,
    /// The lowercase `0x`-prefixed hexadecimal form, zero-padded to `minimum_digits`:
    /// `0x0001000000000000`.
    Hexadecimal { minimum_digits: u16 },
}

/// An array literal `[<elements>]`: `["Record", "Observe"]`. Recursion through
/// `elements` carries the rkyv self-referential bounds, mirroring the other
/// expression slots.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
#[rkyv(
    serialize_bounds(__S: rkyv::ser::Writer + rkyv::ser::Allocator, __S::Error: rkyv::rancor::Source),
    deserialize_bounds(__D::Error: rkyv::rancor::Source),
    bytecheck(bounds(__C: rkyv::validation::ArchiveContext, __C::Error: rkyv::rancor::Source)),
)]
pub struct ArrayExpression {
    #[rkyv(omit_bounds)]
    pub elements: Vec<Expression>,
}

/// The `?` try operator over a fallible inner expression: the postfix `?` in
/// `rkyv::to_bytes::<E>(self).map_err(…)?`. Recursion through `inner` carries the
/// rkyv self-referential bounds.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
#[rkyv(
    serialize_bounds(__S: rkyv::ser::Writer + rkyv::ser::Allocator, __S::Error: rkyv::rancor::Source),
    deserialize_bounds(__D::Error: rkyv::rancor::Source),
    bytecheck(bounds(__C: rkyv::validation::ArchiveContext, __C::Error: rkyv::rancor::Source)),
)]
pub struct TryExpression {
    #[rkyv(omit_bounds)]
    pub inner: Box<Expression>,
}

/// A closure `|<parameters>| <body>`: `|_| SignalFrameError::ArchiveEncode`. The
/// parameters reuse the tuple-element pattern vocabulary (a wildcard `_` or an
/// identifier binding); the body is one expression. Recursion through `body` carries
/// the rkyv self-referential bounds.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
#[rkyv(
    serialize_bounds(__S: rkyv::ser::Writer + rkyv::ser::Allocator, __S::Error: rkyv::rancor::Source),
    deserialize_bounds(__D::Error: rkyv::rancor::Source),
    bytecheck(bounds(__C: rkyv::validation::ArchiveContext, __C::Error: rkyv::rancor::Source)),
)]
pub struct ClosureExpression {
    pub parameters: Vec<PatternElement>,
    #[rkyv(omit_bounds)]
    pub body: Box<Expression>,
}

/// A tuple expression `(<elements>)`: `(route, value)`. A zero-element vector is the
/// unit value `()`. Recursion through `elements` carries the rkyv self-referential
/// bounds.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
#[rkyv(
    serialize_bounds(__S: rkyv::ser::Writer + rkyv::ser::Allocator, __S::Error: rkyv::rancor::Source),
    deserialize_bounds(__D::Error: rkyv::rancor::Source),
    bytecheck(bounds(__C: rkyv::validation::ArchiveContext, __C::Error: rkyv::rancor::Source)),
)]
pub struct TupleExpression {
    #[rkyv(omit_bounds)]
    pub elements: Vec<Expression>,
}

/// An index expression `<base>[<index>]`: `frame[SIGNAL_SHORT_HEADER_BYTE_COUNT..]`.
/// The index is commonly a [`Expression::Range`]. Recursion through `base` and
/// `index` carries the rkyv self-referential bounds.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
#[rkyv(
    serialize_bounds(__S: rkyv::ser::Writer + rkyv::ser::Allocator, __S::Error: rkyv::rancor::Source),
    deserialize_bounds(__D::Error: rkyv::rancor::Source),
    bytecheck(bounds(__C: rkyv::validation::ArchiveContext, __C::Error: rkyv::rancor::Source)),
)]
pub struct IndexExpression {
    #[rkyv(omit_bounds)]
    pub base: Box<Expression>,
    #[rkyv(omit_bounds)]
    pub index: Box<Expression>,
}

/// A half-open range `<start>..<end>` with either bound optional: `..end` (range-to),
/// `start..` (range-from), `start..end` (the closed form). The exclusive `..` operator
/// is a projection concern; the present bounds are stored expressions. Recursion
/// through the bounds carries the rkyv self-referential bounds.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
#[rkyv(
    serialize_bounds(__S: rkyv::ser::Writer + rkyv::ser::Allocator, __S::Error: rkyv::rancor::Source),
    deserialize_bounds(__D::Error: rkyv::rancor::Source),
    bytecheck(bounds(__C: rkyv::validation::ArchiveContext, __C::Error: rkyv::rancor::Source)),
)]
pub struct RangeExpression {
    #[rkyv(omit_bounds)]
    pub start: Option<Box<Expression>>,
    #[rkyv(omit_bounds)]
    pub end: Option<Box<Expression>>,
}

/// A struct or struct-variant literal `<path> { <fields> }`:
/// `FrameBody::Request { exchange, request }`. The `path` is the struct or
/// enum-variant path; the `fields` are the ordered field initializers. Recursion
/// through `fields` (each carrying an optional boxed value) carries the rkyv
/// self-referential bounds.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
#[rkyv(
    serialize_bounds(__S: rkyv::ser::Writer + rkyv::ser::Allocator, __S::Error: rkyv::rancor::Source),
    deserialize_bounds(__D::Error: rkyv::rancor::Source),
    bytecheck(bounds(__C: rkyv::validation::ArchiveContext, __C::Error: rkyv::rancor::Source)),
)]
pub struct StructLiteral {
    pub path: PathNode,
    #[rkyv(omit_bounds)]
    pub fields: Vec<FieldInitializer>,
}

/// One field initializer of a [`StructLiteral`]. A `None` value is the field
/// shorthand form `exchange` — the field name and the in-scope binding coincide; a
/// `Some` value is the explicit `name: <expression>` form. Modeling shorthand as the
/// absent value dissolves the "shorthand or explicit" question into the normal
/// optional-value read rather than a separate flag. Recursion through the boxed value
/// carries the rkyv self-referential bounds.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
#[rkyv(
    serialize_bounds(__S: rkyv::ser::Writer + rkyv::ser::Allocator, __S::Error: rkyv::rancor::Source),
    deserialize_bounds(__D::Error: rkyv::rancor::Source),
    bytecheck(bounds(__C: rkyv::validation::ArchiveContext, __C::Error: rkyv::rancor::Source)),
)]
pub struct FieldInitializer {
    pub name: Identifier,
    #[rkyv(omit_bounds)]
    pub value: Option<Box<Expression>>,
}
