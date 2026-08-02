//! The ordered whole-Logos carrier used by the vertical language slices.
//!
//! This is deliberately separate from the legacy per-item algebra. Every name
//! position carries a complete production encoded-ID chain, while the supported
//! item vocabulary covers type declarations, behavior traits, and associated-type
//! trait implementations without carrying textual Rust spellings.

use capsule_content_identity::{
    ArchiveError, ContentAddressedHash, IdentityHasher, PortableArchive,
};
use signal_sema_translator::{VocabularyEncodedId, VocabularyRoot};

/// Ordered, canonical whole-Logos content.
///
/// Item order is semantic and is therefore retained in the portable archive and
/// in [`content_identity`](Self::content_identity). This value contains no
/// complete NameTree pin and is not a Capsule.
#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct WholeLogos(Vec<WholeLogosItem>);

impl structural_codec::EncodedForm for WholeLogos {
    type VocabularyRoot = VocabularyRoot;
    type Language = protos::Logos;
}

impl WholeLogos {
    /// Construct whole content in semantic item order.
    pub fn new(items: Vec<WholeLogosItem>) -> Self {
        Self(items)
    }

    /// The items in semantic order.
    pub fn items(&self) -> &[WholeLogosItem] {
        &self.0
    }

    /// Consume the carrier without changing item order.
    pub fn into_items(self) -> Vec<WholeLogosItem> {
        self.0
    }

    /// Derive the Whole-Logos content identity from the canonical archive of
    /// the complete ordered carrier.
    ///
    /// The outer Whole-Logos identity variant is applied after hashing. No
    /// whole-content kind, Capsule kind, hash domain, layout tag, or NameTree pin
    /// is folded into the hash bytes. Item variants remain part of the encoded
    /// content itself, so changing an item's structural kind changes the hash.
    /// This is not a Capsule-identity derivation: Capsule pin composition and the
    /// minted-versus-derived Capsule relationship remain outside this carrier.
    pub fn content_identity(&self) -> Result<WholeLogosContentIdentity, WholeLogosArchiveError> {
        let bytes = <Self as PortableArchive>::to_archive_bytes(self)?;
        let mut hasher = IdentityHasher::unprimed();
        hasher.update_length_prefixed(bytes.as_ref());
        Ok(WholeLogosContentIdentity::WholeLogos(
            ContentAddressedHash::from_bytes(hasher.finalize_bytes()),
        ))
    }

    /// Serialize the whole carrier using the shared portable archive discipline.
    pub fn to_archive_bytes(&self) -> Result<Vec<u8>, WholeLogosArchiveError> {
        Ok(<Self as PortableArchive>::to_archive_bytes(self)?
            .as_ref()
            .to_vec())
    }

    /// Restore a whole carrier after rkyv validation and encoded-ID-chain
    /// invariant checks.
    pub fn from_archive_bytes(bytes: &[u8]) -> Result<Self, WholeLogosArchiveError> {
        let restored = <Self as PortableArchive>::from_archive_bytes(bytes)?;
        restored.validate()?;
        Ok(restored)
    }

