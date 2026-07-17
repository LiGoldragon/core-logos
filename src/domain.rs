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
            // Layout 2 covers the shipped archived-shape change. rkyv archives an
            // enum at a fixed size equal to its largest variant, so adding the
            // `Function` item kind (commit be809429) grew `ArchivedCoreItem`'s max
            // size from 47 to 101 bytes: EVERY `CoreItem` value — including
            // pre-existing shapes untouched at the Rust-source level — now
            // re-serializes larger, and its content identity (blake3 over the full
            // archived root) moved. Append-only enum growth at the Rust-source level
            // does NOT imply archived-byte stability under rkyv's fixed-size enum
            // layout. Layout 1 hashed the pre-be809429 shape; layout 2 hashes the
            // shipped shape. Any future change to the archived representation —
            // max-variant growth, discriminant reordering, or field layout — moves
            // hashes and demands a deliberate bump, witnessed by the golden-hash
            // constant in `tests/content_hash.rs`.
            layout: LayoutVersion::new(2),
        }
    }
}
