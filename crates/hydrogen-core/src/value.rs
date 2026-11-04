//! Types et valeurs scalaires manipulés par les composants, événements et règles.

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::error::CoreError;

/// Type de donnée scalaire supporté par un champ de composant ou d'événement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum ValueKind {
    /// Booléen (`true` / `false`).
    Bool,
    /// Entier signé 64 bits.
    Int,
    /// Nombre flottant 64 bits fini.
    Float,
    /// Chaîne de caractères UTF-8.
    Text,
}

/// Valeur dynamique typée stockée dans un champ de composant ou une charge utile d'événement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(tag = "kind", content = "data", rename_all = "snake_case")]
pub enum ScalarValue {
    /// Valeur booléenne.
    Bool(bool),
    /// Valeur entière 64 bits.
    Int(#[ts(type = "number")] i64),
    /// Valeur flottante 64 bits.
    Float(f64),
    /// Valeur textuelle.
    Text(String),
}

impl ScalarValue {
    /// Construit une valeur flottante en garantissant qu'elle est finie (`!NaN` et `!Infinite`).
    ///
    /// # Errors
    /// Retourne [`CoreError::NonFiniteFloat`] si `number` est `NaN` ou infini.
    pub fn try_float(number: f64) -> Result<Self, CoreError> {
        if !number.is_finite() {
            return Err(CoreError::NonFiniteFloat);
        }
        Ok(Self::Float(number))
    }

    /// Retourne le [`ValueKind`] correspondant à cette valeur.
    pub const fn kind(&self) -> ValueKind {
        match self {
            Self::Bool(_) => ValueKind::Bool,
            Self::Int(_) => ValueKind::Int,
            Self::Float(_) => ValueKind::Float,
            Self::Text(_) => ValueKind::Text,
        }
    }

    /// Vérifie que la valeur est valide pour une exécution déterministe (flottants finis).
    pub fn is_deterministic(&self) -> bool {
        match self {
            Self::Float(number) => number.is_finite(),
            Self::Bool(_) | Self::Int(_) | Self::Text(_) => true,
        }
    }
}
