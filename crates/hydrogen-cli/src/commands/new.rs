//! Commande `hydrogen new` : initialisation d'un nouveau projet à partir d'un template officiel.

use std::fs;
use std::path::Path;

use crate::error::CliError;

const APP_PROJECT_TEMPLATE: &str = include_str!("../../../../templates/app/project.json");
const GAME_PROJECT_TEMPLATE: &str = include_str!("../../../../templates/game/project.json");

/// Initialise un nouveau projet Hydrogen dans le répertoire cible avec le template choisi.
///
/// # Errors
/// Retourne [`CliError::UnknownTemplate`] si le modèle n'existe pas,
/// [`CliError::DirectoryNotEmpty`] si le dossier cible contient déjà des éléments,
/// ou [`CliError::Io`] en cas de problème d'écriture.
pub fn execute_new(target_dir: &Path, template: &str) -> Result<(), CliError> {
    let template_content = resolve_template(template)?;
    ensure_target_is_clean(target_dir)?;

    fs::create_dir_all(target_dir)?;
    let project_file = target_dir.join("project.json");
    fs::write(&project_file, template_content)?;

    println!(
        "Projet Hydrogen initialisé avec le modèle '{template}' dans '{}'",
        target_dir.display()
    );
    Ok(())
}

fn resolve_template(template: &str) -> Result<&'static str, CliError> {
    match template {
        "app" => Ok(APP_PROJECT_TEMPLATE),
        "game" => Ok(GAME_PROJECT_TEMPLATE),
        other => Err(CliError::UnknownTemplate {
            template: other.to_string(),
        }),
    }
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
