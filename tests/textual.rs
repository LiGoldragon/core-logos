//! The Logos text mouth: a test-local, sealed typed structuretree plus its
//! lexicon. Text crosses only through `TextualForm`; shared structural-codec
//! evaluation owns parsing, rendering, disjointness, and name transactions.

mod support;

use std::collections::BTreeMap;

use core_logos::{
    Attribute, ConfigurationAttribute, ConfigurationPredicate, DeriveGroup, EncodedItem, Newtype,
    PathNode, TypeReference, Visibility,
};
use name_table::{Identifier, Name, NameTable, NameTableError, NameTransaction};
use raw_discovery::{
    BlockPrefixAttachment, BlockPrefixRule, BlockTreeDiscoveryConfiguration,
    BoundaryDiscoveryConfiguration, BoundaryDiscoveryContext, BoundaryDiscoveryContextIdentifier,
    BoundaryDiscoveryTransition, CharacterClass, RawProfile, TokenProfileError, TriggerIdentifier,
    TriggerSet,
};
use structural_codec::{
    AcceptedDecodeForm, AddressedStructuralTable, ApplicationDelimitedBody,
    ApplicationDelimitedHead, ApplicationDelimitedItems, ApplicationDelimitedRoot,
    ApplicationDelimitedRule, ApplicationHead, ApplicationPayload, ApplicationRoot,
    ApplicationRule, AtomDescriptor, AuthoringError, ConstructorCodec, ContextualTextualPolicy,
    DecodeError, DecodeFormId, EncodeError, EncodedConstructorId, EncodedForm, EncodedLanguage,
    FieldEnd, FieldLink, FieldRole, FieldValue, Position, RuleCoproduct, ScopedEncodedTypeId,
    SharedDescriptor, StableRoleId, StructuralEntry, StructuralRule, StructuralValue,
    StructuralVocabularyIdentity, StructureRecord, TableError, TableIdentityPayload,
    TargetLayoutIdentity, Textual, TextualForm, TextualRenderingPolicy, UnaryRoot, UnaryRule,
};

/// The Logos language marker carried by the test-local encoded wrapper and text view.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct LogosLanguage;

/// `EncodedItem` belongs to core-logos, while `EncodedForm` belongs to the codec. The
/// test-local wrapper connects those external crates without extending the production item API.
#[derive(Clone, Debug, PartialEq)]
struct TextualItem(EncodedItem);

impl EncodedForm for TextualItem {
    type Language = LogosLanguage;
}

const ITEM: ScopedEncodedTypeId = ScopedEncodedTypeId::logos(1);
const ITEM_FIELD: ScopedEncodedTypeId = ScopedEncodedTypeId::logos(2);
const ATTRIBUTE: ScopedEncodedTypeId = ScopedEncodedTypeId::logos(3);
const CONFIGURATION_FIELD: ScopedEncodedTypeId = ScopedEncodedTypeId::logos(4);
const PATH_NODE: ScopedEncodedTypeId = ScopedEncodedTypeId::logos(5);

const SQUARE_BOUNDARY: TriggerIdentifier = TriggerIdentifier::new(1);
const BRACE_BOUNDARY: TriggerIdentifier = TriggerIdentifier::new(2);
const APPLICATION_OPERATOR: TriggerIdentifier = TriggerIdentifier::new(3);
const WHITESPACE_TRIVIA: TriggerIdentifier = TriggerIdentifier::new(5);
const COMMENT_TRIVIA: TriggerIdentifier = TriggerIdentifier::new(6);
const ROOT_CONTEXT: BoundaryDiscoveryContextIdentifier = BoundaryDiscoveryContextIdentifier::new(1);

const ITEM_NEWTYPE: u16 = 0;
const ITEM_FIELD_PATH: u16 = 0;
const ITEM_FIELD_ATTRIBUTES: u16 = 1;
const ATTRIBUTE_TOOL_PATH: u16 = 0;
const ATTRIBUTE_CONFIGURATION: u16 = 1;
const ATTRIBUTE_DERIVE: u16 = 2;
const CONFIGURATION_FEATURE: u16 = 0;
const CONFIGURATION_ATTRIBUTE: u16 = 1;
const PATH_SINGLE: u16 = 0;
const PATH_MULTI: u16 = 1;

macro_rules! role {
    ($name:ident, $stable_id:expr) => {
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
        struct $name;

        impl FieldRole for $name {
            const STABLE_ID: u16 = $stable_id;
        }
    };
}

role!(DelimitedRoot, 901);
role!(DelimitedItems, 902);

