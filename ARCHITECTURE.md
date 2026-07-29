# core-logos architecture

`core-logos` contains two implementation grades: the narrow full-chain
`WholeLogos` carrier used by the first production slice, and a broader legacy
`EncodedItem` graph retained as implementation evidence. This document keeps
those grades explicit so legacy coverage is not mistaken for production law.

## Production carrier: positional and full-chain

The conforming carrier is `WholeLogos`. Its current closed fixture item set
contains attribute-free tuple newtypes and attribute-free, non-generic
enumerations with unit or positional tuple variants. Type references are
either one complete identity or a recursive unary application. Every item and
variant declaration is a complete Universal `VocabularyEncodedId` chain.
Application heads and referenced types may instead carry a complete
language-vocabulary chain after typed Nomos transformation; Rust vocabulary
therefore stays distinct from Universal authored identity.

Fields have no names in encoded data. Deterministic Rust field naming belongs
to textual conversion and remains undesigned. The production carrier does not
admit attributes, generics, named-field structs or variants, aliases,
functions, impl blocks, uses, constants, modules, or expression bodies. Their
presence in the legacy graph does not widen this slice.

## Text and string boundaries

This crate depends on no `syn`, `prettyplease`, `quote`, or proc-macro machinery.
That crate-local dependency boundary does not make every stored graph
stringless: legacy `Expression::StringLiteral` carries an owned `String`, and
legacy identifiers are flat `name_table::Identifier` values. The full-chain
`WholeLogos` path carries neither.

Production Rust conversion belongs to the structural `rust-logos` textual-form
path and does not use `syn`, `quote`, or `prettyplease`. The existing
`textual-rust` sibling and its parser/printer are legacy evidence, not the
production path.

## Capsule carrier boundary

`capsule_from_issued_hash` fixes only the outer kind to `protos::Logos`, whose
stored identity variant is `WholeLogos`. It passes a caller-issued
`ContentAddressedHash` and caller-supplied opaque complete NameTree pin into
`protos::Capsule`. It does not create a whole-Logos encoded carrier, collect
`EncodedItem` values into one, derive or verify a whole-content hash, inspect the
pin, or compose module tables. Complete-pin verification and the
module-table-to-Capsule relationship remain unwired.

The existing per-item `EncodedItem::content_identity` API and archive layout are
unchanged. The new identity producer is dependency-renamed
`capsule-content-identity`; the original dependency remains the established
per-item/archive type in the legacy graph. Flat `Identifier` fields and the old
`NameTable` dependency stay explicit migration debt rather than a chain-migration
claim.

## Ordered whole-Logos content

`WholeLogos` is the smallest honest ordered carrier for the current vertical
fixture slice. Its `Vec<WholeLogosItem>` preserves semantic item order. A
newtype is represented as four positional values: item visibility, declaration
encodedID, wrapped-field visibility, and typed reference. An enumeration
carries item visibility, declaration encodedID, and ordered variants. A variant
carries its declaration encodedID and either unit or non-empty positional tuple
payload. A unary type application carries a head encodedID and one recursively
typed reference.

Both name positions use
`signal_sema_translator::VocabularyEncodedId`, preserving each complete
root-fronted module chain opaquely. Archive restoration validates every
declaration as non-empty Universal vocabulary and every reference as a
non-empty chain in the closed production root set. The carrier does not
resolve, shorten, flatten, or reallocate those chains. `WholeLogosVisibility` deliberately admits
only `Public` and `Private`; broader visibility does not enter this carrier
through the legacy flat-identifier path.

`WholeLogos::content_identity` archives the complete ordered carrier
canonically, hashes those bytes without a whole-content
kind/domain/layout discriminator in the hash input, and then wraps the result
in the local
`WholeLogosContentIdentity::WholeLogos` variant. Item mutation and semantic
reordering therefore move the whole identity.

This local identity is not a Capsule identity. No complete NameTree pin is
available here, so this crate neither constructs
`content_identity::CapsuleIdentity` nor passes the derived hash into
`protos::Capsule`. How module-table snapshots compose into the complete pin, and
whether a Capsule identity is minted or derived, remain unwired. The existing
per-item hashes remain implementation evidence; recursive per-item content
hashing is undiscussed, neither rejected nor approved.

## Legacy per-item content identity

`EncodedItem::content_identity` is `ContentHash::of_core` under
`EncodedLogosDomain`, a `Contextual` hash domain tagged with `LayoutVersion(7)`.
The pre-image is the value's canonical portable-archive bytes; the NameTable is
excluded (it is not part of an encoded value). Two invariants follow and are
tested:

- **Legacy spelling edits are hash-stable.** The encoded value carries the flat
  identifier, not its NameTable spelling, so changing that spelling does not
  move the value's hash. This is not evidence for operational rename or
  full-chain continuity.
