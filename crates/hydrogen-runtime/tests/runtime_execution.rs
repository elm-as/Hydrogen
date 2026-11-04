//! Tests d'intégration et property-based testing pour `hydrogen-runtime`.

#![allow(
    clippy::panic,
    clippy::expect_used,
    clippy::too_many_lines,
    clippy::unwrap_used
)]

use std::collections::BTreeMap;

use hydrogen_core::{ComponentFields, FieldAddress, Identifier, ScalarValue, ValueKind};
use hydrogen_runtime::{Runtime, RuntimeConfig, RuntimeError};
use hydrogen_schema::{
    ComponentDef, ComponentFieldDef, EntityDef, ProjectDef, RuleAction, RuleDef, RuleTrigger,
    CURRENT_SCHEMA_VERSION,
};
use proptest::prelude::*;

fn id(raw: &str) -> Identifier {
    Identifier::new(raw).unwrap_or_else(|_| panic!("identifiant invalide pour test"))
}

fn build_math_project(initial: i64, delta: i64) -> ProjectDef {
    let comp_id = id("math");
    let field_id = id("val");
    let entity_id = id("calculator");

    let mut fields = BTreeMap::new();
    fields.insert(
        field_id.clone(),
        ComponentFieldDef {
            kind: ValueKind::Int,
            default_value: ScalarValue::Int(0),
        },
    );

    let mut init = ComponentFields::new();
    init.insert(field_id.clone(), ScalarValue::Int(initial));
    let mut entity_comps = BTreeMap::new();
    entity_comps.insert(comp_id.clone(), init);

    let target = FieldAddress {
        entity_id: entity_id.clone(),
        component_id: comp_id.clone(),
        field_id,
    };

    ProjectDef {
        schema_version: CURRENT_SCHEMA_VERSION,
        project_id: id("math_project"),
        display_name: "Math Project".to_owned(),
        components: vec![ComponentDef {
            id: comp_id,
            fields,
        }],
        entities: vec![EntityDef {
            id: entity_id,
            components: entity_comps,
        }],
        events: vec![],
        rules: vec![RuleDef {
            id: id("add_delta"),
            trigger: RuleTrigger::OnTick,
            condition: None,
            actions: vec![RuleAction::AddNumeric {
                target,
                delta: ScalarValue::Int(delta),
            }],
        }],
    }
}

#[test]
fn arithmetic_overflow_rolls_back_without_panic() {
    let project = build_math_project(i64::MAX, 1);
    let mut runtime =
        Runtime::new(project, RuntimeConfig::default()).expect("initialisation runtime");

    let target = FieldAddress {
        entity_id: id("calculator"),
        component_id: id("math"),
        field_id: id("val"),
    };

    let err = runtime.tick().expect_err("doit échouer sur overflow");
    assert!(matches!(err, RuntimeError::ArithmeticOverflow { .. }));

    // Transactionnalité : la valeur reste i64::MAX et le compteur de ticks reste à 0
    assert_eq!(
        runtime.world().read_entity_field(&target),
        Some(&ScalarValue::Int(i64::MAX))
    );
    assert_eq!(runtime.tick_count(), 0);
}

proptest! {
    #[test]
    fn determinism_identical_runs_produce_identical_states(
        initial in -10_000i64..10_000i64,
        delta in -100i64..100i64,
        ticks in 1u32..20u32
    ) {
        let mut r1 = Runtime::new(build_math_project(initial, delta), RuntimeConfig::default())
            .expect("init r1");
        let mut r2 = Runtime::new(build_math_project(initial, delta), RuntimeConfig::default())
            .expect("init r2");

        for _ in 0..ticks {
            let res1 = r1.tick();
            let res2 = r2.tick();
            prop_assert_eq!(res1, res2);
        }

        prop_assert_eq!(r1.world(), r2.world());
        prop_assert_eq!(r1.tick_count(), r2.tick_count());
    }
}