/// The one fixture-specific typed record: a root boundary with one repeated interior.
/// Its `Position::role()` links make the archived role identity, rather than an erased
/// ordinal, the only route from the boundary to its contents.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Eq, PartialEq)]
struct DelimitedRule {
    root: Position<DelimitedRoot>,
    items: Position<DelimitedItems>,
}

impl DelimitedRule {
    fn new(
        boundary: TriggerIdentifier,
        element: SharedDescriptor,
        minimum: u64,
        maximum: Option<u64>,
    ) -> Result<Self, AuthoringError> {
        let items = Position::try_new(SharedDescriptor::Repeated {
            minimum,
            maximum,
            element: Box::new(element),
        })?;
        let root = Position::try_new(SharedDescriptor::Delimited {
            boundary,
            content: items.role(),
        })?;
        Ok(Self { root, items })
    }
}

impl StructureRecord for DelimitedRule {
    type View<'record> =
        FieldLink<'record, DelimitedRoot, FieldLink<'record, DelimitedItems, FieldEnd>>;

    fn root_role(&self) -> StableRoleId {
        self.root.role()
    }

    fn fields(&self) -> Self::View<'_> {
        FieldLink::new(&self.root, FieldLink::new(&self.items, FieldEnd))
    }
}

/// The full fixture vocabulary is data-only coproducts of the test-local boundary rule
/// and structural-codec's typed unary/application records. Evaluation remains shared.
type LogosRule = RuleCoproduct<DelimitedRule, StructuralRule>;

fn core_rule(rule: StructuralRule) -> LogosRule {
    RuleCoproduct::Right(rule)
}

fn delimited_rule(rule: DelimitedRule) -> LogosRule {
    RuleCoproduct::Left(rule)
}