- **A structural edit moves the identity.** Changing a wrapped type, a visibility,
  or attribute order changes the encoded value and therefore its hash.

## Legacy composed identifier namespaces

A Logos NameTable owns the `Logos` namespace and composes completed schema slices.
Borrowed `Schema` identifiers retain their exact namespace and local allocation;
they are neither copied, flattened, nor renumbered. New Logos names allocate only
in the Logos home slice. The composition test proves this existing behavior. It
does not prove the approved nested module-owned encodedID-chain model; that
migration remains coordinated downstream work.

## Legacy EncodedItem evidence

Everything in this section is the nonconforming legacy graph. It is useful
archive, lowering, and projection evidence, but it is not production support.

The current `EncodedItem` enum has nine variants: `Newtype`, `Struct`,
`Enumeration`, `Alias`, `ImplBlock`, `Function`, `Use`, `Const`, and `Module`.
It also carries a broad leaf vocabulary for attributes, visibility, flat
identifier paths, generics, fields, variants, types, statements, patterns, and
expressions. `TraitDefinition` is absent. The historical item-ethos
ratification has an unrecovered “otherwise” exception, so this inventory is a
statement of implemented code only.

An `ImplBlock`'s members are the ordered heterogeneous `ImplItem` set — a `Method`,
an `AssociatedType` (`type Err = NotaDecodeError;`), or an `AssociatedConst`
(`const HEADS: &'static [&'static str] = &[…];`) — in source order, so a `type`
binding that precedes its method round-trips in place. `Const` is one node shared by
a top-level const, a module const, and an associated const, because they are one
concept; its visibility is stored data (a trait-impl associated const stores
`Private`). `Module` carries a `Vec<EncodedItem>`; the witnessed shape is the
`short_header` const module.