    fn validate(&self) -> Result<(), WholeLogosArchiveError> {
        for (item_index, item) in self.0.iter().enumerate() {
            match item {
                WholeLogosItem::Newtype(newtype) => {
                    validate_universal_declaration(
                        item_index,
                        WholeLogosEncodedIdPosition::ItemName,
                        newtype.name(),
                    )?;
                    validate_reference(
                        item_index,
                        WholeLogosEncodedIdPosition::NewtypeField,
                        newtype.wrapped(),
                    )?;
                }
                WholeLogosItem::Enumeration(enumeration) => {
                    validate_universal_declaration(
                        item_index,
                        WholeLogosEncodedIdPosition::ItemName,
                        enumeration.name(),
                    )?;
                    for variant in enumeration.variants() {
                        validate_universal_declaration(
                            item_index,
                            WholeLogosEncodedIdPosition::VariantName,
                            variant.name(),
                        )?;
                        if let WholeLogosVariantPayload::Tuple(fields) = variant.payload() {
                            for field in fields.fields() {
                                validate_reference(
                                    item_index,
                                    WholeLogosEncodedIdPosition::VariantField,
                                    field,
                                )?;
                            }
                        }
                    }
                }
                WholeLogosItem::Struct(structure) => {
                    validate_universal_declaration(
                        item_index,
                        WholeLogosEncodedIdPosition::ItemName,
                        structure.name(),
                    )?;
                    for field in structure.fields() {
                        validate_reference(
                            item_index,
                            WholeLogosEncodedIdPosition::StructField,
                            field,
                        )?;
                    }
                }
                WholeLogosItem::TraitDef(trait_definition) => {
                    validate_universal_declaration(
                        item_index,
                        WholeLogosEncodedIdPosition::ItemName,
                        trait_definition.name(),
                    )?;
                    for method in trait_definition.methods() {
                        validate_universal_declaration(
                            item_index,
                            WholeLogosEncodedIdPosition::MethodName,
                            method.name(),
                        )?;
                        for parameter in method.parameters() {
                            validate_reference(
                                item_index,
                                WholeLogosEncodedIdPosition::MethodParameter,
                                parameter,
                            )?;
                        }
                        validate_reference(
                            item_index,
                            WholeLogosEncodedIdPosition::MethodReturn,
                            method.return_type(),
                        )?;
                    }
                }
                WholeLogosItem::TraitImpl(trait_implementation) => {
                    validate_reference(
                        item_index,
                        WholeLogosEncodedIdPosition::ImplementedTrait,
                        trait_implementation.implemented_trait(),
                    )?;
                    validate_reference(
                        item_index,
                        WholeLogosEncodedIdPosition::ImplementingType,
                        trait_implementation.implementing_type(),
                    )?;
                    for binding in trait_implementation.associated_type_bindings() {
                        validate_universal_declaration(
                            item_index,
                            WholeLogosEncodedIdPosition::AssociatedTypeName,
                            binding.name(),
                        )?;
                        validate_reference(
                            item_index,
                            WholeLogosEncodedIdPosition::AssociatedTypeValue,
                            binding.value(),
                        )?;
                    }
                }
            }
        }
        Ok(())
    }
}

fn validate_universal_declaration(
    item_index: usize,
    position: WholeLogosEncodedIdPosition,
    encoded_id: &VocabularyEncodedId,
) -> Result<(), WholeLogosArchiveError> {
    if encoded_id.chain().is_empty() {
        return Err(WholeLogosArchiveError::EmptyEncodedId {
            item_index,
            position,
        });
    }
    if encoded_id.root_variant() != &VocabularyRoot::Universal {
        return Err(WholeLogosArchiveError::NonUniversalEncodedId {
            item_index,
            position,
            root: *encoded_id.root_variant(),
        });
    }
    Ok(())
}

fn validate_reference(
    item_index: usize,
    position: WholeLogosEncodedIdPosition,
    reference: &WholeLogosTypeReference,
) -> Result<(), WholeLogosArchiveError> {
    match reference {
        WholeLogosTypeReference::Identity(encoded_id) => {
            validate_reference_encoded_id(item_index, position, encoded_id)
        }
        WholeLogosTypeReference::Application(application) => {
            validate_reference_encoded_id(
                item_index,
                WholeLogosEncodedIdPosition::ApplicationHead,
                application.head(),
            )?;
            validate_reference(item_index, position, application.payload())
        }
    }
}

fn validate_reference_encoded_id(
    item_index: usize,
    position: WholeLogosEncodedIdPosition,
    encoded_id: &VocabularyEncodedId,
) -> Result<(), WholeLogosArchiveError> {
    if encoded_id.chain().is_empty() {
        return Err(WholeLogosArchiveError::EmptyEncodedId {
            item_index,
            position,
        });
    }
    Ok(())
}

/// The closed item vocabulary admitted by [`WholeLogos`].
#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub enum WholeLogosItem {
    /// An attribute-free, non-generic tuple newtype.
    Newtype(WholeLogosNewtype),
    /// A plain, non-generic positional product declaration.
    Struct(WholeLogosStruct),
    /// An attribute-free, non-generic enumeration.
    Enumeration(WholeLogosEnumeration),
    /// A non-generic behavior trait definition.
    TraitDef(WholeLogosTraitDef),
    /// A trait implementation containing associated-type equalities.
    TraitImpl(WholeLogosTraitImpl),
}

