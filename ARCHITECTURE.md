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
`Contextual` hash domain tagged with `LayoutVersion(1)`. The pre-image is the
value's canonical portable-archive bytes; the NameTable is excluded (it is not part
of a Core value). Two invariants follow and are tested:

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
the leaf vocabulary (attributes, visibility, paths, generics by kind) — **and now
`ImplBlock` and `Function`** (the ontology's `FreeMethod`, modeled as one node that
serves both an impl member and a free function) **and `Use`** (the `use`-import
shape at the head of every generated module).

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
- a method call — `payload.into()`, `self.0.name()`;
- a string literal — `"SignalInputRecord"`;
- a `match` over a scrutinee whose arms map a variant pattern (a unit-like path
  `InputRoute::Record`, or a tuple variant `Self::Record(_)` / `Self::Input(route)`)
  to a body expression (a unit path, a string literal, or a nested match).

Function bodies are a **single tail expression** — the witnessed Tier-1 bodies carry
no statements, so no `let`/`return` statement vocabulary is modeled. Matches are
exhaustive with no wildcard arm; the whole vocabulary is closed and dispatches on
node kind, never on a head string.

`TraitDefinition` remains **left out**: its method signatures are in-subset shape
but its associated types and default bodies would need more than the Tier-1
vocabulary. Class-B bodies (`let` bindings, early `return`, struct-literal
construction `Self { … }`, named field access, `&mut`, closures) are the honest
frontier beyond Tier-1; a body carrying them is not modeled and the TextualRust
reader rejects it loudly. Const generic parameters remain excluded (unwitnessed).

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
- **Shared references only.** `&mut` (exclusive borrow, `&mut self`) is unwitnessed
  in Tier-1 bodies, so `ReferenceType` and `Receiver` model only the shared form.
  *Trigger:* a witnessed Tier-1 signature borrows mutably.
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

Adding `ImplBlock`, `Function`, the two `TypeReference` variants (`Reference`,
`ImplTrait`), and now the `Use` item kind and the `Attribute::Cfg` variant is
**append-only** enum growth: existing variants keep their rkyv discriminants, so
every pre-existing Core value archives to byte-identical bytes and its content
identity does not move. The new item kinds enter identity
hashing (they are `CoreItem` values under `CoreLogosDomain`), but they are new
content getting first-time hashes under the existing layout — there is no prior
layout-1 hash they conflict with. Because no previously-computed identity changes,
the truthful versioning call is to **keep `LayoutVersion(1)`**: bumping would move
every existing hash for no semantic reason, manufacturing a break. The layout
version protects pre-image *format* compatibility, and the format is unchanged for
all existing values — the algebra grew, as it is designed to.

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
