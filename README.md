# core-logos

`WholeLogos` is the production structural carrier for Logos. It stores ordered
positional data and complete translator-issued `VocabularyEncodedId` chains;
Rust spelling is the textual codec's concern.

Its archive and content identity cover the complete ordered value. A separately issued
Capsule can carry the outer `protos::Logos` kind and opaque complete NameTree
pin without deriving or inspecting that pin.

```sh
cargo test --all-targets
nix flake check --max-jobs 0
```
