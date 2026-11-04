//! Représentation en mémoire des entités et de leurs composants (ECS déclaratif).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::error::CoreError;
use crate::identifier::Identifier;
use crate::value::ScalarValue;

/// Dictionnaire déterministe des champs (`nom_champ -> valeur`) d'une instance de composant.
pub type ComponentFields = BTreeMap<Identifier, ScalarValue>;

/// État d'une entité : identifiant unique et table de ses composants attachés.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
pub struct EntityState {
    /// Identifiant unique de l'entité.
    pub id: Identifier,
    /// Composants attachés à l'entité (`id_composant -> champs`).
    pub components: BTreeMap<Identifier, ComponentFields>,
}

impl EntityState {
    /// Crée une nouvelle entité sans composant initial.
    pub fn new(id: Identifier) -> Self {
        Self {
            id,
            components: BTreeMap::new(),
        }
    }

    /// Retourne la valeur d'un champ au sein d'un composant attaché à cette entité.
    pub fn read_field(
        &self,
        component_id: &Identifier,
        field_id: &Identifier,
    ) -> Option<&ScalarValue> {
        self.components
            .get(component_id)
            .and_then(|fields| fields.get(field_id))
    }
}

/// État global du monde regroupant toutes les entités indexées par leur identifiant.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, TS)]
pub struct WorldState {
    /// Table ordonnée des entités actives.
    pub entities: BTreeMap<Identifier, EntityState>,
}

impl WorldState {
    /// Crée un état du monde vide.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insère ou remplace une entité dans l'état du monde.
    pub fn insert_entity(&mut self, entity: EntityState) {
        self.entities.insert(entity.id.clone(), entity);
    }

    /// Lit la valeur d'un champ à l'adresse donnée (`entité.composant.champ`).
    pub fn read_entity_field(&self, target: &FieldAddress) -> Option<&ScalarValue> {
        self.entities
            .get(&target.entity_id)
            .and_then(|entity| entity.read_field(&target.component_id, &target.field_id))
    }

    /// Met à jour un champ sur une entité existante et un composant existant.
    ///
    /// # Errors
    /// Retourne [`CoreError::EntityNotFound`] ou [`CoreError::ComponentNotFound`] si la cible n'existe pas.
    pub fn write_entity_field(
        &mut self,
        target: &FieldAddress,
        new_value: ScalarValue,
    ) -> Result<(), CoreError> {
        let Some(entity) = self.entities.get_mut(&target.entity_id) else {
            return Err(CoreError::EntityNotFound {
                entity_id: target.entity_id.as_str().to_owned(),
            });
        };

        let Some(component) = entity.components.get_mut(&target.component_id) else {
            return Err(CoreError::ComponentNotFound {
                entity_id: target.entity_id.as_str().to_owned(),
                component_id: target.component_id.as_str().to_owned(),
            });
        };

        component.insert(target.field_id.clone(), new_value);
        Ok(())
    }
}

/// Adresse complète d'un champ de composant sur une entité précise.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct FieldAddress {
    /// Identifiant de l'entité ciblée.
    pub entity_id: Identifier,
    /// Identifiant du composant ciblé.
    pub component_id: Identifier,
    /// Identifiant du champ ciblé.
    pub field_id: Identifier,
}
