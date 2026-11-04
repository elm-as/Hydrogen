//! Erreurs typées du moteur d'exécution `hydrogen-runtime`.

use hydrogen_core::CoreError;
use hydrogen_schema::SchemaError;
use thiserror::Error;

/// Erreur survenant lors de l'exécution ou de l'initialisation du runtime.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum RuntimeError {
    /// Erreur provenant du modèle fondamental `hydrogen-core`.
    #[error("erreur core : {0}")]
    Core(#[from] CoreError),

    /// Erreur de schéma ou de validation.
    #[error("erreur schéma : {0}")]
    Schema(#[from] SchemaError),

    /// Dépassement arithmétique lors d'une opération `AddNumeric`.
    #[error("dépassement arithmétique sur le champ '{target}'")]
    ArithmeticOverflow {
        /// Adresse du champ concerné.
        target: String,
    },

    /// Boucle d'événements infinie détectée (dépassement du quota par tick).
    #[error(
        "limite de boucle d'événements dépassée ({limit} événements traités dans le même cycle)"
    )]
    EventLoopLimitExceeded {
        /// Nombre maximal autorisé d'événements par cycle.
        limit: usize,
    },
}
