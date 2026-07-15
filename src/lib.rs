//! # core-logos
//!
//! The stringless Core algebra of Logos — the Rust-equivalent data language,
//! 1-to-1 with Rust at the Core. This crate is **text-free**: it depends on no
//! `syn`, `prettyplease`, `quote`, or proc-macro machinery. Every identifier is an
//! [`name_table::Identifier`] into a NameTable; paths are segment vectors of
//! identifiers. The `::`, the `<>`, the `pub` keyword, and snake_case field names
//! are all projection concerns that materialize far from this crate, in the later
//! `TextualRust` sibling codec.
//!
//! The centerpiece is [`CoreItem`], a closed enum over a shared stringless leaf
//! vocabulary. Content identity ([`CoreItem::content_identity`]) is computed over a
//! value's portable-archive bytes under [`CoreLogosDomain`], with the NameTable
//! excluded — so a rename is hash-stable and a structural edit moves the identity.
//! The NameTable is one continuous identifier space extending the schema NameTable
//! (via [`name_table::NameTable::extend_from`]).

pub mod alias;
pub mod attribute;
pub mod domain;
pub mod enumeration;
pub mod error;
pub mod field;
pub mod generics;
pub mod item;
pub mod newtype;
pub mod path;
pub mod structure;
pub mod type_reference;
pub mod visibility;

pub use alias::Alias;
pub use attribute::{
    Attribute, ConfigurationAttribute, ConfigurationPredicate, DeriveGroup, HelperDerive,
};
pub use domain::CoreLogosDomain;
pub use enumeration::{Enumeration, Variant, VariantPayload};
pub use error::Error;
pub use field::Field;
pub use generics::{GenericParameter, Generics, LifetimeParameter, TypeParameter};
pub use item::CoreItem;
pub use newtype::Newtype;
pub use path::PathNode;
pub use structure::Struct;
pub use type_reference::{TypeApplication, TypeReference};
pub use visibility::Visibility;
