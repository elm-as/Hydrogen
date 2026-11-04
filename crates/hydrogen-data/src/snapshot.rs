//! Structure immuable d'instantané de sauvegarde.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use hydrogen_core::WorldState;
use hydrogen_schema::SchemaVersion;

use crate::error::DataError;

/// Instantané d'état de simulation sérialisable et versionné.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveSnapshot {
    /// Version du schéma avec laquelle la sauvegarde a été produite.
    pub version: SchemaVersion,
    /// Nom ou identifiant du projet source.
    pub project_name: String,
    /// Horodatage logique (compteur de ticks).
    pub tick: u64,
    /// État complet du monde d'entités et composants.
    pub world_state: WorldState,
    /// Métadonnées libres (ex. horodatage réel ISO-8601, libellé utilisateur).
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub metadata: BTreeMap<String, String>,
}

impl SaveSnapshot {
    /// Crée un nouvel instantané de sauvegarde horodaté par un tick.
    pub fn new(project_name: impl Into<String>, tick: u64, world_state: WorldState) -> Self {
        Self {
            version: SchemaVersion::current(),
            project_name: project_name.into(),
            tick,
            world_state,
            metadata: BTreeMap::new(),
        }
    }

    /// Associe une métadonnée textuelle à l'instantané.
    #[must_use]
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Vérifie la compatibilité de version de schéma de cet instantané.
    ///
    /// # Errors
    /// Retourne [`DataError::IncompatibleVersion`] si la version diffère de la version attendue.
    pub fn validate_version(&self, expected: &SchemaVersion) -> Result<(), DataError> {
        if &self.version == expected {
            return Ok(());
        }

        Err(DataError::IncompatibleVersion {
            expected: expected.to_string(),
            actual: self.version.to_string(),
        })
    }
}
