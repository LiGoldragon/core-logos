# core-logos

The stringless encoded-form algebra of Logos.

## Canonical whole carrier

`WholeLogos` is the production carrier: ordered positional data with complete
translator-issued `VocabularyEncodedId` chains and no field names. Its current
closed item vocabulary contains attribute-free tuple newtypes and
attribute-free, non-generic enumerations with unit or positional tuple
variants. Type references retain either one complete chain or a recursive unary
application.

`WholeLogos` implements the canonical structural-codec 0.11 `EncodedForm`
contract. The crate carries one structural-codec dependency and no private
textual grammar; Rust text conversion belongs to the sibling structural
textual-form component.

## Retained execution data

`EncodedItem` is a broader sealed-execution graph over flat
`name_table::Identifier` values. The current Nomos engine still consumes it, so
the data algebra and archive/content-identity witnesses remain. It is not a
second textual or structural-codec surface.

## Identity and capsule boundaries

`WholeLogos::content_identity` hashes the canonical archive of the complete
ordered carrier. `capsule_from_issued_hash` separately fixes the outer kind to
`protos::Logos` and carries a caller-issued content hash plus opaque complete
NameTree pin. It does not derive, inspect, or compose that pin.

## Building

```sh
cargo test --all-targets
nix flake check --print-build-logs
```
