//! The Logos text mouth (epic primary-56d1.43): EncodedLogos values encode to and decode
//! from positional NOTA-family (Protos) text through the ONE trusted structural-codec
//! evaluator walking a sealed structuretree plus the nametree — the strict invariant.
//! There is no bespoke per-type parse or print path here: every spelling is a
//! `StructuralForm` in the sealed table, and `TextualLogos` implements the shared
//! `TextualForm` operation, supplying only `reify` / `reflect` between the generic
//! mirror and EncodedLogos.
//!
//! The spellings are DERIVED from the delivered data shapes through the ruled Protos
//! laws (protos-syntax): an enum variant carrying a payload is `Head.payload` (the
//! `Tick.7` law); a struct is a `{ … }` positional record; a `Vec` is a `[ … ]`
//! vector; a path is a glued-dot right-associative application (`rkyv.Archive`); a
//! unit enum variant is a bare PascalCase atom; a canonical string is a bare atom.
//! EncodedLogos struct fields are structural, not name-data, so the record is purely
//! positional — no field names are written (and none are elided).
//!
//! Witness: the REAL golden `CommitSequence` newtype value — all five fields, the full
//! three-attribute preamble (rustfmt-skip, the feature-gated NOTA derive set, the rkyv
//! derive list) — round-trips value -> text -> value losslessly, and the text carries
//! every datum visibly.
//!
//! Table-seal site: `TextualLogos::build` -> `AddressedStructuralTable::seal` (proved
//! disjoint via `validate_disjoint`). Evaluator entry: `TextualForm::view` / `unview`
//! -> `StructuralEvaluator::encode` / `decode`.

mod support;

use core_logos::{
    Attribute, ConfigurationAttribute, ConfigurationPredicate, DeriveGroup, EncodedItem, Newtype,
    PathNode, TypeReference, Visibility,
};
use name_table::{Identifier, Name, NameResolver, NameTable, NameTableError};
use raw_discovery::{Delimiter, RecognizeError};
use structural_codec::ids::{
    EncodedConstructorId, PositionalSignature, ScopedEncodedTypeId, StructuralRevision,
};
use structural_codec::table::{
    AddressedStructuralTable, EncodedLayoutIdentity, RawProfileIdentity, TableIdentityPayload,
};
use structural_codec::value::StructuralValue;
use structural_codec::{
    AtomForm, ConstructorCodec, DecodeError, EncodeError, SequenceForm, StructuralEntry,
    StructuralForm, Textual, TextualForm,
};

/// The logos language identity — the `T` the mouth's produced
/// `TextualForm<LogosLanguage>` view value carries. A stringless marker.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct LogosLanguage;

// The logos textual universe's type ids (locals in the fixture universe namespace).
const ITEM: ScopedEncodedTypeId = ScopedEncodedTypeId::fixture(1);
const VISIBILITY: ScopedEncodedTypeId = ScopedEncodedTypeId::fixture(2);
const ATTRIBUTE: ScopedEncodedTypeId = ScopedEncodedTypeId::fixture(3);
const CONFIG_PREDICATE: ScopedEncodedTypeId = ScopedEncodedTypeId::fixture(4);
const PATH_NODE: ScopedEncodedTypeId = ScopedEncodedTypeId::fixture(5);
const TYPE_REFERENCE: ScopedEncodedTypeId = ScopedEncodedTypeId::fixture(6);

// The constructor indices inside each type's entry — the disjoint decode alternatives.
const VISIBILITY_PUBLIC: u32 = 0;
const VISIBILITY_PRIVATE: u32 = 1;
const ATTRIBUTE_TOOL_PATH: u32 = 0;
const ATTRIBUTE_CONFIGURATION: u32 = 1;
const ATTRIBUTE_DERIVE: u32 = 2;
const PATH_SINGLE: u32 = 0;
const PATH_MULTI: u32 = 1;

