//! # core-logos
//!
//! The complete, ordered structural carrier for Logos.
//!
//! `WholeLogos` is the only Logos form here. Every named position holds an
//! authority-issued `EncodedName`; Rust spelling is assembled by the sibling
//! textual codec through a sealed name view.

pub mod capsule;
pub mod whole;

pub use capsule::capsule_from_issued_hash;
pub use whole::{
    EmptyTypeArguments, WholeLogos, WholeLogosAssociatedTypeBinding, WholeLogosEnumeration,
    WholeLogosItem, WholeLogosNewtype, WholeLogosSemaTableKey, WholeLogosStorageFingerprint,
    WholeLogosStreamHandle, WholeLogosStreamInitiation, WholeLogosStreamLifecycle,
    WholeLogosStreamTermination, WholeLogosStruct, WholeLogosTable, WholeLogosTraitDef,
    WholeLogosTraitImpl, WholeLogosTraitMethod, WholeLogosTupleFields, WholeLogosTupleFieldsError,
    WholeLogosTypeApplication, WholeLogosTypeAttributes, WholeLogosTypeParameter,
    WholeLogosTypeReference, WholeLogosVariant, WholeLogosVariantPayload, WholeLogosVisibility,
};
