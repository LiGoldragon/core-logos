//! The content-identity domain for EncodedLogos values.

use content_identity::{DomainSeparation, HashDomain, LayoutVersion};

/// The layout-versioned hash domain for every EncodedLogos value. The domain carries
/// the layout version in the type, so "which encoded-form layout" is never a
/// hand-remembered suffix. The NameTable is excluded from every EncodedLogos
/// pre-image (it is not part of an encoded-form value), so a rename is hash-stable by
/// construction and a structural edit moves the identity.
pub struct EncodedLogosDomain;

impl HashDomain for EncodedLogosDomain {
    fn separation() -> DomainSeparation {
        DomainSeparation::Contextual {
            context: "core-logos 2026 stringless encoded-form algebra of logos",
            // Layout 7 adopts namespace-variant encoded identifiers and the
            // canonical EncodedLogos hash context. Every EncodedItem holds
            // identifiers, so replacing the former flat identity representation with
            // `Schema(u16)` and `Logos(u16)` changes its archived representation even
            // when its Rust-shaped data is otherwise identical. The renamed context
            // deliberately changes the domain preamble too. Composed NameTable slices
            // preserve source identifiers without copying, flattening, or renumbering
            // them.
            //
            // Layout 6 adds the ordinary-exchange envelope vocabulary. The
            // `Expression` algebra grew the `StructLiteral` node (the
            // `FrameBody::Request { exchange, request }` bodies `into_frame` /
            // `into_reply_frame` construct), and a struct-literal field is a
            // `FieldInitializer` whose value is an optional boxed expression. rkyv
            // sizes an enum to its largest variant, so this new expression node
            // enlarges every `EncodedItem` value's archived bytes; the bump is deliberate
            // and honest, exactly as the truthful rule demands.
            //
            // Layout history (each bump hashed a strictly larger archived shape):
            //   * Layout 2 hashed the pre-extension shape;
            //   * Layout 3 hashed the class-B/C/D kernel extension (the `Const` and
            //     `Module` item kinds, `ImplBlock`'s `items: Vec<ImplItem>`, and the
            //     expression/type tail growth);
            //   * Layout 4 hashed the tuple-field-visibility extension;
            //   * Layout 5 hashed the ordinary-exchange codec-body vocabulary (`Block`
            //     gained `statements`, `Call` gained `type_arguments`, and the
            //     `Expression` / `TypeReference` / `Pattern` algebras grew the
            //     `Try` / `Closure` / `Tuple` / `Index` / `Range` / tuple-type /
            //     wildcard-pattern nodes the frame encode/decode bodies exercise);
            //   * Layout 6 hashes the ordinary-exchange envelope vocabulary (the
            //     `StructLiteral` expression node the `into_frame` / `into_reply_frame`
            //     bodies construct);
            //   * Layout 7 hashes namespace-variant `u16` identifiers and composed
            //     NameTable slices.
            //
            // Any future archived-representation change — max-variant growth,
            // discriminant reordering, or field layout — moves hashes and demands a
            // deliberate bump, witnessed by the golden-hash constant in
            // `tests/content_hash_witness.rs`.
            layout: LayoutVersion::new(7),
        }
    }
}
