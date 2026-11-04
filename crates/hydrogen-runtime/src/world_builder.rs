//! Construction déterministe de l'état initial du monde (`WorldState`) depuis un [`ProjectDef`].

use std::collections::BTreeMap;

use hydrogen_core::{ComponentFields, EntityState, Identifier, WorldState};
use hydrogen_schema::{ComponentDef, ProjectDef};

use crate::error::RuntimeError;

/// Construit l'état initial du monde en appliquant les valeurs par défaut des composants.
///
/// # Errors
/// Retourne [`RuntimeError::Schema`] si le projet n'est pas valide.
pub fn build_initial_world(project: &ProjectDef) -> Result<WorldState, RuntimeError> {
    project.validate().map_err(RuntimeError::Schema)?;

    let components_index = index_component_defaults(&project.components);
    let mut world = WorldState::new();

    for entity_def in &project.entities {
        let state = build_entity_state(entity_def, &components_index);
        world.insert_entity(state);
    }

    Ok(world)
}

fn index_component_defaults(components: &[ComponentDef]) -> BTreeMap<&Identifier, &ComponentDef> {
    let mut index = BTreeMap::new();
    for comp in components {
        index.insert(&comp.id, comp);
    }
    index
}

fn build_entity_state(
    entity_def: &hydrogen_schema::EntityDef,
    components_index: &BTreeMap<&Identifier, &ComponentDef>,
) -> EntityState {
    let mut entity_state = EntityState::new(entity_def.id.clone());

    for (comp_id, provided_fields) in &entity_def.components {
        let Some(comp_def) = components_index.get(comp_id) else {
            continue;
        };

        let resolved_fields = resolve_fields(comp_def, provided_fields);
        entity_state
            .components
            .insert(comp_id.clone(), resolved_fields);
    }

    entity_state
}

fn resolve_fields(comp_def: &ComponentDef, provided_fields: &ComponentFields) -> ComponentFields {
    let mut fields = ComponentFields::new();

    for (field_id, field_def) in &comp_def.fields {
        let initial_value = provided_fields
            .get(field_id)
            .cloned()
            .unwrap_or_else(|| field_def.default_value.clone());
        fields.insert(field_id.clone(), initial_value);
    }

    fields
}
