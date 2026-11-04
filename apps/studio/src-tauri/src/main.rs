//! Point d'entrée du backend de bureau Hydrogen Studio (Tauri v2).

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;

use hydrogen_core::WorldState;
use hydrogen_runtime::{Runtime, RuntimeConfig};
use hydrogen_schema::ProjectDef;
use serde::{Deserialize, Serialize};

/// Résultat d'une exécution de simulation pas-à-pas renvoyé à l'éditeur.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudioRunResult {
    /// Nombre total de cycles complétés.
    pub tick_count: u64,
    /// Nombre total de règles exécutées.
    pub rules_executed: usize,
    /// Nombre d'événements traités au cours des cycles.
    pub events_processed: usize,
    /// État final du monde.
    pub world: WorldState,
}

#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
fn load_project(path: String) -> Result<ProjectDef, String> {
    let content = fs::read_to_string(&path).map_err(|err| err.to_string())?;
    ProjectDef::from_json_str(&content).map_err(|err| err.to_string())
}

#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
fn save_project(path: String, project: ProjectDef) -> Result<(), String> {
    let json = project.to_pretty_json().map_err(|err| err.to_string())?;
    fs::write(&path, json).map_err(|err| err.to_string())
}

#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
fn validate_project(project: ProjectDef) -> Result<(), String> {
    project.validate().map_err(|err| err.to_string())
}

#[tauri::command]
fn run_project_ticks(project: ProjectDef, ticks: u64) -> Result<StudioRunResult, String> {
    let mut runtime =
        Runtime::new(project, RuntimeConfig::default()).map_err(|err| err.to_string())?;

    let mut rules_executed = 0;
    let mut events_processed = 0;

    for _ in 0..ticks {
        let report = runtime.tick().map_err(|err| err.to_string())?;
        rules_executed += report.rules_executed;
        events_processed += report.events_processed;
    }

    Ok(StudioRunResult {
        tick_count: runtime.tick_count(),
        rules_executed,
        events_processed,
        world: runtime.world().clone(),
    })
}

fn main() {
    let app_result = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            load_project,
            save_project,
            validate_project,
            run_project_ticks
        ])
        .run(tauri::generate_context!());

    if let Err(err) = app_result {
        eprintln!("[Erreur Tauri Studio] {err}");
        std::process::exit(1);
    }
}
