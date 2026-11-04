//! Types d'erreurs pour la persistance et les adaptateurs de stockage.

use thiserror::Error;

/// Erreurs survenant lors de la persistance ou de la restauration d'une sauvegarde.
#[derive(Debug, Error)]
pub enum DataError {
    /// L'emplacement de sauvegarde demandé est introuvable.
    #[error("slot de sauvegarde introuvable : '{0}'")]
    SlotNotFound(String),

    /// L'identifiant de slot de sauvegarde est invalide (sécurité ou format).
    #[error("nom de slot invalide : '{0}' (doit être non-vide, alphanumérique sans séparateur de chemin)")]
    InvalidSlotName(String),

    /// Erreur d'entrée/sortie sur le système de fichiers.
    #[error("erreur I/O : {0}")]
    Io(#[from] std::io::Error),

    /// Erreur lors de la sérialisation ou désérialisation JSON.
    #[error("erreur de sérialisation JSON : {0}")]
    Serialization(#[from] serde_json::Error),

    /// Les données sauvegardées sont corrompues ou incomplètes.
    #[error("données de sauvegarde corrompues : {0}")]
    CorruptedData(String),

    /// La version du schéma de la sauvegarde est incompatible avec le runtime actuel.
    #[error("version de schéma incompatible (attendu '{expected}', reçu '{actual}')")]
    IncompatibleVersion {
        /// Version supportée par le runtime.
        expected: String,
        /// Version trouvée dans la sauvegarde.
        actual: String,
    },
}