`Use` is a `<attrs> <vis> use <base>::{<group>};` node: a base path and an ordered
group of imported leaf identifiers, stored as data. It carries the fixed cfg-gated
NOTA import (`#[cfg(feature = "nota-text")] pub use nota::{NotaDecodeError,
NotaEncode, NotaSource};`) that heads the generated wire modules. Like an impl block
it declares no name (`EncodedItem::name` returns `None`); unlike one it carries its own
visibility, so `with_visibility` stamps it. The plain `#[cfg(...)]` gate is a new
`Attribute::Cfg(ConfigurationPredicate)` variant, reusing the one predicate
vocabulary shared with `cfg_attr` (distinct from `Configuration`, which gates an
inner attribute rather than the item's compilation).

`ImplBlock` and `Function` carry their method **bodies** as data — the closed
**Tier-1 expression vocabulary** (`src/expression.rs`, `src/pattern.rs`), exactly
the class-A-and-kin body shapes the wire goldens exercise and nothing extensible by
string:

- `self`; a reference `&self.0`; a tuple-index field access `self.0`;
- a call of a plain or trait-qualified path callee — `Self(payload.into())`,
  `Self::new(payload)`, `Self::Record(payload)`, `RecordIdentifier::new(payload)`,
  `<Self as Trait>::method(self)`;
- a method call, with an optional turbofish — `payload.into()`, `self.0.name()`,
  `source.parse::<Self>()`;
- a string literal — `"SignalInputRecord"`; an integer literal — `0x0001000000000000`,
  `8`; an array literal — `["Record", "Observe"]`;
- a `match` over a scrutinee whose arms map a variant pattern (a unit-like path
  `InputRoute::Record`, a tuple variant `Self::Record(_)` / `Self::Input(route)`, or
  the wildcard `_` an open-`u64`-header match needs) to a body expression;
- (layout 5, the **ordinary-exchange codec bodies**) a `?` try, a closure
  `|_| SignalFrameError::ArchiveEncode`, a tuple `(route, value)`, an index
  `frame[SIGNAL_SHORT_HEADER_BYTE_COUNT..]`, a half-open range `..n` / `n..`, and a
  turbofished call `rkyv::to_bytes::<rkyv::rancor::Error>(self)`.

Function bodies are a run of **statements followed by a tail expression** (`Block`
carries `statements: Vec<Statement>`): the class-A-and-kin bodies carry no statements
(the single-tail-expression form), while the `encode_signal_frame` /
`decode_signal_frame` codec bodies carry `let` / `let mut` bindings and
expression-statements ahead of their tail. The `let` mutability is a closed
[`LetBinding`] kind, not a boolean. The whole vocabulary is closed and dispatches on
node kind, never on a head string.

`TraitDefinition` as a top-level item is absent. Associated types and consts are
modeled only **inside legacy impl blocks**, where the goldens carry them.
Struct-literal construction (`Self { … }`), named field access, and early `return`
remain the honest frontier beyond the modeled statement vocabulary — the codec bodies
are written in a style that dissolves them (an `.ok_or(…)?` in place of an
`if … { return … }`, a tuple-variant error carrying `header` in place of a
struct-variant literal), so they are not needed and a body demanding them is still
rejected loudly by the TextualRust reader. Const generic parameters remain excluded
(unwitnessed).

Totality is structural: `EncodedItem`'s methods match every variant with no wildcard
arm, so a new item kind is a compile error until its handling is written. An impl
block declares no name, so `EncodedItem::name` returns `Option<Identifier>` — the "does
this item have a name?" question dissolves into a normal `None` rather than a
fabricated identifier, and an impl block has no visibility, so `with_visibility`
returns it unchanged.

## Legacy vocabulary boundaries

The legacy graph records witnessed implementation choices, not a future design:
`Expression::StringLiteral(String)` is string-bearing; integer literals use
typed value-plus-representation data; receivers omit `&mut self`; field access
is tuple-indexed in the modeled bodies; `Use` admits only its brace-group form;
`ConfigurationPredicate` currently admits only `Feature`; and inline `Module`
was built for const members. Function and local-name identity, the
string-literal remedy, and any widening of these shapes remain outside this
document.

## Legacy content identity and layout history

**The current layout is 7.** The historical record below begins with the
layout-2 correction. An earlier version of this document claimed that adding item
kinds was "append-only" enum growth under which "every pre-existing encoded value
archives to byte-identical bytes and its content identity does not move," and
concluded that `LayoutVersion(1)` should be kept. **That reasoning was wrong, and the claim was false.** The commit messages
that shipped the growth are history and stand; this document must tell the truth,
so it records the correction here.

The error was reasoning about append-only-ness at the **Rust source level** and
assuming it carried to the **archived byte level**. It does not, because of how
rkyv lays out enums. rkyv archives an enum at a **fixed size equal to its largest
variant** — every `ArchivedEncodedItem`, regardless of which variant it holds,
occupies the same footprint. Content identity is blake3 over the **full archived
root**, so that footprint is in the pre-image of every value.

Concretely, on the empirical record:

- Commit `be809429` added the `Function` variant, whose archived form grew
  `ArchivedEncodedItem`'s max size from **47 to 101 bytes**. Every `EncodedItem` value —
  including shapes untouched at the Rust source level — therefore re-serialized
  larger and its content hash **moved**: a same-shape `Newtype` went from 51 to 105
  archived bytes and its hash moved from `2c26397e…` to `1c8ae182…`. Yet
  `LayoutVersion` stayed `new(1)` and this document claimed identity did not move.
  The claim was false; the layout should have been bumped at that commit.
- Commit `f7dd7d6b` inserted `Attribute::Cfg` at discriminant index 2, **shifting**
  the `ToolPath` and `HelperDerive` tags. That is a discriminant reordering, not
  append-only growth — it moves the archived tag byte of every attribute at or after
  that index. It happened to be **benign for `EncodedItem`'s hash only because the
  attribute enum's max variant size did not change**, so `ArchivedEncodedItem`'s
  footprint was unaffected. Benign-by-luck is still a layout-relevant change of
  exactly the mis-grounded kind: the safe discipline is to treat any discriminant
  reordering as hash-affecting unless proven otherwise.

The truthful rule, now enforced: **any change to the archived representation moves
hashes and demands a deliberate `LayoutVersion` bump.** That includes max-variant
growth (a new or larger variant enlarging the fixed enum footprint), discriminant
reordering, and field-layout changes. "Append-only" at the Rust source level does
**not** imply archived-byte stability under rkyv's fixed-size enum layout. The
layout version protects the pre-image *format*, and the format changed — the enum
footprint grew — so the version moves with it. `LayoutVersion(1)` hashed the
pre-`be809429` shape; **`LayoutVersion(2)` hashes the shipped shape**, and covers
both the `Function` growth and the `Cfg` discriminant shift in one honest bump.

This class of silent drift shipped because there was **no witness**. There now is:
`tests/content_hash_witness.rs` pins an **absolute** content-hash constant for a
representative value under the current layout. Any future change to the archived
representation fails that test loudly, forcing a deliberate layout bump and a
deliberate constant update rather than a silent hash move.

**Consumers cross a hash boundary at layout 2.** Any consumer advancing its
`core-logos` pin across `be809429` and later moves from layout-1 hashes to
layout-2 hashes: every `EncodedItem` content identity it computes changes. A host
survey at the time of this correction confirmed **no durable store or fixture holds
persisted `EncodedItem` hashes** (everything recomputes, or is tempdir-ephemeral), so
this correction needs no data migration — but the boundary is real, and a consumer
must advance across it only via the deliberate cascade slice, never casually.

### Layout 3: the class-B/C/D kernel extension

**The layout is now 3.** The class-B/C/D kernel extension grew the vocabulary to
cover the goldens' interface-enum ergonomics (constructor and `From` impls, the
cfg-gated `FromStr`/`Display` impls with their associated types and mutable-formatter
signatures), the trace/object-name enums with nested-match `name` methods, and the
class-C stub items (const, const module, associated const). That growth moved the
archived representation in three compounding ways — `EncodedItem` gained `Const` and
`Module` (the latter carrying `Vec<EncodedItem>`); `ImplBlock` replaced its
`Vec<Function>` with a `Vec<ImplItem>`; and `Expression`, `TypeReference`,
`ReferenceType`, and `MethodCall` grew tail variants and fields. All enum growth is
**append-only at the tail** (no discriminant shifted), but by the truthful rule that
does not imply archived-byte stability under rkyv's fixed-size enum layout, so the
version moves. `LayoutVersion(2)` hashed the pre-extension shape; **`LayoutVersion(3)`
hashes the extended shape**, and `tests/content_hash_witness.rs` pins the new
absolute hash deliberately. Consumers (the signal-sema-storage seam) cross this hash
boundary and must re-converge across layout 3 only via the deliberate cascade slice.

