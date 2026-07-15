//! The crate-boundary error type.

use content_identity::ArchiveError;
use name_table::NameTableError;
use thiserror::Error;

/// A failure at the `core-logos` boundary. Content-addressing failures come from
/// the portable-archive layer; name-projection failures come from the NameTable.
#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("content identity failed: {0}")]
    ContentIdentity(#[from] ArchiveError),

    #[error("name resolution failed: {0}")]
    NameResolution(#[from] NameTableError),
}
