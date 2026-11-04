//! Tests unitaires, property-based (`proptest`) et génération des bindings TypeScript.

use std::collections::BTreeMap;
use std::path::PathBuf;

use hydrogen_core::{ComponentFields, FieldAddress, Identifier, ScalarValue, ValueKind};
use hydrogen_schema::{
    export_typescript_bindings, BinaryOperator, ComparisonOperator, ComponentDef,
    ComponentFieldDef, EntityDef, EventDef, EventSource, ProjectDef, RuleAction, RuleCondition,
    RuleDef, RuleTrigger, SchemaError, SchemaVersion, ValueExpr, CURRENT_SCHEMA_VERSION,
    MAX_EXPRESSION_DEPTH,
};
use proptest::prelude::*;

fn parse_id(raw: &str) -> Result<Identifier, SchemaError> {
    Identifier::new(raw).map_err(|err| SchemaError::JsonFormat {
        message: err.to_string(),
    })
}

fn build_counter_component(
    comp_id: &Identifier,
    field_id: &Identifier,
) -> (ComponentDef, FieldAddress, EntityDef) {
    let mut comp_fields = BTreeMap::new();
    comp_fields.insert(
        field_id.clone(),
        ComponentFieldDef {
            kind: ValueKind::Int,
            default_value: ScalarValue::Int(0),
        },
    );
    let entity_id = comp_id.clone();
    let target = FieldAddress {
        entity_id: entity_id.clone(),
        component_id: comp_id.clone(),
        field_id: field_id.clone(),
    };
    let entity = EntityDef {
        id: entity_id,
        components: BTreeMap::new(),
    };
    (
        ComponentDef {
            id: comp_id.clone(),
            fields: comp_fields,
        },
        target,
        entity,
    )
}

fn build_sample_project(initial_counter: i64, step_delta: i64) -> Result<ProjectDef, SchemaError> {
    let counter_comp_id = parse_id("counter")?;
    let value_field_id = parse_id("value")?;
    let event_id = parse_id("threshold_reached")?;

    let (component, mut target, mut entity) =
        build_counter_component(&counter_comp_id, &value_field_id);
    entity.id = parse_id("main_counter")?;
    target.entity_id = entity.id.clone();

    let mut entity_comp_values = ComponentFields::new();
    entity_comp_values.insert(value_field_id, ScalarValue::Int(initial_counter));
    entity
        .components
        .insert(counter_comp_id, entity_comp_values);

    Ok(ProjectDef {
        schema_version: CURRENT_SCHEMA_VERSION,
        project_id: parse_id("sample_counter")?,
        display_name: "Sample Counter".to_owned(),
        components: vec![component],
        entities: vec![entity],
        events: vec![EventDef {
            id: event_id.clone(),
            source: EventSource::Internal,
            payload_fields: BTreeMap::new(),
        }],
        rules: vec![RuleDef {
            id: parse_id("increment_on_tick")?,
            trigger: RuleTrigger::OnTick,
            condition: Some(RuleCondition {
                target: target.clone(),
                operator: ComparisonOperator::LessThan,
                expected: ScalarValue::Int(100).into(),
            }),
            actions: vec![
                RuleAction::AddNumeric {
                    target,
                    delta: ScalarValue::Int(step_delta).into(),
                },
                RuleAction::EmitEvent {
                    event_id,
                    payload: BTreeMap::new(),
                },
            ],
        }],
    })
}

#[test]
fn valid_project_passes_validation_and_json_roundtrip() -> Result<(), SchemaError> {
    let project = build_sample_project(10, 5)?;
    let json = project.to_pretty_json()?;
    let decoded = ProjectDef::from_json_str(&json)?;
    assert_eq!(project, decoded);
    Ok(())
}

#[test]
fn validator_rejects_unsupported_version_and_unknown_references() -> Result<(), SchemaError> {
    let mut bad_version = build_sample_project(0, 1)?;
    bad_version.schema_version = SchemaVersion::new(9, 0, 0);
    assert!(matches!(
        bad_version.validate(),
        Err(SchemaError::UnsupportedVersion { .. })
    ));

    let mut bad_ref = build_sample_project(0, 1)?;
    bad_ref.rules[0].trigger =
        RuleTrigger::OnEvent(Identifier::new("missing_event").map_err(|e| {
            SchemaError::JsonFormat {
                message: e.to_string(),
            }
        })?);
    assert!(matches!(
        bad_ref.validate(),
        Err(SchemaError::UnknownReference { .. })
    ));
    Ok(())
}

#[test]
fn expression_field_and_binary_validation() -> Result<(), SchemaError> {
    let mut project = build_sample_project(0, 1)?;
    let target = FieldAddress::from_parts("main_counter", "counter", "value").map_err(|e| {
        SchemaError::JsonFormat {
            message: e.to_string(),
        }
    })?;

    // Condition avec lecture de champ et opération binaire
    project.rules[0].condition = Some(RuleCondition {
        target: target.clone(),
        operator: ComparisonOperator::LessThan,
        expected: ValueExpr::Binary {
            op: BinaryOperator::Add,
            left: Box::new(ValueExpr::Field {
                field: target.clone(),
            }),
            right: Box::new(ValueExpr::Literal(ScalarValue::Int(10))),
        },
    });

    // Action avec calcul min(100, current + 5)
    project.rules[0].actions[0] = RuleAction::SetField {
        target: target.clone(),
        value: ValueExpr::Binary {
            op: BinaryOperator::Min,
            left: Box::new(ValueExpr::Literal(ScalarValue::Int(100))),
            right: Box::new(ValueExpr::Binary {
                op: BinaryOperator::Add,
                left: Box::new(ValueExpr::Field { field: target }),
                right: Box::new(ValueExpr::Literal(ScalarValue::Int(5))),
            }),
        },
    };

    project.validate()?;
    Ok(())
}