/// A Textual round-trip over EncodedLogos failed.
#[derive(Debug, thiserror::Error)]
enum LogosTextError {
    #[error(transparent)]
    Recognize(#[from] RecognizeError),
    #[error(transparent)]
    Decode(#[from] DecodeError),
    #[error(transparent)]
    Encode(#[from] EncodeError),
    #[error(transparent)]
    SingleChunk(#[from] structural_codec::error::SingleChunkRequired),
    #[error(transparent)]
    NamedChunk(#[from] structural_codec::error::NamedChunkRequired),
    #[error(transparent)]
    Names(#[from] NameTableError),
    #[error("the source held no root object to decode")]
    EmptySource,
    #[error("the decoded mirror did not fit the expected {0} shape")]
    ReifyShape(&'static str),
}

/// The keyword lexicon: every structural keyword the `Literal` forms match on decode
/// and resolve on encode. Kept as one `NameTable` plus the interned identities the
/// forms carry.
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
    fn build() -> Self {
        let mut names = NameTable::new(name_table::IdentifierNamespace::Logos);
        let mut keyword = |text: &str| {
            names
                .intern(Name::new(text))
                .expect("allocate fixed Logos grammar keyword")
        };
        let newtype = keyword("Newtype");
        let tool_path = keyword("ToolPath");
        let configuration = keyword("Configuration");
        let derive = keyword("Derive");
        let feature = keyword("Feature");
        let public = keyword("Public");
        let private = keyword("Private");
        Self {
            names,
            newtype,
            tool_path,
            configuration,
            derive,
            feature,
            public,
            private,
        }
    }
}

/// One textual mouth of EncodedLogos: the sealed structuretree plus the keyword lexicon.
struct TextualLogos {
    table: AddressedStructuralTable,
    lexicon: Lexicon,
}

impl TextualLogos {
    /// Author the structuretree for the golden's shapes, seal it, and prove every
    /// entry's decode alternatives pairwise disjoint.
    fn build() -> Self {
        let lexicon = Lexicon::build();
        let entries = vec![
            Self::item_entry(&lexicon),
            Self::visibility_entry(&lexicon),
            Self::attribute_entry(&lexicon),
            Self::config_predicate_entry(&lexicon),
            Self::path_node_entry(),
            Self::type_reference_entry(),
        ];
        let payload = TableIdentityPayload {
            core_universe: structural_codec::ids::FIXTURE_UNIVERSE,
            core_layout_identity: EncodedLayoutIdentity([7u8; 32]),
            raw_profile_identity: RawProfileIdentity([1u8; 32]),
            committed_lexicon: b"core-logos-textual".to_vec(),
            leaf_codec_contracts: Vec::new(),
            entries: entries
                .into_iter()
                .map(|entry| (entry.core_type, entry))
                .collect(),
        };
        let table = AddressedStructuralTable::seal(StructuralRevision::new(1), payload)
            .expect("seal the logos structuretree");
        table
            .validate_disjoint()
            .expect("every decode alternative is provably disjoint");
        Self { table, lexicon }
    }

    // ===== structuretree authoring =====

    /// A single-constructor entry.
    fn solo(core_type: ScopedEncodedTypeId, form: StructuralForm) -> StructuralEntry {
        StructuralEntry::new(
            core_type,
            vec![ConstructorCodec::new(
                EncodedConstructorId::new(core_type, 0),
                vec![form.clone()],
                form,
                PositionalSignature::default(),
            )],
        )
    }

    fn codec(core_type: ScopedEncodedTypeId, index: u32, form: StructuralForm) -> ConstructorCodec {
        ConstructorCodec::new(
            EncodedConstructorId::new(core_type, index),
            vec![form.clone()],
            form,
            PositionalSignature::default(),
        )
    }

    /// A `Head.payload` application whose head is a structural keyword `Literal`.
    fn keyword_application(keyword: Identifier, payload: StructuralForm) -> StructuralForm {
        StructuralForm::application(StructuralForm::Literal(keyword), payload)
    }

    /// A `[ … ]` vector of a repeated element.
    fn vector(element: StructuralForm) -> StructuralForm {
        StructuralForm::Delimited {
            delimiter: Delimiter::SquareBracket,
            sequence: SequenceForm::zero_or_more(element),
        }
    }

    /// `Newtype.{ <visibility> [<attrs>] <name> <wrapped_visibility> <wrapped> }` — the
    /// EncodedItem::Newtype spelling: the enum-variant keyword head over the newtype's
    /// five positional fields in declaration order.
    fn item_entry(lexicon: &Lexicon) -> StructuralEntry {
        let body = StructuralForm::Delimited {
            delimiter: Delimiter::Brace,
            sequence: SequenceForm::Product(vec![
                StructuralForm::Delegate(VISIBILITY),
                Self::vector(StructuralForm::Delegate(ATTRIBUTE)),
                StructuralForm::pascal_atom(),
                StructuralForm::Delegate(VISIBILITY),
                StructuralForm::Delegate(TYPE_REFERENCE),
            ]),
        };
        Self::solo(ITEM, Self::keyword_application(lexicon.newtype, body))
    }

    /// `Public` / `Private` — unit enum variants as bare keyword atoms, disjoint by
    /// keyword.
    fn visibility_entry(lexicon: &Lexicon) -> StructuralEntry {
        StructuralEntry::new(
            VISIBILITY,
            vec![
                Self::codec(
                    VISIBILITY,
                    VISIBILITY_PUBLIC,
                    StructuralForm::Literal(lexicon.public),
                ),
                Self::codec(
                    VISIBILITY,
                    VISIBILITY_PRIVATE,
                    StructuralForm::Literal(lexicon.private),
                ),
            ],
        )
    }

    /// The three golden attribute kinds, each `Keyword.payload`, disjoint by keyword:
    /// `ToolPath.rustfmt.skip`, `Configuration.{ <predicate> <inner> }`,
    /// `Derive.[ <paths> ]`.
    fn attribute_entry(lexicon: &Lexicon) -> StructuralEntry {
        let tool_path =
            Self::keyword_application(lexicon.tool_path, StructuralForm::Delegate(PATH_NODE));
        let configuration = Self::keyword_application(
            lexicon.configuration,
            StructuralForm::Delimited {
                delimiter: Delimiter::Brace,
                sequence: SequenceForm::Product(vec![
                    StructuralForm::Delegate(CONFIG_PREDICATE),
                    StructuralForm::Delegate(ATTRIBUTE),
                ]),
            },
        );
        let derive = Self::keyword_application(
            lexicon.derive,
            Self::vector(StructuralForm::Delegate(PATH_NODE)),
        );
        StructuralEntry::new(
            ATTRIBUTE,
            vec![
                Self::codec(ATTRIBUTE, ATTRIBUTE_TOOL_PATH, tool_path),
                Self::codec(ATTRIBUTE, ATTRIBUTE_CONFIGURATION, configuration),
                Self::codec(ATTRIBUTE, ATTRIBUTE_DERIVE, derive),
            ],
        )
    }

    /// `Feature.<name>` — the one predicate the golden uses.
    fn config_predicate_entry(lexicon: &Lexicon) -> StructuralEntry {
        let form = Self::keyword_application(
            lexicon.feature,
            StructuralForm::Atom(AtomForm {
                case: None,
                sigil: None,
            }),
        );
        Self::solo(CONFIG_PREDICATE, form)
    }

    /// A path is a glued-dot chain of segments: a single atom, or `head.rest`. The two
    /// forms are disjoint (an atom is not an application). `head.rest` consumes the
    /// head atom before delegating, so the recursion is guarded.
    fn path_node_entry() -> StructuralEntry {
        let single = StructuralForm::Atom(AtomForm {
            case: None,
            sigil: None,
        });
        let multi = StructuralForm::application(
            StructuralForm::Atom(AtomForm {
                case: None,
                sigil: None,
            }),
            StructuralForm::Delegate(PATH_NODE),
        );
        StructuralEntry::new(
            PATH_NODE,
            vec![
                Self::codec(PATH_NODE, PATH_SINGLE, single),
                Self::codec(PATH_NODE, PATH_MULTI, multi),
            ],
        )
    }

    /// The golden's wrapped type is a bare path (`Integer`); `TypeReference::Path`
    /// delegates to the path node. (Other TypeReference kinds are deferred.)
    fn type_reference_entry() -> StructuralEntry {
        Self::solo(TYPE_REFERENCE, StructuralForm::Delegate(PATH_NODE))
    }

    // ===== reify: StructuralValue mirror -> EncodedLogos =====

    fn reify_item(
        &self,
        mirror: &StructuralValue,
        names: &mut NameTable,
    ) -> Result<EncodedItem, LogosTextError> {
        let (constructor, payload) = Self::chosen(mirror, "item")?;
        // Only the Newtype constructor is authored for the golden witness.
        if constructor != 0 {
            return Err(LogosTextError::ReifyShape("item constructor"));
        }
        let body = Self::application_body(payload, "newtype application")?;
        let [visibility, attributes, name, wrapped_visibility, wrapped] =
            Self::delimited(body, "newtype fields")?;
        Ok(EncodedItem::Newtype(Newtype {
            visibility: Self::reify_visibility(visibility)?,
            attributes: self.reify_attributes(attributes, names)?,
            name: Self::atom(name, "newtype name")?,
            wrapped_visibility: Self::reify_visibility(wrapped_visibility)?,
            wrapped: self.reify_type(wrapped)?,
        }))
    }

    fn reify_visibility(mirror: &StructuralValue) -> Result<Visibility, LogosTextError> {
        let inner = Self::delegated(mirror, "visibility delegate")?;
        let (constructor, _) = Self::chosen(inner, "visibility")?;
        match constructor {
            VISIBILITY_PUBLIC => Ok(Visibility::Public),
            VISIBILITY_PRIVATE => Ok(Visibility::Private),
            _ => Err(LogosTextError::ReifyShape("visibility constructor")),
        }
    }

    fn reify_attributes(
        &self,
        mirror: &StructuralValue,
        names: &mut NameTable,
    ) -> Result<Vec<Attribute>, LogosTextError> {
        let elements = Self::delimited_vec(mirror, "attributes vector")?;
        elements
            .iter()
            .map(|element| {
                let inner = Self::delegated(element, "attribute delegate")?;
                self.reify_attribute(inner, names)
            })
            .collect()
    }

    fn reify_attribute(
        &self,
        mirror: &StructuralValue,
        names: &mut NameTable,
    ) -> Result<Attribute, LogosTextError> {
        let (constructor, payload) = Self::chosen(mirror, "attribute")?;
        let body = Self::application_body(payload, "attribute application")?;
        match constructor {
            ATTRIBUTE_TOOL_PATH => {
                let path = Self::delegated(body, "tool path delegate")?;
                Ok(Attribute::ToolPath(Self::reify_path(path)?))
            }
            ATTRIBUTE_CONFIGURATION => {
                let [predicate, inner] = Self::delimited(body, "configuration fields")?;
                Ok(Attribute::Configuration(ConfigurationAttribute {
                    predicate: self.reify_predicate(predicate, names)?,
                    inner: Box::new(
                        self.reify_attribute(
                            Self::delegated(inner, "configuration inner")?,
                            names,
                        )?,
                    ),
                }))
            }
            ATTRIBUTE_DERIVE => {
                let paths = Self::delimited_vec(body, "derive paths")?;
                let paths = paths
                    .iter()
                    .map(|path| Self::reify_path(Self::delegated(path, "derive path delegate")?))
                    .collect::<Result<_, _>>()?;
                Ok(Attribute::Derive(DeriveGroup { paths }))
            }
            _ => Err(LogosTextError::ReifyShape("attribute constructor")),
        }
    }

    fn reify_predicate(
        &self,
        mirror: &StructuralValue,
        _names: &mut NameTable,
    ) -> Result<ConfigurationPredicate, LogosTextError> {
        let inner = Self::delegated(mirror, "predicate delegate")?;
        let (constructor, payload) = Self::chosen(inner, "predicate")?;
        if constructor != 0 {
            return Err(LogosTextError::ReifyShape("predicate constructor"));
        }
        let name = Self::application_body(payload, "feature application")?;
        Ok(ConfigurationPredicate::Feature(Self::atom(
            name,
            "feature name",
        )?))
    }

    fn reify_type(&self, mirror: &StructuralValue) -> Result<TypeReference, LogosTextError> {
        let inner = Self::delegated(mirror, "type delegate")?;
        let (constructor, payload) = Self::chosen(inner, "type reference")?;
        if constructor != 0 {
            return Err(LogosTextError::ReifyShape("type reference constructor"));
        }
        let path = Self::delegated(payload, "type path delegate")?;
        Ok(TypeReference::Path(Self::reify_path(path)?))
    }

    fn reify_path(mirror: &StructuralValue) -> Result<PathNode, LogosTextError> {
        let (constructor, payload) = Self::chosen(mirror, "path")?;
        match constructor {
            PATH_SINGLE => Ok(PathNode {
                segments: vec![Self::atom(payload, "path segment")?],
            }),
            PATH_MULTI => {
                let StructuralValue::Application(head, rest) = payload else {
                    return Err(LogosTextError::ReifyShape("path application"));
                };
                let mut segments = vec![Self::atom(head, "path head segment")?];
                let inner = Self::delegated(rest, "path tail delegate")?;
                segments.extend(Self::reify_path(inner)?.segments);
                Ok(PathNode { segments })
            }
            _ => Err(LogosTextError::ReifyShape("path constructor")),
        }
    }

    // ===== reflect: EncodedLogos -> StructuralValue mirror =====

    fn reflect_item(
        &self,
        item: &EncodedItem,
        names: &mut NameTable,
    ) -> Result<StructuralValue, LogosTextError> {
        let EncodedItem::Newtype(newtype) = item else {
            return Err(LogosTextError::ReifyShape(
                "only Newtype items are authored",
            ));
        };
        let body = StructuralValue::Delimited(vec![
            self.reflect_visibility(&newtype.visibility, names),
            self.reflect_attributes(&newtype.attributes, names)?,
            StructuralValue::Atom(newtype.name),
            self.reflect_visibility(&newtype.wrapped_visibility, names),
            self.reflect_type(&newtype.wrapped, names)?,
        ]);
        Ok(StructuralValue::chosen(
            0,
            StructuralValue::Application(
                Box::new(StructuralValue::Atom(self.keyword(names, "Newtype"))),
                Box::new(body),
            ),
        ))
    }

    fn reflect_visibility(
        &self,
        visibility: &Visibility,
        names: &mut NameTable,
    ) -> StructuralValue {
        let (constructor, keyword) = match visibility {
            Visibility::Public => (VISIBILITY_PUBLIC, "Public"),
            Visibility::Private => (VISIBILITY_PRIVATE, "Private"),
            // Crate / Module are deferred; the golden uses only Public / Private.
            other => panic!("visibility {other:?} is not authored yet"),
        };
        StructuralValue::Delegated(Box::new(StructuralValue::chosen(
            constructor,
            StructuralValue::Atom(self.keyword(names, keyword)),
        )))
    }

    fn reflect_attributes(
        &self,
        attributes: &[Attribute],
        names: &mut NameTable,
    ) -> Result<StructuralValue, LogosTextError> {
        let elements = attributes
            .iter()
            .map(|attribute| {
                Ok(StructuralValue::Delegated(Box::new(
                    self.reflect_attribute(attribute, names)?,
                )))
            })
            .collect::<Result<Vec<_>, LogosTextError>>()?;
        Ok(StructuralValue::Delimited(elements))
    }

    fn reflect_attribute(
        &self,
        attribute: &Attribute,
        names: &mut NameTable,
    ) -> Result<StructuralValue, LogosTextError> {
        match attribute {
            Attribute::ToolPath(path) => Ok(Self::keyword_chosen(
                ATTRIBUTE_TOOL_PATH,
                self.keyword(names, "ToolPath"),
                StructuralValue::Delegated(Box::new(Self::reflect_path(path))),
            )),
            Attribute::Configuration(configuration) => {
                let body = StructuralValue::Delimited(vec![
                    self.reflect_predicate(&configuration.predicate, names),
                    StructuralValue::Delegated(Box::new(
                        self.reflect_attribute(&configuration.inner, names)?,
                    )),
                ]);
                Ok(Self::keyword_chosen(
                    ATTRIBUTE_CONFIGURATION,
                    self.keyword(names, "Configuration"),
                    body,
                ))
            }
            Attribute::Derive(group) => {
                let paths = group
                    .paths
                    .iter()
                    .map(|path| StructuralValue::Delegated(Box::new(Self::reflect_path(path))))
                    .collect();
                Ok(Self::keyword_chosen(
                    ATTRIBUTE_DERIVE,
                    self.keyword(names, "Derive"),
                    StructuralValue::Delimited(paths),
                ))
            }
            other => Err(LogosTextError::ReifyShape(match other {
                Attribute::Cfg(_) => "Cfg attribute deferred",
                Attribute::HelperDerive(_) => "HelperDerive attribute deferred",
                _ => "attribute deferred",
            })),
        }
    }

    fn reflect_predicate(
        &self,
        predicate: &ConfigurationPredicate,
        names: &mut NameTable,
    ) -> StructuralValue {
        let ConfigurationPredicate::Feature(feature) = predicate;
        StructuralValue::Delegated(Box::new(StructuralValue::chosen(
            0,
            StructuralValue::Application(
                Box::new(StructuralValue::Atom(self.keyword(names, "Feature"))),
                Box::new(StructuralValue::Atom(*feature)),
            ),
        )))
    }

    fn reflect_type(
        &self,
        type_reference: &TypeReference,
        _names: &mut NameTable,
    ) -> Result<StructuralValue, LogosTextError> {
        let TypeReference::Path(path) = type_reference else {
            return Err(LogosTextError::ReifyShape(
                "only Path type references authored",
            ));
        };
        Ok(StructuralValue::Delegated(Box::new(
            StructuralValue::chosen(
                0,
                StructuralValue::Delegated(Box::new(Self::reflect_path(path))),
            ),
        )))
    }

    fn reflect_path(path: &PathNode) -> StructuralValue {
        match path.segments.split_first() {
            Some((head, [])) => StructuralValue::chosen(PATH_SINGLE, StructuralValue::Atom(*head)),
            Some((head, rest)) => StructuralValue::chosen(
                PATH_MULTI,
                StructuralValue::Application(
                    Box::new(StructuralValue::Atom(*head)),
                    Box::new(StructuralValue::Delegated(Box::new(Self::reflect_path(
                        &PathNode {
                            segments: rest.to_vec(),
                        },
                    )))),
                ),
            ),
            None => {
                StructuralValue::chosen(PATH_SINGLE, StructuralValue::Atom(Identifier::Logos(0)))
            }
        }
    }

    // ===== mirror-shape helpers =====

    fn keyword_chosen(
        constructor: u32,
        keyword: Identifier,
        payload: StructuralValue,
    ) -> StructuralValue {
        StructuralValue::chosen(
            constructor,
            StructuralValue::Application(
                Box::new(StructuralValue::Atom(keyword)),
                Box::new(payload),
            ),
        )
    }

    fn keyword(&self, names: &mut NameTable, text: &str) -> Identifier {
        let _ = &self.lexicon;
        names
            .intern(Name::new(text))
            .expect("allocate fixed Logos text keyword")
    }

    fn chosen<'value>(
        value: &'value StructuralValue,
        what: &'static str,
    ) -> Result<(u32, &'value StructuralValue), LogosTextError> {
        match value {
            StructuralValue::Chosen {
                constructor,
                payload,
            } => Ok((*constructor, payload.as_ref())),
            _ => Err(LogosTextError::ReifyShape(what)),
        }
    }

    fn delegated<'value>(
        value: &'value StructuralValue,
        what: &'static str,
    ) -> Result<&'value StructuralValue, LogosTextError> {
        match value {
            StructuralValue::Delegated(inner) => Ok(inner.as_ref()),
            _ => Err(LogosTextError::ReifyShape(what)),
        }
    }

    fn application_body<'value>(
        value: &'value StructuralValue,
        what: &'static str,
    ) -> Result<&'value StructuralValue, LogosTextError> {
        match value {
            StructuralValue::Application(_head, payload) => Ok(payload.as_ref()),
            _ => Err(LogosTextError::ReifyShape(what)),
        }
    }

    fn delimited<'value, const N: usize>(
        value: &'value StructuralValue,
        what: &'static str,
    ) -> Result<&'value [StructuralValue; N], LogosTextError> {
        let slice = Self::delimited_vec(value, what)?;
        slice
            .as_slice()
            .try_into()
            .map_err(|_| LogosTextError::ReifyShape(what))
    }

    fn delimited_vec<'value>(
        value: &'value StructuralValue,
        what: &'static str,
    ) -> Result<&'value Vec<StructuralValue>, LogosTextError> {
        match value {
            StructuralValue::Delimited(children) => Ok(children),
            _ => Err(LogosTextError::ReifyShape(what)),
        }
    }

    fn atom(value: &StructuralValue, what: &'static str) -> Result<Identifier, LogosTextError> {
        match value {
            StructuralValue::Atom(identifier) => Ok(*identifier),
            _ => Err(LogosTextError::ReifyShape(what)),
        }
    }
}

