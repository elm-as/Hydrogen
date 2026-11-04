//! Commande `hydrogen new` : initialisation d'un nouveau projet à partir du template embarqué.

use std::fs;
use std::path::Path;

use crate::error::CliError;

const DEFAULT_PROJECT_TEMPLATE: &str = include_str!("../../../../templates/app/project.json");

/// Initialise un nouveau projet Hydrogen dans le répertoire cible.
///
/// # Errors
/// Retourne [`CliError::DirectoryNotEmpty`] si le dossier cible contient déjà des éléments,
/// ou [`CliError::Io`] en cas de problème d'écriture.
pub fn execute_new(target_dir: &Path) -> Result<(), CliError> {
    ensure_target_is_clean(target_dir)?;

    fs::create_dir_all(target_dir)?;
    let project_file = target_dir.join("project.json");
    fs::write(&project_file, DEFAULT_PROJECT_TEMPLATE)?;

    println!(
        "Projet Hydrogen initialisé avec succès dans '{}'",
        target_dir.display()
    );
    Ok(())
}

fn ensure_target_is_clean(target_dir: &Path) -> Result<(), CliError> {
    if !target_dir.exists() {
        return Ok(());
    }

    let is_empty = fs::read_dir(target_dir)?.next().is_none();

    if !is_empty {
        return Err(CliError::DirectoryNotEmpty {
            path: target_dir.to_path_buf(),
        });
    }

    Ok(())
}
