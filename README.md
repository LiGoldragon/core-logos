# core-logos

`core-logos` defines the typed, ordered `WholeLogos` model used between
lowering and textual emission. Its declarations and references carry opaque
`name_table::EncodedName` values; spelling and name allocation belong to the
caller-owned naming boundary.

`WholeLogos` is semantic data, not an object-identity mechanism. The crate
also defines archiveable representations and compatibility tests for those
representations. Storage fingerprints on table declarations describe storage
shape compatibility only; they do not identify a Whole Logos value.

Core Logos has no Protos conversion surface. Protos conversion belongs to a
sibling textual boundary after opaque names have been resolved; that boundary
must create the structural Protos nodes and their source extents. Version 0.28
removes the former `Capsule<Logos, _>` pass-through and its obsolete Protos and
content-identity dependencies.

Run the checks with:

```sh
CARGO_BUILD_JOBS=2 cargo test --all-targets
```
