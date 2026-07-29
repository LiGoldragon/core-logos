# core-logos

The stringless encoded-form algebra of Logos — the Rust-equivalent data language,
1-to-1 with Rust in encoded form.

Logos models a standardized Rust subset as exploded, non-sugared, strictly-typed
positional data. `core-logos` is the encoded-form layer of that model: a closed
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
moves the identity. A Logos NameTable owns the Logos namespace and composes completed schema
slices without copying, flattening, or renumbering their identifiers.

## Capsule carrier

`capsule_from_issued_hash` is the kind-fixed whole-Logos pass-through into
`protos::Capsule<protos::Logos, Pin>`. The caller supplies both the
`ContentAddressedHash` and opaque complete NameTree pin. `core-logos` has no
whole-Logos encoded carrier and does not invent one here: it does not derive a
Capsule hash from `EncodedItem` values, verify content correspondence, inspect or
compose the pin, or claim that its current flat identifiers implement nested
encodedID chains.

`EncodedItem::content_identity` remains the established per-item API and archive
lock. The Capsule pass-through does not reinterpret or replace it.

## Building

```
nix flake check      # build, test, clippy, fmt, doc — the gate
cargo test           # inner-loop tests
```

Published as `0.3.0`. The Capsule surface consumes
`content-identity@f1f9c6efc828acaefd0f751550cd40389d312bf5` under the dependency
name `capsule-content-identity` and
`protos@1435c9aeb7f24e811aca670101e355ff26818ae2`. The legacy per-item/archive
identity and flat name-table graph remain pinned to their existing revisions,
with `core-ethos@b9db643a853b1f52f10a4100a791d5dbc8c7240d` as the current
producer dev-dependency.
