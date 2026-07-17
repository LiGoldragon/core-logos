# core-logos architecture

`core-logos` is slice three of the psyche-authorized language-family proof of
concept: the stringless Core algebra of Logos, the Rust-equivalent data language.
This document records the durable direction — the rulings the crate embodies and
the boundaries it holds — for an agent entering the repository.

## The one ruling that shapes everything: 1-to-1 with Rust, at the Core

Logos is 1-to-1 with Rust at the Core. Every token of Rust meaning is stored data
in a CoreLogos value; nothing is materialized at projection. Concretely:

- The field **name is always present** as a stored `Identifier`. Text elision (a
  field name dropped when it equals the snake_case of its type) is a text
  projection concern that never reaches this crate.
- **Visibility is stored data** — a variant on the general item and field nodes,
  carried verbatim (a `pub(crate)` field stores `Crate`). It is never a minted
  specialized type and never computed from a reference graph at projection.
  `Visibility::Private` is a value whose Rust projection is the empty token stream,
  so the "is there a `pub`?" special case dissolves into a normal node.
- **Both derive groups, and the `cfg_attr` and tool attributes, are ordered
  attribute data** — never computed at projection. The three-attribute golden
  preamble (`#[rustfmt::skip]`, the feature-gated NOTA derive group, the plain
  rkyv derive group) is simply three entries in an ordered `Vec<Attribute>`.

## The text-free boundary

This crate depends on no `syn`, `prettyplease`, `quote`, or proc-macro machinery.
Core never depends on text. The TextualRust codec — `syn` decode and
`prettyplease` encode against the schema-rust goldens — is a **later sibling
crate**. It reads and writes CoreLogos; it does not live here. Keeping the Core
text-free is what lets the same Core be viewed through many textual forms
(TextualLogos, TextualRust, and future emission languages) without any of them
reaching into the Core.

Stringlessness is total: every identifier is a `name_table::Identifier`; paths are
segment vectors of identifiers (dotted in any text form; `::` materializes only at
Rust projection). There is no `Opaque` raw-text attribute variant — it is unused by
the acceptance oracle and would carry raw token text, breaking the text-free
boundary; if a genuinely opaque foreign attribute is ever needed, it belongs with
the TextualRust sibling, not here.

## Content identity

`CoreItem::content_identity` is `ContentHash::of_core` under `CoreLogosDomain`, a
`Contextual` hash domain tagged with `LayoutVersion(4)` (see "Content identity and
layout version" below for why layout 4). The pre-image is the value's canonical
portable-archive bytes; the NameTable is excluded (it is not part of a Core value).
Two invariants follow and are tested:

- **Rename is hash-stable.** The Core carries the identifier, never the string, so
  changing what a name projects to does not move the identity.
- **A structural edit moves the identity.** Changing a wrapped type, a visibility,
  or attribute order changes the Core value and therefore its hash.

## One continuous identifier space

The logos NameTable extends the schema NameTable via `name_table::extend_from`: the
logos table is a higher-index append-extension of the schema table, so a
carried-over schema identifier keeps its exact index. The continuity test builds a
core-schema-populated table and proves existing indices stay stable while new
logos names append above the base.

## Scope: which item kinds this Core carries, and why

The accepted Rust-lowering ontology (`reports/logos/logos-rust-lowering-v1.md`)
names seven item kinds: `Newtype`, `Struct`, `Enumeration`, `Alias`,
`TraitDefinition`, `ImplBlock`, `FreeMethod`. This crate carries the
**wire-contract data subset** — `Newtype`, `Struct`, `Enumeration`, `Alias`, plus
the leaf vocabulary (attributes, visibility, paths, generics by kind) — **and
`ImplBlock` and `Function`** (the ontology's `FreeMethod`, modeled as one node that
serves both an impl member and a free function), **`Use`** (the `use`-import
shape at the head of every generated module), and — from the layout-3 kernel
extension — **`Const` and `Module`** (a const declaration and a const-carrying
inline module, the class-C stub items the signal goldens emit).

An `ImplBlock`'s members are the ordered heterogeneous `ImplItem` set — a `Method`,
an `AssociatedType` (`type Err = NotaDecodeError;`), or an `AssociatedConst`
(`const HEADS: &'static [&'static str] = &[…];`) — in source order, so a `type`
binding that precedes its method round-trips in place. `Const` is one node shared by
a top-level const, a module const, and an associated const, because they are one
concept; its visibility is stored data (a trait-impl associated const stores
`Private`). `Module` carries a `Vec<CoreItem>`; the witnessed shape is the
`short_header` const module.

`Use` is a `<attrs> <vis> use <base>::{<group>};` node: a base path and an ordered
group of imported leaf identifiers, stored as data. It carries the fixed cfg-gated
NOTA import (`#[cfg(feature = "nota-text")] pub use nota::{NotaDecodeError,
NotaEncode, NotaSource};`) that heads the generated wire modules. Like an impl block
it declares no name (`CoreItem::name` returns `None`); unlike one it carries its own
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
  `InputRoute::Record`, or a tuple variant `Self::Record(_)` / `Self::Input(route)`)
  to a body expression (a unit path, a string literal, or a nested match).

