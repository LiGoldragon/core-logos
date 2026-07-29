//! The protos-family structural grammar and landing declarations for Logos.
//!
//! This is the Logos-shaped, plain-NOTA textual form used as the source of a
//! Template(Logos) derivation. It is distinct from the alternate Rust textual
//! form in `rust-logos`.
//!
//! Grammar records and landing declarations are canonical source inputs. Their
//! value-or-future-value counterparts are not stored here; Nomos computes those
//! from this pair.

use raw_discovery::{
    BlockTreeDiscoveryConfiguration, BoundaryDiscoveryConfiguration, BoundaryDiscoveryContext,
    BoundaryDiscoveryContextIdentifier, BoundaryDiscoveryTransition, RawProfile, TriggerIdentifier,
    TriggerSet,
};
use signal_sema_translator::{VocabularyEncodedId, VocabularyRoot};
use structural_codec::{
    AcceptedDecodeForm, AddressedStructuralTable, ApplicationRule, AtomDescriptor,
    BorrowedFieldView, ConstructorCodec, ContextualTextualPolicy, DecodeFormId,
    EncodedConstructorId, EncodedTypeId, FieldRole, FieldVisitor, LandingConstructorDeclaration,
    LandingDeclarationCatalog, LandingFieldDeclaration, LandingShape, LandingTypeDeclaration,
    LanguageDeclaration, LanguageDeclarationError, OrderedSequence, Position, SharedDescriptor,
    StableRoleId, StructuralEntry, StructuralRule, StructuralRuleView,
    StructuralVocabularyIdentity, StructureRecord, TableError, TableIdentityPayload,
    TargetLayoutIdentity, TextualRenderingPolicy, UnaryRule,
};

const PARENTHESIS: TriggerIdentifier = TriggerIdentifier::new(0);
const SQUARE: TriggerIdentifier = TriggerIdentifier::new(1);
const BRACE: TriggerIdentifier = TriggerIdentifier::new(2);
const APPLICATION: TriggerIdentifier = TriggerIdentifier::new(3);
const CARRIER: TriggerIdentifier = TriggerIdentifier::new(4);
const WHITESPACE: TriggerIdentifier = TriggerIdentifier::new(5);
const COMMENT: TriggerIdentifier = TriggerIdentifier::new(6);
const ROOT_CONTEXT: BoundaryDiscoveryContextIdentifier = BoundaryDiscoveryContextIdentifier::new(1);

const FORM: DecodeFormId = DecodeFormId::new(1);
const IDENTITY: u16 = 1;
const SECOND: u16 = 2;
const THIRD: u16 = 3;

/// Complete type identities for the first reviewable Logos language closure.
#[derive(Clone, Debug)]
pub struct LogosLanguageTypeIds {
    pub newtype: VocabularyEncodedId,
    pub enumeration: VocabularyEncodedId,
    pub visibility: VocabularyEncodedId,
    pub attributes: VocabularyEncodedId,
    pub attribute: VocabularyEncodedId,
    pub path: VocabularyEncodedId,
    pub configuration_predicate: VocabularyEncodedId,
    pub derive_group: VocabularyEncodedId,
    pub generics: VocabularyEncodedId,
    pub generic_parameter: VocabularyEncodedId,
    pub type_reference: VocabularyEncodedId,
    pub variant: VocabularyEncodedId,
}

impl LogosLanguageTypeIds {
    fn encoded(&self) -> EncodedTypes {
        EncodedTypes {
            newtype: EncodedTypeId::new(self.newtype.clone()),
            enumeration: EncodedTypeId::new(self.enumeration.clone()),
            visibility: EncodedTypeId::new(self.visibility.clone()),
            attributes: EncodedTypeId::new(self.attributes.clone()),
            attribute: EncodedTypeId::new(self.attribute.clone()),
            path: EncodedTypeId::new(self.path.clone()),
            configuration_predicate: EncodedTypeId::new(self.configuration_predicate.clone()),
            derive_group: EncodedTypeId::new(self.derive_group.clone()),
            generics: EncodedTypeId::new(self.generics.clone()),
            generic_parameter: EncodedTypeId::new(self.generic_parameter.clone()),
            type_reference: EncodedTypeId::new(self.type_reference.clone()),
            variant: EncodedTypeId::new(self.variant.clone()),
        }
    }
}

