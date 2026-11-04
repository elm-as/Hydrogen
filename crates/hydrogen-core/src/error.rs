//! Erreurs typées du domaine fondamental `hydrogen-core`.

use thiserror::Error;

/// Erreur survenant lors de la manipulation des primitives du cœur Hydrogen.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CoreError {
    /// L'identifiant est vide ou dépasse la longueur maximale autorisée (64 caractères).
    #[error("identifiant hors limites (attendu 1..=64 caractères, reçu {length})")]
    InvalidIdentifierLength {
        /// Longueur reçue en octets.
        length: usize,
    },

    /// L'identifiant ne respecte pas le format `snake_case` ASCII (`[a-z][a-z0-9_]*`).
    #[error(
        "identifiant invalide '{raw}' : doit commencer par [a-z] et ne contenir que [a-z0-9_]"
    )]
    InvalidIdentifierFormat {
        /// Chaîne brute rejetée.
        raw: String,
    },

    /// Nombre flottant non fini (`NaN` ou `Infinity`), interdit pour préserver le déterminisme.
    #[error("valeur flottante non finie interdite dans le moteur déterministe")]
    NonFiniteFloat,

    /// Entité introuvable dans l'état courant.
    #[error("entité introuvable : '{entity_id}'")]
    EntityNotFound {
        /// Identifiant de l'entité recherchée.
        entity_id: String,
    },

    /// Composant introuvable sur l'entité ciblée.
    #[error("composant '{component_id}' introuvable sur l'entité '{entity_id}'")]
    ComponentNotFound {
        /// Identifiant de l'entité ciblée.
        entity_id: String,
        /// Identifiant du composant recherché.
        component_id: String,
    },
}
