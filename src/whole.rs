//! The ordered whole-Logos carrier used by the vertical language slices.
//!
//! Every name position carries a complete production encoded-ID chain, while the supported
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
                    validate_newtype_type_parameters(item_index, newtype)?;
                    validate_reference(
                        item_index,
                        WholeLogosEncodedIdPosition::NewtypeField,
                        newtype.wrapped(),
                    )?;
                    validate_parameter_references(
                        item_index,
                        newtype.wrapped(),
                        newtype.type_parameters(),
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
                            if fields.fields().is_empty() {
                                return Err(WholeLogosArchiveError::InvalidTupleVariantArity {
                                    item_index,
                                    found: fields.fields().len(),
                                });
                            }
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
                WholeLogosItem::Table(table) => {
                    validate_universal_declaration(
                        item_index,
                        WholeLogosEncodedIdPosition::ItemName,
                        table.name(),
                    )?;
                    validate_reference(
                        item_index,
                        WholeLogosEncodedIdPosition::TableRecord,
                        table.record(),
                    )?;
                    validate_reference(
                        item_index,
                        WholeLogosEncodedIdPosition::TableKey,
                        table.key(),
                    )?;
                }
                WholeLogosItem::StreamLifecycle(lifecycle) => {
                    validate_universal_declaration(
                        item_index,
                        WholeLogosEncodedIdPosition::StreamName,
                        lifecycle.stream(),
                    )?;
                    validate_universal_declaration(
                        item_index,
                        WholeLogosEncodedIdPosition::StreamInitiationInput,
                        lifecycle.initiation().input(),
                    )?;
                    validate_reference(
                        item_index,
                        WholeLogosEncodedIdPosition::StreamQuery,
                        lifecycle.initiation().query(),
                    )?;
                    validate_reference(
                        item_index,
                        WholeLogosEncodedIdPosition::StreamEvent,
                        lifecycle.initiation().success().event(),
                    )?;
                    validate_universal_declaration(
                        item_index,
                        WholeLogosEncodedIdPosition::StreamIdentity,
                        lifecycle.initiation().success().identity(),
                    )?;
                    validate_universal_declaration(
                        item_index,
                        WholeLogosEncodedIdPosition::StreamInitiationRefusal,
                        lifecycle.initiation().refusal(),
                    )?;
                    validate_universal_declaration(
                        item_index,
                        WholeLogosEncodedIdPosition::StreamTerminationInput,
                        lifecycle.termination().input(),
                    )?;
                    validate_universal_declaration(
                        item_index,
                        WholeLogosEncodedIdPosition::StreamIdentity,
                        lifecycle.termination().identity(),
                    )?;
                    validate_universal_declaration(
                        item_index,
                        WholeLogosEncodedIdPosition::StreamTerminationRefusal,
                        lifecycle.termination().refusal(),
                    )?;
                    if lifecycle.initiation().success().identity()
                        != lifecycle.termination().identity()
                    {
                        return Err(
                            WholeLogosArchiveError::StreamTerminationIdentityDoesNotMatch {
                                item_index,
                                initiation: lifecycle.initiation().success().identity().clone(),
                                termination: lifecycle.termination().identity().clone(),
                            },
                        );
                    }
                    validate_distinct_stream_lifecycle_ids(item_index, lifecycle)?;
                }
            }
        }
        Ok(())
    }
}

fn validate_distinct_stream_lifecycle_ids(
    item_index: usize,
    lifecycle: &WholeLogosStreamLifecycle,
) -> Result<(), WholeLogosArchiveError> {
    let ids = [
        (WholeLogosEncodedIdPosition::StreamName, lifecycle.stream()),
        (
            WholeLogosEncodedIdPosition::StreamInitiationInput,
            lifecycle.initiation().input(),
        ),
        (
            WholeLogosEncodedIdPosition::StreamIdentity,
            lifecycle.initiation().success().identity(),
        ),
        (
            WholeLogosEncodedIdPosition::StreamInitiationRefusal,
            lifecycle.initiation().refusal(),
        ),
        (
            WholeLogosEncodedIdPosition::StreamTerminationInput,
            lifecycle.termination().input(),
        ),
        (
            WholeLogosEncodedIdPosition::StreamTerminationRefusal,
            lifecycle.termination().refusal(),
        ),
    ];
    for (index, (position, encoded_id)) in ids.iter().enumerate() {
        if let Some((prior_position, _)) = ids[..index]
            .iter()
            .find(|(_, prior_id)| *prior_id == *encoded_id)
        {
            return Err(WholeLogosArchiveError::RepeatedStreamLifecycleId {
                item_index,
                first: *prior_position,
                second: *position,
                encoded_id: (*encoded_id).clone(),
            });
        }
    }
    Ok(())
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
        WholeLogosTypeReference::Parameter(name) => validate_reference_encoded_id(
            item_index,
            WholeLogosEncodedIdPosition::TypeParameterName,
            name,
        ),
        WholeLogosTypeReference::Application(application) => {
            validate_reference_encoded_id(
                item_index,
                WholeLogosEncodedIdPosition::ApplicationHead,
                application.head(),
            )?;
            if application.arguments().is_empty() {
                return Err(WholeLogosArchiveError::EmptyTypeArguments { item_index });
            }
            for argument in application.arguments() {
                validate_reference(item_index, position, argument)?;
            }
            Ok(())
        }
    }
}