/// An attribute-free, non-generic newtype declaration.
///
/// Item visibility, declared encoded ID, wrapped-field visibility, and the
/// referenced type are retained as distinct named roles. Both IDs retain their
/// complete root-fronted chains opaquely; this carrier neither resolves nor
/// rewrites them.
#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct WholeLogosNewtype {
    visibility: WholeLogosVisibility,
    name: VocabularyEncodedId,
    wrapped_visibility: WholeLogosVisibility,
    wrapped: WholeLogosTypeReference,
}

impl WholeLogosNewtype {
    /// Construct one newtype item.
    pub fn new(
        visibility: WholeLogosVisibility,
        name: VocabularyEncodedId,
        wrapped_visibility: WholeLogosVisibility,
        wrapped: WholeLogosTypeReference,
    ) -> Self {
        Self {
            visibility,
            name,
            wrapped_visibility,
            wrapped,
        }
    }

    /// The item visibility.
    pub const fn visibility(&self) -> &WholeLogosVisibility {
        &self.visibility
    }

    /// The declaration's complete encoded-ID chain.
    pub const fn name(&self) -> &VocabularyEncodedId {
        &self.name
    }

    /// The wrapped field's visibility.
    pub const fn wrapped_visibility(&self) -> &WholeLogosVisibility {
        &self.wrapped_visibility
    }

    /// The wrapped type's complete encoded-ID chain.
    pub const fn wrapped(&self) -> &WholeLogosTypeReference {
        &self.wrapped
    }
}

/// A plain positional product declaration.
///
/// Field names are absent because the source language carries field meaning by
/// position. A textual assembly must project stable local field spellings; no
/// output identity is allocated here.
#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct WholeLogosStruct {
    visibility: WholeLogosVisibility,
    name: VocabularyEncodedId,
    fields: Vec<WholeLogosTypeReference>,
}

impl WholeLogosStruct {
    /// Construct one positional product.
    pub fn new(
        visibility: WholeLogosVisibility,
        name: VocabularyEncodedId,
        fields: Vec<WholeLogosTypeReference>,
    ) -> Self {
        Self {
            visibility,
            name,
            fields,
        }
    }

    /// Item visibility.
    pub const fn visibility(&self) -> &WholeLogosVisibility {
        &self.visibility
    }

    /// Complete declaration identity.
    pub const fn name(&self) -> &VocabularyEncodedId {
        &self.name
    }

    /// Positional field types in semantic order.
    pub fn fields(&self) -> &[WholeLogosTypeReference] {
        &self.fields
    }
}

/// Positional type reference carried by Whole Logos.
#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub enum WholeLogosTypeReference {
    /// One complete Universal or language-vocabulary encoded-ID chain.
    Identity(VocabularyEncodedId),
    /// One unary application.
    Application(WholeLogosTypeApplication),
}

/// Application head and its one payload.
#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
#[rkyv(
    serialize_bounds(__S: rkyv::ser::Writer + rkyv::ser::Allocator, __S::Error: rkyv::rancor::Source),
    deserialize_bounds(__D::Error: rkyv::rancor::Source),
    bytecheck(bounds(__C: rkyv::validation::ArchiveContext, __C::Error: rkyv::rancor::Source)),
)]
pub struct WholeLogosTypeApplication {
    head: VocabularyEncodedId,
    #[rkyv(omit_bounds)]
    payload: Box<WholeLogosTypeReference>,
}

impl WholeLogosTypeApplication {
    /// Construct a unary application.
    pub fn new(head: VocabularyEncodedId, payload: WholeLogosTypeReference) -> Self {
        Self {
            head,
            payload: Box::new(payload),
        }
    }

    /// Complete application-head identity.
    pub const fn head(&self) -> &VocabularyEncodedId {
        &self.head
    }

    /// Payload reference.
    pub const fn payload(&self) -> &WholeLogosTypeReference {
        &self.payload
    }
}

/// Attribute-free, non-generic enumeration.
#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct WholeLogosEnumeration {
    visibility: WholeLogosVisibility,
    name: VocabularyEncodedId,
    variants: Vec<WholeLogosVariant>,
}

impl WholeLogosEnumeration {
    /// Construct one enumeration.
    pub fn new(
        visibility: WholeLogosVisibility,
        name: VocabularyEncodedId,
        variants: Vec<WholeLogosVariant>,
    ) -> Self {
        Self {
            visibility,
            name,
            variants,
        }
    }

    /// Item visibility.
    pub const fn visibility(&self) -> &WholeLogosVisibility {
        &self.visibility
    }

