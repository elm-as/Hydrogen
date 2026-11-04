//! Test d'intégration complet du projet pilote 'rogue-core'.
//! Valide la boucle de gameplay, la cascade d'événements, les règles conditionnelles,
//! et la persistance/restauration via `hydrogen-data`.

#![allow(
    clippy::panic,
    clippy::expect_used,
    clippy::too_many_lines,
    clippy::unwrap_used
)]

use tempfile::tempdir;

use hydrogen_core::{FieldAddress, ScalarValue};
use hydrogen_data::{FsStore, SaveSnapshot, SaveStore};
use hydrogen_runtime::{Runtime, RuntimeConfig};
use hydrogen_schema::ProjectDef;

const ROGUE_TEMPLATE_JSON: &str = include_str!("../../../templates/game/project.json");

fn address(entity: &str, component: &str, field: &str) -> FieldAddress {
    FieldAddress::from_parts(entity, component, field).expect("adresse valide")
}

#[test]
fn rogue_core_pilot_gameplay_cascade_and_time_travel() {
    let project = ProjectDef::from_json_str(ROGUE_TEMPLATE_JSON)
        .expect("le schéma rogue-core doit être valide");

    let mut runtime =
        Runtime::new(project.clone(), RuntimeConfig::default()).expect("initialisation runtime");

    let hero_gold_addr = address("hero", "inventory", "gold");
    let goblin_hp_addr = address("goblin", "health", "current");
    let goblin_alive_addr = address("goblin", "stats", "is_alive");
    let turn_addr = address("game_state", "dungeon", "turn");

    // Vérification de l'état initial
    assert_eq!(
        runtime.world().read_entity_field(&goblin_hp_addr),
        Some(&ScalarValue::Int(30))
    );
    assert_eq!(
        runtime.world().read_entity_field(&hero_gold_addr),
        Some(&ScalarValue::Int(0))
    );

    // Cycle 1 : Tour 1 - le héros attaque le gobelin (30 - 15 = 15 PV)
    let step_1 = runtime.tick().expect("tick 1");
    assert_eq!(step_1.rules_executed, 2); // tick_turn + hero_attack
    assert_eq!(
        runtime.world().read_entity_field(&turn_addr),
        Some(&ScalarValue::Int(1))
    );
    assert_eq!(
        runtime.world().read_entity_field(&goblin_hp_addr),
        Some(&ScalarValue::Int(15))
    );
    assert_eq!(
        runtime.world().read_entity_field(&goblin_alive_addr),
        Some(&ScalarValue::Bool(true))
    );

    // Sauvegarde du Tour 1 via FsStore
    let dir = tempdir().expect("dossier temporaire");
    let mut store = FsStore::new(dir.path()).expect("création FsStore");

    let snap_turn_1 =
        SaveSnapshot::new("rogue_core", runtime.tick_count(), runtime.world().clone())
            .with_metadata("label", "Fin du Tour 1");
    store
        .save("turn_1", &snap_turn_1)
        .expect("sauvegarde turn_1");

    // Cycle 2 : Tour 2 - le héros achève le gobelin (15 - 15 = 0 PV)
    // Cascade : hero_attack -> check_goblin_death -> reward_on_goblin_death (+50 or)
    let step_2 = runtime.tick().expect("tick 2");
    assert_eq!(step_2.rules_executed, 4); // tick_turn + attack + death + reward
    assert_eq!(
        runtime.world().read_entity_field(&turn_addr),
        Some(&ScalarValue::Int(2))
    );
    assert_eq!(
        runtime.world().read_entity_field(&goblin_hp_addr),
        Some(&ScalarValue::Int(0))
    );
    assert_eq!(
        runtime.world().read_entity_field(&goblin_alive_addr),
        Some(&ScalarValue::Bool(false))
    );
    assert_eq!(
        runtime.world().read_entity_field(&hero_gold_addr),
        Some(&ScalarValue::Int(50))
    );

    // Cycle 3 : Tour 3 - le gobelin est mort, aucune nouvelle attaque
    let step_3 = runtime.tick().expect("tick 3");
    assert_eq!(step_3.rules_executed, 1); // seulement tick_turn
    assert_eq!(
        runtime.world().read_entity_field(&turn_addr),
        Some(&ScalarValue::Int(3))
    );
    assert_eq!(
        runtime.world().read_entity_field(&hero_gold_addr),
        Some(&ScalarValue::Int(50))
    );

    // Restauration du Tour 1 et vérification du voyage dans le temps
    let loaded_turn_1 = store.load("turn_1").expect("chargement turn_1");
    assert_eq!(loaded_turn_1.tick, 1);
    assert_eq!(
        loaded_turn_1.metadata.get("label").map(String::as_str),
        Some("Fin du Tour 1")
    );

    runtime.restore_world_state(loaded_turn_1.world_state, loaded_turn_1.tick);
    assert_eq!(runtime.tick_count(), 1);
    assert_eq!(
        runtime.world().read_entity_field(&goblin_hp_addr),
        Some(&ScalarValue::Int(15))
    );

    // Rejouer le cycle 2 depuis la sauvegarde restaurée produit exactement le même état
    let rewind_step = runtime.tick().expect("tick rejoué");
    assert_eq!(rewind_step.rules_executed, 4);
    assert_eq!(
        runtime.world().read_entity_field(&goblin_hp_addr),
        Some(&ScalarValue::Int(0))
    );
    assert_eq!(
        runtime.world().read_entity_field(&hero_gold_addr),
        Some(&ScalarValue::Int(50))
    );
}