/// A textual round-trip over `EncodedItem` failed.
#[derive(Debug, thiserror::Error)]
enum LogosTextError {
    #[error(transparent)]
    Authoring(#[from] AuthoringError),
    #[error(transparent)]
    Decode(#[from] DecodeError),
    #[error(transparent)]
    Encode(#[from] EncodeError),
    #[error(transparent)]
    Names(#[from] NameTableError),
    #[error(transparent)]
    Profile(#[from] TokenProfileError),
    #[error(transparent)]
    SingleChunk(#[from] structural_codec::SingleChunkRequired),
    #[error(transparent)]
    Table(#[from] TableError),
    #[error("the source held no root object to decode")]
    EmptySource,
    #[error("the decoded mirror did not fit the expected {0} shape")]
    ReifyShape(&'static str),
}

/// The structural-keyword lexicon. It is separate from the caller-owned Logos namespace.
struct Lexicon {
    names: NameTable,
    newtype: Identifier,
    tool_path: Identifier,
    configuration: Identifier,
    derive: Identifier,
    feature: Identifier,
    public: Identifier,
    private: Identifier,
}

impl Lexicon {
    fn build() -> Result<Self, NameTableError> {
        let mut names = NameTable::new(name_table::IdentifierNamespace::LogosStandard);
        let mut keyword = |text: &str| names.intern(Name::new(text));
        Ok(Self {
            newtype: keyword("Newtype")?,
            tool_path: keyword("ToolPath")?,
            configuration: keyword("Configuration")?,
            derive: keyword("Derive")?,
            feature: keyword("Feature")?,
            public: keyword("Public")?,
            private: keyword("Private")?,
            names,
        })
    }
}

/// One textual mouth: the sealed test table and its literal lexicon.
struct TextualLogos {
    table: AddressedStructuralTable<LogosRule>,
    lexicon: Lexicon,
}

impl TextualLogos {
    fn build(reverse_attribute_forms: bool) -> Result<Self, LogosTextError> {
        let lexicon = Lexicon::build()?;
        let profile = RawProfile::standard().seal()?;
        let entries = vec![
            Self::item_entry(&lexicon)?,
            Self::item_field_entry()?,
            Self::attribute_entry(&lexicon, reverse_attribute_forms)?,
            Self::configuration_field_entry(&lexicon)?,
            Self::path_entry()?,
        ]
        .into_iter()
        .map(|entry| (entry.encoded_type(), entry))
        .collect::<BTreeMap<_, _>>();
        let table = AddressedStructuralTable::seal(
            TableIdentityPayload::new(
                EncodedLanguage::Logos,
                TargetLayoutIdentity::derive(b"core-logos textual fixture typed layout R4"),
                profile.identity(),
                StructuralVocabularyIdentity::language(
                    b"core-logos textual fixture typed vocabulary R4",
                ),
                Self::block_discovery(),
                TextualRenderingPolicy::new(vec![ContextualTextualPolicy::new(
                    ROOT_CONTEXT,
                    Some(WHITESPACE_TRIVIA),
                    None,
                )]),
                entries,
            ),
            &profile,
        )?;
        Ok(Self { table, lexicon })
    }

    fn block_discovery() -> BlockTreeDiscoveryConfiguration {
        BlockTreeDiscoveryConfiguration::new(
            BoundaryDiscoveryConfiguration::new(
                ROOT_CONTEXT,
                vec![BoundaryDiscoveryContext::new(
                    ROOT_CONTEXT,
                    TriggerSet::new(vec![
                        SQUARE_BOUNDARY,
                        BRACE_BOUNDARY,
                        WHITESPACE_TRIVIA,
                        COMMENT_TRIVIA,
                    ]),
                )],
                vec![
                    BoundaryDiscoveryTransition::new(ROOT_CONTEXT, SQUARE_BOUNDARY, ROOT_CONTEXT),
                    BoundaryDiscoveryTransition::new(ROOT_CONTEXT, BRACE_BOUNDARY, ROOT_CONTEXT),
                ],
            ),
            vec![
                BlockPrefixAttachment::new(
                    SQUARE_BOUNDARY,
                    BlockPrefixRule::new(".", CharacterClass::AsciiAlphabetic),
                ),
                BlockPrefixAttachment::new(
                    BRACE_BOUNDARY,
                    BlockPrefixRule::new(".", CharacterClass::AsciiAlphabetic),
                ),
            ],
        )
    }

    fn codec(
        type_id: ScopedEncodedTypeId,
        constructor: u16,
        rule: LogosRule,
    ) -> ConstructorCodec<LogosRule> {
        ConstructorCodec::new(
            EncodedConstructorId::under(type_id, constructor),
            vec![AcceptedDecodeForm::new(DecodeFormId::new(1), rule.clone())],
            rule,
        )
    }

    fn unary(descriptor: SharedDescriptor) -> Result<LogosRule, LogosTextError> {
        Ok(core_rule(StructuralRule::Unary(UnaryRule::new(
            descriptor,
        )?)))
    }

    fn application(
        head: SharedDescriptor,
        payload: SharedDescriptor,
    ) -> Result<LogosRule, LogosTextError> {
        Ok(core_rule(StructuralRule::Application(
            ApplicationRule::new(APPLICATION_OPERATOR, head, payload)?,
        )))
    }

    fn application_delimited(
        head: SharedDescriptor,
        boundary: TriggerIdentifier,
        element: SharedDescriptor,
        minimum: u64,
        maximum: Option<u64>,
    ) -> Result<LogosRule, LogosTextError> {
        Ok(core_rule(StructuralRule::ApplicationDelimited(
            ApplicationDelimitedRule::new(
                APPLICATION_OPERATOR,
                boundary,
                head,
                element,
                minimum,
                maximum,
            )?,
        )))
    }

    fn delimited(
        boundary: TriggerIdentifier,
        element: SharedDescriptor,
        minimum: u64,
        maximum: Option<u64>,
    ) -> Result<LogosRule, LogosTextError> {
        Ok(delimited_rule(DelimitedRule::new(
            boundary, element, minimum, maximum,
        )?))
    }

    fn delegate(target: ScopedEncodedTypeId) -> SharedDescriptor {
        SharedDescriptor::Delegate {
            target,
            payload: None,
        }
    }

    fn atom() -> SharedDescriptor {
        SharedDescriptor::Atom(AtomDescriptor::any_case())
    }

    /// `Newtype.{ <field> … }`. The bounded repeated body carries five ordered
    /// semantic fields; `reify_item` verifies the exact semantic arity and roles.
    fn item_entry(lexicon: &Lexicon) -> Result<StructuralEntry<LogosRule>, LogosTextError> {
        let rule = Self::application_delimited(
            SharedDescriptor::Literal(lexicon.newtype),
            BRACE_BOUNDARY,
            Self::delegate(ITEM_FIELD),
            5,
            Some(5),
        )?;
        Ok(StructuralEntry::new(
            ITEM,
            vec![Self::codec(ITEM, ITEM_NEWTYPE, rule)],
        ))
    }

    /// A newtype position is either a path-shaped atom/application or its bracketed
    /// attribute sequence. These alternatives are boundary-disjoint.
    fn item_field_entry() -> Result<StructuralEntry<LogosRule>, LogosTextError> {
        let path = Self::unary(Self::delegate(PATH_NODE))?;
        let attributes = Self::delimited(SQUARE_BOUNDARY, Self::delegate(ATTRIBUTE), 0, None)?;
        Ok(StructuralEntry::new(
            ITEM_FIELD,
            vec![
                Self::codec(ITEM_FIELD, ITEM_FIELD_PATH, path),
                Self::codec(ITEM_FIELD, ITEM_FIELD_ATTRIBUTES, attributes),
            ],
        ))
    }

    fn attribute_entry(
        lexicon: &Lexicon,
        reverse_forms: bool,
    ) -> Result<StructuralEntry<LogosRule>, LogosTextError> {
        let tool_path = Self::application(
            SharedDescriptor::Literal(lexicon.tool_path),
            Self::delegate(PATH_NODE),
        )?;
        let configuration = Self::application_delimited(
            SharedDescriptor::Literal(lexicon.configuration),
            BRACE_BOUNDARY,
            Self::delegate(CONFIGURATION_FIELD),
            2,
            Some(2),
        )?;
        let derive = Self::application_delimited(
            SharedDescriptor::Literal(lexicon.derive),
            SQUARE_BOUNDARY,
            Self::delegate(PATH_NODE),
            0,
            None,
        )?;
        let mut constructors = vec![
            Self::codec(ATTRIBUTE, ATTRIBUTE_TOOL_PATH, tool_path),
            Self::codec(ATTRIBUTE, ATTRIBUTE_CONFIGURATION, configuration),
            Self::codec(ATTRIBUTE, ATTRIBUTE_DERIVE, derive),
        ];
        if reverse_forms {
            constructors.reverse();
        }
        Ok(StructuralEntry::new(ATTRIBUTE, constructors))
    }

    /// `Feature.<name>` and nested attributes are disjoint by their literal heads.
    fn configuration_field_entry(
        lexicon: &Lexicon,
    ) -> Result<StructuralEntry<LogosRule>, LogosTextError> {
        let feature = Self::application(SharedDescriptor::Literal(lexicon.feature), Self::atom())?;
        let attribute = Self::unary(Self::delegate(ATTRIBUTE))?;
        Ok(StructuralEntry::new(
            CONFIGURATION_FIELD,
            vec![
                Self::codec(CONFIGURATION_FIELD, CONFIGURATION_FEATURE, feature),
                Self::codec(CONFIGURATION_FIELD, CONFIGURATION_ATTRIBUTE, attribute),
            ],
        ))
    }

    /// A right-recursive dotted path. The bare and application forms are disjoint;
    /// repeated callers retain their separator cue under structural-codec's fixed cursor law.
    fn path_entry() -> Result<StructuralEntry<LogosRule>, LogosTextError> {
        let single = Self::unary(Self::atom())?;
        let multi = Self::application(Self::atom(), Self::delegate(PATH_NODE))?;
        Ok(StructuralEntry::new(
            PATH_NODE,
            vec![
                Self::codec(PATH_NODE, PATH_SINGLE, single),
                Self::codec(PATH_NODE, PATH_MULTI, multi),
            ],
        ))
    }

    fn app_head(value: &StructuralValue, what: &'static str) -> Result<Identifier, LogosTextError> {
        match value.field::<ApplicationHead>() {
            Some(FieldValue::Atom(identifier)) => Ok(*identifier),
            _ => Err(LogosTextError::ReifyShape(what)),
        }
    }

    fn app_payload<'value>(
        value: &'value StructuralValue,
        what: &'static str,
    ) -> Result<&'value FieldValue, LogosTextError> {
        value
            .field::<ApplicationPayload>()
            .ok_or(LogosTextError::ReifyShape(what))
    }

    fn app_delimited_items<'value>(
        value: &'value StructuralValue,
        what: &'static str,
    ) -> Result<&'value [FieldValue], LogosTextError> {
        match value.field::<ApplicationDelimitedItems>() {
            Some(FieldValue::Repeated(items)) => Ok(items),
            _ => Err(LogosTextError::ReifyShape(what)),
        }
    }

    fn delimited_items<'value>(
        value: &'value StructuralValue,
        what: &'static str,
    ) -> Result<&'value [FieldValue], LogosTextError> {
        match value.field::<DelimitedItems>() {
            Some(FieldValue::Repeated(items)) => Ok(items),
            _ => Err(LogosTextError::ReifyShape(what)),
        }
    }

    fn unary_delegated<'value>(
        value: &'value StructuralValue,
        what: &'static str,
    ) -> Result<&'value StructuralValue, LogosTextError> {
        match value.field::<UnaryRoot>() {
            Some(FieldValue::Delegated(inner)) => Ok(inner),
            _ => Err(LogosTextError::ReifyShape(what)),
        }
    }

