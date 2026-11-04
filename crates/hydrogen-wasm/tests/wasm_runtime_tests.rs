//! Tests d'intégration des bindings WebAssembly `hydrogen-wasm`.

#![allow(
    clippy::panic,
    clippy::expect_used,
    clippy::too_many_lines,
    clippy::unwrap_used
)]

use hydrogen_wasm::WasmRuntime;

const SAMPLE_PROJECT: &str = r#"{
  "schema_version": { "major": 0, "minor": 2, "patch": 0 },
  "project_id": "wasm_counter",
  "display_name": "WASM Counter Test",
  "components": [
    {
      "id": "counter",
      "fields": {
        "value": {
          "kind": "int",
          "default_value": { "kind": "int", "data": 0 }
        }
      }
    }
  ],
  "entities": [
    {
      "id": "hero",
      "components": {
        "counter": {
          "value": { "kind": "int", "data": 10 }
        }
      }
    }
  ],
  "events": [
    {
      "id": "player_boost",
      "source": "player",
      "payload_fields": {}
    }
  ],
  "rules": [
    {
      "id": "inc",
      "trigger": { "kind": "on_tick" },
      "condition": null,
      "actions": [
        {
          "kind": "add_numeric",
          "target": { "entity_id": "hero", "component_id": "counter", "field_id": "value" },
          "delta": { "kind": "int", "data": 2 }
        }
      ]
    },
    {
      "id": "boost_on_player_event",
      "trigger": { "kind": "on_event", "event_id": "player_boost" },
      "condition": null,
      "actions": [
        {
          "kind": "add_numeric",
          "target": { "entity_id": "hero", "component_id": "counter", "field_id": "value" },
          "delta": { "kind": "int", "data": 50 }
        }
      ]
    }
  ]
}"#;

#[test]
fn wasm_runtime_initializes_and_ticks_correctly() {
    let mut runtime = WasmRuntime::new(SAMPLE_PROJECT).expect("initialisation wasm runtime");
    assert_eq!(runtime.get_tick_count(), 0);

    let report_json = runtime.tick().expect("tick 1");
    assert!(report_json.contains("\"tick_index\":1"));
    assert_eq!(runtime.get_tick_count(), 1);

    let world_json = runtime.get_world_state_json().expect("world json");
    assert!(world_json.contains("\"hero\""));
    assert!(world_json.contains("\"data\":12"));
}

#[test]
fn wasm_runtime_handles_player_events_and_state_restoration() {
    let mut runtime = WasmRuntime::new(SAMPLE_PROJECT).expect("initialisation wasm runtime");

    // Déclenchement de l'événement joueur `player_boost`
    runtime
        .publish_event_by_id("player_boost", None)
        .expect("publication player event");

    // Tick 1 : inc (+2) + boost joueur (+50) = 10 + 2 + 50 = 62
    let report_json = runtime.tick().expect("tick 1 après boost");
    assert!(report_json.contains("\"rules_executed\":2"));
    let world_after_boost = runtime.get_world_state_json().expect("world");
    assert!(world_after_boost.contains("\"data\":62"));

    // Restauration d'un ancien état sauvegardé (valeur 20 à tick 5)
    let saved_state = world_after_boost.replace("\"data\":62", "\"data\":20");
    runtime
        .restore_world_state(&saved_state, 5)
        .expect("restauration état");
    assert_eq!(runtime.get_tick_count(), 5);

    let world_after_restore = runtime.get_world_state_json().expect("world restauré");
    assert!(world_after_restore.contains("\"data\":20"));

    // Prochain tick : repart de 20 -> 22 à tick 6
    runtime.tick().expect("tick 6");
    assert_eq!(runtime.get_tick_count(), 6);
    let world_tick_6 = runtime.get_world_state_json().expect("world tick 6");
    assert!(world_tick_6.contains("\"data\":22"));
}

#[test]
fn wasm_runtime_rejects_invalid_json() {
    let bad_json = "{ broken json";
    assert!(WasmRuntime::new(bad_json).is_err());
}
