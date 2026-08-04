use capsule_content_identity::{IdentityHasher, PortableArchive};
use core_logos::{
    WholeLogos, WholeLogosContentIdentity, WholeLogosEnumeration, WholeLogosItem,
    WholeLogosNewtype, WholeLogosStorageFingerprint, WholeLogosStreamHandle,
    WholeLogosStreamInitiation, WholeLogosStreamLifecycle, WholeLogosStreamTermination,
    WholeLogosTable, WholeLogosTupleFields, WholeLogosTypeApplication, WholeLogosTypeAttributes,
    WholeLogosTypeReference, WholeLogosVariant, WholeLogosVariantPayload, WholeLogosVisibility,
};
use encoded_name_table::LocalEncodedId;
use signal_sema_translator::{VocabularyEncodedId, VocabularyRoot};

// Version 8 retains strict type-parameter/bound carriers. An intentional
// archive change replaces the preceding versioned content identity.
const WHOLE_LOGOS_ARCHIVE_V8_IDENTITY: [u8; 32] = [
    0xd9, 0xff, 0x22, 0x80, 0xfa, 0xc8, 0xdf, 0x78, 0xdb, 0x37, 0xae, 0x2f, 0xcf, 0x41, 0x34, 0x0f,
    0xb5, 0x1d, 0x86, 0x3b, 0x7b, 0xa7, 0xb9, 0xdb, 0x2b, 0x13, 0x08, 0xec, 0xf9, 0x91, 0xb0, 0x9a,
];

fn encoded_id(root: VocabularyRoot, chain: &[u16]) -> VocabularyEncodedId {
    VocabularyEncodedId::new(
        root,
        chain.iter().copied().map(LocalEncodedId::new).collect(),
    )
    .expect("non-empty fixture encoded ID")
}

fn newtype(name: &[u16], wrapped: &[u16]) -> WholeLogosItem {
    WholeLogosItem::Newtype(WholeLogosNewtype::new(
        WholeLogosVisibility::Public,
        encoded_id(VocabularyRoot::Universal, name),
        WholeLogosVisibility::Private,
        WholeLogosTypeReference::Identity(encoded_id(VocabularyRoot::Universal, wrapped)),
    ))
}

fn stream_lifecycle(event: &[u16]) -> WholeLogosItem {
    let identity = encoded_id(VocabularyRoot::Universal, &[70, 3]);
    WholeLogosItem::StreamLifecycle(WholeLogosStreamLifecycle::new(
        encoded_id(VocabularyRoot::Universal, &[70, 1]),
        WholeLogosStreamInitiation::new(
            encoded_id(VocabularyRoot::Universal, &[70, 2]),
            WholeLogosTypeReference::Identity(encoded_id(VocabularyRoot::Universal, &[71, 1])),
            WholeLogosStreamHandle::new(
                identity.clone(),
                WholeLogosTypeReference::Identity(encoded_id(VocabularyRoot::Universal, event)),
            ),
            encoded_id(VocabularyRoot::Universal, &[70, 4]),
        ),
        WholeLogosStreamTermination::new(
            encoded_id(VocabularyRoot::Universal, &[70, 5]),
            identity,
            encoded_id(VocabularyRoot::Universal, &[70, 6]),
        ),
    ))
}

fn hash(whole: &WholeLogos) -> [u8; 32] {
    *whole
        .content_identity()
        .expect("whole content identity")
        .content_addressed_hash()
        .bytes()
}

#[test]
fn enum_and_application_shapes_retain_every_complete_chain_through_archive() {
    let vector = encoded_id(VocabularyRoot::Rust, &[4]);
    let integer = encoded_id(VocabularyRoot::Rust, &[3]);
    let application = WholeLogosTypeReference::Application(
        WholeLogosTypeApplication::new(
            vector.clone(),
            vec![WholeLogosTypeReference::Identity(integer.clone())],
        )
        .expect("non-empty application"),
    );
    let original = WholeLogos::new(vec![
        WholeLogosItem::Newtype(WholeLogosNewtype::new(
            WholeLogosVisibility::Public,
            encoded_id(VocabularyRoot::Universal, &[8, 5]),
            WholeLogosVisibility::Private,
            application.clone(),
        )),
        WholeLogosItem::Enumeration(WholeLogosEnumeration::new(
            WholeLogosVisibility::Public,
            encoded_id(VocabularyRoot::Universal, &[8, 6]),
            vec![
                WholeLogosVariant::new(
                    encoded_id(VocabularyRoot::Universal, &[8, 6, 1]),
                    WholeLogosVariantPayload::Unit,
                ),
                WholeLogosVariant::new(
                    encoded_id(VocabularyRoot::Universal, &[8, 6, 2]),
                    WholeLogosVariantPayload::Tuple(
                        WholeLogosTupleFields::new(vec![application]).expect("single-field tuple"),
                    ),
                ),
            ],
        )),
    ]);

    let archive = original
        .to_archive_bytes()
        .expect("archive broadened carrier");
    assert_eq!(
        WholeLogos::from_archive_bytes(&archive).expect("restore broadened carrier"),
        original
    );
}

