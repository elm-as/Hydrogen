//! Tests d'intégration des commandes de la CLI `hydrogen`.

#![allow(
    clippy::panic,
    clippy::expect_used,
    clippy::too_many_lines,
    clippy::unwrap_used
)]

use std::fs;

use hydrogen_cli::commands::{execute_new, execute_run, execute_validate};
use hydrogen_cli::error::CliError;
use tempfile::tempdir;

#[test]
fn new_creates_valid_project_file_and_rejects_non_empty_dir() {
    let dir = tempdir().expect("création dossier temporaire");
    let target = dir.path().join("my_app");

    // Création initiale réussie
    assert!(execute_new(&target).is_ok());
    let project_file = target.join("project.json");
    assert!(project_file.exists());

    // Validation du contenu généré
    assert!(execute_validate(&project_file).is_ok());

    // Rejet si le dossier n'est pas vide
    let err = execute_new(&target).expect_err("doit refuser un dossier non vide");
    assert!(matches!(err, CliError::DirectoryNotEmpty { .. }));
    assert_eq!(err.exit_code(), 3);
}

#[test]
fn validate_rejects_corrupted_json() {
    let dir = tempdir().expect("création dossier temporaire");
    let bad_file = dir.path().join("broken.json");
    fs::write(&bad_file, "{ invalid json content").expect("écriture");

    let err = execute_validate(&bad_file).expect_err("doit échouer sur JSON corrompu");
    assert_eq!(err.exit_code(), 1);
}

#[test]
fn run_executes_specified_ticks_on_new_project() {
    let dir = tempdir().expect("création dossier temporaire");
    let target = dir.path().join("counter_app");

    execute_new(&target).expect("création projet");
    let project_file = target.join("project.json");

    // Exécution de 5 ticks
    assert!(execute_run(&project_file, 5).is_ok());
}
