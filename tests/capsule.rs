use capsule_content_identity::ContentAddressedHash;
use protos::{Capsule, CapsuleIdentityVariant, Logos};

#[derive(Debug, Eq, PartialEq)]
struct OpaqueCompleteNameTreePin([u8; 41]);

#[test]
fn caller_values_are_carried_in_a_whole_logos_kind_capsule() {
    let pin_bytes = [0xb8; 41];
    let capsule: Capsule<Logos, OpaqueCompleteNameTreePin> = core_logos::capsule_from_issued_hash(
        ContentAddressedHash::from_bytes([0x42; 32]),
        OpaqueCompleteNameTreePin(pin_bytes),
    );

    assert_eq!(
        capsule.content_identity().variant(),
        CapsuleIdentityVariant::WholeLogos
    );
    assert_eq!(capsule.complete_nametree_pin().0, pin_bytes);
}
