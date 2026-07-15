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
`TraitDefinition`, `ImplBlock`, `FreeMethod`. This crate is scoped to the
**wire-contract data subset the goldens exercise**: `Newtype`, `Struct`,
`Enumeration`, `Alias`, plus the leaf vocabulary (attributes, visibility, paths,
generics by kind).

`TraitDefinition`, `ImplBlock`, and `FreeMethod` are deliberately **left out** of
this slice. They appear in the goldens (`reports/codex-rust-construct-survey.md`
§1–§2: trait definitions, `From` impls, per-variant constructors, `Display`/`Error`
pairs), but their method **bodies** are arbitrary Rust logic the Core does not yet
model as data (the survey's "beyond a data-shape generator" tier; the lowering
report §6.4 "honest frontier"). Their type and trait skeletons are in-subset, but
modeling them without a body vocabulary would be incomplete. A closed enum grows by
design, not by speculation: these variants are added when the TextualRust sibling
and a method-body vocabulary land. Const generic parameters are excluded on the
same basis (the survey did not witness them).

Totality is structural: `CoreItem`'s methods match every variant with no wildcard
arm, so a new item kind is a compile error until its handling is written.

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