    /// Complete declaration identity.
    pub const fn name(&self) -> &VocabularyEncodedId {
        &self.name
    }

    /// Variants in semantic order.
    pub fn variants(&self) -> &[WholeLogosVariant] {
        &self.variants
    }
}

/// A behavior trait definition whose method receivers are implied.
#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct WholeLogosTraitDef {
    visibility: WholeLogosVisibility,
    name: VocabularyEncodedId,
    methods: Vec<WholeLogosTraitMethod>,
}

impl WholeLogosTraitDef {
    /// Construct one trait definition.
    pub fn new(
        visibility: WholeLogosVisibility,
        name: VocabularyEncodedId,
        methods: Vec<WholeLogosTraitMethod>,
    ) -> Self {
        Self {
            visibility,
            name,
            methods,
        }
    }

    /// Item visibility.
    pub const fn visibility(&self) -> &WholeLogosVisibility {
        &self.visibility
    }

    /// Complete trait declaration identity.
    pub const fn name(&self) -> &VocabularyEncodedId {
        &self.name
    }

    /// Method signatures in semantic order.
    pub fn methods(&self) -> &[WholeLogosTraitMethod] {
        &self.methods
    }
}

/// One trait method signature.
///
/// The receiver is not stored: membership in the trait implies it, and Rust
/// assembly emits `&self`. Parameter names are likewise assembly-local because
/// Ethos carries only positional parameter types.
#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct WholeLogosTraitMethod {
    name: VocabularyEncodedId,
    parameters: Vec<WholeLogosTypeReference>,
    return_type: WholeLogosTypeReference,
}

impl WholeLogosTraitMethod {
    /// Construct one receiver-implied signature.
    pub fn new(
        name: VocabularyEncodedId,
        parameters: Vec<WholeLogosTypeReference>,
        return_type: WholeLogosTypeReference,
    ) -> Self {
        Self {
            name,
            parameters,
            return_type,
        }
    }

    /// Complete method identity.
    pub const fn name(&self) -> &VocabularyEncodedId {
        &self.name
    }

    /// Positional parameter types.
    pub fn parameters(&self) -> &[WholeLogosTypeReference] {
        &self.parameters
    }

    /// Explicit last-position return type.
    pub const fn return_type(&self) -> &WholeLogosTypeReference {
        &self.return_type
    }
}

/// A trait implementation with associated-type equalities.
#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct WholeLogosTraitImpl {
    implemented_trait: WholeLogosTypeReference,
    implementing_type: WholeLogosTypeReference,
    associated_type_bindings: Vec<WholeLogosAssociatedTypeBinding>,
}

impl WholeLogosTraitImpl {
    /// Construct one trait implementation.
    pub fn new(
        implemented_trait: WholeLogosTypeReference,
        implementing_type: WholeLogosTypeReference,
        associated_type_bindings: Vec<WholeLogosAssociatedTypeBinding>,
    ) -> Self {
        Self {
            implemented_trait,
            implementing_type,
            associated_type_bindings,
        }
    }

    /// Implemented trait reference.
    pub const fn implemented_trait(&self) -> &WholeLogosTypeReference {
        &self.implemented_trait
    }

    /// Implementing self type.
    pub const fn implementing_type(&self) -> &WholeLogosTypeReference {
        &self.implementing_type
    }

    /// Associated-type equalities in semantic order.
    pub fn associated_type_bindings(&self) -> &[WholeLogosAssociatedTypeBinding] {
        &self.associated_type_bindings
    }
}

/// One associated-type equality inside a trait implementation.
#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct WholeLogosAssociatedTypeBinding {
    name: VocabularyEncodedId,
    value: WholeLogosTypeReference,
}

impl WholeLogosAssociatedTypeBinding {
    /// Construct `type name = value;`.
    pub fn new(name: VocabularyEncodedId, value: WholeLogosTypeReference) -> Self {
        Self { name, value }
    }

    /// Complete associated-type identity.
    pub const fn name(&self) -> &VocabularyEncodedId {
        &self.name
    }

    /// Bound type reference.
    pub const fn value(&self) -> &WholeLogosTypeReference {
        &self.value
    }
}

/// One enumeration variant.
#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct WholeLogosVariant {
    name: VocabularyEncodedId,
    payload: WholeLogosVariantPayload,
}

