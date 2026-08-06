//! The kind-fixed whole-Logos Capsule carrier boundary.

use capsule_content_identity::ContentAddressedHash;
use protos::{Capsule, Logos};

/// Carry a caller-issued hash and caller-supplied complete NameTree pin in a
/// whole-Logos-kind Capsule.
///
/// This is a pass-through constructor. It does not create a whole-Logos encoded
/// carrier, derive the hash from complete [`WholeLogos`](crate::WholeLogos) values,
/// compare the hash with component content, inspect or compose the pin, or
/// verify that the pin is complete. Module-table composition and the
/// correspondence between this carrier and Logos content remain unwired.
pub fn capsule_from_issued_hash<CompleteNameTreePin>(
    hash: ContentAddressedHash,
    complete_nametree_pin: CompleteNameTreePin,
) -> Capsule<Logos, CompleteNameTreePin> {
    Capsule::new(hash, complete_nametree_pin)
}
