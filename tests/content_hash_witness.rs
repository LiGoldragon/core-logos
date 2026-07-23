//! Cross-version golden-hash witness: an ABSOLUTE content-hash constant for a
//! representative EncodedLogos value under the current layout version.
//!
//! This is the witness whose absence let a false stability claim ship. When
//! `core-logos` commit be809429 added the `Function` item kind, rkyv's fixed-size
//! enum layout grew `ArchivedEncodedItem` from 47 to 101 bytes, so every value's
//! archived bytes — and therefore its blake3 content identity — moved, while the
//! layout version and the ARCHITECTURE claimed the identity was stable. A pinned
//! absolute hash makes that class of change impossible to ship silently: it fails
//! this test loudly.
//!
//! If this test fails, the archived representation of a EncodedLogos value changed.
//! That is a layout event, never a casual edit: bump `EncodedLogosDomain`'s
//! `LayoutVersion` in `src/domain.rs`, document why the archived shape moved, and
//! update the constant below DELIBERATELY to the new hash. Do not "fix" the test by
//! pasting the new hash without bumping the layout version — that reproduces the
//! exact defect this witness exists to catch.

mod support;

use content_identity::HashDomain;
use core_logos::EncodedLogosDomain;
use name_table::NameTable;

/// The content identity of the `CommitSequence` golden newtype under the current
/// EncodedLogos layout, as a lowercase hex blake3 digest. Layout 7 adopts
/// namespace-variant `Identifier` values with `u16` locals, replacing the former
/// flat identifier representation in every archived EncodedLogos value, and names
/// the canonical EncodedLogos hash context. The golden fixture allocates its names
/// in the Logos namespace in a fixed order, so its archived bytes — and this
/// witness — are reproducible.
const COMMIT_SEQUENCE_IDENTITY_LAYOUT_7: &str =
    "3f2d85f564a74df7962f4e9a110fdab92b1dc1899edd8f418314e254f285e73d";

#[test]
fn commit_sequence_identity_is_pinned_under_the_current_layout() {
    // The layout version this witness pins must be the one the domain currently
    // reports. If the domain moved to a new layout, the constant above is stale by
    // definition and must be re-derived deliberately.
    assert_eq!(
        EncodedLogosDomain::layout_version().value(),
        7,
        "the witnessed layout version moved; re-derive the pinned hash deliberately",
    );

    let mut names = NameTable::new(name_table::IdentifierNamespace::Logos);
    let item = support::commit_sequence(&mut names);
    let identity = item.content_identity().expect("content identity");

    assert_eq!(
        identity.to_hexadecimal(),
        COMMIT_SEQUENCE_IDENTITY_LAYOUT_7,
        "the archived representation of CommitSequence changed — this is a layout \
         event: bump EncodedLogosDomain's LayoutVersion in src/domain.rs, document why \
         the archived shape moved, and update this constant deliberately",
    );
}
