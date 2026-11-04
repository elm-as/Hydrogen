//! Moteur d'exécution déclaratif des définitions Hydrogen (`hydrogen-runtime`).
//! Exécute les règles, orchestre les cascades d'événements et maintient l'état du monde.

pub mod action;
pub mod condition;
pub mod config;
pub mod engine;
pub mod error;
pub mod world_builder;

pub use action::apply_action;
pub use condition::evaluate_condition;
pub use config::RuntimeConfig;
pub use engine::{Runtime, TickReport};
pub use error::RuntimeError;
pub use world_builder::build_initial_world;

#[cfg(test)]
#[allow(
    clippy::panic,
    clippy::expect_used,
    clippy::too_many_lines,
    clippy::unwrap_used
)]
mod tests {
    use std::collections::BTreeMap;

    use hydrogen_core::{ComponentFields, FieldAddress, Identifier, ScalarValue, ValueKind};
    use hydrogen_schema::{
        ComparisonOperator, ComponentDef, ComponentFieldDef, EntityDef, EventDef, ProjectDef,
        RuleAction, RuleCondition, RuleDef, RuleTrigger, CURRENT_SCHEMA_VERSION,
    };

    use super::*;

    fn id(raw: &str) -> Identifier {
        Identifier::new(raw).unwrap_or_else(|_| panic!("identifiant invalide pour test"))
    }

    fn sample_counter_project() -> ProjectDef {
        let counter_comp = id("counter");
        let value_field = id("value");
        let entity_id = id("player");
        let event_id = id("reached_fifty");

        let mut fields = BTreeMap::new();
        fields.insert(
            value_field.clone(),
            ComponentFieldDef {
                kind: ValueKind::Int,
                default_value: ScalarValue::Int(0),
            },
        );

        let mut initial_fields = ComponentFields::new();
        initial_fields.insert(value_field.clone(), ScalarValue::Int(48));
        let mut entity_comps = BTreeMap::new();
        entity_comps.insert(counter_comp.clone(), initial_fields);

        let target = FieldAddress {
            entity_id: entity_id.clone(),
            component_id: counter_comp.clone(),
            field_id: value_field.clone(),
        };

        ProjectDef {
            schema_version: CURRENT_SCHEMA_VERSION,
            project_id: id("test_project"),
            display_name: "Test Project".to_owned(),
            components: vec![ComponentDef {
                id: counter_comp,
                fields,
            }],
            entities: vec![EntityDef {
                id: entity_id,
                components: entity_comps,
            }],
            events: vec![EventDef {
                id: event_id.clone(),
                payload_fields: BTreeMap::new(),
            }],
            rules: vec![
                RuleDef {
                    id: id("increment_each_tick"),
                    trigger: RuleTrigger::OnTick,
                    condition: None,
                    actions: vec![RuleAction::AddNumeric {
                        target: target.clone(),
                        delta: ScalarValue::Int(1),
                    }],
                },
                RuleDef {
                    id: id("emit_on_50"),
                    trigger: RuleTrigger::OnTick,
                    condition: Some(RuleCondition {
                        target: target.clone(),
                        operator: ComparisonOperator::Equal,
                        expected: ScalarValue::Int(50),
                    }),
                    actions: vec![RuleAction::EmitEvent {
                        event_id: event_id.clone(),
                        payload: BTreeMap::new(),
                    }],
                },
                RuleDef {
                    id: id("bonus_on_reached_50"),
                    trigger: RuleTrigger::OnEvent(event_id),
                    condition: None,
                    actions: vec![RuleAction::AddNumeric {
                        target,
                        delta: ScalarValue::Int(10),
                    }],
                },
            ],
        }
    }

    #[test]
    fn tick_increments_and_cascades_event_in_same_tick() {
        let project = sample_counter_project();
        let mut runtime =
            Runtime::new(project, RuntimeConfig::default()).expect("initialisation runtime");

        let target = FieldAddress {
            entity_id: id("player"),
            component_id: id("counter"),
            field_id: id("value"),
        };

        // Tick 1: 48 -> 49 (pas d'événement 50)
        let report_1 = runtime.tick().expect("tick 1 valide");
        assert_eq!(report_1.tick_index, 1);
        assert_eq!(
            runtime.world().read_entity_field(&target),
            Some(&ScalarValue::Int(49))
        );

        // Tick 2: 49 -> 50, émet reached_fifty, cascade bonus +10 -> 60
        let report_2 = runtime.tick().expect("tick 2 valide");
        assert_eq!(report_2.tick_index, 2);
        assert_eq!(report_2.events_processed, 1);
        assert_eq!(
            runtime.world().read_entity_field(&target),
            Some(&ScalarValue::Int(60))
        );
    }

    #[test]
    fn event_loop_limit_aborts_and_rolls_back_cleanly() {
        let mut project = sample_counter_project();
        let loop_event = id("reached_fifty");
        // Règle récursive infinie : OnEvent -> EmitEvent
        project.rules.push(RuleDef {
            id: id("infinite_loop"),
            trigger: RuleTrigger::OnEvent(loop_event.clone()),
            condition: None,
            actions: vec![RuleAction::EmitEvent {
                event_id: loop_event,
                payload: BTreeMap::new(),
            }],
        });

        let config = RuntimeConfig::with_max_events(10);
        let mut runtime = Runtime::new(project, config).expect("initialisation runtime");

        let target = FieldAddress {
            entity_id: id("player"),
            component_id: id("counter"),
            field_id: id("value"),
        };

        // Tick 1 amène le compteur à 49
        runtime.tick().expect("tick 1 ok");
        assert_eq!(
            runtime.world().read_entity_field(&target),
            Some(&ScalarValue::Int(49))
        );

        // Tick 2 déclenche l'événement et boucle au-delà de 10 événements -> erreur
        let error = runtime
            .tick()
            .expect_err("doit échouer sur limite de boucle");
        assert_eq!(error, RuntimeError::EventLoopLimitExceeded { limit: 10 });

        // Vérification de la transactionnalité : le monde n'est pas altéré par le tick 2 avorté
        assert_eq!(
            runtime.world().read_entity_field(&target),
            Some(&ScalarValue::Int(49))
        );
        assert_eq!(runtime.tick_count(), 1);
    }
}