fn validate_newtype_type_parameters(
    item_index: usize,
    newtype: &WholeLogosNewtype,
) -> Result<(), WholeLogosArchiveError> {
    for (index, parameter) in newtype.type_parameters().iter().enumerate() {
        validate_reference_encoded_id(
            item_index,
            WholeLogosEncodedIdPosition::TypeParameterName,
            parameter.name(),
        )?;
        validate_reference_encoded_id(
            item_index,
            WholeLogosEncodedIdPosition::TypeParameterBound,
            parameter.bound(),
        )?;
        if newtype.type_parameters()[..index]
            .iter()
            .any(|prior| prior.name() == parameter.name())
        {
            return Err(WholeLogosArchiveError::DuplicateTypeParameterName {
                item_index,
                name: parameter.name().clone(),
            });
        }
    }
    Ok(())
}

fn validate_parameter_references(
    item_index: usize,
    reference: &WholeLogosTypeReference,
    parameters: &[WholeLogosTypeParameter],
) -> Result<(), WholeLogosArchiveError> {
    match reference {
        WholeLogosTypeReference::Identity(_) => Ok(()),
        WholeLogosTypeReference::Parameter(name) => {
            if parameters.iter().any(|parameter| parameter.name() == name) {
                Ok(())
            } else {
                Err(WholeLogosArchiveError::UndeclaredTypeParameter {
                    item_index,
                    name: name.clone(),
                })
            }
        }
        WholeLogosTypeReference::Application(application) => {
            for argument in application.arguments() {
                validate_parameter_references(item_index, argument, parameters)?;
            }
            Ok(())
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
    /// A non-generic tuple newtype.
    Newtype(WholeLogosNewtype),
    /// A non-generic positional product declaration.
    Struct(WholeLogosStruct),
    /// A non-generic enumeration.
    Enumeration(WholeLogosEnumeration),
    /// A non-generic behavior trait definition.
    TraitDef(WholeLogosTraitDef),
    /// A trait implementation containing associated-type equalities.
    TraitImpl(WholeLogosTraitImpl),
    /// A domain-keyed Sema table declaration.
    Table(WholeLogosTable),
    /// One resolved stream lifecycle, from authored initiation through typed
    /// direct success and its separately named termination input/refusal.
    StreamLifecycle(WholeLogosStreamLifecycle),
}

/// A complete, lowered lifecycle for one authored stream declaration.
///
/// The authored declaration has initiation syntax only; termination is implied
/// by that declaration. The generated contract nevertheless contains a
/// separate termination input because consuming a running handle is a distinct
/// runtime action. No registry, queue, allocation, or runtime behavior lives
/// in this archiveable language contract.
#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct WholeLogosStreamLifecycle {
    stream: VocabularyEncodedId,
    initiation: WholeLogosStreamInitiation,
    termination: WholeLogosStreamTermination,
}

impl WholeLogosStreamLifecycle {
    /// Construct the complete lifecycle for one authored stream declaration.
    pub fn new(
        stream: VocabularyEncodedId,
        initiation: WholeLogosStreamInitiation,
        termination: WholeLogosStreamTermination,
    ) -> Self {
        Self {
            stream,
            initiation,
            termination,
        }
    }

    /// Authored stream declaration identity.
    pub const fn stream(&self) -> &VocabularyEncodedId {
        &self.stream
    }

    /// Initiation input, typed query, direct success handle, and refusal.
    pub const fn initiation(&self) -> &WholeLogosStreamInitiation {
        &self.initiation
    }

    /// Separate termination input over that same typed handle and its refusal.
    pub const fn termination(&self) -> &WholeLogosStreamTermination {
        &self.termination
    }
}

/// The fully resolved initiation operation of a stream lifecycle.
///
/// A Rust Logos assembly emits its [`success`](Self::success) as
/// `protos::Stream<Event>`: the event type remains explicit here and the
/// identity is the generated handle's stable typed identity.
#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct WholeLogosStreamInitiation {
    input: VocabularyEncodedId,
    query: WholeLogosTypeReference,
    success: WholeLogosStreamHandle,
    refusal: VocabularyEncodedId,
}

