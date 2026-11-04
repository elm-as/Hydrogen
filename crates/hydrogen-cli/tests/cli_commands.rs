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

    // Création initiale réussie avec template par défaut
    assert!(execute_new(&target, "app").is_ok());
    let project_file = target.join("project.json");
    assert!(project_file.exists());

    // Validation du contenu généré
    assert!(execute_validate(&project_file).is_ok());

    // Rejet si le dossier n'est pas vide
    let err = execute_new(&target, "app").expect_err("doit refuser un dossier non vide");
    assert!(matches!(err, CliError::DirectoryNotEmpty { .. }));
    assert_eq!(err.exit_code(), 3);
}

#[test]
fn new_creates_valid_game_template_and_rejects_unknown_template() {
    let dir = tempdir().expect("création dossier temporaire");
    let target = dir.path().join("my_game");

    // Création réussie avec template game
    assert!(execute_new(&target, "game").is_ok());
    let project_file = target.join("project.json");
    assert!(project_file.exists());
    assert!(execute_validate(&project_file).is_ok());

    // Rejet de template inconnu
    let bad_target = dir.path().join("invalid_proj");
    let err = execute_new(&bad_target, "non_existent").expect_err("template inconnu");
    assert!(matches!(err, CliError::UnknownTemplate { .. }));
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

    execute_new(&target, "app").expect("création projet");
    let project_file = target.join("project.json");

    assert!(execute_run(&project_file, 5).is_ok());
}
