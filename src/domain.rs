//! The content-identity domain for CoreLogos values.

use content_identity::{DomainSeparation, HashDomain, LayoutVersion};

/// The layout-versioned hash domain for every CoreLogos value. The domain carries
/// the layout version in the type, so "which Core layout" is never a
/// hand-remembered suffix. The NameTable is excluded from every CoreLogos
/// pre-image (it is not part of a Core value), so a rename is hash-stable by
/// construction and a structural edit moves the identity.
pub struct CoreLogosDomain;

impl HashDomain for CoreLogosDomain {
    fn separation() -> DomainSeparation {
        DomainSeparation::Contextual {
            context: "core-logos 2026 stringless core algebra of logos",
            layout: LayoutVersion::new(1),
        }
    }
}
