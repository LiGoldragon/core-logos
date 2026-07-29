//! The ordered whole-Logos carrier used by the first vertical slice.
//!
//! This is deliberately separate from the legacy per-item algebra. Every name
//! position carries a complete production encoded-ID chain, while the supported
//! item vocabulary is only the attribute-free, non-generic tuple newtype needed
//! by the first executable witness.

use capsule_content_identity::{
    ArchiveError, ContentAddressedHash, IdentityHasher, PortableArchive,
};
use signal_sema_translator::VocabularyEncodedId;

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
                    if newtype.name().chain().is_empty() {
                        return Err(WholeLogosArchiveError::EmptyEncodedId {
                            item_index,
                            position: NewtypeEncodedIdPosition::Name,
                        });
                    }
                    if newtype.wrapped().chain().is_empty() {
                        return Err(WholeLogosArchiveError::EmptyEncodedId {
                            item_index,
                            position: NewtypeEncodedIdPosition::Wrapped,
                        });
                    }
                }
            }
        }
        Ok(())
    }
}

/// The closed item vocabulary admitted by [`WholeLogos`] in the first slice.
#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub enum WholeLogosItem {
    /// An attribute-free, non-generic tuple newtype.
    Newtype(WholeLogosNewtype),
}

/// An attribute-free, non-generic tuple newtype, stored positionally.
///
/// The positions are item visibility, declared encoded ID, wrapped-field
/// visibility, and referenced-type encoded ID. Both IDs retain their complete
/// root-fronted chains opaquely; this carrier neither resolves nor rewrites
/// them.
#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct WholeLogosNewtype(
    WholeLogosVisibility,
    VocabularyEncodedId,
    WholeLogosVisibility,
    VocabularyEncodedId,
);

impl WholeLogosNewtype {
    /// Construct one positional newtype item.
    pub fn new(
        visibility: WholeLogosVisibility,
        name: VocabularyEncodedId,
        wrapped_visibility: WholeLogosVisibility,
        wrapped: VocabularyEncodedId,
    ) -> Self {
        Self(visibility, name, wrapped_visibility, wrapped)
    }

    /// The item visibility.
    pub const fn visibility(&self) -> &WholeLogosVisibility {
        &self.0
    }

    /// The declaration's complete encoded-ID chain.
    pub const fn name(&self) -> &VocabularyEncodedId {
        &self.1
    }

    /// The tuple field's visibility.
    pub const fn wrapped_visibility(&self) -> &WholeLogosVisibility {
        &self.2
    }

    /// The wrapped type's complete encoded-ID chain.
    pub const fn wrapped(&self) -> &VocabularyEncodedId {
        &self.3
    }
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
pub enum NewtypeEncodedIdPosition {
    /// The declaration identity.
    Name,
    /// The wrapped type reference.
    Wrapped,
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
        position: NewtypeEncodedIdPosition,
    },
}