Function bodies are a **single tail expression** — the witnessed Tier-1 bodies carry
no statements, so no `let`/`return` statement vocabulary is modeled. Matches are
exhaustive with no wildcard arm; the whole vocabulary is closed and dispatches on
node kind, never on a head string.

`TraitDefinition` as a top-level item remains **left out**: a trait declaration's
default bodies and member declarations are a separate growth. (Associated types and
consts are now modeled **inside impl blocks**, where the goldens carry them.)
Class-B *statement* bodies (`let` bindings, early `return`, struct-literal
construction `Self { … }`, named field access, closures) are the honest frontier
beyond the single-tail-expression body; a body carrying them is not modeled and the
TextualRust reader rejects it loudly. Const generic parameters remain excluded
(unwitnessed).

Totality is structural: `CoreItem`'s methods match every variant with no wildcard
arm, so a new item kind is a compile error until its handling is written. An impl
block declares no name, so `CoreItem::name` returns `Option<Identifier>` — the "does
this item have a name?" question dissolves into a normal `None` rather than a
fabricated identifier, and an impl block has no visibility, so `with_visibility`
returns it unchanged.

## Named revisable leans (Tier-1 vocabulary boundary)

Every choice below the psyche rulings is a revisable lean with a stated trigger:

- **The Tier-1 body vocabulary boundary** is drawn at the class-A-and-kin shapes
  above. *Trigger:* a witnessed wire body needs a shape outside it (a statement, a
  struct literal, a binary operator, a closure) — extend the vocabulary then, not
  speculatively.
- **String-literal content is stored data, not an interned name.** A name is
  interned and excluded from content identity (rename-stable); a string literal's
  content is semantic and is hashed as part of the value, so `Expression::StringLiteral`
  carries a `String`. This is the one place a Core value holds owned text, and it is
  literal-value data, not the raw-token-text escape hatch the text-free boundary
  forbids. *Trigger:* if a projection ever needs to intern literal content, revisit.
- **Reference mutability is modeled; `&mut self` is not.** The layout-3 growth added
  `&mut` to `ReferenceType` (a stored `ReferenceMutability` kind) because the
  witnessed `Display::fmt` signature borrows the formatter mutably
  (`&mut std::fmt::Formatter<'_>`) — the trigger fired. The `Receiver` still models
  only `self`/`&self`; a `&mut self` receiver stays unwitnessed. *Trigger:* a
  witnessed Tier-1 body takes `&mut self`.
- **Integer literals are value-plus-representation, never stored text.** An
  `IntegerLiteral` carries a `u128` value and a closed `IntegerRepresentation`
  (`Decimal`, or `Hexadecimal { minimum_digits }` for the zero-padded `0x…` form),
  so `0x0001000000000000` round-trips byte-exact without the Core holding raw token
  text — the stringless boundary holds and the string-literal exception below stays
  the *only* owned-text field. The hexadecimal form is **lowercase** (the goldens'
  digits are `0`/`1`, case-agnostic). *Trigger:* a witnessed literal needs uppercase
  hex, a digit separator, a suffix, or a non-decimal/non-hex radix.
- **Const modules carry consts only.** `Module` models the witnessed `short_header`
  shape — an inline module whose members are `Const`s. A module carrying an enum,
  impl, or other item is a broader growth point (it re-exposes the derive-group
  trailing-comma layout that a context-free `DeriveGroup` does not yet carry
  faithfully). *Trigger:* a witnessed module needs a non-const member — model it
  together with a trailing-comma-faithful `DeriveGroup` then.
- **Slice and lifetime types, in position.** `TypeReference::Slice` (`[&'static str]`)
  and `TypeReference::Lifetime` (the `'_` of `Formatter<'_>`) join `Reference` and
  `ImplTrait` as the signature/const-type and generic-argument shapes — legitimate
  by position, never in wire-data field position. *Trigger:* a witnessed shape needs
  another positioned type kind.
- **Tuple-index field access only.** Named field access (`self.origin_route`) is
  unwitnessed in a fully-Tier-1 body (the impls that use it carry class-B struct
  literals and are rejected whole). *Trigger:* a fully-Tier-1 body accesses a named
  field.
- **Use imports are the brace-group form only.** `Use` models `use <base>::{<group>};`
  — the one shape the wire goldens exercise (the NOTA import). A bare import
  (`use foo::Bar;`), a glob (`use foo::*;`), and an aliased import (`use foo::Bar as
  Baz;`) are unwitnessed growth points the closed shape does not carry. *Trigger:* a
  witnessed generated module needs one of those import shapes.
