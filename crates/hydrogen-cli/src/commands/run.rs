//! Commande `hydrogen run` : exécution déterministe d'un projet sur N cycles d'horloge.

use std::fs;
use std::path::Path;

use hydrogen_runtime::{Runtime, RuntimeConfig};
use hydrogen_schema::ProjectDef;

use crate::error::CliError;

/// Charge et exécute un projet déclaratif sur le nombre de cycles spécifié.
///
/// # Errors
/// Retourne [`CliError`] en cas de fichier inaccessible, schéma invalide ou erreur runtime.
pub fn execute_run(project_path: &Path, ticks: u64) -> Result<(), CliError> {
    let content = fs::read_to_string(project_path).map_err(|err| CliError::InvalidProjectFile {
        path: project_path.to_path_buf(),
        reason: err.to_string(),
    })?;

    let project = ProjectDef::from_json_str(&content).map_err(CliError::Schema)?;
    let mut runtime = Runtime::new(project, RuntimeConfig::default()).map_err(CliError::Runtime)?;

    println!(
        "Démarrage de l'exécution : '{}' pour {} cycle(s)...",
        runtime.project().display_name,
        ticks
    );

    let mut total_rules_executed = 0;
    let mut total_events_processed = 0;

    for _ in 0..ticks {
        let report = runtime.tick().map_err(CliError::Runtime)?;
        total_rules_executed += report.rules_executed;
        total_events_processed += report.events_processed;
    }

    println!(
        "Fin d'exécution : {} cycles complétés, {} règles exécutées, {} événements traités.",
        runtime.tick_count(),
        total_rules_executed,
        total_events_processed
    );

    let world_json = serde_json::to_string_pretty(runtime.world()).map_err(|err| {
        CliError::Schema(hydrogen_schema::SchemaError::JsonFormat {
            message: err.to_string(),
        })
    })?;

    println!("État final du monde :\n{world_json}");
    Ok(())
}