**The short-header const *values* are modeled as data, not re-decided.** The
`short_header` module's `0x…` const values (`INPUT_RECORD`, `INPUT_OBSERVE`, …) are
transcribed from the golden's existing text into `IntegerLiteral` value-plus-
representation data. This crate **models the golden's existing bytes**; it does not
decide the short-header byte layout, which is a separate psyche-pending question
(`.9`). Modeling a value as an `IntegerLiteral` says nothing about what that value
*should* be — a later layout decision changes the golden and thus the transcribed
data, with no change to this vocabulary.

### Layout 4: tuple-field visibility

**The layout is now 4.** The newtype form gained tuple-field visibility: `Newtype`
carries a `wrapped_visibility: Visibility` between `name` and `wrapped`, so the
single tuple field of a `pub`-field tuple struct (`TraceEvent(pub ObjectName)`) is
stored data exactly as visibility is on the legacy item and `Field` nodes.
This closes the last class-D gap — the trace goldens declare
`pub struct TraceEvent(pub ObjectName);`, whose `pub` field the layout-3 `Newtype`
could not model, so the declaration was not byte-exact-projectable. This is a
legacy `Newtype` archive-layout fact, not a named-field production rule. `Private`
projects to the empty token stream, so a bare newtype (`CommitSequence(Integer)`)
stores `Private` and projects unchanged — the "no `pub` on the field" special case
dissolves into the normal case. rkyv archives a struct as the concatenation of its
fields, so the new field enlarged every `Newtype` value's archived bytes and, because
`EncodedItem` is a fixed-size enum sized to its largest variant, moved every
`EncodedItem` value's archived bytes. `LayoutVersion(3)` hashed the class-B/C/D shape;
**`LayoutVersion(4)` hashes the tuple-field-visibility shape**, and
`tests/content_hash_witness.rs` pins the new absolute hash deliberately. Consumers
(the signal-sema-storage seam) cross this hash boundary and must re-converge across
layout 4 only via the deliberate cascade slice, never casually.

### Layout 5 and layout 6: ordinary-exchange vocabulary

Layout 5 added the ordinary-exchange codec-body vocabulary: `Block` gained ordered
statements, `Call` gained type arguments, and the `Expression`, `TypeReference`,
and `Pattern` algebras gained their witnessed variants. Layout 6 then added the
ordinary-exchange envelope vocabulary, including `StructLiteral`. Each change
moved archived bytes and received its own deliberate layout bump.

### Layout 7: namespace-variant identifiers

Layout 7 adopts namespace-variant `Identifier` values with `u16` locals and the
canonical `EncodedItem` hash context. Every `EncodedItem` stores identifiers, so
replacing the former flat representation changes its archived bytes even when the
Rust-shaped data is otherwise identical; the renamed context deliberately changes
the domain preamble as well. The `CommitSequence` absolute witness at layout 7 is
`3f2d85f564a74df7962f4e9a110fdab92b1dc1899edd8f418314e254f285e73d`.

## Release-train status

The crate **git-pins** its published dependencies (`content-identity`,
`name-table`, and the `core-ethos` dev-dependency) at exact revisions — the green
path. It rides the multi-repository release train pending this session's
delta-audit; the git pins hold the reproducible build meanwhile. Cargo, Nix, and
cache authority stay separate: the lock file carries the revisions, and
`nix flake check` (crane) is the gate.

## Layout

`src/whole.rs` owns the narrow full-chain production carrier. The remaining
files implement the legacy graph: the closed `item` algebra, its content
identity, leaf families, and item implementations for newtype, structure,
enumeration, alias, impl block, function, use, const, and module. Tests live
under `tests/`.