impl WholeLogosVariant {
    /// Construct one variant.
    pub fn new(name: VocabularyEncodedId, payload: WholeLogosVariantPayload) -> Self {
        Self { name, payload }
    }

    /// Complete declaration identity.
    pub const fn name(&self) -> &VocabularyEncodedId {
        &self.name
    }

    /// Unit or positional tuple payload.
    pub const fn payload(&self) -> &WholeLogosVariantPayload {
        &self.payload
    }
}

/// Closed variant-payload vocabulary.
#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub enum WholeLogosVariantPayload {
    /// Unit variant.
    Unit,
    /// One or more positional tuple fields.
    Tuple(WholeLogosTupleFields),
}

/// Positional tuple fields. No stored field names exist.
#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct WholeLogosTupleFields(Vec<WholeLogosTypeReference>);

impl WholeLogosTupleFields {
    /// Construct a non-empty tuple payload.
    pub fn new(fields: Vec<WholeLogosTypeReference>) -> Result<Self, EmptyWholeLogosTupleFields> {
        if fields.is_empty() {
            Err(EmptyWholeLogosTupleFields)
        } else {
            Ok(Self(fields))
        }
    }

    /// Positional payload fields.
    pub fn fields(&self) -> &[WholeLogosTypeReference] {
        &self.0
    }
}

/// A tuple-payload construction attempted to encode no fields.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("tuple variant payload requires at least one positional field")]
pub struct EmptyWholeLogosTupleFields;

/// Visibility admitted by the attribute-free newtype slice.
///
/// Broader Rust visibility forms remain in the legacy item algebra and do not
/// enter this carrier until a typed full-chain shape needs them.
#[derive(Clone, Copy, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub enum WholeLogosVisibility {
    /// Rust `pub`.
    Public,
    /// No emitted visibility token.
    Private,
}

/// A whole-content identity whose kind exists only in this outer variant.
///
/// This is intentionally not `content_identity::CapsuleIdentity`: no complete
/// NameTree pin participates in this derivation.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    rkyv::Archive,
    rkyv::Deserialize,
    rkyv::Serialize,
)]
pub enum WholeLogosContentIdentity {
    /// Identity of one complete ordered [`WholeLogos`] value.
    WholeLogos(ContentAddressedHash),
}

impl WholeLogosContentIdentity {
    /// The inner pure-content hash.
    pub const fn content_addressed_hash(self) -> ContentAddressedHash {
        match self {
            Self::WholeLogos(hash) => hash,
        }
    }
}

/// Which encoded-ID position failed validation while loading an archive.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WholeLogosEncodedIdPosition {
    /// Item declaration.
    ItemName,
    /// Newtype field reference.
    NewtypeField,
    /// Positional product field reference.
    StructField,
    /// Enumeration variant declaration.
    VariantName,
    /// Enumeration tuple-field reference.
    VariantField,
    /// Trait method declaration.
    MethodName,
    /// Trait method parameter reference.
    MethodParameter,
    /// Trait method return reference.
    MethodReturn,
    /// Implemented trait reference.
    ImplementedTrait,
    /// Implementing self-type reference.
    ImplementingType,
    /// Associated-type declaration.
    AssociatedTypeName,
    /// Associated-type value reference.
    AssociatedTypeValue,
    /// Unary application head.
    ApplicationHead,
}

/// A typed failure while hashing or restoring whole-Logos content.
#[derive(Clone, Debug, thiserror::Error)]
pub enum WholeLogosArchiveError {
    /// Canonical archive serialization or validated reconstruction failed.
    #[error("whole-Logos portable archive failed: {0}")]
    Archive(#[from] ArchiveError),

    /// A stored name position contains the empty chain reserved for table
    /// addresses.
    #[error("whole-Logos item {item_index} has an empty encoded-ID chain at {position:?}")]
    EmptyEncodedId {
        /// Ordered item index.
        item_index: usize,
        /// Positional encoded-ID role.
        position: WholeLogosEncodedIdPosition,
    },

    /// A declaration belongs to language-owned rather than shared vocabulary.
    #[error("whole-Logos item {item_index} declares non-Universal root {root:?} at {position:?}")]
    NonUniversalEncodedId {
        /// Ordered item index.
        item_index: usize,
        /// Positional encoded-ID role.
        position: WholeLogosEncodedIdPosition,
        /// Unexpected vocabulary root.
        root: VocabularyRoot,
    },
}