/// Fixed vocabulary values used by the structural Logos form.
#[derive(Clone, Debug)]
pub struct LogosLanguageWords {
    pub public: VocabularyEncodedId,
    pub private: VocabularyEncodedId,
}

#[derive(Clone, Debug)]
struct EncodedTypes {
    newtype: EncodedTypeId<VocabularyRoot>,
    enumeration: EncodedTypeId<VocabularyRoot>,
    visibility: EncodedTypeId<VocabularyRoot>,
    attributes: EncodedTypeId<VocabularyRoot>,
    attribute: EncodedTypeId<VocabularyRoot>,
    path: EncodedTypeId<VocabularyRoot>,
    configuration_predicate: EncodedTypeId<VocabularyRoot>,
    derive_group: EncodedTypeId<VocabularyRoot>,
    generics: EncodedTypeId<VocabularyRoot>,
    generic_parameter: EncodedTypeId<VocabularyRoot>,
    type_reference: EncodedTypeId<VocabularyRoot>,
    variant: EncodedTypeId<VocabularyRoot>,
}

macro_rules! role {
    ($name:ident, $id:expr) => {
        #[derive(
            rkyv::Archive,
            rkyv::Serialize,
            rkyv::Deserialize,
            Clone,
            Copy,
            Debug,
            Eq,
            Hash,
            Ord,
            PartialEq,
            PartialOrd,
        )]
        pub struct $name;

        impl FieldRole for $name {
            const STABLE_ID: u16 = $id;
        }
    };
}

role!(NewtypeRoot, 100);
role!(NewtypeVisibility, 101);
role!(NewtypeAttributes, 102);
role!(NewtypeName, 103);
role!(NewtypeWrappedVisibility, 104);
role!(NewtypeWrapped, 105);

role!(EnumerationRoot, 110);
role!(EnumerationVisibility, 111);
role!(EnumerationAttributes, 112);
role!(EnumerationName, 113);
role!(EnumerationGenerics, 114);
role!(EnumerationVariantsBody, 115);
role!(EnumerationVariants, 116);

role!(DelimitedRoot, 120);
role!(DelimitedItems, 121);

/// The fixed positional source record for a Logos newtype.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct NewtypeRecord {
    root: Position<NewtypeRoot, VocabularyRoot>,
    visibility: Position<NewtypeVisibility, VocabularyRoot>,
    attributes: Position<NewtypeAttributes, VocabularyRoot>,
    name: Position<NewtypeName, VocabularyRoot>,
    wrapped_visibility: Position<NewtypeWrappedVisibility, VocabularyRoot>,
    wrapped: Position<NewtypeWrapped, VocabularyRoot>,
}

impl NewtypeRecord {
    fn new(types: &EncodedTypes) -> Result<Self, structural_codec::AuthoringError> {
        Ok(Self {
            root: Position::try_new(SharedDescriptor::OrderedSequence(
                OrderedSequence::try_new::<NewtypeVisibility>()?
                    .then::<NewtypeAttributes>()?
                    .then::<NewtypeName>()?
                    .then::<NewtypeWrappedVisibility>()?
                    .then::<NewtypeWrapped>()?,
            ))?,
            visibility: Position::try_new(delegate(&types.visibility))?,
            attributes: Position::try_new(repeated(&types.attribute, 0, None))?,
            name: Position::try_new(SharedDescriptor::Declaration(AtomDescriptor::any_case()))?,
            wrapped_visibility: Position::try_new(delegate(&types.visibility))?,
            wrapped: Position::try_new(delegate(&types.type_reference))?,
        })
    }
}

/// Borrowed field view for [`NewtypeRecord`].
pub struct NewtypeView<'a>(&'a NewtypeRecord);

impl BorrowedFieldView<VocabularyRoot> for NewtypeView<'_> {
    fn expose<Visitor: FieldVisitor<VocabularyRoot>>(&self, visitor: &mut Visitor) {
        visitor.field(&self.0.root);
        visitor.field(&self.0.visibility);
        visitor.field(&self.0.attributes);
        visitor.field(&self.0.name);
        visitor.field(&self.0.wrapped_visibility);
        visitor.field(&self.0.wrapped);
    }
}

impl StructureRecord<VocabularyRoot> for NewtypeRecord {
    type View<'a> = NewtypeView<'a>;

