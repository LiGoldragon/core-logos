use core_logos::{
    WholeLogos, WholeLogosItem, WholeLogosNewtype, WholeLogosStreamHandle,
    WholeLogosStreamInitiation, WholeLogosStreamLifecycle, WholeLogosStreamTermination,
    WholeLogosTypeReference, WholeLogosVisibility,
};
use name_table::EncodedName;

fn name(seed: u8) -> EncodedName {
    EncodedName::from_archive_bytes([seed; 16])
}

fn round_trip(value: &WholeLogos) -> WholeLogos {
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(value).expect("archive opaque Logos");
    rkyv::from_bytes::<WholeLogos, rkyv::rancor::Error>(&bytes).expect("restore opaque Logos")
}

#[test]
fn whole_logos_is_an_opaque_lowered_carrier() {
    let value = WholeLogos::new(vec![WholeLogosItem::Newtype(WholeLogosNewtype::new(
        WholeLogosVisibility::Public,
        name(1),
        WholeLogosVisibility::Private,
        WholeLogosTypeReference::Identity(name(2)),
    ))]);

    assert_eq!(round_trip(&value), value);
}

#[test]
fn every_stream_role_uses_an_opaque_encoded_name() {
    let handle = name(4);
    let value = WholeLogos::new(vec![WholeLogosItem::StreamLifecycle(
        WholeLogosStreamLifecycle::new(
            name(1),
            WholeLogosStreamInitiation::new(
                name(2),
                WholeLogosTypeReference::Identity(name(3)),
                WholeLogosStreamHandle::new(handle, WholeLogosTypeReference::Identity(name(5))),
                name(6),
            ),
            WholeLogosStreamTermination::new(name(7), handle, name(8)),
        ),
    )]);

    assert_eq!(round_trip(&value), value);
}
