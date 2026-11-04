//! Export automatique des définitions de types TypeScript vers `packages/bindings/src/generated`.

use std::path::Path;

use hydrogen_core::{
    EntityState, FieldAddress, Identifier, RuntimeEvent, ScalarValue, ValueKind, WorldState,
};
use ts_rs::TS;

use crate::error::SchemaError;
use crate::expression::{BinaryOperator, ValueExpr};
use crate::model::{ComponentDef, ComponentFieldDef, EntityDef, EventDef};
use crate::project::ProjectDef;
use crate::rule::{ComparisonOperator, RuleAction, RuleCondition, RuleDef, RuleTrigger};
use crate::version::SchemaVersion;

/// Génère tous les fichiers de bindings TypeScript dans le répertoire `output_dir`.
///
/// # Errors
/// Retourne [`SchemaError::JsonFormat`] si l'export sur disque échoue.
pub fn export_typescript_bindings(output_dir: &Path) -> Result<(), SchemaError> {
    let map_err = |err: ts_rs::ExportError| SchemaError::JsonFormat {
        message: err.to_string(),
    };

    Identifier::export_all_to(output_dir).map_err(map_err)?;
    ValueKind::export_all_to(output_dir).map_err(map_err)?;
    ScalarValue::export_all_to(output_dir).map_err(map_err)?;
    FieldAddress::export_all_to(output_dir).map_err(map_err)?;
    EntityState::export_all_to(output_dir).map_err(map_err)?;
    WorldState::export_all_to(output_dir).map_err(map_err)?;
    RuntimeEvent::export_all_to(output_dir).map_err(map_err)?;
    SchemaVersion::export_all_to(output_dir).map_err(map_err)?;
    ComponentFieldDef::export_all_to(output_dir).map_err(map_err)?;
    ComponentDef::export_all_to(output_dir).map_err(map_err)?;
    EntityDef::export_all_to(output_dir).map_err(map_err)?;
    EventDef::export_all_to(output_dir).map_err(map_err)?;
    BinaryOperator::export_all_to(output_dir).map_err(map_err)?;
    ValueExpr::export_all_to(output_dir).map_err(map_err)?;
    RuleTrigger::export_all_to(output_dir).map_err(map_err)?;
    ComparisonOperator::export_all_to(output_dir).map_err(map_err)?;
    RuleCondition::export_all_to(output_dir).map_err(map_err)?;
    RuleAction::export_all_to(output_dir).map_err(map_err)?;
    RuleDef::export_all_to(output_dir).map_err(map_err)?;
    ProjectDef::export_all_to(output_dir).map_err(map_err)?;

    Ok(())
}