    fn root_role(&self) -> StableRoleId {
        self.root.role()
    }

    fn fields(&self) -> Self::View<'_> {
        NewtypeView(self)
    }
}

/// The fixed positional source record for a Logos enumeration.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct EnumerationRecord {
    root: Position<EnumerationRoot, VocabularyRoot>,
    visibility: Position<EnumerationVisibility, VocabularyRoot>,
    attributes: Position<EnumerationAttributes, VocabularyRoot>,
    name: Position<EnumerationName, VocabularyRoot>,
    generics: Position<EnumerationGenerics, VocabularyRoot>,
    variants_body: Position<EnumerationVariantsBody, VocabularyRoot>,
    variants: Position<EnumerationVariants, VocabularyRoot>,
}

impl EnumerationRecord {
    fn new(types: &EncodedTypes) -> Result<Self, structural_codec::AuthoringError> {
        let variants = Position::try_new(repeated(&types.variant, 0, None))?;
        Ok(Self {
            root: Position::try_new(SharedDescriptor::OrderedSequence(
                OrderedSequence::try_new::<EnumerationVisibility>()?
                    .then::<EnumerationAttributes>()?
                    .then::<EnumerationName>()?
                    .then::<EnumerationGenerics>()?
                    .then::<EnumerationVariantsBody>()?,
            ))?,
            visibility: Position::try_new(delegate(&types.visibility))?,
            attributes: Position::try_new(repeated(&types.attribute, 0, None))?,
            name: Position::try_new(SharedDescriptor::Declaration(AtomDescriptor::any_case()))?,
            generics: Position::try_new(delegate(&types.generics))?,
            variants_body: Position::try_new(SharedDescriptor::Delimited {
                boundary: SQUARE,
                content: variants.role(),
            })?,
            variants,
        })
    }
}

/// Borrowed field view for [`EnumerationRecord`].
pub struct EnumerationView<'a>(&'a EnumerationRecord);

impl BorrowedFieldView<VocabularyRoot> for EnumerationView<'_> {
    fn expose<Visitor: FieldVisitor<VocabularyRoot>>(&self, visitor: &mut Visitor) {
        visitor.field(&self.0.root);
        visitor.field(&self.0.visibility);
        visitor.field(&self.0.attributes);
        visitor.field(&self.0.name);
        visitor.field(&self.0.generics);
        visitor.field(&self.0.variants_body);
        visitor.field(&self.0.variants);
    }
}

impl StructureRecord<VocabularyRoot> for EnumerationRecord {
    type View<'a> = EnumerationView<'a>;

    fn root_role(&self) -> StableRoleId {
        self.root.role()
    }

    fn fields(&self) -> Self::View<'_> {
        EnumerationView(self)
    }
}

/// A delimited sequence used by attributes, derive groups, and generics.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct DelimitedSequenceRecord {
    root: Position<DelimitedRoot, VocabularyRoot>,
    items: Position<DelimitedItems, VocabularyRoot>,
}

impl DelimitedSequenceRecord {
    fn new(
        boundary: TriggerIdentifier,
        item: &EncodedTypeId<VocabularyRoot>,
        minimum: u64,
        maximum: Option<u64>,
    ) -> Result<Self, structural_codec::AuthoringError> {
        let items = Position::try_new(repeated(item, minimum, maximum))?;
        Ok(Self {
            root: Position::try_new(SharedDescriptor::Delimited {
                boundary,
                content: items.role(),
            })?,
            items,
        })
    }
}

/// Borrowed field view for [`DelimitedSequenceRecord`].
pub struct DelimitedSequenceView<'a>(&'a DelimitedSequenceRecord);

impl BorrowedFieldView<VocabularyRoot> for DelimitedSequenceView<'_> {
    fn expose<Visitor: FieldVisitor<VocabularyRoot>>(&self, visitor: &mut Visitor) {
        visitor.field(&self.0.root);
        visitor.field(&self.0.items);
    }
}

impl StructureRecord<VocabularyRoot> for DelimitedSequenceRecord {
    type View<'a> = DelimitedSequenceView<'a>;

    fn root_role(&self) -> StableRoleId {
        self.root.role()
    }

    fn fields(&self) -> Self::View<'_> {
        DelimitedSequenceView(self)
    }
}

