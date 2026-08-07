# core-logos

`core-logos` defines the typed, ordered `WholeLogos` model used between
lowering and textual emission. Its declarations and references carry opaque
`name_table::EncodedName` values; spelling and name allocation belong to the
caller-owned naming boundary.

`WholeLogos` is semantic data, not an object-identity mechanism. The crate
also defines archiveable representations and compatibility tests for those
representations. Storage fingerprints on table declarations describe storage
shape compatibility only; they do not identify a Whole Logos value.

Run the checks with:

```sh
CARGO_BUILD_JOBS=2 cargo test --all-targets
```
