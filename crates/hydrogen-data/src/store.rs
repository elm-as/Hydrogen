//! Trait abstrait de stockage et validation des identifiants de slots.

use crate::error::DataError;
use crate::snapshot::SaveSnapshot;

/// Contrat de persistance pour les sauvegardes de simulation.
pub trait SaveStore {
    /// Enregistre un instantané dans le slot désigné.
    ///
    /// # Errors
    /// Retourne [`DataError::InvalidSlotName`] si le nom est non conforme,
    /// ou une erreur d'entrée/sortie selon l'adaptateur.
    fn save(&mut self, slot: &str, snapshot: &SaveSnapshot) -> Result<(), DataError>;

    /// Charge l'instantané présent dans le slot désigné.
    ///
    /// # Errors
    /// Retourne [`DataError::SlotNotFound`] si l'emplacement est absent,
    /// [`DataError::InvalidSlotName`] si le nom est non conforme,
    /// ou [`DataError::CorruptedData`] si les données ne peuvent pas être décodées.
    fn load(&self, slot: &str) -> Result<SaveSnapshot, DataError>;

    /// Supprime la sauvegarde du slot désigné. Retourne `true` si le slot existait.
    ///
    /// # Errors
    /// Retourne [`DataError::InvalidSlotName`] si le nom est non conforme,
    /// ou une erreur d'entrée/sortie selon l'adaptateur.
    fn delete(&mut self, slot: &str) -> Result<bool, DataError>;

    /// Liste l'ensemble des slots disponibles, ordonnés alphabétiquement.
    ///
    /// # Errors
    /// Retourne une erreur I/O si le parcours du stockage échoue.
    fn list_slots(&self) -> Result<Vec<String>, DataError>;

    /// Vérifie si un slot donné existe.
    ///
    /// # Errors
    /// Retourne [`DataError::InvalidSlotName`] si le nom est non conforme,
    /// ou une erreur selon l'adaptateur sous-jacent.
    fn exists(&self, slot: &str) -> Result<bool, DataError> {
        validate_slot_name(slot)?;
        let slots = self.list_slots()?;
        Ok(slots.iter().any(|s| s == slot))
    }
}

/// Valide qu'un nom de slot est sécurisé contre toute traversée de chemin.
///
/// Doit être composé de caractères alphanumériques ASCII, tirets ou underscores,
/// avec une longueur comprise entre 1 et 64 caractères.
///
/// # Errors
/// Retourne [`DataError::InvalidSlotName`] si le nom est vide, trop long ou contient des caractères interdits.
pub fn validate_slot_name(slot: &str) -> Result<(), DataError> {
    if slot.is_empty() || slot.len() > 64 {
        return Err(DataError::InvalidSlotName(slot.to_string()));
    }

    let is_valid = slot
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
    if !is_valid {
        return Err(DataError::InvalidSlotName(slot.to_string()));
    }

    Ok(())
}
