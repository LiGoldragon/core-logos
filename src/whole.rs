//! The ordered whole-Logos carrier used by the vertical language slices.
//!
//! Every name position carries an authority-issued opaque encoded name, while
//! the supported item vocabulary covers type declarations, behavior traits,
//! and associated-type trait implementations without carrying textual Rust
//! spellings.

use name_table::{EncodedName, TrueNamed};

/// Ordered, canonical whole-Logos content.
///
/// Item order is semantic and retained in its direct portable rkyv archive.
/// This value contains no textual projection, name-table pin, or custom
/// whole-document hash.
#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct WholeLogos(Vec<WholeLogosItem>);

impl TrueNamed for WholeLogos {}

impl structural_codec::EncodedForm for WholeLogos {
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
    stream: EncodedName,
    initiation: WholeLogosStreamInitiation,
    termination: WholeLogosStreamTermination,
}

impl WholeLogosStreamLifecycle {
    /// Construct the complete lifecycle for one authored stream declaration.
    pub fn new(
        stream: EncodedName,
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
    pub const fn stream(&self) -> &EncodedName {
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
    input: EncodedName,
    query: WholeLogosTypeReference,
    success: WholeLogosStreamHandle,
    refusal: EncodedName,
}

impl WholeLogosStreamInitiation {
    /// Construct one stream-initiation operation.
    pub fn new(
        input: EncodedName,
        query: WholeLogosTypeReference,
        success: WholeLogosStreamHandle,
        refusal: EncodedName,
    ) -> Self {
        Self {
            input,
            query,
            success,
            refusal,
        }
    }

    /// Generated input identity for the initiation query.
    pub const fn input(&self) -> &EncodedName {
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
    pub const fn refusal(&self) -> &EncodedName {
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
    identity: EncodedName,
    event: WholeLogosTypeReference,
}

impl WholeLogosStreamHandle {
    /// Construct the typed direct-success handle.
    pub fn new(identity: EncodedName, event: WholeLogosTypeReference) -> Self {
        Self { identity, event }
    }

    /// Generated handle identity.
    pub const fn identity(&self) -> &EncodedName {
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
    input: EncodedName,
    identity: EncodedName,
    refusal: EncodedName,
}

impl WholeLogosStreamTermination {
    /// Construct the termination operation over an existing stream handle.
    pub fn new(input: EncodedName, identity: EncodedName, refusal: EncodedName) -> Self {
        Self {
            input,
            identity,
            refusal,
        }
    }

    /// Generated input identity for the termination request.
    pub const fn input(&self) -> &EncodedName {
        &self.input
    }

    /// The same generated typed handle returned by initiation success.
    pub const fn identity(&self) -> &EncodedName {
        &self.identity
    }

    /// Generated refusal identity for unknown or already-closed handles.
    pub const fn refusal(&self) -> &EncodedName {
        &self.refusal
    }
}

/// A non-generic newtype declaration with a typed emission policy.
///
/// Item visibility, declared encoded ID, wrapped-field visibility, and the
/// referenced type are retained as distinct named roles. Both names remain
/// opaque authority-issued references; this carrier neither resolves nor
/// rewrites them.
#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct WholeLogosNewtype {
    attributes: WholeLogosTypeAttributes,
    visibility: WholeLogosVisibility,
    name: EncodedName,
    type_parameters: Vec<WholeLogosTypeParameter>,
    wrapped_visibility: WholeLogosVisibility,
    wrapped: WholeLogosTypeReference,
}

impl WholeLogosNewtype {
    /// Construct one newtype item.
    pub fn new(
        visibility: WholeLogosVisibility,
        name: EncodedName,
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

    /// The declaration's opaque encoded name.
    pub const fn name(&self) -> &EncodedName {
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

    /// The wrapped type's opaque encoded name.
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
    name: EncodedName,
    fields: Vec<WholeLogosTypeReference>,
}

impl WholeLogosStruct {
    /// Construct one positional product.
    pub fn new(
        visibility: WholeLogosVisibility,
        name: EncodedName,
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
    pub const fn name(&self) -> &EncodedName {
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
    archive_identity: EncodedName,
}

impl WholeLogosSemaTableKey {
    /// Carry one declared key identity into Logos without reconstructing it
    /// from a Rust spelling or archive bytes.
    pub fn new(archive_identity: EncodedName) -> Self {
        Self {
            reference: WholeLogosTypeReference::Identity(archive_identity),
            archive_identity,
        }
    }

    /// Generated Rust key type.
    pub const fn reference(&self) -> &WholeLogosTypeReference {
        &self.reference
    }

    /// Exact source-declared archive identity that authorized the key type.
    pub const fn archive_identity(&self) -> &EncodedName {
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
    name: EncodedName,
    record: WholeLogosTypeReference,
    key: WholeLogosSemaTableKey,
    record_storage: WholeLogosStorageFingerprint,
    key_storage: WholeLogosStorageFingerprint,
}

impl WholeLogosTable {
    /// Construct one typed domain table declaration.
    pub fn new(
        name: EncodedName,
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
    pub const fn name(&self) -> &EncodedName {
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
    /// One opaque encoded type reference.
    Identity(EncodedName),
    /// A use of an item-local type parameter, preserving its proper name.
    Parameter(EncodedName),
    /// One non-empty adjacent angle application such as `Result<Vector<T>, E>`.
    Application(WholeLogosTypeApplication),
}

/// An item-local type parameter with a trait-quality bound.
#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct WholeLogosTypeParameter {
    name: EncodedName,
    bound: EncodedName,
}

impl WholeLogosTypeParameter {
    /// Construct one retained parameter and its concept-layer bound.
    pub fn new(name: EncodedName, bound: EncodedName) -> Self {
        Self { name, bound }
    }

    /// Proper parameter name, never Rust-renamed in this carrier.
    pub const fn name(&self) -> &EncodedName {
        &self.name
    }

    /// Authored trait-quality bound.
    pub const fn bound(&self) -> &EncodedName {
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
    head: EncodedName,
    #[rkyv(omit_bounds)]
    arguments: Vec<WholeLogosTypeReference>,
}

impl WholeLogosTypeApplication {
    /// Construct an application with one or more arguments.
    pub fn new(
        head: EncodedName,
        arguments: Vec<WholeLogosTypeReference>,
    ) -> Result<Self, EmptyTypeArguments> {
        if arguments.is_empty() {
            Err(EmptyTypeArguments)
        } else {
            Ok(Self { head, arguments })
        }
    }

    /// Complete application-head identity.
    pub const fn head(&self) -> &EncodedName {
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
    name: EncodedName,
    variants: Vec<WholeLogosVariant>,
}

impl WholeLogosEnumeration {
    /// Construct one enumeration.
    pub fn new(
        visibility: WholeLogosVisibility,
        name: EncodedName,
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
    pub const fn name(&self) -> &EncodedName {
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
    name: EncodedName,
    methods: Vec<WholeLogosTraitMethod>,
}

impl WholeLogosTraitDef {
    /// Construct one trait definition.
    pub fn new(
        visibility: WholeLogosVisibility,
        name: EncodedName,
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
    pub const fn name(&self) -> &EncodedName {
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
    name: EncodedName,
    parameters: Vec<WholeLogosTypeReference>,
    return_type: WholeLogosTypeReference,
}

impl WholeLogosTraitMethod {
    /// Construct one receiver-implied signature.
    pub fn new(
        name: EncodedName,
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
    pub const fn name(&self) -> &EncodedName {
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
    name: EncodedName,
    value: WholeLogosTypeReference,
}

impl WholeLogosAssociatedTypeBinding {
    /// Construct `type name = value;`.
    pub fn new(name: EncodedName, value: WholeLogosTypeReference) -> Self {
        Self { name, value }
    }

    /// Complete associated-type identity.
    pub const fn name(&self) -> &EncodedName {
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
    name: EncodedName,
    payload: WholeLogosVariantPayload,
}

impl WholeLogosVariant {
    /// Construct one variant.
    pub fn new(name: EncodedName, payload: WholeLogosVariantPayload) -> Self {
        Self { name, payload }
    }

    /// Complete declaration identity.
    pub const fn name(&self) -> &EncodedName {
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
/// Broader Rust visibility forms remain outside this carrier until a typed
/// shape needs them.
#[derive(Clone, Copy, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub enum WholeLogosVisibility {
    /// Rust `pub`.
    Public,
    /// No emitted visibility token.
    Private,
}
