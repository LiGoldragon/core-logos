use core_logos::{
    WholeLogos, WholeLogosAssociatedTypeBinding, WholeLogosItem, WholeLogosStruct,
    WholeLogosTraitDef, WholeLogosTraitImpl, WholeLogosTraitMethod, WholeLogosTypeReference,
    WholeLogosVisibility,
};
use name_table::EncodedName;

fn name(seed: u8) -> EncodedName {
    EncodedName::from_archive_bytes([seed; 16])
}

#[test]
fn structural_and_trait_slices_preserve_opaque_names_through_rkyv() {
    let reference = |seed| WholeLogosTypeReference::Identity(name(seed));
    let logos = WholeLogos::new(vec![
        WholeLogosItem::Struct(WholeLogosStruct::new(
            WholeLogosVisibility::Public,
            name(1),
            vec![reference(2)],
        )),
        WholeLogosItem::TraitDef(WholeLogosTraitDef::new(
            WholeLogosVisibility::Public,
            name(3),
            vec![WholeLogosTraitMethod::new(
                name(4),
                vec![reference(5)],
                reference(6),
            )],
        )),
        WholeLogosItem::TraitImpl(WholeLogosTraitImpl::new(
            reference(7),
            reference(8),
            vec![WholeLogosAssociatedTypeBinding::new(name(9), reference(10))],
        )),
    ]);
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&logos).expect("archive Logos");
    assert_eq!(
        rkyv::from_bytes::<WholeLogos, rkyv::rancor::Error>(&bytes).expect("restore Logos"),
        logos
    );
}
