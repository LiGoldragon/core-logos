# core-logos

The stringless encoded-form algebra of Logos — the Rust-equivalent data language,
1-to-1 with Rust in encoded form.

Logos models a standardized Rust subset as exploded, non-sugared, strictly-typed
positional data. `core-logos` is the **EncodedForm** layer of that model: a closed
`EncodedItem` algebra over a shared stringless leaf vocabulary. It is **text-free** by
design — it depends on no `syn`, `prettyplease`, `quote`, or proc-macro machinery.
Rendering Logos to and from Rust text is the job of a later sibling crate,
`TextualRust`; the encoded form never depends on text.

## What "stringless" and "1-to-1 in encoded form" mean here

- Every identifier is a `name_table::Identifier` into a NameTable; paths are
  segment vectors of identifiers. The `::` separator, the `<>` of a generic
  application, the `pub` keyword, and snake_case field names are all *projection*
  concerns that materialize far from this crate.
- Every token of Rust meaning is stored data. Visibility is a stored variant on
  the general item and field nodes (never a minted `PublicStruct`/`PrivateStruct`
  type). Both derive groups and the `cfg_attr` and tool attributes are ordered
  attribute data — never computed at projection.
- Generics lower by kind, never by a string name.

## The algebra

`EncodedItem` is a closed enum — exhaustively matched, no wildcard arms — over four
data-shape item kinds and a shared leaf vocabulary:

- Items: `Newtype`, `Struct`, `Enumeration`, `Alias`.
- Leaves: `Visibility`, `Attribute` (`Derive` / `Configuration` / `ToolPath` /
  `HelperDerive`), `TypeReference` / `TypeApplication`, `PathNode`, `Field`,
  `Variant` / `VariantPayload`, `Generics` / `GenericParameter`, and the
  `name_table::Identifier` leaf.

The trait, impl, and free-method item kinds of the full Rust-lowering ontology are
deliberately out of scope for this text-free encoded form — see `ARCHITECTURE.md`.

## Content identity

`EncodedItem::content_identity` hashes a value over its canonical portable-archive
bytes under a layout-versioned `EncodedLogosDomain`, with the NameTable excluded from
the pre-image. So a rename is hash-stable by construction, and a structural edit
moves the identity. A component NameTable owns its `Logos` slice and composes
read-only `Schema` and `LogosStandard` slices without `extend_from` or copying.

## Building

```
nix flake check      # build, test, clippy, fmt, doc — the gate
cargo test           # inner-loop tests
```

Version `0.3.0` introduces the Logos standard identifier slice and its composed
NameTable boundary. It consumes `content-identity` and `name-table` as pinned git
dependencies (`core-schema` is a dev-dependency for producer compatibility tests).
