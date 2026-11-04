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

#[test]
fn rogue_core_dynamic_stat_changes_and_player_actions() {
    let mut project =
        ProjectDef::from_json_str(ROGUE_TEMPLATE_JSON).expect("schéma rogue-core valide");

    // Modification dynamique de l'attaque du héros via les données : 15 -> 30 (one-shot du gobelin)
    let attack_id = hydrogen_core::Identifier::new("attack").expect("id valide");
    let stats_id = hydrogen_core::Identifier::new("stats").expect("id valide");
    for entity in &mut project.entities {
        if entity.id.as_str() == "hero" {
            if let Some(comp) = entity.components.get_mut(&stats_id) {
                comp.insert(attack_id.clone(), ScalarValue::Int(30));
            }
        }
    }

    let mut runtime =
        Runtime::new(project, RuntimeConfig::default()).expect("initialisation runtime");

    let goblin_hp = address("goblin", "health", "current");
    let goblin_alive = address("goblin", "stats", "is_alive");
    let hero_gold = address("hero", "inventory", "gold");
    let hero_potions = address("hero", "inventory", "potions");
    let hero_hp = address("hero", "health", "current");

    // Cycle 1 avec attaque=30 : dégâts = 30 - 0 = 30 -> PV = 0 en un seul tour !
    let report_1 = runtime.tick().expect("tick 1");
    assert_eq!(report_1.rules_executed, 4); // tick + attack + death + reward
    assert_eq!(
        runtime.world().read_entity_field(&goblin_hp),
        Some(&ScalarValue::Int(0))
    );
    assert_eq!(
        runtime.world().read_entity_field(&goblin_alive),
        Some(&ScalarValue::Bool(false))
    );
    assert_eq!(
        runtime.world().read_entity_field(&hero_gold),
        Some(&ScalarValue::Int(50))
    );

    // Test de l'action joueur : boire une potion
    // On blesse d'abord le héros à 50 PV
    let hp_id = hydrogen_core::Identifier::new("current").expect("id valide");
    let health_id = hydrogen_core::Identifier::new("health").expect("id valide");
    let hero_id = hydrogen_core::Identifier::new("hero").expect("id valide");
    let mut wounded_world = runtime.world().clone();
    if let Some(entity) = wounded_world.entities.get_mut(&hero_id) {
        if let Some(comp) = entity.components.get_mut(&health_id) {
            comp.insert(hp_id, ScalarValue::Int(50));
        }
    }
    runtime.restore_world_state(wounded_world, runtime.tick_count());

    // Déclenchement de l'événement joueur `player_drink_potion`
    runtime.publish_event(hydrogen_core::RuntimeEvent::new(
        hydrogen_core::Identifier::new("player_drink_potion").expect("event id"),
    ));

    // Exécution du tick traitant l'action joueur
    let potion_report = runtime.tick().expect("tick potion");
    assert!(potion_report.rules_executed >= 1);
    // Potions passent de 2 à 1
    assert_eq!(
        runtime.world().read_entity_field(&hero_potions),
        Some(&ScalarValue::Int(1))
    );
    // PV passent de 50 à 50 + 30 = 80
    assert_eq!(
        runtime.world().read_entity_field(&hero_hp),
        Some(&ScalarValue::Int(80))
    );
}
