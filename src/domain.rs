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
            // Layout 5 adds the ordinary-exchange codec-body vocabulary. `Block`
            // gained `statements: Vec<Statement>` (the `let` bindings ahead of a codec
            // body's tail), `Call` gained `type_arguments: Vec<TypeReference>` (the
            // turbofish in `rkyv::to_bytes::<E>(self)`), the `Expression` algebra grew
            // the `Try` / `Closure` / `Tuple` / `Index` / `Range` nodes the frame
            // encode/decode bodies exercise, `TypeReference` grew the `Tuple` type (the
            // `(InputRoute, Self)` decode return), and `Pattern` grew the `Wildcard` arm
            // the open-`u64`-header match needs. rkyv archives a struct as the
            // concatenation of its fields and an enum sized to its largest variant, so
            // this growth enlarges every `CoreItem` value's archived bytes; the bump is
            // deliberate and honest, exactly as the truthful rule demands.
            //
            // Layout history (each bump hashed a strictly larger archived shape):
            //   * Layout 2 hashed the pre-extension shape;
            //   * Layout 3 hashed the class-B/C/D kernel extension (the `Const` and
            //     `Module` item kinds, `ImplBlock`'s `items: Vec<ImplItem>`, and the
            //     expression/type tail growth);
            //   * Layout 4 hashed the tuple-field-visibility extension;
            //   * Layout 5 hashes the ordinary-exchange codec-body vocabulary.
            //
            // Any future archived-representation change — max-variant growth,
            // discriminant reordering, or field layout — moves hashes and demands a
            // deliberate bump, witnessed by the golden-hash constant in
            // `tests/content_hash_witness.rs`.
            layout: LayoutVersion::new(5),
        }
    }
}
