//! The fixed, schema-independent Logos identifier slice.
//!
//! The data below is the one source of truth for the closed `LogosStandard`
//! vocabulary. The macro generates both the typed identifiers used by the engine and
//! the canonical slice that resolves them at the NameTable/emission boundary. No
//! schema-derived name belongs here.

use name_table::{Identifier, IdentifierNamespace, Name, NameTable, NameTableError};

macro_rules! logos_standard_vocabulary {
    ($(($variant:ident, $constant:ident, $spelling:literal)),+ $(,)?) => {
        /// The fixed Logos vocabulary, in stable namespace-local allocation order.
        ///
        /// The discriminant is the `u16` carried by `Identifier::LogosStandard`.
        /// Adding, removing, or reordering an entry changes the standard NameTable
        /// slice and is therefore a deliberate producer compatibility change.
        #[repr(u16)]
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum StandardIdentifier {
            $($variant,)+
        }

        impl StandardIdentifier {
            /// Every fixed Logos object in its canonical allocation order.
            pub const ALL: &[Self] = &[$(Self::$variant,)+];

            /// The encoded identifier generated from this vocabulary entry.
            pub const fn identifier(self) -> Identifier {
                Identifier::LogosStandard(self as u16)
            }

            /// The sole textual spelling for this fixed object, materialized only
            /// while the standard NameTable slice is built or a later emitter reads it.
            pub const fn spelling(self) -> &'static str {
                match self {
                    $(Self::$variant => $spelling,)+
                }
            }
        }

        $(
            /// A generated typed reference into the immutable Logos standard slice.
            pub const $constant: Identifier = StandardIdentifier::$variant.identifier();
        )+
    };
}

logos_standard_vocabulary!(
    (Integer, INTEGER, "Integer"),
    (String, STRING, "String"),
    (Boolean, BOOLEAN, "Boolean"),
    (Bytes, BYTES, "Bytes"),
    (Vector, VECTOR, "Vec"),
    (Optional, OPTIONAL, "Option"),
    (ScopeOf, SCOPE_OF, "ScopeOf"),
    (Map, MAP, "Map"),
    (Path, PATH, "Path"),
    (Rustfmt, RUSTFMT, "rustfmt"),
    (Skip, SKIP, "skip"),
    (StandardLibrary, STANDARD_LIBRARY, "std"),
    (StringModule, STRING_MODULE, "string"),
    (Unsigned64, UNSIGNED_64, "u64"),
    (RustBoolean, RUST_BOOLEAN, "bool"),
    (NotaTextFeature, NOTA_TEXT_FEATURE, "nota-text"),
    (Nota, NOTA, "nota"),
    (NotaDecodeError, NOTA_DECODE_ERROR, "NotaDecodeError"),
    (NotaEncode, NOTA_ENCODE, "NotaEncode"),
    (NotaSource, NOTA_SOURCE, "NotaSource"),
    (Copy, COPY, "Copy"),
    (Archive, ARCHIVE, "Archive"),
    (Serialize, SERIALIZE, "Serialize"),
    (Deserialize, DESERIALIZE, "Deserialize"),
    (Clone, CLONE, "Clone"),
    (Debug, DEBUG, "Debug"),
    (PartialEq, PARTIAL_EQ, "PartialEq"),
    (Eq, EQ, "Eq"),
);

/// Build Logos's complete, immutable standard NameTable slice.
///
/// The generated constants and this slice derive from the same closed vocabulary.
/// Callers compose this completed slice into their Logos NameTable; they never
/// allocate one of these names into the Logos-owned append slice.
pub fn standard_name_table() -> Result<NameTable, NameTableError> {
    let mut names = NameTable::new(IdentifierNamespace::LogosStandard);
    for standard in StandardIdentifier::ALL {
        let allocated = names.intern(Name::new(standard.spelling()))?;
        debug_assert_eq!(allocated, standard.identifier());
    }
    Ok(names)
}
