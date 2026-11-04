//! Erreurs typées et codes de retour de l'outil en ligne de commande `hydrogen-cli`.

use std::path::PathBuf;

use hydrogen_runtime::RuntimeError;
use hydrogen_schema::SchemaError;
use thiserror::Error;

/// Erreurs retournées par les commandes de la CLI Hydrogen.
#[derive(Debug, Error)]
pub enum CliError {
    /// Erreur d'accès ou de manipulation du système de fichiers.
    #[error("erreur d'entrée/sortie : {0}")]
    Io(#[from] std::io::Error),

    /// Erreur de validation ou de désérialisation du schéma.
    #[error("erreur de schéma : {0}")]
    Schema(#[from] SchemaError),

    /// Erreur survenue pendant l'exécution d'un cycle runtime.
    #[error("erreur d'exécution : {0}")]
    Runtime(#[from] RuntimeError),

    /// Le répertoire de destination n'est pas vide.
    #[error("le répertoire cible n'est pas vide : {path}")]
    DirectoryNotEmpty {
        /// Chemin du répertoire rejeté.
        path: PathBuf,
    },

    /// Le fichier ciblé n'a pas pu être lu ou désérialisé.
    #[error("fichier de projet invalide à '{path}' : {reason}")]
    InvalidProjectFile {
        /// Chemin du fichier en cause.
        path: PathBuf,
        /// Description de l'anomalie.
        reason: String,
    },

    /// Le modèle demandé est inconnu.
    #[error("template inconnu '{template}' (templates disponibles : 'app', 'game')")]
    UnknownTemplate {
        /// Nom du template demandé.
        template: String,
    },
}

impl CliError {
    /// Retourne le code de sortie du processus associé à cette erreur.
    pub const fn exit_code(&self) -> i32 {
        match self {
            Self::Schema(_) | Self::InvalidProjectFile { .. } | Self::UnknownTemplate { .. } => 1,
            Self::Runtime(_) => 2,
            Self::Io(_) | Self::DirectoryNotEmpty { .. } => 3,
        }
    }
}