impl WholeLogosStreamInitiation {
    /// Construct one stream-initiation operation.
    pub fn new(
        input: VocabularyEncodedId,
        query: WholeLogosTypeReference,
        success: WholeLogosStreamHandle,
        refusal: VocabularyEncodedId,
    ) -> Self {
        Self {
            input,
            query,
            success,
            refusal,
        }
    }

    /// Generated input identity for the initiation query.
    pub const fn input(&self) -> &VocabularyEncodedId {
        &self.input
    }

    /// Typed query accepted by initiation.
    pub const fn query(&self) -> &WholeLogosTypeReference {
        &self.query
    }

    /// The direct typed success handle.
    pub const fn success(&self) -> &WholeLogosStreamHandle {
        &self.success
    }

    /// Generated refusal identity for an invalid initiation query.
    pub const fn refusal(&self) -> &VocabularyEncodedId {
        &self.refusal
    }
}

/// The typed direct success of stream initiation.
///
/// This is the contract-level representation of `protos::Stream<Event>`; it
/// records the generated handle identity and its event type without assigning
/// a runtime identifier or storing an event queue.
#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct WholeLogosStreamHandle {
    identity: VocabularyEncodedId,
    event: WholeLogosTypeReference,
}

impl WholeLogosStreamHandle {
    /// Construct the typed direct-success handle.
    pub fn new(identity: VocabularyEncodedId, event: WholeLogosTypeReference) -> Self {
        Self { identity, event }
    }

    /// Generated handle identity.
    pub const fn identity(&self) -> &VocabularyEncodedId {
        &self.identity
    }

    /// Event type carried by the generated `protos::Stream<Event>` handle.
    pub const fn event(&self) -> &WholeLogosTypeReference {
        &self.event
    }
}

/// The implied stream-termination operation rendered as explicit generated
/// input/output shape.
#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct WholeLogosStreamTermination {
    input: VocabularyEncodedId,
    identity: VocabularyEncodedId,
    refusal: VocabularyEncodedId,
}

impl WholeLogosStreamTermination {
    /// Construct the termination operation over an existing stream handle.
    pub fn new(
        input: VocabularyEncodedId,
        identity: VocabularyEncodedId,
        refusal: VocabularyEncodedId,
    ) -> Self {
        Self {
            input,
            identity,
            refusal,
        }
    }

    /// Generated input identity for the termination request.
    pub const fn input(&self) -> &VocabularyEncodedId {
        &self.input
    }

    /// The same generated typed handle returned by initiation success.
    pub const fn identity(&self) -> &VocabularyEncodedId {
        &self.identity
    }

    /// Generated refusal identity for unknown or already-closed handles.
    pub const fn refusal(&self) -> &VocabularyEncodedId {
        &self.refusal
    }
}

