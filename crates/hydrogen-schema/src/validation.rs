//! Validation pure et déterministe d'une définition de projet Hydrogen (`ProjectDef`).

use std::collections::{BTreeMap, BTreeSet};

use hydrogen_core::{Identifier, ScalarValue, ValueKind};

use crate::error::SchemaError;
use crate::expression_validation::{ensure_expr_kind, is_numeric, ExprScope};
use crate::model::{ComponentDef, EntityDef, EventDef};
use crate::project::ProjectDef;
use crate::rule::{ComparisonOperator, RuleAction, RuleCondition, RuleDef, RuleTrigger};
use crate::schema_index::SchemaIndex;

/// Valide intégralement une définition de projet Hydrogen (version, composants, entités, événements, règles).
///
/// # Errors
/// Retourne la première [`SchemaError`] rencontrée si un invariant métier ou de typage est violé.
pub fn validate_project_def(project: &ProjectDef) -> Result<(), SchemaError> {
    project.schema_version.ensure_compatible()?;

    let components = index_components(&project.components)?;
    let events = index_events(&project.events)?;
    let entities = index_entities(&project.entities, &components)?;

    let index = SchemaIndex {
        components,
        entities,
        events,
    };

    validate_rules(&project.rules, &index)
}

fn index_components(
    components: &[ComponentDef],
) -> Result<BTreeMap<&Identifier, &ComponentDef>, SchemaError> {
    let mut map = BTreeMap::new();
    for component in components {
        if map.insert(&component.id, component).is_some() {
            return Err(SchemaError::DuplicateIdentifier {
                section: "components",
                id: component.id.as_str().to_owned(),
            });
        }
        for (field_id, field_def) in &component.fields {
            validate_scalar_type(
                &format!("component.{}.{}", component.id, field_id),
                field_def.kind,
                &field_def.default_value,
            )?;
        }
    }
    Ok(map)
}

fn index_events(events: &[EventDef]) -> Result<BTreeMap<&Identifier, &EventDef>, SchemaError> {
    let mut map = BTreeMap::new();
    for event in events {
        if map.insert(&event.id, event).is_some() {
            return Err(SchemaError::DuplicateIdentifier {
                section: "events",
                id: event.id.as_str().to_owned(),
            });
        }
    }
    Ok(map)
}

fn index_entities<'a>(
    entities: &'a [EntityDef],
    components: &BTreeMap<&'a Identifier, &'a ComponentDef>,
) -> Result<BTreeMap<&'a Identifier, &'a EntityDef>, SchemaError> {
    let mut map = BTreeMap::new();
    for entity in entities {
        if map.insert(&entity.id, entity).is_some() {
            return Err(SchemaError::DuplicateIdentifier {
                section: "entities",
                id: entity.id.as_str().to_owned(),
            });
        }
        validate_entity_components(entity, components)?;
    }
    Ok(map)
}

fn validate_entity_components(
    entity: &EntityDef,
    components: &BTreeMap<&Identifier, &ComponentDef>,
) -> Result<(), SchemaError> {
    for (comp_id, fields) in &entity.components {
        let Some(comp_def) = components.get(comp_id) else {
            return Err(SchemaError::UnknownReference {
                context: format!("entity '{}'", entity.id),
                target: format!("component '{comp_id}'"),
            });
        };

        for (field_id, value) in fields {
            let Some(field_def) = comp_def.fields.get(field_id) else {
                return Err(SchemaError::UnknownReference {
                    context: format!("entity '{}.{}'", entity.id, comp_id),
                    target: format!("field '{field_id}'"),
                });
            };
            validate_scalar_type(
                &format!("entity.{}.{}.{}", entity.id, comp_id, field_id),
                field_def.kind,
                value,
            )?;
        }
    }
    Ok(())
}

fn validate_rules(rules: &[RuleDef], index: &SchemaIndex<'_>) -> Result<(), SchemaError> {
    let mut seen_ids = BTreeSet::new();
    for rule in rules {
        if !seen_ids.insert(&rule.id) {
            return Err(SchemaError::DuplicateIdentifier {
                section: "rules",
                id: rule.id.as_str().to_owned(),
            });
        }
        validate_single_rule(rule, index)?;
    }
    Ok(())
}

