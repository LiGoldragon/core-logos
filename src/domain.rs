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
            // Layout 4 adds tuple-field visibility to the newtype form. `Newtype`
            // gained a `wrapped_visibility: Visibility` field between `name` and
            // `wrapped`, so the tuple field of a `pub`-field tuple struct
            // (`TraceEvent(pub ObjectName)`) is stored data, never a projection-time
            // guess. rkyv archives a struct as the concatenation of its fields, so a
            // new field enlarges every `Newtype` value's archived bytes — and,
            // because `CoreItem` is a fixed-size enum sized to its largest variant,
            // the archived bytes of every `CoreItem` value moved with it. This bump
            // is deliberate and honest, exactly as the truthful rule demands.
            //
            // Layout history (each bump hashed a strictly larger archived shape):
            //   * Layout 2 hashed the pre-extension shape;
            //   * Layout 3 hashed the class-B/C/D kernel extension (the `Const` and
            //     `Module` item kinds, `ImplBlock`'s `items: Vec<ImplItem>`, and the
            //     expression/type tail growth);
            //   * Layout 4 hashes the tuple-field-visibility extension.
            //
            // Any future archived-representation change — max-variant growth,
            // discriminant reordering, or field layout — moves hashes and demands a
            // deliberate bump, witnessed by the golden-hash constant in
            // `tests/content_hash_witness.rs`.
            layout: LayoutVersion::new(4),
        }
    }
}
