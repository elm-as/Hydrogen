//! Benchmarks Criterion pour mesurer les performances d'un cycle de calcul (`Runtime::tick`).

#![allow(
    missing_docs,
    clippy::panic,
    clippy::expect_used,
    clippy::too_many_lines,
    clippy::unwrap_used
)]

use std::collections::BTreeMap;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use hydrogen_core::{ComponentFields, FieldAddress, Identifier, ScalarValue, ValueKind};
use hydrogen_runtime::{Runtime, RuntimeConfig};
use hydrogen_schema::{
    ComponentDef, ComponentFieldDef, EntityDef, EventDef, ProjectDef, RuleAction, RuleDef,
    RuleTrigger, CURRENT_SCHEMA_VERSION,
};

fn id(raw: &str) -> Identifier {
    Identifier::new(raw).unwrap_or_else(|_| panic!("identifiant invalide pour bench"))
}

fn create_bench_project(rules_count: usize) -> ProjectDef {
    let comp_id = id("stats");
    let field_id = id("score");
    let entity_id = id("actor");

    let mut fields = BTreeMap::new();
    fields.insert(
        field_id.clone(),
        ComponentFieldDef {
            kind: ValueKind::Int,
            default_value: ScalarValue::Int(0),
        },
    );

    let mut init = ComponentFields::new();
    init.insert(field_id.clone(), ScalarValue::Int(0));
    let mut entity_comps = BTreeMap::new();
    entity_comps.insert(comp_id.clone(), init);

    let target = FieldAddress {
        entity_id: entity_id.clone(),
        component_id: comp_id.clone(),
        field_id,
    };

    let mut rules = Vec::with_capacity(rules_count);
    for i in 0..rules_count {
        rules.push(RuleDef {
            id: id(&format!("rule_{i}")),
            trigger: RuleTrigger::OnTick,
            condition: None,
            actions: vec![RuleAction::AddNumeric {
                target: target.clone(),
                delta: ScalarValue::Int(1),
            }],
        });
    }

    ProjectDef {
        schema_version: CURRENT_SCHEMA_VERSION,
        project_id: id("bench_project"),
        display_name: "Bench Project".to_owned(),
        components: vec![ComponentDef {
            id: comp_id,
            fields,
        }],
        entities: vec![EntityDef {
            id: entity_id,
            components: entity_comps,
        }],
        events: vec![],
        rules,
    }
}

fn create_cascade_project(depth: usize) -> ProjectDef {
    let comp_id = id("stats");
    let field_id = id("score");
    let entity_id = id("actor");

    let mut fields = BTreeMap::new();
    fields.insert(
        field_id.clone(),
        ComponentFieldDef {
            kind: ValueKind::Int,
            default_value: ScalarValue::Int(0),
        },
    );

    let mut init = ComponentFields::new();
    init.insert(field_id.clone(), ScalarValue::Int(0));
    let mut entity_comps = BTreeMap::new();
    entity_comps.insert(comp_id.clone(), init);

    let target = FieldAddress {
        entity_id: entity_id.clone(),
        component_id: comp_id.clone(),
        field_id,
    };

    let mut events = Vec::with_capacity(depth);
    let mut rules = Vec::with_capacity(depth);

    for i in 0..depth {
        let evt_id = id(&format!("step_{i}"));
        events.push(EventDef {
            id: evt_id.clone(),
            payload_fields: BTreeMap::new(),
        });

        let trigger = if i == 0 {
            RuleTrigger::OnTick
        } else {
            RuleTrigger::OnEvent(id(&format!("step_{}", i - 1)))
        };

        let mut actions = vec![RuleAction::AddNumeric {
            target: target.clone(),
            delta: ScalarValue::Int(1),
        }];

        if i + 1 < depth {
            actions.push(RuleAction::EmitEvent {
                event_id: id(&format!("step_{i}")),
                payload: BTreeMap::new(),
            });
        }

        rules.push(RuleDef {
            id: id(&format!("rule_{i}")),
            trigger,
            condition: None,
            actions,
        });
    }

    ProjectDef {
        schema_version: CURRENT_SCHEMA_VERSION,
        project_id: id("cascade_project"),
        display_name: "Cascade Project".to_owned(),
        components: vec![ComponentDef {
            id: comp_id,
            fields,
        }],
        entities: vec![EntityDef {
            id: entity_id,
            components: entity_comps,
        }],
        events,
        rules,
    }
}

fn bench_tick_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("runtime_tick_scaling");

    for rules_count in [1, 100, 1000] {
        group.bench_with_input(
            BenchmarkId::new("rules", rules_count),
            &rules_count,
            |b, &count| {
                let project = create_bench_project(count);
                let mut runtime =
                    Runtime::new(project, RuntimeConfig::default()).expect("init bench");
                b.iter(|| {
                    black_box(runtime.tick().expect("tick bench"));
                });
            },
        );
    }
    group.finish();
}

fn bench_event_cascade(c: &mut Criterion) {
    let mut group = c.benchmark_group("runtime_event_cascade");

    for depth in [5, 20, 50] {
        group.bench_with_input(BenchmarkId::new("depth", depth), &depth, |b, &d| {
            let project = create_cascade_project(d);
            let mut runtime = Runtime::new(project, RuntimeConfig::default()).expect("init bench");
            b.iter(|| {
                black_box(runtime.tick().expect("cascade bench"));
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_tick_scaling, bench_event_cascade);
criterion_main!(benches);