impl Textual for TextualLogos {
    type Encoded = EncodedItem;
    type Language = LogosLanguage;
    type Error = LogosTextError;

    fn structuretree(&self) -> &AddressedStructuralTable {
        &self.table
    }

    fn lexicon(&self) -> Option<&dyn NameResolver> {
        Some(&self.lexicon.names)
    }

    fn missing_root_object(&self) -> LogosTextError {
        LogosTextError::EmptySource
    }

    fn reify(
        &self,
        _expected: ScopedEncodedTypeId,
        mirror: &StructuralValue,
        names: &mut NameTable,
    ) -> Result<EncodedItem, LogosTextError> {
        self.reify_item(mirror, names)
    }

    fn reflect(
        &self,
        _expected: ScopedEncodedTypeId,
        encoded: &EncodedItem,
        names: &mut NameTable,
    ) -> Result<StructuralValue, LogosTextError> {
        self.reflect_item(encoded, names)
    }
}

/// The flagship witness: the REAL golden `CommitSequence` newtype round-trips
/// value -> text -> value losslessly through the sealed structuretree + nametree, and
/// the text carries every datum — the whole attribute preamble included — visibly.
#[test]
fn golden_commit_sequence_round_trips_through_the_organs() {
    let mouth = TextualLogos::build();
    let mut names = NameTable::new(name_table::IdentifierNamespace::Logos);
    let golden = support::commit_sequence(&mut names);

    // view: the EncodedForm value -> canonical Protos text through the organs. Text
    // crosses only inside the mouth's `TextualForm<LogosLanguage>` value.
    let text: TextualForm<LogosLanguage> =
        mouth.view(ITEM, &golden, &mut names).expect("view golden");
    let text_str = text.sole_text().expect("sole view text");
    println!("golden CommitSequence text:\n{text_str}");

    // Every datum is visible — the psyche rejected any text that hides the attributes.
    for datum in [
        "Newtype",
        "Public",
        "rustfmt.skip",
        "Feature",
        "nota-text",
        "nota.NotaDecode",
        "nota.NotaDecodeTraced",
        "nota.NotaEncode",
        "rkyv.Archive",
        "rkyv.Serialize",
        "rkyv.Deserialize",
        "Clone",
        "Debug",
        "PartialEq",
        "Eq",
        "CommitSequence",
        "Private",
        "Integer",
    ] {
        assert!(
            text_str.contains(datum),
            "the text must carry `{datum}` visibly: {text_str}"
        );
    }

    // unview: the text -> the EncodedForm value, through the same organs.
    let decoded = mouth
        .unview(ITEM, &text, &mut names)
        .expect("unview golden");
    assert_eq!(golden, decoded, "value -> text -> value is lossless");

    // The recovered value re-views to byte-identical text.
    let re_viewed = mouth.view(ITEM, &decoded, &mut names).expect("re-view");
    assert_eq!(
        text, re_viewed,
        "the canonical text is stable across the round-trip"
    );
}
