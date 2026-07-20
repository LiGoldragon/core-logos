//! The Logos NameTable and emission boundary.
//!
//! Typed schema-to-Logos transformation carries only encoded values and encoded
//! identifiers. This boundary composes completed identifier slices and performs the
//! ruled eager name derivations before later textual emission. It is deliberately
//! the only Logos-side home that materializes a derived spelling.

use name_table::{Identifier, Name, NameTable, NameTableError};

use crate::standard::standard_name_table;

/// The kind of eager name derivation requested by a Logos projection.
///
/// This is typed intent, not a string operation inside the schema-to-Logos
/// transformation. The boundary resolves the source identifier, derives its name,
/// and allocates the result in the Logos-owned slice.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NameDerivation {
    FieldName,
    ScreamingSnakeCase,
    PascalCase,
}

/// One component-owned, composed Logos NameTable at the emission boundary.
///
/// Its home slice is `Logos`; it borrows the completed `LogosStandard` and `Schema`
/// slices. The borrowed identifiers remain unchanged and are never copied or
/// re-numbered. Eagerly derived names allocate only in the Logos home slice.
#[derive(Debug)]
pub struct LogosNameBoundary {
    names: NameTable,
}

impl LogosNameBoundary {
    /// Compose the fixed Logos vocabulary and a completed schema name slice into a
    /// fresh Logos-owned table.
    pub fn from_schema(schema_names: &NameTable) -> Result<Self, NameTableError> {
        let standards = standard_name_table()?;
        let names = NameTable::new(name_table::IdentifierNamespace::Logos)
            .compose(&standards)?
            .compose(schema_names)?;
        Ok(Self { names })
    }

    /// Resolve `source` and eagerly allocate its requested derived spelling in the
    /// Logos slice. This is the sanctioned string boundary; Nomos remains typed.
    pub fn derive_name(
        &mut self,
        source: Identifier,
        derivation: NameDerivation,
    ) -> Result<Identifier, NameTableError> {
        let derived = match derivation {
            NameDerivation::FieldName => self.names.resolve(source)?.field_name(),
            NameDerivation::ScreamingSnakeCase => self.names.resolve(source)?.screaming(),
            NameDerivation::PascalCase => self.names.resolve(source)?.pascal_case(),
        };
        self.names.intern(Name::new(derived))
    }

    /// The composed one-NameTable view used by later projections.
    pub fn names(&self) -> &NameTable {
        &self.names
    }

    /// Finish boundary work and return the component-owned composed table.
    pub fn into_names(self) -> NameTable {
        self.names
    }
}
