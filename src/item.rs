//! The closed EncodedItem algebra and its content identity.

use crate::alias::Alias;
use crate::attribute::Attribute;
use crate::const_item::Const;
use crate::domain::EncodedLogosDomain;
use crate::enumeration::Enumeration;
use crate::error::Error;
use crate::function::Function;
use crate::impl_block::ImplBlock;
use crate::module::Module;
use crate::newtype::Newtype;
use crate::structure::Struct;
use crate::use_import::Use;
use content_identity::ContentHash;
use name_table::Identifier;

/// The closed algebra of Logos items — 1-to-1 with the Rust wire-contract subset
/// the goldens exercise. Stringless: every identifier is an `Identifier` into the
/// NameTable, every path a segment vector. The enum is closed and its methods
/// match every variant with no wildcard arm, so a new item kind is a compile
/// error until its handling is written — the algebra grows by design.
///
/// The trait definition and free-method item kinds of the accepted ontology remain
/// out of scope for this text-free Core (see the crate ARCHITECTURE). `ImplBlock`
/// and `Function` are modeled: their bodies are the closed Tier-1 expression
/// vocabulary the wire goldens exercise, carried as data. `Use` models the
/// `use`-import shape — the cfg-gated NOTA import at the head of every generated
/// module — as stored data.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum EncodedItem {
    Newtype(Newtype),
    Struct(Struct),
    Enumeration(Enumeration),
    Alias(Alias),
    ImplBlock(ImplBlock),
    Function(Function),
    Use(Use),
    Const(Const),
    Module(Module),
}

impl EncodedItem {
    /// The declared name of this item as a stored `Identifier`, when it has one.
    /// Exhaustive over the closed algebra — no wildcard arm. An impl block declares
    /// no name (it attaches functions to a self type), so it dissolves the "does
    /// this item have a name?" question into a normal `None` rather than a fabricated
    /// identifier.
    pub fn name(&self) -> Option<Identifier> {
        match self {
            EncodedItem::Newtype(newtype) => Some(newtype.name),
            EncodedItem::Struct(structure) => Some(structure.name),
            EncodedItem::Enumeration(enumeration) => Some(enumeration.name),
            EncodedItem::Alias(alias) => Some(alias.name),
            EncodedItem::Function(function) => Some(function.name),
            EncodedItem::Const(const_item) => Some(const_item.name),
            EncodedItem::Module(module) => Some(module.name),
            EncodedItem::ImplBlock(_) => None,
            // A use import declares no name — it brings names in, it does not
            // declare one — so it dissolves into the same `None` as an impl block.
            EncodedItem::Use(_) => None,
        }
    }

    /// This item with its visibility replaced. Exhaustive over the closed algebra —
    /// no wildcard arm. The stamping verb lives on the item that owns the
    /// visibility field: a caller lowering authoritative API intent (a schema
    /// declaration's Public/Private) stamps the produced item without reaching into
    /// each variant. Attributes, name, and structure are untouched, so this never
    /// moves anything but the visibility a projection reads. An impl block has no
    /// visibility (Rust has no `pub impl`), so it is returned unchanged.
    pub fn with_visibility(mut self, visibility: crate::visibility::Visibility) -> Self {
        match &mut self {
            EncodedItem::Newtype(newtype) => newtype.visibility = visibility,
            EncodedItem::Struct(structure) => structure.visibility = visibility,
            EncodedItem::Enumeration(enumeration) => enumeration.visibility = visibility,
            EncodedItem::Alias(alias) => alias.visibility = visibility,
            EncodedItem::Function(function) => function.visibility = visibility,
            EncodedItem::Const(const_item) => const_item.visibility = visibility,
            EncodedItem::Module(module) => module.visibility = visibility,
            EncodedItem::ImplBlock(_) => {}
            // A use import carries its own visibility (`pub use`), so it is stamped
            // like any other visible item.
            EncodedItem::Use(use_import) => use_import.visibility = visibility,
        }
        self
    }

    /// The ordered attribute preamble of this item. Exhaustive over the closed
    /// algebra — no wildcard arm.
    pub fn attributes(&self) -> &[Attribute] {
        match self {
            EncodedItem::Newtype(newtype) => &newtype.attributes,
            EncodedItem::Struct(structure) => &structure.attributes,
            EncodedItem::Enumeration(enumeration) => &enumeration.attributes,
            EncodedItem::Alias(alias) => &alias.attributes,
            EncodedItem::ImplBlock(impl_block) => &impl_block.attributes,
            EncodedItem::Function(function) => &function.attributes,
            EncodedItem::Use(use_import) => &use_import.attributes,
            EncodedItem::Const(const_item) => &const_item.attributes,
            EncodedItem::Module(module) => &module.attributes,
        }
    }

    /// The content identity of this item over its canonical portable-archive bytes,
    /// domain-separated and layout-versioned by `EncodedLogosDomain`. The NameTable is
    /// not part of the pre-image, so renaming is hash-stable and a structural edit
    /// moves the identity.
    pub fn content_identity(&self) -> Result<ContentHash<EncodedLogosDomain>, Error> {
        Ok(ContentHash::of_core(self)?)
    }
}
