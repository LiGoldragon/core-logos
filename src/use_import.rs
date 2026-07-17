//! A `use` import declaration.

use crate::attribute::Attribute;
use crate::path::PathNode;
use crate::visibility::Visibility;
use name_table::Identifier;

/// A use import: `<attrs> <vis> use <base>::{<group>};`. Covers the cfg-gated NOTA
/// import at the head of every generated wire module
/// (`pub use nota::{NotaDecodeError, NotaEncode, NotaSource};`): the `base` is the
/// crate/module path (`nota`) and the `group` is the ordered vector of imported leaf
/// identifiers. Stringless: the base is a segment vector and each imported name is an
/// `Identifier`; the `::`, the `{}`, and the `, ` all materialize at projection.
///
/// The brace-group form is the one the wire goldens exercise. A bare import
/// (`use foo::Bar;`), a glob, and an aliased import (`as`) are by-design growth
/// points the closed shape does not yet carry.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Use {
    pub visibility: Visibility,
    pub attributes: Vec<Attribute>,
    pub base: PathNode,
    pub group: Vec<Identifier>,
}
