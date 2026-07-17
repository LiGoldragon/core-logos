//! The content-identity domain for CoreLogos values.

use content_identity::{DomainSeparation, HashDomain, LayoutVersion};

/// The layout-versioned hash domain for every CoreLogos value. The domain carries
/// the layout version in the type, so "which Core layout" is never a
/// hand-remembered suffix. The NameTable is excluded from every CoreLogos
/// pre-image (it is not part of a Core value), so a rename is hash-stable by
/// construction and a structural edit moves the identity.
pub struct CoreLogosDomain;

impl HashDomain for CoreLogosDomain {
    fn separation() -> DomainSeparation {
        DomainSeparation::Contextual {
            context: "core-logos 2026 stringless core algebra of logos",
            // Layout 3 covers the class-B/C/D kernel extension. rkyv archives an
            // enum at a fixed size equal to its largest variant and hashes blake3
            // over the full archived root, so growing the vocabulary moved every
            // `CoreItem` value's archived bytes and therefore its content identity.
            // This bump is deliberate and honest, exactly as the truthful rule
            // demands. The archived shape changed in three compounding ways:
            //   * `CoreItem` gained the `Const` and `Module` variants (`Module`
            //     carries `Vec<CoreItem>`, enlarging the fixed enum footprint);
            //   * `ImplBlock` replaced `functions: Vec<Function>` with
            //     `items: Vec<ImplItem>`, a new enum wrapping the member kinds;
            //   * `Expression`, `TypeReference`, `ReferenceType`, and `MethodCall`
            //     grew tail variants and fields (integer/array literals, slice and
            //     lifetime types, reference mutability, method turbofish).
            // All enum growth is append-only at the tail — no discriminant shifted —
            // but append-only at the Rust-source level does NOT imply archived-byte
            // stability under rkyv's fixed-size enum layout, so the version moves.
            // Layout 2 hashed the pre-extension shape; layout 3 hashes the extended
            // shape. Any future archived-representation change — max-variant growth,
            // discriminant reordering, or field layout — moves hashes and demands a
            // deliberate bump, witnessed by the golden-hash constant in
            // `tests/content_hash_witness.rs`.
            layout: LayoutVersion::new(3),
        }
    }
}