    fn delegated<'value>(
        value: &'value FieldValue,
        what: &'static str,
    ) -> Result<&'value StructuralValue, LogosTextError> {
        match value {
            FieldValue::Delegated(inner) => Ok(inner),
            _ => Err(LogosTextError::ReifyShape(what)),
        }
    }

    fn reify_item(
        &self,
        mirror: &StructuralValue,
        names: &NameTransaction<'_>,
    ) -> Result<EncodedItem, LogosTextError> {
        if mirror.constructor() != EncodedConstructorId::under(ITEM, ITEM_NEWTYPE) {
            return Err(LogosTextError::ReifyShape("item constructor"));
        }
        let [visibility, attributes, name, wrapped_visibility, wrapped] =
            Self::app_delimited_items(mirror, "newtype fields")?
        else {
            return Err(LogosTextError::ReifyShape("newtype fields"));
        };
        Ok(EncodedItem::Newtype(Newtype {
            visibility: self.reify_visibility(self.reify_item_path(visibility)?, names)?,
            attributes: self.reify_item_attributes(attributes, names)?,
            name: Self::single_identifier(self.reify_item_path(name)?, "newtype name")?,
            wrapped_visibility: self
                .reify_visibility(self.reify_item_path(wrapped_visibility)?, names)?,
            wrapped: TypeReference::Path(self.reify_item_path(wrapped)?),
        }))
    }

    fn reify_item_path(&self, value: &FieldValue) -> Result<PathNode, LogosTextError> {
        let field = Self::delegated(value, "newtype field delegate")?;
        if field.constructor() != EncodedConstructorId::under(ITEM_FIELD, ITEM_FIELD_PATH) {
            return Err(LogosTextError::ReifyShape("newtype path field"));
        }
        Self::reify_path(Self::unary_delegated(field, "newtype path")?)
    }

    fn reify_item_attributes(
        &self,
        value: &FieldValue,
        names: &NameTransaction<'_>,
    ) -> Result<Vec<Attribute>, LogosTextError> {
        let field = Self::delegated(value, "newtype attributes delegate")?;
        if field.constructor() != EncodedConstructorId::under(ITEM_FIELD, ITEM_FIELD_ATTRIBUTES) {
            return Err(LogosTextError::ReifyShape("newtype attributes field"));
        }
        Self::delimited_items(field, "newtype attributes")?
            .iter()
            .map(|attribute| {
                self.reify_attribute(Self::delegated(attribute, "attribute delegate")?, names)
            })
            .collect()
    }

    fn reify_visibility(
        &self,
        path: PathNode,
        names: &NameTransaction<'_>,
    ) -> Result<Visibility, LogosTextError> {
        let identifier = Self::single_identifier(path, "visibility")?;
        match names.resolve(identifier)?.as_str() {
            "Public" => Ok(Visibility::Public),
            "Private" => Ok(Visibility::Private),
            _ => Err(LogosTextError::ReifyShape("visibility spelling")),
        }
    }

    fn single_identifier(path: PathNode, what: &'static str) -> Result<Identifier, LogosTextError> {
        match path.segments.as_slice() {
            [identifier] => Ok(*identifier),
            _ => Err(LogosTextError::ReifyShape(what)),
        }
    }

    fn reify_attribute(
        &self,
        mirror: &StructuralValue,
        names: &NameTransaction<'_>,
    ) -> Result<Attribute, LogosTextError> {
        match mirror.constructor().local() {
            ATTRIBUTE_TOOL_PATH => Ok(Attribute::ToolPath(Self::reify_path(Self::delegated(
                Self::app_payload(mirror, "tool path payload")?,
                "tool path",
            )?)?)),
            ATTRIBUTE_CONFIGURATION => {
                let [predicate, inner] = Self::app_delimited_items(mirror, "configuration fields")?
                else {
                    return Err(LogosTextError::ReifyShape("configuration fields"));
                };
                Ok(Attribute::Configuration(ConfigurationAttribute {
                    predicate: self.reify_configuration_predicate(predicate)?,
                    inner: Box::new(self.reify_configuration_attribute(inner, names)?),
                }))
            }
            ATTRIBUTE_DERIVE => Ok(Attribute::Derive(DeriveGroup {
                paths: Self::app_delimited_items(mirror, "derive paths")?
                    .iter()
                    .map(|path| Self::reify_path(Self::delegated(path, "derive path")?))
                    .collect::<Result<_, _>>()?,
            })),
            _ => Err(LogosTextError::ReifyShape("attribute constructor")),
        }
    }

    fn reify_configuration_predicate(
        &self,
        value: &FieldValue,
    ) -> Result<ConfigurationPredicate, LogosTextError> {
        let field = Self::delegated(value, "configuration predicate delegate")?;
        if field.constructor()
            != EncodedConstructorId::under(CONFIGURATION_FIELD, CONFIGURATION_FEATURE)
        {
            return Err(LogosTextError::ReifyShape("configuration predicate"));
        }
        match Self::app_payload(field, "feature payload")? {
            FieldValue::Atom(identifier) => Ok(ConfigurationPredicate::Feature(*identifier)),
            _ => Err(LogosTextError::ReifyShape("feature payload")),
        }
    }

    fn reify_configuration_attribute(
        &self,
        value: &FieldValue,
        names: &NameTransaction<'_>,
    ) -> Result<Attribute, LogosTextError> {
        let field = Self::delegated(value, "configuration attribute delegate")?;
        if field.constructor()
            != EncodedConstructorId::under(CONFIGURATION_FIELD, CONFIGURATION_ATTRIBUTE)
        {
            return Err(LogosTextError::ReifyShape("configuration attribute"));
        }
        self.reify_attribute(
            Self::unary_delegated(field, "configuration attribute")?,
            names,
        )
    }

    fn reify_path(mirror: &StructuralValue) -> Result<PathNode, LogosTextError> {
        match mirror.constructor().local() {
            PATH_SINGLE => match mirror.field::<UnaryRoot>() {
                Some(FieldValue::Atom(identifier)) => Ok(PathNode {
                    segments: vec![*identifier],
                }),
                _ => Err(LogosTextError::ReifyShape("path atom")),
            },
            PATH_MULTI => {
                let head = Self::app_head(mirror, "path head")?;
                let tail = Self::delegated(Self::app_payload(mirror, "path tail")?, "path tail")?;
                let mut segments = vec![head];
                segments.extend(Self::reify_path(tail)?.segments);
                Ok(PathNode { segments })
            }
            _ => Err(LogosTextError::ReifyShape("path constructor")),
        }
    }

    fn reflect_item(&self, item: &EncodedItem) -> Result<StructuralValue, LogosTextError> {
        let EncodedItem::Newtype(newtype) = item else {
            return Err(LogosTextError::ReifyShape(
                "only Newtype items are authored",
            ));
        };
        let public = Self::path_from_identifier(self.lexicon.public);
        let private = Self::path_from_identifier(self.lexicon.private);
        Self::application_delimited_mirror(
            ITEM,
            ITEM_NEWTYPE,
            self.lexicon.newtype,
            vec![
                Self::item_path_field(public)?,
                self.item_attributes_field(&newtype.attributes)?,
                Self::item_path_field(Self::path_from_identifier(newtype.name))?,
                Self::item_path_field(private)?,
                Self::item_path_field(Self::type_path(&newtype.wrapped)?)?,
            ],
        )
    }

    fn item_path_field(path: PathNode) -> Result<FieldValue, LogosTextError> {
        Ok(FieldValue::Delegated(Box::new(Self::unary_mirror(
            ITEM_FIELD,
            ITEM_FIELD_PATH,
            FieldValue::Delegated(Box::new(Self::reflect_path(&path)?)),
        )?)))
    }

    fn item_attributes_field(
        &self,
        attributes: &[Attribute],
    ) -> Result<FieldValue, LogosTextError> {
        let items = attributes
            .iter()
            .map(|attribute| {
                self.reflect_attribute(attribute)
                    .map(|value| FieldValue::Delegated(Box::new(value)))
            })
            .collect::<Result<_, _>>()?;
        Ok(FieldValue::Delegated(Box::new(Self::delimited_mirror(
            ITEM_FIELD,
            ITEM_FIELD_ATTRIBUTES,
            items,
        )?)))
    }

    fn reflect_attribute(&self, attribute: &Attribute) -> Result<StructuralValue, LogosTextError> {
        match attribute {
            Attribute::ToolPath(path) => Self::application_mirror(
                ATTRIBUTE,
                ATTRIBUTE_TOOL_PATH,
                self.lexicon.tool_path,
                FieldValue::Delegated(Box::new(Self::reflect_path(path)?)),
            ),
            Attribute::Configuration(configuration) => Self::application_delimited_mirror(
                ATTRIBUTE,
                ATTRIBUTE_CONFIGURATION,
                self.lexicon.configuration,
                vec![
                    FieldValue::Delegated(Box::new(
                        self.reflect_configuration_predicate(&configuration.predicate)?,
                    )),
                    FieldValue::Delegated(Box::new(
                        self.reflect_configuration_attribute(&configuration.inner)?,
                    )),
                ],
            ),
            Attribute::Derive(group) => Self::application_delimited_mirror(
                ATTRIBUTE,
                ATTRIBUTE_DERIVE,
                self.lexicon.derive,
                group
                    .paths
                    .iter()
                    .map(|path| {
                        Self::reflect_path(path).map(|value| FieldValue::Delegated(Box::new(value)))
                    })
                    .collect::<Result<_, _>>()?,
            ),
            Attribute::Cfg(_) | Attribute::HelperDerive(_) => {
                Err(LogosTextError::ReifyShape("attribute deferred"))
            }
        }
    }

    fn reflect_configuration_predicate(
        &self,
        predicate: &ConfigurationPredicate,
    ) -> Result<StructuralValue, LogosTextError> {
        let ConfigurationPredicate::Feature(feature) = predicate;
        Self::application_mirror(
            CONFIGURATION_FIELD,
            CONFIGURATION_FEATURE,
            self.lexicon.feature,
            FieldValue::Atom(*feature),
        )
    }

    fn reflect_configuration_attribute(
        &self,
        attribute: &Attribute,
    ) -> Result<StructuralValue, LogosTextError> {
        Self::unary_mirror(
            CONFIGURATION_FIELD,
            CONFIGURATION_ATTRIBUTE,
            FieldValue::Delegated(Box::new(self.reflect_attribute(attribute)?)),
        )
    }

    fn type_path(type_reference: &TypeReference) -> Result<PathNode, LogosTextError> {
        match type_reference {
            TypeReference::Path(path) => Ok(path.clone()),
            _ => Err(LogosTextError::ReifyShape(
                "only Path type references authored",
            )),
        }
    }

    fn path_from_identifier(identifier: Identifier) -> PathNode {
        PathNode {
            segments: vec![identifier],
        }
    }

    fn reflect_path(path: &PathNode) -> Result<StructuralValue, LogosTextError> {
        let Some((head, tail)) = path.segments.split_first() else {
            return Err(LogosTextError::ReifyShape("empty path"));
        };
        if tail.is_empty() {
            return Self::unary_mirror(PATH_NODE, PATH_SINGLE, FieldValue::Atom(*head));
        }
        Self::application_mirror(
            PATH_NODE,
            PATH_MULTI,
            *head,
            FieldValue::Delegated(Box::new(Self::reflect_path(&PathNode {
                segments: tail.to_vec(),
            })?)),
        )
    }

    fn unary_mirror(
        type_id: ScopedEncodedTypeId,
        constructor: u16,
        root: FieldValue,
    ) -> Result<StructuralValue, LogosTextError> {
        let mut record = StructuralValue::record(EncodedConstructorId::under(type_id, constructor));
        record.insert::<UnaryRoot>(root)?;
        Ok(record.finish())
    }

    fn application_mirror(
        type_id: ScopedEncodedTypeId,
        constructor: u16,
        head: Identifier,
        payload: FieldValue,
    ) -> Result<StructuralValue, LogosTextError> {
        let head_value = FieldValue::Atom(head);
        let root = FieldValue::Application {
            head: Box::new(head_value.clone()),
            payload: Box::new(payload.clone()),
        };
        let mut record = StructuralValue::record(EncodedConstructorId::under(type_id, constructor));
        record.insert::<ApplicationRoot>(root)?;
        record.insert::<ApplicationHead>(head_value)?;
        record.insert::<ApplicationPayload>(payload)?;
        Ok(record.finish())
    }

    fn application_delimited_mirror(
        type_id: ScopedEncodedTypeId,
        constructor: u16,
        head: Identifier,
        items: Vec<FieldValue>,
    ) -> Result<StructuralValue, LogosTextError> {
        let head_value = FieldValue::Atom(head);
        let items_value = FieldValue::Repeated(items);
        let body_value = FieldValue::Delimited(Box::new(items_value.clone()));
        let root = FieldValue::Application {
            head: Box::new(head_value.clone()),
            payload: Box::new(body_value.clone()),
        };
        let mut record = StructuralValue::record(EncodedConstructorId::under(type_id, constructor));
        record.insert::<ApplicationDelimitedRoot>(root)?;
        record.insert::<ApplicationDelimitedHead>(head_value)?;
        record.insert::<ApplicationDelimitedBody>(body_value)?;
        record.insert::<ApplicationDelimitedItems>(items_value)?;
        Ok(record.finish())
    }

    fn delimited_mirror(
        type_id: ScopedEncodedTypeId,
        constructor: u16,
        items: Vec<FieldValue>,
    ) -> Result<StructuralValue, LogosTextError> {
        let items_value = FieldValue::Repeated(items);
        let root = FieldValue::Delimited(Box::new(items_value.clone()));
        let mut record = StructuralValue::record(EncodedConstructorId::under(type_id, constructor));
        record.insert::<DelimitedRoot>(root)?;
        record.insert::<DelimitedItems>(items_value)?;
        Ok(record.finish())
    }
}