#[test]
fn rust_vocabulary_references_survive_archive_while_declarations_stay_universal() {
    let rust_integer = encoded_id(VocabularyRoot::Rust, &[3]);
    let original = WholeLogos::new(vec![WholeLogosItem::Newtype(WholeLogosNewtype::new(
        WholeLogosVisibility::Public,
        encoded_id(VocabularyRoot::Universal, &[8, 5]),
        WholeLogosVisibility::Private,
        WholeLogosTypeReference::Identity(rust_integer.clone()),
    ))]);
    let archive = original.to_archive_bytes().expect("archive Rust reference");
    assert_eq!(
        WholeLogos::from_archive_bytes(&archive).expect("restore Rust reference"),
        original
    );

    let invalid_declaration =
        WholeLogos::new(vec![WholeLogosItem::Newtype(WholeLogosNewtype::new(
            WholeLogosVisibility::Public,
            encoded_id(VocabularyRoot::Rust, &[8, 5]),
            WholeLogosVisibility::Private,
            WholeLogosTypeReference::Identity(rust_integer),
        ))]);
    let invalid_archive = invalid_declaration
        .to_archive_bytes()
        .expect("archive invalid declaration fixture");
    assert!(matches!(
        WholeLogos::from_archive_bytes(&invalid_archive),
        Err(core_logos::WholeLogosArchiveError::NonUniversalEncodedId {
            position: core_logos::WholeLogosEncodedIdPosition::ItemName,
            root: VocabularyRoot::Rust,
            ..
        })
    ));
}

#[test]
fn ordered_typed_items_and_complete_encoded_id_chains_survive_archive() {
    let original = WholeLogos::new(vec![newtype(&[17, 23, 41], &[3, 5])]);
    let bytes = original.to_archive_bytes().expect("archive whole Logos");
    let restored = WholeLogos::from_archive_bytes(&bytes).expect("restore whole Logos");

    assert_eq!(restored, original);
    let WholeLogosItem::Newtype(newtype) = &restored.items()[0] else {
        panic!("newtype fixture")
    };
    assert_eq!(
        newtype
            .name()
            .chain()
            .iter()
            .map(|local| local.value())
            .collect::<Vec<_>>(),
        vec![17, 23, 41],
    );
    assert_eq!(
        match newtype.wrapped() {
            WholeLogosTypeReference::Identity(identity) => identity,
            WholeLogosTypeReference::Parameter(_) => panic!("parameter reference"),
            WholeLogosTypeReference::Application(_) => panic!("identity reference"),
        }
        .chain()
        .iter()
        .map(|local| local.value())
        .collect::<Vec<_>>(),
        vec![3, 5],
    );
    assert_eq!(newtype.name().root_variant(), &VocabularyRoot::Universal);
    assert_eq!(
        match newtype.wrapped() {
            WholeLogosTypeReference::Identity(identity) => identity.root_variant(),
            WholeLogosTypeReference::Parameter(_) => panic!("parameter reference"),
            WholeLogosTypeReference::Application(_) => panic!("identity reference"),
        },
        &VocabularyRoot::Universal
    );
}

#[test]
fn item_order_is_part_of_whole_content_identity() {
    let first = newtype(&[1, 1], &[7]);
    let second = newtype(&[1, 2], &[7]);

    let in_source_order = WholeLogos::new(vec![first.clone(), second.clone()]);
    let reversed = WholeLogos::new(vec![second, first]);

    assert_ne!(hash(&in_source_order), hash(&reversed));
}

