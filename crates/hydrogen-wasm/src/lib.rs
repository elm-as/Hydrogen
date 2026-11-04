//! Bindings WebAssembly du moteur déclaratif Hydrogen (`hydrogen-wasm`).
//! Expose `WasmRuntime` pour exécuter les projets directement dans le navigateur via `wasm-bindgen`.

use hydrogen_core::RuntimeEvent;
use hydrogen_runtime::{Runtime, RuntimeConfig};
use hydrogen_schema::ProjectDef;
use wasm_bindgen::prelude::*;

/// Wrapper WebAssembly isolant une instance de moteur d'exécution Hydrogen.
#[wasm_bindgen]
pub struct WasmRuntime {
    inner: Runtime,
}

#[wasm_bindgen]
impl WasmRuntime {
    /// Initialise un nouveau moteur déclaratif depuis la chaîne JSON d'une définition de projet.
    ///
    /// # Errors
    /// Retourne une erreur sous forme de chaîne si le schéma est invalide ou incompatible.
    #[wasm_bindgen(constructor)]
    pub fn new(project_json: &str) -> Result<WasmRuntime, String> {
        let project = ProjectDef::from_json_str(project_json).map_err(|err| err.to_string())?;

        let inner =
            Runtime::new(project, RuntimeConfig::default()).map_err(|err| err.to_string())?;

        Ok(Self { inner })
    }

    /// Exécute un cycle complet (règles et cascades d'événements) et retourne le rapport en JSON.
    ///
    /// # Errors
    /// Retourne une erreur en cas de boucle infinie ou de débordement arithmétique.
    #[wasm_bindgen]
    pub fn tick(&mut self) -> Result<String, String> {
        let report = self.inner.tick().map_err(|err| err.to_string())?;

        serde_json::to_string(&report).map_err(|err| err.to_string())
    }

    /// Publie un événement au format JSON dans la file du runtime pour traitement au prochain tick.
    ///
    /// # Errors
    /// Retourne une erreur si la charge utile de l'événement est mal formée.
    #[wasm_bindgen]
    pub fn publish_event(&mut self, event_json: &str) -> Result<(), String> {
        let event: RuntimeEvent =
            serde_json::from_str(event_json).map_err(|err| err.to_string())?;

        self.inner.publish_event(event);
        Ok(())
    }

    /// Exporte l'état complet du monde sous forme de chaîne JSON déterministe.
    ///
    /// # Errors
    /// Retourne une erreur si la sérialisation JSON échoue.
    #[wasm_bindgen]
    pub fn get_world_state_json(&self) -> Result<String, String> {
        serde_json::to_string(self.inner.world()).map_err(|err| err.to_string())
    }

    /// Retourne la définition initiale du projet au format JSON.
    ///
    /// # Errors
    /// Retourne une erreur si la sérialisation JSON échoue.
    #[wasm_bindgen]
    pub fn get_project_json(&self) -> Result<String, String> {
        serde_json::to_string(self.inner.project()).map_err(|err| err.to_string())
    }

    /// Retourne le nombre total de cycles complétés depuis le démarrage.
    #[wasm_bindgen]
    pub fn get_tick_count(&self) -> u64 {
        self.inner.tick_count()
    }
}