impl Textual<LogosRule> for TextualLogos {
    type Encoded = TextualItem;
    type Language = LogosLanguage;
    type Error = LogosTextError;

    fn structuretree(&self) -> &AddressedStructuralTable<LogosRule> {
        &self.table
    }

    fn lexicon(&self) -> Option<&NameTable> {
        Some(&self.lexicon.names)
    }

    fn missing_root_object(&self) -> LogosTextError {
        LogosTextError::EmptySource
    }

    fn reify(
        &self,
        expected: ScopedEncodedTypeId,
        mirror: &StructuralValue,
        names: &mut NameTransaction<'_>,
    ) -> Result<TextualItem, LogosTextError> {
        if expected != ITEM {
            return Err(LogosTextError::ReifyShape("expected item type"));
        }
        self.reify_item(mirror, names).map(TextualItem)
    }

    fn reflect(
        &self,
        expected: ScopedEncodedTypeId,
        encoded: &TextualItem,
        _names: &NameTable,
    ) -> Result<StructuralValue, LogosTextError> {
        if expected != ITEM {
            return Err(LogosTextError::ReifyShape("expected item type"));
        }
        self.reflect_item(&encoded.0)
    }
}

const GOLDEN_TEXT: &str = "Newtype.{Public [ToolPath.rustfmt.skip Configuration.{Feature.nota-text Derive.[nota.NotaDecode nota.NotaDecodeTraced nota.NotaEncode]} Derive.[rkyv.Archive rkyv.Serialize rkyv.Deserialize Clone Debug PartialEq Eq]] CommitSequence Private Integer}";

