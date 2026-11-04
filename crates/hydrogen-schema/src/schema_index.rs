//! Index de résolution des références d'un projet, partagé par les validateurs.

use std::collections::BTreeMap;

use hydrogen_core::{FieldAddress, Identifier, ValueKind};

use crate::error::SchemaError;
use crate::model::{ComponentDef, EntityDef, EventDef};

/// Tables de résolution rapide construites une seule fois par validation de projet.
pub(crate) struct SchemaIndex<'a> {
    pub(crate) components: BTreeMap<&'a Identifier, &'a ComponentDef>,
    pub(crate) entities: BTreeMap<&'a Identifier, &'a EntityDef>,
    pub(crate) events: BTreeMap<&'a Identifier, &'a EventDef>,
}

impl SchemaIndex<'_> {
    /// Résout le type déclaré d'un champ adressé (`entité.composant.champ`).
    ///
    /// # Errors
    /// Retourne [`SchemaError::UnknownReference`] si l'entité, le composant ou le champ
    /// n'existe pas, ou si l'entité ne porte pas ce composant.
    pub(crate) fn resolve_field_kind(
        &self,
        address: &FieldAddress,
        context: &str,
    ) -> Result<ValueKind, SchemaError> {
        let unknown = |target: String| SchemaError::UnknownReference {
            context: context.to_owned(),
            target,
        };

        let Some(entity) = self.entities.get(&address.entity_id) else {
            return Err(unknown(format!("entity '{}'", address.entity_id)));
        };

        if !entity.components.contains_key(&address.component_id) {
            return Err(unknown(format!(
                "component '{}' on entity '{}'",
                address.component_id, address.entity_id
            )));
        }

        let Some(component_def) = self.components.get(&address.component_id) else {
            return Err(unknown(format!("component '{}'", address.component_id)));
        };

        component_def
            .fields
            .get(&address.field_id)
            .map(|field_def| field_def.kind)
            .ok_or_else(|| {
                unknown(format!(
                    "field '{}' on component '{}'",
                    address.field_id, address.component_id
                ))
            })
    }
}
