//! Archive and invariant witnesses for the second vertical vocabulary slice.

use core_logos::{
    WholeLogos, WholeLogosAssociatedTypeBinding, WholeLogosItem, WholeLogosStruct,
    WholeLogosTraitDef, WholeLogosTraitImpl, WholeLogosTraitMethod, WholeLogosTypeApplication,
    WholeLogosTypeReference, WholeLogosVisibility,
};
use encoded_name_table::LocalEncodedId;
use signal_sema_translator::{VocabularyEncodedId, VocabularyRoot};

fn identity(root: VocabularyRoot, chain: &[u16]) -> VocabularyEncodedId {
    VocabularyEncodedId::new(
        root,
        chain.iter().copied().map(LocalEncodedId::new).collect(),
    )
    .expect("complete fixture identity")
}

fn universal(chain: &[u16]) -> VocabularyEncodedId {
    identity(VocabularyRoot::Universal, chain)
}

fn reference(chain: &[u16]) -> WholeLogosTypeReference {
    WholeLogosTypeReference::Identity(universal(chain))
}

#[test]
fn struct_trait_definition_and_trait_impl_round_trip_as_canonical_whole_logos() {
    let vector_entry = WholeLogosTypeReference::Application(WholeLogosTypeApplication::new(
        identity(VocabularyRoot::Rust, &[4]),
        reference(&[30]),
    ));
    let logos = WholeLogos::new(vec![
        WholeLogosItem::Struct(WholeLogosStruct::new(
            WholeLogosVisibility::Public,
            universal(&[10]),
            vec![reference(&[11]), vector_entry],
        )),
        WholeLogosItem::TraitDef(WholeLogosTraitDef::new(
            WholeLogosVisibility::Public,
            universal(&[20]),
            vec![WholeLogosTraitMethod::new(
                universal(&[20, 1]),
                vec![reference(&[30]), reference(&[31])],
                reference(&[32]),
            )],
        )),
        WholeLogosItem::TraitImpl(WholeLogosTraitImpl::new(
            reference(&[40]),
            reference(&[41]),
            vec![WholeLogosAssociatedTypeBinding::new(
                universal(&[40, 1]),
                reference(&[42]),
            )],
        )),
    ]);

    let archive = logos.to_archive_bytes().expect("archive slice-two Logos");
    assert_eq!(
        WholeLogos::from_archive_bytes(&archive).expect("restore slice-two Logos"),
        logos
    );
}

#[test]
fn method_declarations_remain_universal_while_references_may_be_rust_vocabulary() {
    let invalid = WholeLogos::new(vec![WholeLogosItem::TraitDef(WholeLogosTraitDef::new(
        WholeLogosVisibility::Public,
        universal(&[20]),
        vec![WholeLogosTraitMethod::new(
            identity(VocabularyRoot::Rust, &[20, 1]),
            vec![WholeLogosTypeReference::Identity(identity(
                VocabularyRoot::Rust,
                &[30],
            ))],
            reference(&[32]),
        )],
    ))]);

    let archive = invalid
        .to_archive_bytes()
        .expect("archive invalid fixture before invariant load");
    assert!(matches!(
        WholeLogos::from_archive_bytes(&archive),
        Err(core_logos::WholeLogosArchiveError::NonUniversalEncodedId {
            position: core_logos::WholeLogosEncodedIdPosition::MethodName,
            root: VocabularyRoot::Rust,
            ..
        })
    ));
}
