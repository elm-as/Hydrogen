//! Structure racine d'un projet Hydrogen (`ProjectDef`) et fonctions de sérialisation JSON.

use hydrogen_core::Identifier;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::error::SchemaError;
use crate::model::{ComponentDef, EntityDef, EventDef};
use crate::rule::RuleDef;
use crate::validation::validate_project_def;
use crate::version::SchemaVersion;

/// Définition déclarative complète d'un projet Hydrogen (source de vérité unique).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
pub struct ProjectDef {
    /// Version du schéma de définition utilisé.
    pub schema_version: SchemaVersion,
    /// Identifiant unique du projet.
    pub project_id: Identifier,
    /// Nom d'affichage lisible du projet.
    pub display_name: String,
    /// Catalogue des schémas de composants du projet.
    pub components: Vec<ComponentDef>,
    /// Entités initiales instanciées au démarrage.
    pub entities: Vec<EntityDef>,
    /// Catalogue des schémas d'événements du projet.
    pub events: Vec<EventDef>,
    /// Règles déclaratives orchestrant l'évolution de l'état.
    pub rules: Vec<RuleDef>,
}

impl ProjectDef {
    /// Vérifie tous les invariants sémantiques et de typage de cette définition.
    ///
    /// # Errors
    /// Retourne une [`SchemaError`] si la définition est invalide.
    pub fn validate(&self) -> Result<(), SchemaError> {
        validate_project_def(self)
    }

    /// Désérialise et valide immédiatement une définition de projet depuis une chaîne JSON.
    ///
    /// # Errors
    /// Retourne [`SchemaError::JsonFormat`] si le JSON est mal formé, ou l'erreur de validation.
    pub fn from_json_str(json: &str) -> Result<Self, SchemaError> {
        let project: Self =
            serde_json::from_str(json).map_err(|error| SchemaError::JsonFormat {
                message: error.to_string(),
            })?;
        project.validate()?;
        Ok(project)
    }

    /// Valide et sérialise la définition de projet en JSON formaté déterministe.
    ///
    /// # Errors
    /// Retourne une [`SchemaError`] si la définition est invalide ou non sérialisable.
    pub fn to_pretty_json(&self) -> Result<String, SchemaError> {
        self.validate()?;
        serde_json::to_string_pretty(self).map_err(|error| SchemaError::JsonFormat {
            message: error.to_string(),
        })
    }
}
