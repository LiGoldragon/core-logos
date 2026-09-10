# core-logos architecture

## Model boundary

`WholeLogos` is the crate's ordered semantic carrier. It contains lowered
items such as newtypes, structs, enumerations, traits, implementations, and
tables. Names are opaque `EncodedName` values, so this crate does not assign
names or turn them into text.

Sibling crates own parsing, lowering, and target-language emission. They pass
typed Whole Logos values into or out of this model; naming mechanisms remain
outside this crate.

The model therefore does not implement `Protosizable` or manufacture an
extent. A textual sibling can only construct the final Protos structure and
its source extents after resolving the opaque names. The retired whole-Logos
Capsule constructor carried caller values without performing that conversion
and is not part of this boundary.

## Archive and storage compatibility

The public model is archiveable with rkyv, and compatibility tests protect
selected tuple layouts. These archive details support transport and evolution,
not semantic equality or ownership.

Table declarations retain record and key storage fingerprints. A fingerprint
captures the compatibility of one storage shape. It is deliberately narrower
than semantic equality and does not stand for the identity of the table or
the enclosing Whole Logos value.
