//! Cross-version golden-hash witness: an ABSOLUTE content-hash constant for a
//! representative CoreLogos value under the current layout version.
//!
//! This is the witness whose absence let a false stability claim ship. When
//! `core-logos` commit be809429 added the `Function` item kind, rkyv's fixed-size
//! enum layout grew `ArchivedCoreItem` from 47 to 101 bytes, so every value's
//! archived bytes — and therefore its blake3 content identity — moved, while the
//! layout version and the ARCHITECTURE claimed the identity was stable. A pinned
//! absolute hash makes that class of change impossible to ship silently: it fails
//! this test loudly.
//!
//! If this test fails, the archived representation of a CoreLogos value changed.
//! That is a layout event, never a casual edit: bump `CoreLogosDomain`'s
//! `LayoutVersion` in `src/domain.rs`, document why the archived shape moved, and
//! update the constant below DELIBERATELY to the new hash. Do not "fix" the test by
//! pasting the new hash without bumping the layout version — that reproduces the
//! exact defect this witness exists to catch.

mod support;

use core_logos::CoreLogosDomain;
use content_identity::HashDomain;
use name_table::NameTable;

/// The content identity of the `CommitSequence` golden newtype under the current
/// CoreLogos layout, as a lowercase hex blake3 digest. Pinned at layout 2, the
/// version that hashes the shipped archived shape (post-be809429). The value is a
/// deterministic function of the golden fixture: `commit_sequence` interns into a
/// fresh NameTable in a fixed order, so the stored identifier indices — and thus the
/// archived bytes — are reproducible.
const COMMIT_SEQUENCE_IDENTITY_LAYOUT_2: &str =
    "29c313bf0336b204dbdccba9d6b20cc466cc72ed331285530fdeeee630a5277e";

#[test]
fn commit_sequence_identity_is_pinned_under_the_current_layout() {
    // The layout version this witness pins must be the one the domain currently
    // reports. If the domain moved to a new layout, the constant above is stale by
    // definition and must be re-derived deliberately.
    assert_eq!(
        CoreLogosDomain::layout_version().value(),
        2,
        "the witnessed layout version moved; re-derive the pinned hash deliberately",
    );

    let mut names = NameTable::new();
    let item = support::commit_sequence(&mut names);
    let identity = item.content_identity().expect("content identity");

    assert_eq!(
        identity.to_hexadecimal(),
        COMMIT_SEQUENCE_IDENTITY_LAYOUT_2,
        "the archived representation of CommitSequence changed — this is a layout \
         event: bump CoreLogosDomain's LayoutVersion in src/domain.rs, document why \
         the archived shape moved, and update this constant deliberately",
    );
}
