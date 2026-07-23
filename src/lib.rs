//! # core-logos
//!
//! The stringless encoded-form algebra of Logos — the Rust-equivalent data language,
//! 1-to-1 with Rust. This crate is **text-free**: it depends on no
//! `syn`, `prettyplease`, `quote`, or proc-macro machinery. Every identifier is an
//! [`name_table::Identifier`] into a NameTable; paths are segment vectors of
//! identifiers. The `::`, the `<>`, the `pub` keyword, and snake_case field names
//! are all projection concerns that materialize far from this crate, in the later
//! `TextualRust` sibling codec.
//!
//! The centerpiece is [`EncodedItem`], a closed enum over a shared stringless leaf
//! vocabulary. Content identity ([`EncodedItem::content_identity`]) is computed over a
//! value's portable-archive bytes under [`EncodedLogosDomain`], with the NameTable
//! excluded — so a rename is hash-stable and a structural edit moves the identity.
//! A Logos NameTable owns its Logos namespace and composes completed schema slices
//! without copying, flattening, or renumbering their identifiers.

pub mod alias;
pub mod attribute;
pub mod const_item;
pub mod domain;
pub mod enumeration;
pub mod error;
pub mod expression;
pub mod field;
pub mod function;
pub mod generics;
pub mod impl_block;
pub mod item;
pub mod module;
pub mod newtype;
pub mod path;
pub mod pattern;
pub mod structure;
pub mod type_reference;
pub mod use_import;
pub mod visibility;

pub use alias::Alias;
pub use attribute::{
    Attribute, ConfigurationAttribute, ConfigurationPredicate, DeriveGroup, HelperDerive,
};
pub use const_item::Const;
pub use domain::EncodedLogosDomain;
pub use enumeration::{Enumeration, Variant, VariantPayload};
pub use error::Error;
pub use expression::{
    ArrayExpression, Call, Callee, ClosureExpression, Expression, FieldInitializer,
    IndexExpression, IntegerLiteral, IntegerRepresentation, Match, MatchArm, MethodCall,
    QualifiedPath, RangeExpression, ReferenceExpression, StructLiteral, TryExpression,
    TupleExpression, TupleFieldAccess,
};
pub use field::Field;
pub use function::{Block, Function, LetBinding, LetStatement, Parameter, Receiver, Statement};
pub use generics::{GenericParameter, Generics, LifetimeParameter, TypeParameter};
pub use impl_block::{AssociatedType, ImplBlock, ImplItem};
pub use item::EncodedItem;
pub use module::Module;
pub use newtype::Newtype;
pub use path::PathNode;
pub use pattern::{Pattern, PatternElement, TupleVariantPattern};
pub use structure::Struct;
pub use type_reference::{
    ImplTraitType, ReferenceMutability, ReferenceType, SliceType, TupleType, TypeApplication,
    TypeReference,
};
pub use use_import::Use;
pub use visibility::Visibility;
