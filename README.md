# core-logos

The Logos encoded-form crate. It currently contains a conforming, deliberately
narrow full-chain carrier and a broader legacy flat-identifier graph.

`WholeLogos` is the production precedent: ordered positional data with complete
translator-issued encodedID chains and no field names. `EncodedItem` is retained
legacy evidence. The crate itself depends on no `syn`, `prettyplease`, `quote`,
or proc-macro machinery, but that does not make every legacy value string-free:
`Expression::StringLiteral` carries an owned `String`.

## The production carrier

`WholeLogos` admits one item shape: an attribute-free, non-generic tuple
newtype. Its four positional values are item visibility, declaration encodedID,
wrapped-field visibility, and referenced-type encodedID. Both identities are
complete `VocabularyEncodedId` chains. The carrier does not resolve, flatten,
shorten, or allocate them.

Fields have no stored names. Deterministic names for Rust text remain a
textual-form concern and are not designed here. Attributes, generics, structs,
enums, aliases, functions, impl blocks, uses, constants, modules, and expression
bodies are outside this carrier's current support.

## Legacy EncodedItem evidence

`EncodedItem` is a separate nonconforming graph over flat
`name_table::Identifier` values, stored field-name identifiers, NameTable
composition, and a string-bearing literal variant. Its current item variants
are `Newtype`, `Struct`, `Enumeration`, `Alias`, `ImplBlock`, `Function`, `Use`,
`Const`, and `Module`. `TraitDefinition` is absent.

That broad inventory is regression and migration evidence, not production
coverage. See `ARCHITECTURE.md` for its exact implemented boundaries and archive
history.

## Legacy per-item content identity

`EncodedItem::content_identity` hashes a value over its canonical portable-archive
bytes under a layout-versioned `EncodedLogosDomain`, with the NameTable excluded from
the pre-image. A legacy spelling edit therefore leaves the flat-identifier value's
hash unchanged, while a structural edit moves it. This is not proof of the
approved nested module-owned authority or its operational rename.

## Capsule carrier

`capsule_from_issued_hash` is the kind-fixed whole-Logos pass-through into
`protos::Capsule<protos::Logos, Pin>`. The caller supplies both the
`ContentAddressedHash` and opaque complete NameTree pin. This helper neither
constructs nor validates `WholeLogos`: it does not derive a Capsule hash from
either carrier, verify content correspondence, inspect or compose the pin, or
claim that legacy flat identifiers implement nested encodedID chains.

`EncodedItem::content_identity` remains the established per-item API and archive
lock. The Capsule pass-through does not reinterpret or replace it.

## Ordered whole content

`WholeLogos` is the separate first-slice carrier for ordered typed items whose
name positions use complete production `VocabularyEncodedId` chains. The
current closed item vocabulary is one attribute-free, non-generic tuple
newtype. Its whole-content identity is derived from the canonical archive of
the complete ordered carrier and wrapped in
`WholeLogosContentIdentity::WholeLogos`; the outer kind is not folded into the
hash bytes.

This is deliberately not a Capsule-identity derivation. Complete NameTree pin
composition and the minted-versus-derived Capsule relationship remain open and
unwired.

## Building

```
nix flake check      # build, test, clippy, fmt, doc — the gate
cargo test           # inner-loop tests
```

Published as `0.4.0`. Exact producer revisions live in `Cargo.toml` and
`Cargo.lock`; the manifest deliberately keeps the full-chain and legacy
dependency worlds distinct.