fn logos_names(mouth: &TextualLogos) -> NameTable {
    NameTable::new(name_table::IdentifierNamespace::Logos)
        .compose(&mouth.lexicon.names)
        .expect("compose the Logos standard lexicon")
}

/// The full golden preserves its canonical spelling and semantic `EncodedItem` value through
/// the one shared evaluator path.
#[test]
fn golden_commit_sequence_round_trips_through_the_organs() {
    let mouth = TextualLogos::build(false).expect("seal typed logos table");
    let mut names = logos_names(&mouth);
    let golden = TextualItem(support::commit_sequence(&mut names));

    let text: TextualForm<LogosLanguage> = mouth.view(ITEM, &golden, &names).expect("view golden");
    let text_str = text.sole_text().expect("sole view text");
    assert_eq!(
        text_str, GOLDEN_TEXT,
        "the fixture spelling stays canonical"
    );

    let decoded = mouth
        .unview(ITEM, &text, &mut names)
        .expect("unview golden");
    assert_eq!(golden, decoded, "value -> text -> value is lossless");
    assert_eq!(mouth.view(ITEM, &decoded, &names).expect("re-view"), text);
}

/// Constructor-vector order is not a decode priority. The seal proves the literal-head
/// alternatives disjoint, so reversing their authored order preserves the decoded value.
#[test]
fn attribute_decode_is_independent_of_authoring_order() {
    for reverse in [false, true] {
        let mouth = TextualLogos::build(reverse).expect("seal typed logos table");
        let mut names = logos_names(&mouth);
        let decoded = mouth
            .unview(
                ITEM,
                &TextualForm::single(GOLDEN_TEXT.to_owned()),
                &mut names,
            )
            .expect("decode canonical golden");
        let expected = TextualItem(support::commit_sequence(&mut names));
        assert_eq!(decoded, expected, "reverse={reverse}");
    }
}

/// A source that reaches reification with an invalid semantic visibility must not commit any
/// speculative names. The shared `Textual::unview` transaction supplies the rollback law.
#[test]
fn malformed_newtype_rolls_back_speculative_names() {
    let mouth = TextualLogos::build(false).expect("seal typed logos table");
    let mut names = logos_names(&mouth);
    let before = names.len();
    let malformed = "Newtype.{Unrecognized [ToolPath.rustfmt.skip] CommitSequence Private Integer}";
    assert!(matches!(
        mouth.unview(ITEM, &TextualForm::single(malformed.to_owned()), &mut names),
        Err(LogosTextError::ReifyShape("visibility spelling"))
    ));
    assert_eq!(
        names.len(),
        before,
        "failed parsing commits no source names"
    );
}