#[test]
fn a_behavior_affecting_item_mutation_moves_whole_content_identity() {
    let integer = WholeLogos::new(vec![newtype(&[1, 1], &[3])]);
    let boolean = WholeLogos::new(vec![newtype(&[1, 1], &[4])]);

    assert_ne!(hash(&integer), hash(&boolean));
}

#[test]
fn stream_lifecycle_archives_typed_direct_success_and_separate_termination() {
    let original = WholeLogos::new(vec![stream_lifecycle(&[72, 1])]);
    let archive = original
        .to_archive_bytes()
        .expect("archive resolved stream lifecycle");
    let restored =
        WholeLogos::from_archive_bytes(&archive).expect("restore resolved stream lifecycle");

    assert_eq!(restored, original);
    let WholeLogosItem::StreamLifecycle(lifecycle) = &restored.items()[0] else {
        panic!("stream lifecycle fixture")
    };
    assert_eq!(
        lifecycle.initiation().success().identity(),
        lifecycle.termination().identity()
    );
    assert_eq!(
        lifecycle.initiation().success().event(),
        &WholeLogosTypeReference::Identity(encoded_id(VocabularyRoot::Universal, &[72, 1]))
    );
}

#[test]
fn stream_event_type_is_part_of_whole_logos_content_identity() {
    let first_event = WholeLogos::new(vec![stream_lifecycle(&[72, 1])]);
    let second_event = WholeLogos::new(vec![stream_lifecycle(&[72, 2])]);

    assert_ne!(hash(&first_event), hash(&second_event));
}

#[test]
fn the_whole_logos_variant_is_outside_the_pure_content_hash() {
    let whole = WholeLogos::new(vec![newtype(&[1, 9], &[3])]);
    let canonical_bytes = whole.to_archive_bytes().expect("canonical whole content");
    let mut oracle = IdentityHasher::unprimed();
    oracle.update_length_prefixed(&canonical_bytes);
    let expected = oracle.finalize_bytes();

    let identity = whole.content_identity().expect("whole content identity");
    assert!(matches!(identity, WholeLogosContentIdentity::WholeLogos(_)));
    assert_eq!(identity.content_addressed_hash().bytes(), &expected);
    assert_eq!(expected, WHOLE_LOGOS_ARCHIVE_V8_IDENTITY);
}

#[test]
fn picked_up_type_parameters_retain_names_and_bounds_through_archive() {
    let ordered = encoded_id(VocabularyRoot::Universal, &[60]);
    let result = encoded_id(VocabularyRoot::Rust, &[61]);
    let error = encoded_id(VocabularyRoot::Universal, &[62]);
    let wrapped = WholeLogosTypeReference::Application(
        WholeLogosTypeApplication::new(
            result,
            vec![
                WholeLogosTypeReference::Parameter(ordered.clone()),
                WholeLogosTypeReference::Identity(error),
            ],
        )
        .expect("non-empty Result application"),
    );
    let original = WholeLogos::new(vec![WholeLogosItem::Newtype(
        WholeLogosNewtype::new(
            WholeLogosVisibility::Public,
            encoded_id(VocabularyRoot::Universal, &[63]),
            WholeLogosVisibility::Private,
            wrapped,
        )
        .with_type_parameters(vec![core_logos::WholeLogosTypeParameter::new(
            ordered.clone(),
            ordered,
        )]),
    )]);

    let archive = original
        .to_archive_bytes()
        .expect("archive parameterized newtype");
    assert_eq!(
        WholeLogos::from_archive_bytes(&archive).expect("restore parameterized newtype"),
        original
    );
}

#[test]
fn nested_nary_type_applications_retain_argument_order_through_archive() {
    let result = encoded_id(VocabularyRoot::Rust, &[50]);
    let vector = encoded_id(VocabularyRoot::Rust, &[51]);
    let ordered = encoded_id(VocabularyRoot::Universal, &[52]);
    let error = encoded_id(VocabularyRoot::Universal, &[53]);
    let wrapped = WholeLogosTypeReference::Application(
        WholeLogosTypeApplication::new(
            result,
            vec![
                WholeLogosTypeReference::Application(
                    WholeLogosTypeApplication::new(
                        vector,
                        vec![WholeLogosTypeReference::Identity(ordered)],
                    )
                    .expect("nested Vector application"),
                ),
                WholeLogosTypeReference::Identity(error),
            ],
        )
        .expect("n-ary Result application"),
    );
    let original = WholeLogos::new(vec![WholeLogosItem::Newtype(WholeLogosNewtype::new(
        WholeLogosVisibility::Public,
        encoded_id(VocabularyRoot::Universal, &[54]),
        WholeLogosVisibility::Private,
        wrapped,
    ))]);

    let archive = original
        .to_archive_bytes()
        .expect("archive n-ary application");
    assert_eq!(
        WholeLogos::from_archive_bytes(&archive).expect("restore n-ary application"),
        original
    );
}