/// A non-generic newtype declaration with a typed emission policy.
///
/// Item visibility, declared encoded ID, wrapped-field visibility, and the
/// referenced type are retained as distinct named roles. Both IDs retain their
/// complete root-fronted chains opaquely; this carrier neither resolves nor
/// rewrites them.
#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct WholeLogosNewtype {
    attributes: WholeLogosTypeAttributes,
    visibility: WholeLogosVisibility,
    name: VocabularyEncodedId,
    type_parameters: Vec<WholeLogosTypeParameter>,
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
            attributes: WholeLogosTypeAttributes::Plain,
            visibility,
            name,
            type_parameters: Vec::new(),
            wrapped_visibility,
            wrapped,
        }
    }

    /// Select the canonical attribute preamble emitted for this declaration.
    pub const fn with_attributes(mut self, attributes: WholeLogosTypeAttributes) -> Self {
        self.attributes = attributes;
        self
    }

    /// Attach the ordered trait-quality parameters picked up by this item.
    pub fn with_type_parameters(mut self, type_parameters: Vec<WholeLogosTypeParameter>) -> Self {
        self.type_parameters = type_parameters;
        self
    }

    /// Canonical declaration-attribute policy.
    pub const fn attributes(&self) -> WholeLogosTypeAttributes {
        self.attributes
    }

    /// The item visibility.
    pub const fn visibility(&self) -> &WholeLogosVisibility {
        &self.visibility
    }

    /// The declaration's complete encoded-ID chain.
    pub const fn name(&self) -> &VocabularyEncodedId {
        &self.name
    }

    /// Type parameters in first-use order.
    pub fn type_parameters(&self) -> &[WholeLogosTypeParameter] {
        &self.type_parameters
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

/// A positional product declaration with a typed emission policy.
///
/// Field names are absent because the source language carries field meaning by
/// position. A textual assembly must project stable local field spellings; no
/// output identity is allocated here.
#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct WholeLogosStruct {
    attributes: WholeLogosTypeAttributes,
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
            attributes: WholeLogosTypeAttributes::Plain,
            visibility,
            name,
            fields,
        }
    }

    /// Select the canonical attribute preamble emitted for this declaration.
    pub const fn with_attributes(mut self, attributes: WholeLogosTypeAttributes) -> Self {
        self.attributes = attributes;
        self
    }

    /// Canonical declaration-attribute policy.
    pub const fn attributes(&self) -> WholeLogosTypeAttributes {
        self.attributes
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

/// One source-provenanced Sema key archive contract.
///
/// The key reference remains the type exposed to generated Rust, while the
/// separate archive identity proves that Nomos resolved it from a concrete
/// source declaration. Construction accepts the identity once, so the two
/// representations cannot diverge after the Nomos boundary.
#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct WholeLogosSemaTableKey {
    reference: WholeLogosTypeReference,
    archive_identity: VocabularyEncodedId,
}

impl WholeLogosSemaTableKey {
    /// Carry one declared key identity into Logos without reconstructing it
    /// from a Rust spelling or archive bytes.
    pub fn new(archive_identity: VocabularyEncodedId) -> Self {
        Self {
            reference: WholeLogosTypeReference::Identity(archive_identity.clone()),
            archive_identity,
        }
    }

    /// Generated Rust key type.
    pub const fn reference(&self) -> &WholeLogosTypeReference {
        &self.reference
    }

    /// Exact source-declared archive identity that authorized the key type.
    pub const fn archive_identity(&self) -> &VocabularyEncodedId {
        &self.archive_identity
    }
}

/// One Sema table and the exact record/key types that define its stored shape.
///
/// The table name is its stable encoded identity. Its current textual spelling
/// is a NameTree concern resolved only at Rust assembly, while the content hash
/// remains stable across a rename.
#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct WholeLogosTable {
    name: VocabularyEncodedId,
    record: WholeLogosTypeReference,
    key: WholeLogosSemaTableKey,
    record_storage: WholeLogosStorageFingerprint,
    key_storage: WholeLogosStorageFingerprint,
}

impl WholeLogosTable {
    /// Construct one typed domain table declaration.
    pub fn new(
        name: VocabularyEncodedId,
        record: WholeLogosTypeReference,
        key: WholeLogosSemaTableKey,
        record_storage: WholeLogosStorageFingerprint,
        key_storage: WholeLogosStorageFingerprint,
    ) -> Self {
        Self {
            name,
            record,
            key,
            record_storage,
            key_storage,
        }
    }

    /// Stable table/family identity.
    pub const fn name(&self) -> &VocabularyEncodedId {
        &self.name
    }

    /// Stored record type.
    pub const fn record(&self) -> &WholeLogosTypeReference {
        &self.record
    }

    /// Authored key type.
    pub const fn key(&self) -> &WholeLogosTypeReference {
        self.key.reference()
    }

    /// Source-provenanced key archive contract.
    pub const fn key_provenance(&self) -> &WholeLogosSemaTableKey {
        &self.key
    }

    /// Deterministic fingerprint of the record's complete generated storage
    /// shape, including transitive local declarations and explicit external
    /// storage contracts.
    pub const fn record_storage(&self) -> WholeLogosStorageFingerprint {
        self.record_storage
    }

    /// Deterministic fingerprint of the key's complete storage shape.
    pub const fn key_storage(&self) -> WholeLogosStorageFingerprint {
        self.key_storage
    }

