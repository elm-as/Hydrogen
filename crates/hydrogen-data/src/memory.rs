//! Adaptateur de stockage en mémoire vive (zéro I/O).

use std::collections::BTreeMap;

use crate::error::DataError;
use crate::snapshot::SaveSnapshot;
use crate::store::{validate_slot_name, SaveStore};

/// Adaptateur en mémoire volatile, sans entrée/sortie disque.
#[derive(Debug, Default, Clone)]
pub struct MemoryStore {
    slots: BTreeMap<String, SaveSnapshot>,
}

impl MemoryStore {
    /// Crée un nouveau magasin en mémoire vide.
    pub fn new() -> Self {
        Self::default()
    }

    /// Nombre de sauvegardes actuellement stockées.
    pub fn len(&self) -> usize {
        self.slots.len()
    }

    /// Indique si le magasin est vide.
    pub fn is_empty(&self) -> bool {
        self.slots.is_empty()
    }

    /// Vide l'ensemble des slots en mémoire.
    pub fn clear(&mut self) {
        self.slots.clear();
    }
}

impl SaveStore for MemoryStore {
    fn save(&mut self, slot: &str, snapshot: &SaveSnapshot) -> Result<(), DataError> {
        validate_slot_name(slot)?;
        self.slots.insert(slot.to_string(), snapshot.clone());
        Ok(())
    }

    fn load(&self, slot: &str) -> Result<SaveSnapshot, DataError> {
        validate_slot_name(slot)?;
        self.slots
            .get(slot)
            .cloned()
            .ok_or_else(|| DataError::SlotNotFound(slot.to_string()))
    }

    fn delete(&mut self, slot: &str) -> Result<bool, DataError> {
        validate_slot_name(slot)?;
        Ok(self.slots.remove(slot).is_some())
    }

    fn list_slots(&self) -> Result<Vec<String>, DataError> {
        Ok(self.slots.keys().cloned().collect())
    }
}
