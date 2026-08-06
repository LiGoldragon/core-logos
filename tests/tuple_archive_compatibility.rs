use std::mem::{align_of, size_of};

use core_logos::{WholeLogosTypeApplication, WholeLogosTypeReference};
use name_table::EncodedName;

#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
#[rkyv(
    serialize_bounds(__S: rkyv::ser::Writer + rkyv::ser::Allocator, __S::Error: rkyv::rancor::Source),
    deserialize_bounds(__D::Error: rkyv::rancor::Source),
    bytecheck(bounds(__C: rkyv::validation::ArchiveContext, __C::Error: rkyv::rancor::Source)),
)]
struct NamedApplication {
    head: EncodedName,
    #[rkyv(omit_bounds)]
    arguments: Vec<WholeLogosTypeReference>,
}

#[test]
fn type_application_keeps_its_frozen_tuple_archive_shape() {
    let head = EncodedName::from_archive_bytes([1; 16]);
    let argument = WholeLogosTypeReference::Identity(EncodedName::from_archive_bytes([2; 16]));
    let production = WholeLogosTypeApplication::new(head, vec![argument.clone()])
        .expect("non-empty application");
    let named = NamedApplication {
        head,
        arguments: vec![argument],
    };

    let production_bytes =
        rkyv::to_bytes::<rkyv::rancor::Error>(&production).expect("archive production tuple");
    let named_bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&named).expect("archive mirror tuple");
    assert_eq!(production_bytes.as_slice(), named_bytes.as_slice());
    assert_eq!(
        size_of::<rkyv::Archived<WholeLogosTypeApplication>>(),
        size_of::<rkyv::Archived<NamedApplication>>()
    );
    assert_eq!(
        align_of::<rkyv::Archived<WholeLogosTypeApplication>>(),
        align_of::<rkyv::Archived<NamedApplication>>()
    );
}
