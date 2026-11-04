//! Application des actions déclaratives (`RuleAction`) sur l'état du monde et le bus d'événements.

use hydrogen_core::{EventBus, FieldAddress, RuntimeEvent, ScalarValue, WorldState};
use hydrogen_schema::RuleAction;

use crate::error::RuntimeError;

/// Applique une action déclarative sur le monde ou émet un nouvel événement.
///
/// # Errors
/// Retourne [`RuntimeError::ArithmeticOverflow`] en cas de débordement de calcul,
/// ou [`RuntimeError::Core`] si la cible n'existe pas.
pub fn apply_action(
    action: &RuleAction,
    world: &mut WorldState,
    bus: &mut EventBus,
) -> Result<(), RuntimeError> {
    match action {
        RuleAction::SetField { target, value } => world
            .write_entity_field(target, value.clone())
            .map_err(RuntimeError::Core),
        RuleAction::AddNumeric { target, delta } => apply_add_numeric(target, delta, world),
        RuleAction::EmitEvent { event_id, payload } => {
            bus.publish(RuntimeEvent::with_payload(
                event_id.clone(),
                payload.clone(),
            ));
            Ok(())
        }
    }
}

fn apply_add_numeric(
    target: &FieldAddress,
    delta: &ScalarValue,
    world: &mut WorldState,
) -> Result<(), RuntimeError> {
    let current = world.read_entity_field(target).cloned().ok_or_else(|| {
        RuntimeError::Core(hydrogen_core::CoreError::ComponentNotFound {
            entity_id: target.entity_id.as_str().to_owned(),
            component_id: target.component_id.as_str().to_owned(),
        })
    })?;

    let updated = calculate_sum(&current, delta, target)?;
    world
        .write_entity_field(target, updated)
        .map_err(RuntimeError::Core)
}

fn calculate_sum(
    current: &ScalarValue,
    delta: &ScalarValue,
    target: &FieldAddress,
) -> Result<ScalarValue, RuntimeError> {
    let overflow_error = || RuntimeError::ArithmeticOverflow {
        target: format!(
            "{}.{}.{}",
            target.entity_id, target.component_id, target.field_id
        ),
    };

    match (current, delta) {
        (ScalarValue::Int(a), ScalarValue::Int(b)) => a
            .checked_add(*b)
            .map(ScalarValue::Int)
            .ok_or_else(overflow_error),
        (ScalarValue::Float(a), ScalarValue::Float(b)) => {
            let result = a + b;
            if result.is_finite() {
                Ok(ScalarValue::Float(result))
            } else {
                Err(overflow_error())
            }
        }
        _ => Err(overflow_error()),
    }
}
