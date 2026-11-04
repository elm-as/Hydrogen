//! Erreurs typées de validation et de sérialisation des schémas Hydrogen.

use thiserror::Error;

/// Erreur retournée lors de la validation ou du décodage d'une définition de projet Hydrogen.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SchemaError {
    /// La version du schéma n'est pas compatible avec le moteur courant.
    #[error("version de schéma non supportée : trouvé {found}, supporté {supported}")]
    UnsupportedVersion {
        /// Version présente dans le fichier de définition.
        found: String,
        /// Version supportée par le moteur.
        supported: String,
    },

    /// Deux éléments de même catégorie partagent le même identifiant.
    #[error("identifiant dupliqué '{id}' dans la section {section}")]
    DuplicateIdentifier {
        /// Catégorie concernée (`components`, `entities`, `events`, `rules`).
        section: &'static str,
        /// Identifiant en doublon.
        id: String,
    },

    /// Une valeur flottante non finie (`NaN` ou `Infinity`) a été détectée.
    #[error("valeur flottante non finie détectée dans {context}")]
    NonDeterministicValue {
        /// Emplacement de la valeur invalide.
        context: String,
    },

    /// Le type d'une valeur ne correspond pas au [`hydrogen_core::ValueKind`] déclaré dans le schéma.
    #[error("type invalide pour '{target}' : attendu {expected:?}, reçu {actual:?}")]
    TypeMismatch {
        /// Chemin complet du champ en erreur.
        target: String,
        /// Type attendu par le schéma.
        expected: hydrogen_core::ValueKind,
        /// Type effectivement fourni.
        actual: hydrogen_core::ValueKind,
    },

    /// Une référence pointe vers un composant, une entité, un champ ou un événement inexistant.
    #[error("référence introuvable dans {context} : {target}")]
    UnknownReference {
        /// Contexte de la référence (entité ou règle).
        context: String,
        /// Cible introuvable.
        target: String,
    },

    /// Une opération numérique (`AddNumeric`, comparaison `<`/`>`) cible un champ non numérique.
    #[error("opération numérique invalide sur '{target}' de type {actual:?}")]
    InvalidNumericOperation {
        /// Chemin du champ ou de la règle.
        target: String,
        /// Type non numérique rencontré.
        actual: hydrogen_core::ValueKind,
    },

    /// Une règle ne contient aucune action à exécuter.
    #[error("la règle '{rule_id}' doit contenir au moins une action")]
    EmptyRuleActions {
        /// Identifiant de la règle vide.
        rule_id: String,
    },

    /// Un arbre d'expression dépasse la profondeur maximale autorisée.
    #[error("expression trop profonde dans {context} (maximum {max_depth} niveaux)")]
    ExpressionTooDeep {
        /// Emplacement de l'expression.
        context: String,
        /// Profondeur maximale autorisée.
        max_depth: usize,
    },

    /// Erreur de format JSON lors du chargement ou de l'export.
    #[error("erreur JSON : {message}")]
    JsonFormat {
        /// Description de l'erreur `serde_json`.
        message: String,
    },
}
