//! Commande `hydrogen validate` : validation statique et sémantique d'un projet sans exécution.

use std::fs;
use std::path::Path;

use hydrogen_schema::ProjectDef;

use crate::error::CliError;

/// Valide le fichier de définition de projet Hydrogen indiqué.
///
/// # Errors
/// Retourne [`CliError::Io`] si le fichier est inaccessible, ou [`CliError::Schema`]
/// si le schéma ou les contraintes sémantiques sont violés.
pub fn execute_validate(project_path: &Path) -> Result<(), CliError> {
    let content = fs::read_to_string(project_path).map_err(|err| CliError::InvalidProjectFile {
        path: project_path.to_path_buf(),
        reason: err.to_string(),
    })?;

    let project = ProjectDef::from_json_str(&content).map_err(CliError::Schema)?;

    println!(
        "Validation réussie : '{}' ({}) - {} composant(s), {} entité(s), {} règle(s).",
        project.display_name,
        project.project_id,
        project.components.len(),
        project.entities.len(),
        project.rules.len()
    );

    Ok(())
}