    /// Content hash of this exact record/key declaration.
    pub fn schema_hash(&self) -> Result<[u8; 32], ArchiveError> {
        let bytes = <Self as PortableArchive>::to_archive_bytes(self)?;
        let mut hasher = IdentityHasher::unprimed();
        hasher.update_length_prefixed(bytes.as_ref());
        Ok(hasher.finalize_bytes())
    }
}

/// One authoritative storage-shape fingerprint carried into a table schema.
///
/// Locally generated shapes derive this value from their complete structural
/// declaration graph. External types require the owning assembly to supply the
/// corresponding content/ABI contract explicitly; an encoded type name alone
/// is never treated as evidence of byte compatibility.
#[derive(Clone, Copy, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct WholeLogosStorageFingerprint([u8; 32]);

impl WholeLogosStorageFingerprint {
    /// Construct from one complete storage contract fingerprint.
    pub const fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Fingerprint bytes.
    pub const fn bytes(self) -> [u8; 32] {
        self.0
    }
}

/// Positional type reference carried by Whole Logos.
#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub enum WholeLogosTypeReference {
    /// One complete Universal or language-vocabulary encoded-ID chain.
    Identity(VocabularyEncodedId),
    /// A use of an item-local type parameter, preserving its proper name.
    Parameter(VocabularyEncodedId),
    /// One non-empty adjacent angle application such as `Result<Vector<T>, E>`.
    Application(WholeLogosTypeApplication),
}

/// An item-local type parameter with a trait-quality bound.
#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct WholeLogosTypeParameter {
    name: VocabularyEncodedId,
    bound: VocabularyEncodedId,
}

impl WholeLogosTypeParameter {
    /// Construct one retained parameter and its concept-layer bound.
    pub fn new(name: VocabularyEncodedId, bound: VocabularyEncodedId) -> Self {
        Self { name, bound }
    }

    /// Proper parameter name, never Rust-renamed in this carrier.
    pub const fn name(&self) -> &VocabularyEncodedId {
        &self.name
    }

    /// Authored trait-quality bound.
    pub const fn bound(&self) -> &VocabularyEncodedId {
        &self.bound
    }
}

/// One non-empty, ordered generic application.
#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
#[rkyv(
    serialize_bounds(__S: rkyv::ser::Writer + rkyv::ser::Allocator, __S::Error: rkyv::rancor::Source),
    deserialize_bounds(__D::Error: rkyv::rancor::Source),
    bytecheck(bounds(__C: rkyv::validation::ArchiveContext, __C::Error: rkyv::rancor::Source)),
)]
pub struct WholeLogosTypeApplication {
    head: VocabularyEncodedId,
    #[rkyv(omit_bounds)]
    arguments: Vec<WholeLogosTypeReference>,
}

impl WholeLogosTypeApplication {
    /// Construct an application with one or more arguments.
    pub fn new(
        head: VocabularyEncodedId,
        arguments: Vec<WholeLogosTypeReference>,
    ) -> Result<Self, EmptyTypeArguments> {
        if arguments.is_empty() {
            Err(EmptyTypeArguments)
        } else {
            Ok(Self { head, arguments })
        }
    }

    /// Complete application-head identity.
    pub const fn head(&self) -> &VocabularyEncodedId {
        &self.head
    }

    /// Arguments in authored order.
    pub fn arguments(&self) -> &[WholeLogosTypeReference] {
        &self.arguments
    }
}

/// A type application construction attempted to encode no arguments.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("type application requires at least one argument")]
pub struct EmptyTypeArguments;

/// A non-generic enumeration with a typed emission policy.
#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct WholeLogosEnumeration {
    attributes: WholeLogosTypeAttributes,
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
            attributes: WholeLogosTypeAttributes::Plain,
            visibility,
            name,
            variants,
        }
    }

    /// Select the canonical attribute preamble emitted for this declaration.
    pub const fn with_attributes(mut self, attributes: WholeLogosTypeAttributes) -> Self {
        self.attributes = attributes;
        self
    }

    /// Canonical declaration-attribute policy.
    pub const fn attributes(&self) -> WholeLogosTypeAttributes {
        self.attributes
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
    /// One or more positional payload fields.
    Tuple(WholeLogosTupleFields),
}

/// The nonempty positional fields of an enumeration payload.
#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct WholeLogosTupleFields(Vec<WholeLogosTypeReference>);

