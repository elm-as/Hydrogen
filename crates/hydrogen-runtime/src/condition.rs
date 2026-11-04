//! Évaluation pure des conditions de règles déclaratives (`RuleCondition`).

use std::cmp::Ordering;

use hydrogen_core::{ScalarValue, WorldState};
use hydrogen_schema::{ComparisonOperator, RuleCondition};

/// Évalue si la condition d'une règle est satisfaite sur l'état courant du monde.
pub fn evaluate_condition(condition: &RuleCondition, world: &WorldState) -> bool {
    let Some(actual) = world.read_entity_field(&condition.target) else {
        return false;
    };

    match condition.operator {
        ComparisonOperator::Equal => actual == &condition.expected,
        ComparisonOperator::NotEqual => actual != &condition.expected,
        ComparisonOperator::GreaterThan => {
            compare_numeric(actual, &condition.expected).is_some_and(Ordering::is_gt)
        }
        ComparisonOperator::GreaterOrEqual => {
            compare_numeric(actual, &condition.expected).is_some_and(Ordering::is_ge)
        }
        ComparisonOperator::LessThan => {
            compare_numeric(actual, &condition.expected).is_some_and(Ordering::is_lt)
        }
        ComparisonOperator::LessOrEqual => {
            compare_numeric(actual, &condition.expected).is_some_and(Ordering::is_le)
        }
    }
}

fn compare_numeric(actual: &ScalarValue, expected: &ScalarValue) -> Option<Ordering> {
    match (actual, expected) {
        (ScalarValue::Int(a), ScalarValue::Int(b)) => Some(a.cmp(b)),
        (ScalarValue::Float(a), ScalarValue::Float(b)) => a.partial_cmp(b),
        _ => None,
    }
}
