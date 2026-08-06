# core-logos architecture

`WholeLogos` is the only Logos data model in this crate. Its ordered items and
complete encoded IDs are semantic data. Structural decoding and Rust text
projection belong to sibling crates.

`WholeLogos::content_identity` derives from its portable archive. Capsule
assembly accepts a caller-issued hash and opaque complete NameTree pin, keeping
issued identity separate from pure content.

Tables carry their declared record/key shape and derived schema hash. They do
not preserve an adopted physical Sema family or an alternate emitted coordinate.