impl WholeLogosTupleFields {
    /// Construct a nonempty tuple payload.
    pub fn new(fields: Vec<WholeLogosTypeReference>) -> Result<Self, WholeLogosTupleFieldsError> {
        if fields.is_empty() {
            Err(WholeLogosTupleFieldsError {
                found: fields.len(),
            })
        } else {
            Ok(Self(fields))
        }
    }

    /// Positional payload fields.
    pub fn fields(&self) -> &[WholeLogosTypeReference] {
        &self.0
    }
}

/// A tuple payload did not contain any positional fields.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("tuple variant payload requires at least one positional field, found {found}")]
pub struct WholeLogosTupleFieldsError {
    found: usize,
}

impl WholeLogosTupleFieldsError {
    /// Refused positional-field count.
    pub const fn found(self) -> usize {
        self.found
    }
}

/// Canonical attribute policy carried by Whole Logos type declarations.
///
/// `Wire` is the existing Interface `WireAttributes` preamble: rustfmt is
/// skipped, NOTA derives are feature-gated, and the rkyv plus ordinary value
/// derives are emitted. The rkyv 0.8 little-endian, 32-bit pointer, unaligned
/// archive settings remain dependency features of the consuming Rust crate.
#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize,
)]
pub enum WholeLogosTypeAttributes {
    /// No declaration attributes; used by Nexus types.
    #[default]
    Plain,
    /// The canonical Interface wire preamble.
    Wire,
    /// The portable rkyv value preamble for Sema record types.
    Stored,
}

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
    /// Stored record reference of a Sema table.
    TableRecord,
    /// Authored key reference of a Sema table.
    TableKey,
    /// Generic application head.
    ApplicationHead,
    /// Item-local type parameter name.
    TypeParameterName,
    /// Trait-quality bound of an item-local type parameter.
    TypeParameterBound,
    /// Authored stream declaration identity.
    StreamName,
    /// Generated input identity for stream initiation.
    StreamInitiationInput,
    /// Typed initiation-query reference.
    StreamQuery,
    /// Typed stream event reference.
    StreamEvent,
    /// Generated typed-stream handle identity.
    StreamIdentity,
    /// Generated refusal identity for stream initiation.
    StreamInitiationRefusal,
    /// Generated input identity for stream termination.
    StreamTerminationInput,
    /// Generated refusal identity for stream termination.
    StreamTerminationRefusal,
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

    /// An archived tuple variant bypassed the nonempty constructor law.
    #[error("whole-Logos item {item_index} has tuple variant arity {found}; expected at least one")]
    InvalidTupleVariantArity {
        /// Ordered item index.
        item_index: usize,
        /// Archived positional-field count.
        found: usize,
    },

    /// An archived type application bypassed the non-empty constructor law.
    #[error("whole-Logos item {item_index} has a type application with no arguments")]
    EmptyTypeArguments {
        /// Item containing the invalid type application.
        item_index: usize,
    },

    /// An item declared the same type-parameter name twice.
    #[error("whole-Logos item {item_index} repeats type parameter {name:?}")]
    DuplicateTypeParameterName {
        /// Item containing the duplicate.
        item_index: usize,
        /// Repeated parameter name.
        name: VocabularyEncodedId,
    },

    /// A parameter use had no item-local declaration.
    #[error("whole-Logos item {item_index} uses undeclared type parameter {name:?}")]
    UndeclaredTypeParameter {
        /// Item containing the use.
        item_index: usize,
        /// Missing parameter name.
        name: VocabularyEncodedId,
    },

    /// The generated termination operation did not consume the exact typed
    /// handle returned by initiation success.
    #[error(
        "whole-Logos item {item_index} terminates {termination:?}, not initiation handle {initiation:?}"
    )]
    StreamTerminationIdentityDoesNotMatch {
        /// Item containing the lifecycle contract.
        item_index: usize,
        /// Handle identity yielded by initiation success.
        initiation: VocabularyEncodedId,
        /// Handle identity consumed by termination.
        termination: VocabularyEncodedId,
    },

    /// Two distinct generated stream roles reused one identity.
    #[error(
        "whole-Logos item {item_index} reuses {encoded_id:?} for stream roles {first:?} and {second:?}"
    )]
    RepeatedStreamLifecycleId {
        /// Item containing the lifecycle contract.
        item_index: usize,
        /// First semantic role that used the identity.
        first: WholeLogosEncodedIdPosition,
        /// Second semantic role that reused the identity.
        second: WholeLogosEncodedIdPosition,
        /// Reused generated identity.
        encoded_id: VocabularyEncodedId,
    },
}