- **The cfg gate is a `Feature` predicate.** `Attribute::Cfg` reuses
  `ConfigurationPredicate`, whose sole variant is `Feature(Identifier)` — the only
  gate the goldens exercise (`#[cfg(feature = "nota-text")]`). *Trigger:* a witnessed
  gate needs `all`/`any`/`not` or a non-feature key.

## Content identity and layout version across this growth

**The layout is 2, and the correction below records why.** An earlier version of
this document claimed that adding item kinds was "append-only" enum growth under
which "every pre-existing Core value archives to byte-identical bytes and its
content identity does not move," and concluded that `LayoutVersion(1)` should be
kept. **That reasoning was wrong, and the claim was false.** The commit messages
that shipped the growth are history and stand; this document must tell the truth,
so it records the correction here.

The error was reasoning about append-only-ness at the **Rust source level** and
assuming it carried to the **archived byte level**. It does not, because of how
rkyv lays out enums. rkyv archives an enum at a **fixed size equal to its largest
variant** — every `ArchivedCoreItem`, regardless of which variant it holds,
occupies the same footprint. Content identity is blake3 over the **full archived
root**, so that footprint is in the pre-image of every value.

Concretely, on the empirical record:

- Commit `be809429` added the `Function` variant, whose archived form grew
  `ArchivedCoreItem`'s max size from **47 to 101 bytes**. Every `CoreItem` value —
  including shapes untouched at the Rust source level — therefore re-serialized
  larger and its content hash **moved**: a same-shape `Newtype` went from 51 to 105
  archived bytes and its hash moved from `2c26397e…` to `1c8ae182…`. Yet
  `LayoutVersion` stayed `new(1)` and this document claimed identity did not move.
  The claim was false; the layout should have been bumped at that commit.
- Commit `f7dd7d6b` inserted `Attribute::Cfg` at discriminant index 2, **shifting**
  the `ToolPath` and `HelperDerive` tags. That is a discriminant reordering, not
  append-only growth — it moves the archived tag byte of every attribute at or after
  that index. It happened to be **benign for `CoreItem`'s hash only because the
  attribute enum's max variant size did not change**, so `ArchivedCoreItem`'s
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
layout-2 hashes: every CoreLogos content identity it computes changes. A host
survey at the time of this correction confirmed **no durable store or fixture holds
persisted CoreLogos hashes** (everything recomputes, or is tempdir-ephemeral), so
this correction needs no data migration — but the boundary is real, and a consumer
must advance across it only via the deliberate cascade slice, never casually.

### Layout 3: the class-B/C/D kernel extension

**The layout is now 3.** The class-B/C/D kernel extension grew the vocabulary to
cover the goldens' interface-enum ergonomics (constructor and `From` impls, the
cfg-gated `FromStr`/`Display` impls with their associated types and mutable-formatter
signatures), the trace/object-name enums with nested-match `name` methods, and the
class-C stub items (const, const module, associated const). That growth moved the
archived representation in three compounding ways — `CoreItem` gained `Const` and
`Module` (the latter carrying `Vec<CoreItem>`); `ImplBlock` replaced its
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
stored data exactly as visibility is at the item level and the named-field level.
This closes the last class-D gap — the trace goldens declare
`pub struct TraceEvent(pub ObjectName);`, whose `pub` field the layout-3 `Newtype`
could not model, so the declaration was not byte-exact-projectable. `Private`
projects to the empty token stream, so a bare newtype (`CommitSequence(Integer)`)
stores `Private` and projects unchanged — the "no `pub` on the field" special case
dissolves into the normal case. rkyv archives a struct as the concatenation of its
fields, so the new field enlarged every `Newtype` value's archived bytes and, because
`CoreItem` is a fixed-size enum sized to its largest variant, moved every
`CoreItem` value's archived bytes. `LayoutVersion(3)` hashed the class-B/C/D shape;
**`LayoutVersion(4)` hashes the tuple-field-visibility shape**, and
`tests/content_hash_witness.rs` pins the new absolute hash deliberately. Consumers
(the signal-sema-storage seam) cross this hash boundary and must re-converge across
layout 4 only via the deliberate cascade slice, never casually.

## Release-train status

The crate **git-pins** its published dependencies (`content-identity`,
`name-table`, and the `core-schema` dev-dependency) at exact revisions — the green
path. It rides the multi-repository release train pending this session's
delta-audit; the git pins hold the reproducible build meanwhile. Cargo, Nix, and
cache authority stay separate: the lock file carries the revisions, and
`nix flake check` (crane) is the gate.

## Layout

One concern per file under `src/`: the closed `item` algebra and its content
identity; one file per leaf family (`attribute`, `type_reference`, `generics`,
`path`, `visibility`, `field`); one file per item kind (`newtype`, `structure`,
`enumeration`, `alias`); the `domain` marker and the crate `error`. Tests live
under `tests/`, one file per behavior, with the golden-pair fixtures in
`tests/support`.
