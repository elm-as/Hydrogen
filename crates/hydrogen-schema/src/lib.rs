//! Schémas de définition versionnés, validation pure et génération de bindings TypeScript.
//! Dépend exclusivement de `hydrogen-core`.

pub mod bindings;
pub mod error;
pub mod expression;
pub(crate) mod expression_validation;
pub mod model;
pub mod project;
pub mod rule;
pub(crate) mod schema_index;
pub mod validation;
pub mod version;

pub use bindings::export_typescript_bindings;
pub use error::SchemaError;
pub use expression::{BinaryOperator, ValueExpr, MAX_EXPRESSION_DEPTH};
pub use model::{ComponentDef, ComponentFieldDef, EntityDef, EventDef};
pub use project::ProjectDef;
pub use rule::{ComparisonOperator, RuleAction, RuleCondition, RuleDef, RuleTrigger};
pub use validation::validate_project_def;
pub use version::{SchemaVersion, CURRENT_SCHEMA_VERSION};