fn validate_single_rule(rule: &RuleDef, index: &SchemaIndex<'_>) -> Result<(), SchemaError> {
    if rule.actions.is_empty() {
        return Err(SchemaError::EmptyRuleActions {
            rule_id: rule.id.as_str().to_owned(),
        });
    }

    if let RuleTrigger::OnEvent(event_id) = &rule.trigger {
        if !index.events.contains_key(event_id) {
            return Err(SchemaError::UnknownReference {
                context: format!("rule '{}' trigger", rule.id),
                target: format!("event '{event_id}'"),
            });
        }
    }

    if let Some(condition) = &rule.condition {
        validate_condition(&rule.id, condition, index)?;
    }

    for action in &rule.actions {
        validate_action(&rule.id, action, index)?;
    }
    Ok(())
}

fn validate_condition(
    rule_id: &Identifier,
    condition: &RuleCondition,
    index: &SchemaIndex<'_>,
) -> Result<(), SchemaError> {
    let context = format!("rule '{rule_id}' condition");
    let field_kind = index.resolve_field_kind(&condition.target, &context)?;

    ensure_expr_kind(
        field_kind,
        &condition.expected,
        &ExprScope {
            context: &context,
            index,
        },
    )?;

    let requires_numeric = matches!(
        condition.operator,
        ComparisonOperator::GreaterThan
            | ComparisonOperator::GreaterOrEqual
            | ComparisonOperator::LessThan
            | ComparisonOperator::LessOrEqual
    );

    if requires_numeric && !is_numeric(field_kind) {
        return Err(SchemaError::InvalidNumericOperation {
            target: context,
            actual: field_kind,
        });
    }
    Ok(())
}

struct EmitEventCheck<'a> {
    context: &'a str,
    event_id: &'a Identifier,
    payload: &'a BTreeMap<Identifier, ScalarValue>,
}

fn validate_action(
    rule_id: &Identifier,
    action: &RuleAction,
    index: &SchemaIndex<'_>,
) -> Result<(), SchemaError> {
    let context = format!("rule '{rule_id}' action");
    let scope = ExprScope {
        context: &context,
        index,
    };

    match action {
        RuleAction::SetField { target, value } => {
            let kind = index.resolve_field_kind(target, &context)?;
            ensure_expr_kind(kind, value, &scope)
        }
        RuleAction::AddNumeric { target, delta } => {
            let kind = index.resolve_field_kind(target, &context)?;
            if !is_numeric(kind) {
                return Err(SchemaError::InvalidNumericOperation {
                    target: context,
                    actual: kind,
                });
            }
            ensure_expr_kind(kind, delta, &scope)
        }
        RuleAction::EmitEvent { event_id, payload } => validate_emit_event(
            &EmitEventCheck {
                context: &context,
                event_id,
                payload,
            },
            index,
        ),
    }
}

fn validate_emit_event(
    check: &EmitEventCheck<'_>,
    index: &SchemaIndex<'_>,
) -> Result<(), SchemaError> {
    let EmitEventCheck {
        context,
        event_id,
        payload,
    } = *check;
    let Some(event_def) = index.events.get(event_id) else {
        return Err(SchemaError::UnknownReference {
            context: context.to_owned(),
            target: format!("event '{event_id}'"),
        });
    };

    for (field_id, expected_kind) in &event_def.payload_fields {
        let Some(value) = payload.get(field_id) else {
            return Err(SchemaError::UnknownReference {
                context: format!("{context} emit '{event_id}'"),
                target: format!("missing payload field '{field_id}'"),
            });
        };
        validate_scalar_type(
            &format!("{context}.{event_id}.{field_id}"),
            *expected_kind,
            value,
        )?;
    }

    for field_id in payload.keys() {
        if !event_def.payload_fields.contains_key(field_id) {
            return Err(SchemaError::UnknownReference {
                context: format!("{context} emit '{event_id}'"),
                target: format!("unexpected payload field '{field_id}'"),
            });
        }
    }
    Ok(())
}

fn validate_scalar_type(
    target: &str,
    expected: ValueKind,
    value: &ScalarValue,
) -> Result<(), SchemaError> {
    if !value.is_deterministic() {
        return Err(SchemaError::NonDeterministicValue {
            context: target.to_owned(),
        });
    }

    let actual = value.kind();
    if actual != expected {
        return Err(SchemaError::TypeMismatch {
            target: target.to_owned(),
            expected,
            actual,
        });
    }
    Ok(())
}
