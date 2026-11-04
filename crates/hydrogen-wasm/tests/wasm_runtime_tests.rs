//! Tests d'intégration des bindings WebAssembly `hydrogen-wasm`.

#![allow(
    clippy::panic,
    clippy::expect_used,
    clippy::too_many_lines,
    clippy::unwrap_used
)]

use hydrogen_wasm::WasmRuntime;

const SAMPLE_PROJECT: &str = r#"{
  "schema_version": { "major": 0, "minor": 1, "patch": 0 },
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
  "events": [],
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
fn wasm_runtime_rejects_invalid_json() {
    let bad_json = "{ broken json";
    assert!(WasmRuntime::new(bad_json).is_err());
}
