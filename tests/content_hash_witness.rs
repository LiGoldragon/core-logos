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
/// EncodedLogos layout, as a lowercase hex blake3 digest. Pinned at layout 6, the
/// version that adds the ordinary-exchange envelope vocabulary to the archived shape
/// (the `Expression` algebra grew the `StructLiteral` node the `into_frame` /
/// `into_reply_frame` bodies construct, enlarging the fixed-size `EncodedItem` archived
/// bytes). `commit_sequence` uses none of the new nodes, so its projection is
/// unchanged; only its archived bytes — and therefore this pinned digest — moved with
/// the layout. The value is a deterministic function of the golden fixture:
/// `commit_sequence` interns into a fresh NameTable in a fixed order, so the stored
/// identifier indices — and thus the archived bytes — are reproducible.
const COMMIT_SEQUENCE_IDENTITY_LAYOUT_6: &str =
    "8553cb87a1be10e2c7634a913aba1c3fe4bf15840b649d5e39fec334a6667301";

#[test]
fn commit_sequence_identity_is_pinned_under_the_current_layout() {
    // The layout version this witness pins must be the one the domain currently
    // reports. If the domain moved to a new layout, the constant above is stale by
    // definition and must be re-derived deliberately.
    assert_eq!(
        EncodedLogosDomain::layout_version().value(),
        6,
        "the witnessed layout version moved; re-derive the pinned hash deliberately",
    );

    let mut names = NameTable::new();
    let item = support::commit_sequence(&mut names);
    let identity = item.content_identity().expect("content identity");

    assert_eq!(
        identity.to_hexadecimal(),
        COMMIT_SEQUENCE_IDENTITY_LAYOUT_6,
        "the archived representation of CommitSequence changed — this is a layout \
         event: bump EncodedLogosDomain's LayoutVersion in src/domain.rs, document why \
         the archived shape moved, and update this constant deliberately",
    );
}
