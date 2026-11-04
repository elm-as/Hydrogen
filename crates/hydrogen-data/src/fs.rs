//! Adaptateur de persistance atomique sur le système de fichiers local.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::error::DataError;
use crate::snapshot::SaveSnapshot;
use crate::store::{validate_slot_name, SaveStore};

/// Adaptateur de stockage écrivant chaque slot sous forme de fichier JSON atomique.
#[derive(Debug, Clone)]
pub struct FsStore {
    root_dir: PathBuf,
}

impl FsStore {
    /// Crée un adaptateur de persistance disque et garantit l'existence du dossier racine.
    ///
    /// # Errors
    /// Retourne [`DataError::Io`] si la création du dossier racine échoue.
    pub fn new(root_dir: impl Into<PathBuf>) -> Result<Self, DataError> {
        let path = root_dir.into();
        if !path.exists() {
            fs::create_dir_all(&path)?;
        }
        Ok(Self { root_dir: path })
    }

    /// Référence vers le dossier racine contenant les sauvegardes.
    pub fn root_dir(&self) -> &Path {
        &self.root_dir
    }

    fn slot_path(&self, slot: &str) -> PathBuf {
        self.root_dir.join(format!("{slot}.json"))
    }

    fn temp_path(&self, slot: &str) -> PathBuf {
        self.root_dir.join(format!("{slot}.tmp"))
    }
}

impl SaveStore for FsStore {
    fn save(&mut self, slot: &str, snapshot: &SaveSnapshot) -> Result<(), DataError> {
        validate_slot_name(slot)?;

        let payload = serde_json::to_vec_pretty(snapshot)?;
        let temp_path = self.temp_path(slot);
        let final_path = self.slot_path(slot);

        // Écriture atomique via fichier temporaire synchronisé puis renommé
        let mut file = fs::File::create(&temp_path)?;
        file.write_all(&payload)?;
        file.sync_all()?;
        drop(file);

        fs::rename(&temp_path, &final_path)?;
        Ok(())
    }

    fn load(&self, slot: &str) -> Result<SaveSnapshot, DataError> {
        validate_slot_name(slot)?;
        let path = self.slot_path(slot);

        if !path.exists() {
            return Err(DataError::SlotNotFound(slot.to_string()));
        }

        let content = fs::read_to_string(&path)?;
        serde_json::from_str(&content)
            .map_err(|err| DataError::CorruptedData(format!("{err} ({slot})")))
    }

    fn delete(&mut self, slot: &str) -> Result<bool, DataError> {
        validate_slot_name(slot)?;
        let path = self.slot_path(slot);

        if !path.exists() {
            return Ok(false);
        }

        fs::remove_file(&path)?;
        Ok(true)
    }

    fn list_slots(&self) -> Result<Vec<String>, DataError> {
        let entries = fs::read_dir(&self.root_dir)?;
        let mut slots = Vec::new();

        for entry_res in entries {
            let entry = entry_res?;
            let path = entry.path();

            let is_json = path.extension().is_some_and(|ext| ext == "json");
            if !is_json {
                continue;
            }

            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                if validate_slot_name(stem).is_ok() {
                    slots.push(stem.to_string());
                }
            }
        }

        slots.sort();
        Ok(slots)
    }
}
