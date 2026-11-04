//! Identifiants métier validés (`snake_case` ASCII strict) pour entités, composants et règles.

use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use ts_rs::TS;

use crate::error::CoreError;

const MAX_IDENTIFIER_LENGTH: usize = 64;

/// Identifiant canonique immuable respectant le motif `[a-z][a-z0-9_]{0,63}`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, TS)]
#[ts(type = "string")]
pub struct Identifier(String);

impl Serialize for Identifier {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for Identifier {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = String::deserialize(deserializer)?;
        Self::new(raw).map_err(serde::de::Error::custom)
    }
}

impl Identifier {
    /// Crée et valide un nouvel identifiant métier.
    ///
    /// # Errors
    /// Retourne [`CoreError::InvalidIdentifierLength`] si la longueur n'est pas dans `1..=64`,
    /// ou [`CoreError::InvalidIdentifierFormat`] si le format `[a-z][a-z0-9_]*` n'est pas respecté.
    pub fn new(raw: impl Into<String>) -> Result<Self, CoreError> {
        let candidate = raw.into();
        validate_identifier_str(&candidate)?;
        Ok(Self(candidate))
    }

    /// Retourne la vue en tranche de caractères de l'identifiant.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Vérifie qu'une chaîne respecte les invariants d'un identifiant Hydrogen.
///
/// # Errors
/// Retourne une [`CoreError`] si la chaîne est vide, trop longue ou mal formée.
pub fn validate_identifier_str(candidate: &str) -> Result<(), CoreError> {
    let length = candidate.len();
    if length == 0 || length > MAX_IDENTIFIER_LENGTH {
        return Err(CoreError::InvalidIdentifierLength { length });
    }

    let mut bytes = candidate.bytes();
    let Some(first) = bytes.next() else {
        return Err(CoreError::InvalidIdentifierLength { length: 0 });
    };

    if !first.is_ascii_lowercase() {
        return Err(CoreError::InvalidIdentifierFormat {
            raw: candidate.to_owned(),
        });
    }

    let is_valid_tail =
        bytes.all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_');
    if !is_valid_tail {
        return Err(CoreError::InvalidIdentifierFormat {
            raw: candidate.to_owned(),
        });
    }

    Ok(())
}

impl fmt::Display for Identifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl TryFrom<String> for Identifier {
    type Error = CoreError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for Identifier {
    type Error = CoreError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<Identifier> for String {
    fn from(identifier: Identifier) -> Self {
        identifier.0
    }
}