#[test]
fn type_applications_refuse_empty_argument_lists() {
    assert_eq!(
        WholeLogosTypeApplication::new(encoded_id(VocabularyRoot::Rust, &[55]), vec![]),
        Err(core_logos::EmptyTypeArguments)
    );
}

#[test]
fn tuple_variants_refuse_zero_and_multiple_fields_without_rewriting_payloads() {
    assert_eq!(
        WholeLogosTupleFields::new(Vec::new())
            .expect_err("unit payload has its own variant")
            .found(),
        0,
    );
    let error = WholeLogosTupleFields::new(vec![
        WholeLogosTypeReference::Identity(encoded_id(VocabularyRoot::Universal, &[1])),
        WholeLogosTypeReference::Identity(encoded_id(VocabularyRoot::Universal, &[2])),
    ])
    .expect_err("multi-field data requires a named struct payload");
    assert_eq!(error.found(), 2);
}

#[test]
fn type_attribute_policy_is_canonical_content() {
    let plain = newtype(&[1, 1], &[7]);
    let WholeLogosItem::Newtype(newtype) = plain.clone() else {
        panic!("newtype fixture")
    };
    let wire = WholeLogosItem::Newtype(newtype.with_attributes(WholeLogosTypeAttributes::Wire));

    assert_ne!(
        hash(&WholeLogos::new(vec![plain])),
        hash(&WholeLogos::new(vec![wire]))
    );
}

#[test]
fn sema_table_record_and_key_shape_survive_archive_and_move_schema_identity() {
    let name = encoded_id(VocabularyRoot::Universal, &[31]);
    let record = WholeLogosTypeReference::Identity(encoded_id(VocabularyRoot::Universal, &[32]));
    let domain_key =
        WholeLogosTypeReference::Identity(encoded_id(VocabularyRoot::Universal, &[33]));
    let record_storage = WholeLogosStorageFingerprint::new([7; 32]);
    let changed_record_storage = WholeLogosStorageFingerprint::new([8; 32]);
    let key_storage = WholeLogosStorageFingerprint::new([9; 32]);
    let table = WholeLogosTable::new(
        name.clone(),
        record.clone(),
        domain_key.clone(),
        record_storage,
        key_storage,
    );
    let changed = WholeLogosTable::new(
        name,
        record,
        domain_key,
        changed_record_storage,
        key_storage,
    );
    let whole = WholeLogos::new(vec![WholeLogosItem::Table(table.clone())]);

    let archive = whole.to_archive_bytes().expect("archive Sema table");
    assert_eq!(
        WholeLogos::from_archive_bytes(&archive).expect("restore Sema table"),
        whole,
    );
    assert_ne!(
        table.schema_hash().expect("table schema hash"),
        changed.schema_hash().expect("changed table schema hash"),
    );
    assert_eq!(table.record_storage(), record_storage);
    assert_eq!(table.key_storage(), key_storage);
}

#[test]
fn malformed_archives_are_refused_before_a_carrier_is_returned() {
    let whole = WholeLogos::new(vec![newtype(&[1, 9], &[3])]);
    let bytes = whole.to_archive_bytes().expect("archive whole content");

    assert!(WholeLogos::from_archive_bytes(b"not a whole Logos archive").is_err());
    assert!(WholeLogos::from_archive_bytes(&bytes[..bytes.len() - 1]).is_err());
}

#[test]
fn portable_archive_and_inherent_archive_surfaces_are_identical() {
    let whole = WholeLogos::new(vec![newtype(&[2, 6, 18], &[3])]);
    let direct = whole.to_archive_bytes().expect("inherent archive");
    let shared =
        <WholeLogos as PortableArchive>::to_archive_bytes(&whole).expect("shared portable archive");

    assert_eq!(direct, shared.as_ref());
}
