//! Schémas de définition des composants, des entités initiales et des événements.

use std::collections::BTreeMap;

use hydrogen_core::{ComponentFields, Identifier, ScalarValue, ValueKind};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Définition d'un champ typé au sein d'un schéma de composant.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
pub struct ComponentFieldDef {
    /// Type scalaire attendu pour ce champ.
    pub kind: ValueKind,
    /// Valeur par défaut appliquée et vérifiée au chargement.
    pub default_value: ScalarValue,
}

/// Schéma déclaratif d'un composant réutilisable entre entités.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
pub struct ComponentDef {
    /// Identifiant unique du composant.
    pub id: Identifier,
    /// Table ordonnée des champs du composant (`id_champ -> définition`).
    pub fields: BTreeMap<Identifier, ComponentFieldDef>,
}

/// Définition déclarative d'une entité initiale dans un projet Hydrogen.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
pub struct EntityDef {
    /// Identifiant unique de l'entité.
    pub id: Identifier,
    /// Valeurs initiales des composants attachés (`id_composant -> (id_champ -> valeur)`).
    pub components: BTreeMap<Identifier, ComponentFields>,
}

/// Schéma déclaratif d'un événement pouvant être émis ou écouté par les règles.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
pub struct EventDef {
    /// Identifiant unique de l'événement.
    pub id: Identifier,
    /// Types attendus pour la charge utile de l'événement (`id_champ -> type`).
    pub payload_fields: BTreeMap<Identifier, ValueKind>,
}