#[test]
fn expression_rejects_type_mismatch_and_excessive_depth() -> Result<(), SchemaError> {
    let mut project = build_sample_project(0, 1)?;
    let target = FieldAddress::from_parts("main_counter", "counter", "value").map_err(|e| {
        SchemaError::JsonFormat {
            message: e.to_string(),
        }
    })?;

    // Type mismatch : additionner un Int avec un Text
    project.rules[0].actions[0] = RuleAction::SetField {
        target: target.clone(),
        value: ValueExpr::Binary {
            op: BinaryOperator::Add,
            left: Box::new(ValueExpr::Literal(ScalarValue::Int(10))),
            right: Box::new(ValueExpr::Literal(ScalarValue::Text("error".to_owned()))),
        },
    };

    assert!(matches!(
        project.validate(),
        Err(SchemaError::InvalidNumericOperation { .. })
    ));

    // Arbre dépassant la profondeur max (MAX_EXPRESSION_DEPTH = 16)
    let mut deep_expr = ValueExpr::Literal(ScalarValue::Int(1));
    for _ in 0..=MAX_EXPRESSION_DEPTH {
        deep_expr = ValueExpr::Binary {
            op: BinaryOperator::Add,
            left: Box::new(deep_expr),
            right: Box::new(ValueExpr::Literal(ScalarValue::Int(1))),
        };
    }

    project.rules[0].actions[0] = RuleAction::SetField {
        target,
        value: deep_expr,
    };

    assert!(matches!(
        project.validate(),
        Err(SchemaError::ExpressionTooDeep { .. })
    ));

    Ok(())
}

#[test]
fn backwards_compatible_with_v0_1_0_syntax() -> Result<(), SchemaError> {
    // Schéma v0.1.0 utilisant des littéraux scalaires bruts
    let json_v0_1_0 = r#"{
        "schema_version": { "major": 0, "minor": 1, "patch": 0 },
        "project_id": "legacy_project",
        "display_name": "Projet V1",
        "components": [{
            "id": "stats",
            "fields": { "hp": { "kind": "int", "default_value": { "kind": "int", "data": 10 } } }
        }],
        "entities": [{
            "id": "hero",
            "components": { "stats": { "hp": { "kind": "int", "data": 10 } } }
        }],
        "events": [],
        "rules": [{
            "id": "tick_rule",
            "trigger": { "kind": "on_tick" },
            "condition": null,
            "actions": [{
                "kind": "add_numeric",
                "target": { "entity_id": "hero", "component_id": "stats", "field_id": "hp" },
                "delta": { "kind": "int", "data": 1 }
            }]
        }]
    }"#;

    let project = ProjectDef::from_json_str(json_v0_1_0)?;
    assert_eq!(project.project_id.as_str(), "legacy_project");
    project.validate()?;
    Ok(())
}

#[test]
fn validator_rejects_type_mismatch_on_entity_field() -> Result<(), SchemaError> {
    let mut project = build_sample_project(0, 1)?;
    let comp_id = Identifier::new("counter").map_err(|e| SchemaError::JsonFormat {
        message: e.to_string(),
    })?;
    let field_id = Identifier::new("value").map_err(|e| SchemaError::JsonFormat {
        message: e.to_string(),
    })?;

    if let Some(fields) = project.entities[0].components.get_mut(&comp_id) {
        fields.insert(field_id, ScalarValue::Text("wrong_type".to_owned()));
    }

    assert!(matches!(
        project.validate(),
        Err(SchemaError::TypeMismatch {
            expected: ValueKind::Int,
            actual: ValueKind::Text,
            ..
        })
    ));
    Ok(())
}

#[test]
fn generate_typescript_bindings_into_packages_bindings() -> Result<(), SchemaError> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let output_dir = manifest_dir.join("../../packages/bindings/src/generated");
    export_typescript_bindings(&output_dir)
}

proptest! {
    #[test]
    fn proptest_valid_project_json_roundtrip(
        initial in -10_000i64..10_000i64,
        delta in -500i64..500i64
    ) {
        let project = build_sample_project(initial, delta);
        prop_assert!(project.is_ok());
        if let Ok(valid_project) = project {
            let encoded = valid_project.to_pretty_json();
            prop_assert!(encoded.is_ok());
            if let Ok(json_str) = encoded {
                let decoded = ProjectDef::from_json_str(&json_str);
                prop_assert_eq!(decoded, Ok(valid_project));
            }
        }
    }

    #[test]
    fn proptest_identifier_validation_matches_grammar(
        valid_id in "[a-z][a-z0-9_]{0,63}"
    ) {
        let parsed = Identifier::new(&valid_id);
        prop_assert!(parsed.is_ok());
    }
}
