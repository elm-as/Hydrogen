//! Gestion et validation de la version du schéma de définition Hydrogen.

use std::fmt;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::error::SchemaError;

/// Version courante du schéma supportée par `hydrogen-schema`.
pub const CURRENT_SCHEMA_VERSION: SchemaVersion = SchemaVersion {
    major: 0,
    minor: 1,
    patch: 0,
};

/// Version sémantique (`MAJOR.MINOR.PATCH`) d'un fichier de définition Hydrogen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, TS)]
pub struct SchemaVersion {
    /// Version majeure (changements cassants).
    pub major: u16,
    /// Version mineure (ajouts rétrocompatibles).
    pub minor: u16,
    /// Version de correctif.
    pub patch: u16,
}

impl SchemaVersion {
    /// Crée une version de schéma explicite.
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Vérifie que cette version est compatible avec [`CURRENT_SCHEMA_VERSION`].
    ///
    /// # Errors
    /// Retourne [`SchemaError::UnsupportedVersion`] si la version majeure diffère ou si
    /// la version mineure demandée est supérieure à celle du moteur.
    pub fn ensure_compatible(self) -> Result<(), SchemaError> {
        let is_same_major = self.major == CURRENT_SCHEMA_VERSION.major;
        let is_supported_minor = self.minor <= CURRENT_SCHEMA_VERSION.minor;

        if !is_same_major || !is_supported_minor {
            return Err(SchemaError::UnsupportedVersion {
                found: self.to_string(),
                supported: CURRENT_SCHEMA_VERSION.to_string(),
            });
        }
        Ok(())
    }
}

impl Default for SchemaVersion {
    fn default() -> Self {
        CURRENT_SCHEMA_VERSION
    }
}

impl fmt::Display for SchemaVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}
