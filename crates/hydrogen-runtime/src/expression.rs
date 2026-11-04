//! Évaluation pure des expressions de calcul déclaratives (`ValueExpr`).

use hydrogen_core::{CoreError, FieldAddress, ScalarValue, WorldState};
use hydrogen_schema::{BinaryOperator, ValueExpr};

use crate::error::RuntimeError;

/// Évalue une expression déclarative sur l'état courant du monde.
///
/// # Errors
/// Retourne [`RuntimeError::Core`] si un champ référencé n'existe pas ou contient un float non fini,
/// [`RuntimeError::ArithmeticOverflow`] en cas de dépassement,
/// ou [`RuntimeError::InvalidNumericOperation`] sur des types incompatibles.
pub fn evaluate_expr(expr: &ValueExpr, world: &WorldState) -> Result<ScalarValue, RuntimeError> {
    match expr {
        ValueExpr::Literal(val) => evaluate_literal(val),
        ValueExpr::Field { field } => evaluate_field(field, world),
        ValueExpr::Binary { op, left, right } => {
            let left_val = evaluate_expr(left, world)?;
            let right_val = evaluate_expr(right, world)?;
            compute_binary_op(*op, &left_val, &right_val)
        }
    }
}

fn evaluate_literal(val: &ScalarValue) -> Result<ScalarValue, RuntimeError> {
    if !val.is_deterministic() {
        return Err(RuntimeError::Core(CoreError::NonFiniteFloat));
    }
    Ok(val.clone())
}

fn evaluate_field(field: &FieldAddress, world: &WorldState) -> Result<ScalarValue, RuntimeError> {
    world.read_entity_field(field).cloned().ok_or_else(|| {
        RuntimeError::Core(CoreError::ComponentNotFound {
            entity_id: field.entity_id.as_str().to_owned(),
            component_id: field.component_id.as_str().to_owned(),
        })
    })
}

fn compute_binary_op(
    op: BinaryOperator,
    left: &ScalarValue,
    right: &ScalarValue,
) -> Result<ScalarValue, RuntimeError> {
    match (left, right) {
        (ScalarValue::Int(a), ScalarValue::Int(b)) => compute_int_op(op, *a, *b),
        (ScalarValue::Float(a), ScalarValue::Float(b)) => compute_float_op(op, *a, *b),
        _ => Err(RuntimeError::InvalidNumericOperation {
            operation: format!("{op:?}"),
            types: format!("{left:?}, {right:?}"),
        }),
    }
}

fn compute_int_op(op: BinaryOperator, a: i64, b: i64) -> Result<ScalarValue, RuntimeError> {
    let result = match op {
        BinaryOperator::Add => a.checked_add(b),
        BinaryOperator::Sub => a.checked_sub(b),
        BinaryOperator::Mul => a.checked_mul(b),
        BinaryOperator::Min => Some(std::cmp::min(a, b)),
        BinaryOperator::Max => Some(std::cmp::max(a, b)),
    };

    result
        .map(ScalarValue::Int)
        .ok_or_else(|| RuntimeError::ArithmeticOverflow {
            target: format!("calcul entier {a} {op:?} {b}"),
        })
}

fn compute_float_op(op: BinaryOperator, a: f64, b: f64) -> Result<ScalarValue, RuntimeError> {
    let result = match op {
        BinaryOperator::Add => a + b,
        BinaryOperator::Sub => a - b,
        BinaryOperator::Mul => a * b,
        BinaryOperator::Min => a.min(b),
        BinaryOperator::Max => a.max(b),
    };

    if !result.is_finite() {
        return Err(RuntimeError::ArithmeticOverflow {
            target: format!("calcul flottant non fini {a} {op:?} {b}"),
        });
    }

    Ok(ScalarValue::Float(result))
}
