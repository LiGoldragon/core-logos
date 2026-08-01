use std::mem::{align_of, size_of};

use core_logos::{
    WholeLogosEnumeration, WholeLogosNewtype, WholeLogosTupleFields, WholeLogosTypeApplication,
    WholeLogosTypeReference, WholeLogosVariant, WholeLogosVariantPayload, WholeLogosVisibility,
};
use encoded_name_table::LocalEncodedId;
use signal_sema_translator::{VocabularyEncodedId, VocabularyRoot};

#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
struct NamedWholeLogosNewtype {
    visibility: WholeLogosVisibility,
    name: VocabularyEncodedId,
    wrapped_visibility: WholeLogosVisibility,
    wrapped: WholeLogosTypeReference,
}

#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
#[rkyv(
    serialize_bounds(__S: rkyv::ser::Writer + rkyv::ser::Allocator, __S::Error: rkyv::rancor::Source),
    deserialize_bounds(__D::Error: rkyv::rancor::Source),
    bytecheck(bounds(__C: rkyv::validation::ArchiveContext, __C::Error: rkyv::rancor::Source)),
)]
struct NamedWholeLogosTypeApplication {
    head: VocabularyEncodedId,
    #[rkyv(omit_bounds)]
    payload: Box<WholeLogosTypeReference>,
}

#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
struct NamedWholeLogosEnumeration {
    visibility: WholeLogosVisibility,
    name: VocabularyEncodedId,
    variants: Vec<WholeLogosVariant>,
}

#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
struct NamedWholeLogosVariant {
    name: VocabularyEncodedId,
    payload: WholeLogosVariantPayload,
}

macro_rules! assert_archive_compatible {
    ($production_type:ty, $named_type:ty, $production:expr, $named:expr) => {{
        let production = $production;
        let named = $named;
        let production_bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&production)
            .expect("archive production tuple carrier");
        let named_bytes =
            rkyv::to_bytes::<rkyv::rancor::Error>(&named).expect("archive named-field mirror");

        assert_eq!(
            production_bytes.as_slice(),
            named_bytes.as_slice(),
            "tuple and named-field carriers must emit identical bytes",
        );
        assert_eq!(
            size_of::<rkyv::Archived<$production_type>>(),
            size_of::<rkyv::Archived<$named_type>>(),
            "archived sizes must match",
        );
        assert_eq!(
            align_of::<rkyv::Archived<$production_type>>(),
            align_of::<rkyv::Archived<$named_type>>(),
            "archived alignments must match",
        );

        let _: &rkyv::Archived<$production_type> =
            rkyv::access::<rkyv::Archived<$production_type>, rkyv::rancor::Error>(&named_bytes)
                .expect("access named bytes through production archived layout");
        let _: &rkyv::Archived<$named_type> =
            rkyv::access::<rkyv::Archived<$named_type>, rkyv::rancor::Error>(&production_bytes)
                .expect("access production bytes through named archived layout");

        let production_from_named =
            rkyv::from_bytes::<$production_type, rkyv::rancor::Error>(&named_bytes)
                .expect("restore production carrier from named bytes");
        let named_from_production =
            rkyv::from_bytes::<$named_type, rkyv::rancor::Error>(&production_bytes)
                .expect("restore named carrier from production bytes");
        assert_eq!(production_from_named, production);
        assert_eq!(named_from_production, named);

        assert_eq!(
            rkyv::to_bytes::<rkyv::rancor::Error>(&production_from_named)
                .expect("reserialize production carrier restored from named bytes")
                .as_slice(),
            named_bytes.as_slice(),
        );
        assert_eq!(
            rkyv::to_bytes::<rkyv::rancor::Error>(&named_from_production)
                .expect("reserialize named carrier restored from production bytes")
                .as_slice(),
            production_bytes.as_slice(),
        );
    }};
}

// Trait exception — the proper trait cannot be determined: this function is an
// entry point whose contract is supplied by Rust's test harness.
#[test]
fn named_fields_preserve_every_whole_logos_tuple_carrier_archive() {
    let encoded_id = |root, chain: &[u16]| {
        VocabularyEncodedId::new(
            root,
            chain.iter().copied().map(LocalEncodedId::new).collect(),
        )
        .expect("nonempty fixture encoded ID")
    };

    let application_head = encoded_id(VocabularyRoot::Rust, &[41, 3]);
    let application_payload =
        WholeLogosTypeReference::Identity(encoded_id(VocabularyRoot::Universal, &[41, 5, 7]));
    assert_archive_compatible!(
        WholeLogosTypeApplication,
        NamedWholeLogosTypeApplication,
        WholeLogosTypeApplication::new(application_head.clone(), application_payload.clone()),
        NamedWholeLogosTypeApplication {
            head: application_head,
            payload: Box::new(application_payload),
        }
    );

    let newtype_name = encoded_id(VocabularyRoot::Universal, &[43, 11]);
    let wrapped = WholeLogosTypeReference::Application(WholeLogosTypeApplication::new(
        encoded_id(VocabularyRoot::Rust, &[43, 13]),
        WholeLogosTypeReference::Identity(encoded_id(VocabularyRoot::Rust, &[43, 17, 19])),
    ));
    assert_archive_compatible!(
        WholeLogosNewtype,
        NamedWholeLogosNewtype,
        WholeLogosNewtype::new(
            WholeLogosVisibility::Public,
            newtype_name.clone(),
            WholeLogosVisibility::Private,
            wrapped.clone(),
        ),
        NamedWholeLogosNewtype {
            visibility: WholeLogosVisibility::Public,
            name: newtype_name,
            wrapped_visibility: WholeLogosVisibility::Private,
            wrapped,
        }
    );

    let variant_name = encoded_id(VocabularyRoot::Universal, &[47, 23, 29]);
    let variant_payload = WholeLogosVariantPayload::Tuple(
        WholeLogosTupleFields::new(vec![
            WholeLogosTypeReference::Identity(encoded_id(VocabularyRoot::Rust, &[47, 31])),
            WholeLogosTypeReference::Application(WholeLogosTypeApplication::new(
                encoded_id(VocabularyRoot::Rust, &[47, 37]),
                WholeLogosTypeReference::Identity(encoded_id(
                    VocabularyRoot::Universal,
                    &[47, 41, 43],
                )),
            )),
        ])
        .expect("nonempty variant payload"),
    );
    assert_archive_compatible!(
        WholeLogosVariant,
        NamedWholeLogosVariant,
        WholeLogosVariant::new(variant_name.clone(), variant_payload.clone()),
        NamedWholeLogosVariant {
            name: variant_name.clone(),
            payload: variant_payload.clone(),
        }
    );

    let variants = vec![
        WholeLogosVariant::new(
            encoded_id(VocabularyRoot::Universal, &[53, 59]),
            WholeLogosVariantPayload::Unit,
        ),
        WholeLogosVariant::new(variant_name, variant_payload),
    ];
    let enumeration_name = encoded_id(VocabularyRoot::Universal, &[53, 61]);
    assert_archive_compatible!(
        WholeLogosEnumeration,
        NamedWholeLogosEnumeration,
        WholeLogosEnumeration::new(
            WholeLogosVisibility::Private,
            enumeration_name.clone(),
            variants.clone(),
        ),
        NamedWholeLogosEnumeration {
            visibility: WholeLogosVisibility::Private,
            name: enumeration_name,
            variants,
        }
    );
}
