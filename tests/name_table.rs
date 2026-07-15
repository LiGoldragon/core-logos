//! NameTable continuity: the logos identifier space extends a core-schema table
//! as one continuous append-only space, keeping carried-over indices stable.

mod support;

use core_schema::FixtureFamily;
use name_table::{Identifier, Name, NameTable};

#[test]
fn extending_a_core_schema_table_keeps_existing_indices_stable() {
    let family = FixtureFamily::build();
    let base = family.universe().names();
    let base_len = base.len();
    assert!(
        base_len > 0,
        "the core-schema fixture populates the base table"
    );

    // Snapshot every base identifier's name.
    let base_names: Vec<Name> = (0..base_len)
        .map(|index| base.resolve(Identifier::new(index as u32)).unwrap().clone())
        .collect();

    let mut logos = NameTable::extend_from(base);

    // Every carried-over identifier resolves to the same name in the logos table.
    for (index, name) in base_names.iter().enumerate() {
        assert_eq!(logos.resolve(Identifier::new(index as u32)).unwrap(), name);
    }

    // Re-interning a carried-over name returns its original identifier — stable.
    assert_eq!(logos.intern(base_names[0].clone()), Identifier::new(0));

    // A new logos-only name appends above the schema space — one continuous space.
    let fresh = logos.intern(Name::new("LogosOnlyMarker"));
    assert!(
        fresh.value() as usize >= base_len,
        "new identifiers append above the base table",
    );
}

#[test]
fn a_logos_item_built_over_the_extended_table_is_content_addressable() {
    let family = FixtureFamily::build();
    let mut logos = NameTable::extend_from(family.universe().names());

    let item = support::commit_sequence(&mut logos);

    // A content identity computes over the extended, continuous table.
    item.content_identity()
        .expect("content identity over the extended table");
    assert_eq!(
        logos.resolve(item.name()).unwrap().as_str(),
        "CommitSequence"
    );
}