/// Every record shape used by the first protos-family Logos grammar.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum LogosRule {
    Newtype(NewtypeRecord),
    Enumeration(EnumerationRecord),
    Delimited(DelimitedSequenceRecord),
    Structural(StructuralRule<VocabularyRoot>),
}

/// Borrowed data-only view of [`LogosRule`].
pub enum LogosRuleView<'a> {
    Newtype(NewtypeView<'a>),
    Enumeration(EnumerationView<'a>),
    Delimited(DelimitedSequenceView<'a>),
    Structural(StructuralRuleView<'a, VocabularyRoot>),
}

impl BorrowedFieldView<VocabularyRoot> for LogosRuleView<'_> {
    fn expose<Visitor: FieldVisitor<VocabularyRoot>>(&self, visitor: &mut Visitor) {
        match self {
            Self::Newtype(view) => view.expose(visitor),
            Self::Enumeration(view) => view.expose(visitor),
            Self::Delimited(view) => view.expose(visitor),
            Self::Structural(view) => view.expose(visitor),
        }
    }
}

impl StructureRecord<VocabularyRoot> for LogosRule {
    type View<'a> = LogosRuleView<'a>;

    fn root_role(&self) -> StableRoleId {
        match self {
            Self::Newtype(record) => record.root_role(),
            Self::Enumeration(record) => record.root_role(),
            Self::Delimited(record) => record.root_role(),
            Self::Structural(record) => record.root_role(),
        }
    }

    fn fields(&self) -> Self::View<'_> {
        match self {
            Self::Newtype(record) => LogosRuleView::Newtype(record.fields()),
            Self::Enumeration(record) => LogosRuleView::Enumeration(record.fields()),
            Self::Delimited(record) => LogosRuleView::Delimited(record.fields()),
            Self::Structural(record) => LogosRuleView::Structural(record.fields()),
        }
    }
}

/// The sealed grammar/declaration pair used as Template(Logos)'s two inputs.
pub struct LogosLanguage {
    types: EncodedTypes,
    grammar: AddressedStructuralTable<VocabularyRoot, LogosRule>,
    landing: LandingDeclarationCatalog<VocabularyRoot>,
}

impl LogosLanguage {
    pub fn seal(
        ids: LogosLanguageTypeIds,
        words: LogosLanguageWords,
    ) -> Result<Self, LogosLanguageError> {
        let types = ids.encoded();
        let profile = RawProfile::standard().seal()?;
        let grammar = seal_grammar(&types, &words, &profile)?;
        let landing = landing_catalog(&types, &words)?;
        let language = Self {
            types,
            grammar,
            landing,
        };
        for root in [
            &language.types.newtype,
            &language.types.enumeration,
            &language.types.attributes,
        ] {
            LanguageDeclaration::new(&language.grammar, &language.landing).verify_root(root)?;
        }
        Ok(language)
    }

    pub const fn grammar(&self) -> &AddressedStructuralTable<VocabularyRoot, LogosRule> {
        &self.grammar
    }

    pub const fn landing(&self) -> &LandingDeclarationCatalog<VocabularyRoot> {
        &self.landing
    }

    pub fn newtype_type(&self) -> &EncodedTypeId<VocabularyRoot> {
        &self.types.newtype
    }

    pub fn enumeration_type(&self) -> &EncodedTypeId<VocabularyRoot> {
        &self.types.enumeration
    }

    pub fn attributes_type(&self) -> &EncodedTypeId<VocabularyRoot> {
        &self.types.attributes
    }
}

fn delegate(target: &EncodedTypeId<VocabularyRoot>) -> SharedDescriptor<VocabularyRoot> {
    SharedDescriptor::Delegate {
        target: target.clone(),
        payload: None,
    }
}

fn repeated(
    target: &EncodedTypeId<VocabularyRoot>,
    minimum: u64,
    maximum: Option<u64>,
) -> SharedDescriptor<VocabularyRoot> {
    SharedDescriptor::Repeated {
        minimum,
        maximum,
        element: Box::new(delegate(target)),
    }
}

fn structural(rule: StructuralRule<VocabularyRoot>) -> LogosRule {
    LogosRule::Structural(rule)
}

fn unary(
    descriptor: SharedDescriptor<VocabularyRoot>,
) -> Result<LogosRule, structural_codec::AuthoringError> {
    Ok(structural(StructuralRule::Unary(UnaryRule::new(
        descriptor,
    )?)))
}

fn application(
    head: SharedDescriptor<VocabularyRoot>,
    payload: SharedDescriptor<VocabularyRoot>,
) -> Result<LogosRule, structural_codec::AuthoringError> {
    Ok(structural(StructuralRule::Application(
        ApplicationRule::new(APPLICATION, head, payload)?,
    )))
}

fn seal_grammar(
    types: &EncodedTypes,
    words: &LogosLanguageWords,
    profile: &raw_discovery::SealedTokenProfile,
) -> Result<AddressedStructuralTable<VocabularyRoot, LogosRule>, LogosLanguageError> {
    let entries = vec![
        entry(
            &types.newtype,
            vec![(IDENTITY, LogosRule::Newtype(NewtypeRecord::new(types)?))],
        ),
        entry(
            &types.enumeration,
            vec![(
                IDENTITY,
                LogosRule::Enumeration(EnumerationRecord::new(types)?),
            )],
        ),
        entry(
            &types.attributes,
            vec![(
                IDENTITY,
                LogosRule::Delimited(DelimitedSequenceRecord::new(
                    SQUARE,
                    &types.attribute,
                    0,
                    None,
                )?),
            )],
        ),
        entry(
            &types.attribute,
            vec![
                (IDENTITY, unary(delegate(&types.derive_group))?),
                (
                    SECOND,
                    application(
                        delegate(&types.configuration_predicate),
                        delegate(&types.derive_group),
                    )?,
                ),
                (
                    THIRD,
                    application(
                        SharedDescriptor::Reference(AtomDescriptor::any_case()),
                        delegate(&types.path),
                    )?,
                ),
            ],
        ),
        entry(
            &types.path,
            vec![
                (
                    IDENTITY,
                    unary(SharedDescriptor::Reference(AtomDescriptor::any_case()))?,
                ),
                (
                    SECOND,
                    application(
                        SharedDescriptor::Reference(AtomDescriptor::any_case()),
                        delegate(&types.path),
                    )?,
                ),
            ],
        ),
        entry(
            &types.configuration_predicate,
            vec![(
                IDENTITY,
                unary(SharedDescriptor::Carrier {
                    carrier: CARRIER,
                    content: Box::new(SharedDescriptor::Reference(AtomDescriptor::any_case())),
                })?,
            )],
        ),
        entry(
            &types.derive_group,
            vec![(
                IDENTITY,
                LogosRule::Delimited(DelimitedSequenceRecord::new(SQUARE, &types.path, 0, None)?),
            )],
        ),
        entry(
            &types.generics,
            vec![(
                IDENTITY,
                LogosRule::Delimited(DelimitedSequenceRecord::new(
                    PARENTHESIS,
                    &types.generic_parameter,
                    0,
                    None,
                )?),
            )],
        ),
        entry(
            &types.generic_parameter,
            vec![(
                IDENTITY,
                unary(SharedDescriptor::Declaration(AtomDescriptor::any_case()))?,
            )],
        ),
        entry(
            &types.type_reference,
            vec![(IDENTITY, unary(delegate(&types.path))?)],
        ),
        entry(
            &types.variant,
            vec![(
                IDENTITY,
                unary(SharedDescriptor::Declaration(AtomDescriptor::any_case()))?,
            )],
        ),
        entry(
            &types.visibility,
            vec![
                (
                    IDENTITY,
                    unary(SharedDescriptor::Literal(words.public.clone()))?,
                ),
                (
                    SECOND,
                    unary(SharedDescriptor::Literal(words.private.clone()))?,
                ),
            ],
        ),
    ];
    Ok(AddressedStructuralTable::seal(
        TableIdentityPayload::new(
            TargetLayoutIdentity::derive(b"core-logos protos landing declarations v1"),
            profile.identity(),
            StructuralVocabularyIdentity::language(b"core-logos protos structural grammar v1"),
            discovery(),
            TextualRenderingPolicy::new(vec![ContextualTextualPolicy::new(
                ROOT_CONTEXT,
                Some(WHITESPACE),
                Some(CARRIER),
            )]),
            entries,
        ),
        profile,
    )?)
}

fn entry(
    encoded_type: &EncodedTypeId<VocabularyRoot>,
    rules: Vec<(u16, LogosRule)>,
) -> StructuralEntry<VocabularyRoot, LogosRule> {
    StructuralEntry::new(
        encoded_type.clone(),
        rules
            .into_iter()
            .map(|(local, rule)| {
                ConstructorCodec::new(
                    EncodedConstructorId::under(encoded_type, local),
                    vec![AcceptedDecodeForm::new(FORM, rule.clone())],
                    rule,
                )
            })
            .collect(),
    )
}

fn landing_catalog(
    types: &EncodedTypes,
    words: &LogosLanguageWords,
) -> Result<LandingDeclarationCatalog<VocabularyRoot>, LogosLanguageError> {
    let declarations = vec![
        one_landing(
            &types.newtype,
            vec![
                LandingFieldDeclaration::for_role::<NewtypeVisibility>(LandingShape::Type(
                    types.visibility.clone(),
                )),
                LandingFieldDeclaration::for_role::<NewtypeAttributes>(LandingShape::sequence(
                    0,
                    None,
                    LandingShape::Type(types.attribute.clone()),
                )),
                LandingFieldDeclaration::for_role::<NewtypeName>(LandingShape::Declaration),
                LandingFieldDeclaration::for_role::<NewtypeWrappedVisibility>(LandingShape::Type(
                    types.visibility.clone(),
                )),
                LandingFieldDeclaration::for_role::<NewtypeWrapped>(LandingShape::Type(
                    types.type_reference.clone(),
                )),
            ],
        ),
        one_landing(
            &types.enumeration,
            vec![
                LandingFieldDeclaration::for_role::<EnumerationVisibility>(LandingShape::Type(
                    types.visibility.clone(),
                )),
                LandingFieldDeclaration::for_role::<EnumerationAttributes>(LandingShape::sequence(
                    0,
                    None,
                    LandingShape::Type(types.attribute.clone()),
                )),
                LandingFieldDeclaration::for_role::<EnumerationName>(LandingShape::Declaration),
                LandingFieldDeclaration::for_role::<EnumerationGenerics>(LandingShape::Type(
                    types.generics.clone(),
                )),
                LandingFieldDeclaration::for_role::<EnumerationVariants>(LandingShape::sequence(
                    0,
                    None,
                    LandingShape::Type(types.variant.clone()),
                )),
            ],
        ),
        delimited_landing(&types.attributes, &types.attribute),
        LandingTypeDeclaration::new(
            types.attribute.clone(),
            vec![
                landing_constructor(
                    &types.attribute,
                    IDENTITY,
                    vec![LandingFieldDeclaration::for_role::<
                        structural_codec::UnaryRoot,
                    >(LandingShape::Type(
                        types.derive_group.clone(),
                    ))],
                ),
                landing_constructor(
                    &types.attribute,
                    SECOND,
                    vec![
                        LandingFieldDeclaration::for_role::<structural_codec::ApplicationHead>(
                            LandingShape::Type(types.configuration_predicate.clone()),
                        ),
                        LandingFieldDeclaration::for_role::<structural_codec::ApplicationPayload>(
                            LandingShape::Type(types.derive_group.clone()),
                        ),
                    ],
                ),
                landing_constructor(
                    &types.attribute,
                    THIRD,
                    vec![
                        LandingFieldDeclaration::for_role::<structural_codec::ApplicationHead>(
                            LandingShape::Reference,
                        ),
                        LandingFieldDeclaration::for_role::<structural_codec::ApplicationPayload>(
                            LandingShape::Type(types.path.clone()),
                        ),
                    ],
                ),
            ],
        ),
        LandingTypeDeclaration::new(
            types.path.clone(),
            vec![
                landing_constructor(
                    &types.path,
                    IDENTITY,
                    vec![LandingFieldDeclaration::for_role::<
                        structural_codec::UnaryRoot,
                    >(LandingShape::Reference)],
                ),
                landing_constructor(
                    &types.path,
                    SECOND,
                    vec![
                        LandingFieldDeclaration::for_role::<structural_codec::ApplicationHead>(
                            LandingShape::Reference,
                        ),
                        LandingFieldDeclaration::for_role::<structural_codec::ApplicationPayload>(
                            LandingShape::Type(types.path.clone()),
                        ),
                    ],
                ),
            ],
        ),
        one_landing(
            &types.configuration_predicate,
            vec![LandingFieldDeclaration::for_role::<
                structural_codec::UnaryRoot,
            >(LandingShape::Reference)],
        ),
        delimited_landing(&types.derive_group, &types.path),
        delimited_landing(&types.generics, &types.generic_parameter),
        one_landing(
            &types.generic_parameter,
            vec![LandingFieldDeclaration::for_role::<
                structural_codec::UnaryRoot,
            >(LandingShape::Declaration)],
        ),
        one_landing(
            &types.type_reference,
            vec![LandingFieldDeclaration::for_role::<
                structural_codec::UnaryRoot,
            >(LandingShape::Type(types.path.clone()))],
        ),
        one_landing(
            &types.variant,
            vec![LandingFieldDeclaration::for_role::<
                structural_codec::UnaryRoot,
            >(LandingShape::Declaration)],
        ),
        LandingTypeDeclaration::new(
            types.visibility.clone(),
            vec![
                landing_constructor(
                    &types.visibility,
                    IDENTITY,
                    vec![LandingFieldDeclaration::for_role::<
                        structural_codec::UnaryRoot,
                    >(LandingShape::Literal(
                        words.public.clone(),
                    ))],
                ),
                landing_constructor(
                    &types.visibility,
                    SECOND,
                    vec![LandingFieldDeclaration::for_role::<
                        structural_codec::UnaryRoot,
                    >(LandingShape::Literal(
                        words.private.clone(),
                    ))],
                ),
            ],
        ),
    ];
    Ok(LandingDeclarationCatalog::try_new(declarations)?)
}

fn one_landing(
    encoded_type: &EncodedTypeId<VocabularyRoot>,
    fields: Vec<LandingFieldDeclaration<VocabularyRoot>>,
) -> LandingTypeDeclaration<VocabularyRoot> {
    LandingTypeDeclaration::new(
        encoded_type.clone(),
        vec![landing_constructor(encoded_type, IDENTITY, fields)],
    )
}

fn delimited_landing(
    encoded_type: &EncodedTypeId<VocabularyRoot>,
    item: &EncodedTypeId<VocabularyRoot>,
) -> LandingTypeDeclaration<VocabularyRoot> {
    one_landing(
        encoded_type,
        vec![LandingFieldDeclaration::for_role::<DelimitedItems>(
            LandingShape::sequence(0, None, LandingShape::Type(item.clone())),
        )],
    )
}

fn landing_constructor(
    encoded_type: &EncodedTypeId<VocabularyRoot>,
    local: u16,
    fields: Vec<LandingFieldDeclaration<VocabularyRoot>>,
) -> LandingConstructorDeclaration<VocabularyRoot> {
    LandingConstructorDeclaration::new(EncodedConstructorId::under(encoded_type, local), fields)
}

fn discovery() -> BlockTreeDiscoveryConfiguration {
    let active = TriggerSet::new(vec![
        PARENTHESIS,
        SQUARE,
        BRACE,
        CARRIER,
        WHITESPACE,
        COMMENT,
    ]);
    BlockTreeDiscoveryConfiguration::new(
        BoundaryDiscoveryConfiguration::new(
            ROOT_CONTEXT,
            vec![BoundaryDiscoveryContext::new(ROOT_CONTEXT, active)],
            vec![
                BoundaryDiscoveryTransition::new(ROOT_CONTEXT, PARENTHESIS, ROOT_CONTEXT),
                BoundaryDiscoveryTransition::new(ROOT_CONTEXT, SQUARE, ROOT_CONTEXT),
                BoundaryDiscoveryTransition::new(ROOT_CONTEXT, BRACE, ROOT_CONTEXT),
            ],
        ),
        vec![],
    )
}

#[derive(Debug, thiserror::Error)]
pub enum LogosLanguageError {
    #[error(transparent)]
    Profile(#[from] raw_discovery::TokenProfileError),
    #[error(transparent)]
    Authoring(#[from] structural_codec::AuthoringError),
    #[error(transparent)]
    Table(Box<TableError<VocabularyRoot>>),
    #[error(transparent)]
    Declaration(#[from] LanguageDeclarationError<VocabularyRoot>),
}

impl From<TableError<VocabularyRoot>> for LogosLanguageError {
    fn from(error: TableError<VocabularyRoot>) -> Self {
        Self::Table(Box::new(error))
    }
}
