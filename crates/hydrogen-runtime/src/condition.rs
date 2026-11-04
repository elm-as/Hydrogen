//! Évaluation pure des conditions de règles déclaratives (`RuleCondition`).

use std::cmp::Ordering;

use hydrogen_core::{ScalarValue, WorldState};
use hydrogen_schema::{ComparisonOperator, RuleCondition};

use crate::error::RuntimeError;
use crate::expression::evaluate_expr;

/// Évalue si la condition d'une règle est satisfaite sur l'état courant du monde.
///
/// # Errors
/// Retourne [`RuntimeError`] si l'évaluation de l'expression attendue échoue.
pub fn evaluate_condition(
    condition: &RuleCondition,
    world: &WorldState,
) -> Result<bool, RuntimeError> {
    let Some(actual) = world.read_entity_field(&condition.target) else {
        return Ok(false);
    };

    let expected = evaluate_expr(&condition.expected, world)?;

    let is_met = match condition.operator {
        ComparisonOperator::Equal => actual == &expected,
        ComparisonOperator::NotEqual => actual != &expected,
        ComparisonOperator::GreaterThan => {
            compare_numeric(actual, &expected).is_some_and(Ordering::is_gt)
        }
        ComparisonOperator::GreaterOrEqual => {
            compare_numeric(actual, &expected).is_some_and(Ordering::is_ge)
        }
        ComparisonOperator::LessThan => {
            compare_numeric(actual, &expected).is_some_and(Ordering::is_lt)
        }
        ComparisonOperator::LessOrEqual => {
            compare_numeric(actual, &expected).is_some_and(Ordering::is_le)
        }
    };

    Ok(is_met)
}

fn compare_numeric(actual: &ScalarValue, expected: &ScalarValue) -> Option<Ordering> {
    match (actual, expected) {
        (ScalarValue::Int(a), ScalarValue::Int(b)) => Some(a.cmp(b)),
        (ScalarValue::Float(a), ScalarValue::Float(b)) => a.partial_cmp(b),
        _ => None,
    }
}
