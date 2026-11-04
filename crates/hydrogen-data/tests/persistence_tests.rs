//! Tests d'intégration et de robustesse pour la couche de persistance.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::collections::BTreeMap;
use tempfile::tempdir;

use hydrogen_core::{EntityState, Identifier, ScalarValue, WorldState};
use hydrogen_data::{DataError, FsStore, MemoryStore, SaveSnapshot, SaveStore};
use hydrogen_schema::SchemaVersion;

fn sample_world_state() -> WorldState {
    let mut state = WorldState::new();
    let entity_id = Identifier::new("player").expect("valid entity id");
    let comp_id = Identifier::new("position").expect("valid comp id");
    let field_id = Identifier::new("x").expect("valid field id");

    let mut fields = BTreeMap::new();
    fields.insert(field_id, ScalarValue::Int(42));

    let mut components = BTreeMap::new();
    components.insert(comp_id, fields);

    state.entities.insert(
        entity_id,
        EntityState {
            id: Identifier::new("player").expect("valid entity id"),
            components,
        },
    );
    state
}

#[test]
fn memory_store_saves_loads_and_lists_slots() {
    let mut store = MemoryStore::new();
    let snapshot =
        SaveSnapshot::new("hero_quest", 10, sample_world_state()).with_metadata("author", "tester");

    assert!(!store.exists("slot_1").expect("exists check"));
    store.save("slot_1", &snapshot).expect("save should work");

    assert!(store.exists("slot_1").expect("exists check"));
    assert_eq!(store.list_slots().expect("list"), vec!["slot_1"]);

    let loaded = store.load("slot_1").expect("load should work");
    assert_eq!(loaded.project_name, "hero_quest");
    assert_eq!(loaded.tick, 10);
    assert_eq!(
        loaded.metadata.get("author").map(String::as_str),
        Some("tester")
    );

    let deleted = store.delete("slot_1").expect("delete should work");
    assert!(deleted);
    assert!(!store.exists("slot_1").expect("exists check"));
}

#[test]
fn store_rejects_unsafe_and_empty_slot_names() {
    let mut store = MemoryStore::new();
    let snapshot = SaveSnapshot::new("test", 1, sample_world_state());

    let invalid_names = [
        "",
        "../escape",
        "slot/sub",
        "slot name with spaces",
        "slot*invalid",
    ];
    for name in invalid_names {
        assert!(matches!(
            store.save(name, &snapshot),
            Err(DataError::InvalidSlotName(_))
        ));
        assert!(matches!(
            store.load(name),
            Err(DataError::InvalidSlotName(_))
        ));
        assert!(matches!(
            store.delete(name),
            Err(DataError::InvalidSlotName(_))
        ));
    }
}

#[test]
fn fs_store_saves_atomically_and_recovers_state() {
    let dir = tempdir().expect("tempdir");
    let mut store = FsStore::new(dir.path()).expect("store creation");

    let snapshot_a = SaveSnapshot::new("alpha", 100, sample_world_state());
    let snapshot_b = SaveSnapshot::new("beta", 200, sample_world_state());

    store.save("slot_a", &snapshot_a).expect("save slot_a");
    store.save("slot_b", &snapshot_b).expect("save slot_b");

    let mut slots = store.list_slots().expect("list slots");
    slots.sort();
    assert_eq!(slots, vec!["slot_a", "slot_b"]);

    let loaded_a = store.load("slot_a").expect("load slot_a");
    assert_eq!(loaded_a.project_name, "alpha");
    assert_eq!(loaded_a.tick, 100);

    let deleted = store.delete("slot_a").expect("delete slot_a");
    assert!(deleted);
    assert_eq!(store.list_slots().expect("list remaining"), vec!["slot_b"]);
}

#[test]
fn fs_store_reports_corrupted_data_on_invalid_file() {
    let dir = tempdir().expect("tempdir");
    let store = FsStore::new(dir.path()).expect("store creation");

    let corrupted_path = dir.path().join("broken.json");
    std::fs::write(corrupted_path, "{ broken json payload").expect("write broken");

    let load_res = store.load("broken");
    assert!(matches!(load_res, Err(DataError::CorruptedData(_))));
}

#[test]
fn snapshot_validates_schema_version_compatibility() {
    let mut snapshot = SaveSnapshot::new("demo", 0, sample_world_state());
    assert!(snapshot.validate_version(&SchemaVersion::current()).is_ok());

    snapshot.version = SchemaVersion::new(99, 0, 0);
    let check = snapshot.validate_version(&SchemaVersion::current());
    assert!(matches!(check, Err(DataError::IncompatibleVersion { .. })));
}
