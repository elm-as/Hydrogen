//! Inférence et contrôle de type des expressions de valeur (`ValueExpr`).

use hydrogen_core::ValueKind;

use crate::error::SchemaError;
use crate::expression::{ValueExpr, MAX_EXPRESSION_DEPTH};
use crate::schema_index::SchemaIndex;

/// Contexte de validation d'une expression : emplacement lisible et index de résolution.
pub(crate) struct ExprScope<'a> {
    pub(crate) context: &'a str,
    pub(crate) index: &'a SchemaIndex<'a>,
}

/// Vérifie qu'une expression est bien formée et produit le type attendu.
///
/// # Errors
/// Retourne [`SchemaError::TypeMismatch`] si le type produit diffère de `expected`,
/// ou toute erreur de [`infer_expr_kind`].
pub(crate) fn ensure_expr_kind(
    expected: ValueKind,
    expr: &ValueExpr,
    scope: &ExprScope<'_>,
) -> Result<(), SchemaError> {
    let actual = infer_expr_kind(expr, scope, 1)?;
    if actual != expected {
        return Err(SchemaError::TypeMismatch {
            target: scope.context.to_owned(),
            expected,
            actual,
        });
    }
    Ok(())
}

/// Déduit le type produit par une expression en vérifiant références et opérandes.
///
/// # Errors
/// - [`SchemaError::ExpressionTooDeep`] au-delà de [`MAX_EXPRESSION_DEPTH`] niveaux.
/// - [`SchemaError::NonDeterministicValue`] pour un littéral flottant non fini.
/// - [`SchemaError::UnknownReference`] pour un champ inexistant.
/// - [`SchemaError::InvalidNumericOperation`] si une opération porte sur un type non numérique.
/// - [`SchemaError::TypeMismatch`] si les deux opérandes n'ont pas le même type.
fn infer_expr_kind(
    expr: &ValueExpr,
    scope: &ExprScope<'_>,
    depth: usize,
) -> Result<ValueKind, SchemaError> {
    if depth > MAX_EXPRESSION_DEPTH {
        return Err(SchemaError::ExpressionTooDeep {
            context: scope.context.to_owned(),
            max_depth: MAX_EXPRESSION_DEPTH,
        });
    }

    match expr {
        ValueExpr::Literal(value) if !value.is_deterministic() => {
            Err(SchemaError::NonDeterministicValue {
                context: scope.context.to_owned(),
            })
        }
        ValueExpr::Literal(value) => Ok(value.kind()),
        ValueExpr::Field { field } => scope.index.resolve_field_kind(field, scope.context),
        ValueExpr::Binary { left, right, .. } => {
            let left_kind = infer_expr_kind(left, scope, depth + 1)?;
            let right_kind = infer_expr_kind(right, scope, depth + 1)?;
            ensure_numeric_operands(left_kind, right_kind, scope.context)
        }
    }
}

fn ensure_numeric_operands(
    left: ValueKind,
    right: ValueKind,
    context: &str,
) -> Result<ValueKind, SchemaError> {
    if !is_numeric(left) {
        return Err(SchemaError::InvalidNumericOperation {
            target: context.to_owned(),
            actual: left,
        });
    }
    if !is_numeric(right) {
        return Err(SchemaError::InvalidNumericOperation {
            target: context.to_owned(),
            actual: right,
        });
    }

    // Pas de conversion implicite Int/Float : elle masquerait des pertes de précision.
    if left != right {
        return Err(SchemaError::TypeMismatch {
            target: context.to_owned(),
            expected: left,
            actual: right,
        });
    }
    Ok(left)
}

/// Indique si un type supporte les opérations arithmétiques et les comparaisons d'ordre.
pub(crate) const fn is_numeric(kind: ValueKind) -> bool {
    matches!(kind, ValueKind::Int | ValueKind::Float)
}
