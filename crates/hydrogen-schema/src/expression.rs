//! Expressions de valeur déclaratives (`ValueExpr`) utilisées par les règles.
//!
//! Une expression est un arbre pur : littéral, lecture de champ ou opération binaire.
//! La forme arborescente (plutôt qu'une formule texte) correspond un pour un aux nœuds
//! de l'éditeur visuel, et se valide sans parseur.

use hydrogen_core::{FieldAddress, ScalarValue};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Profondeur maximale d'un arbre d'expression, pour borner le coût d'évaluation
/// et la récursion du validateur comme de l'évaluateur.
pub const MAX_EXPRESSION_DEPTH: usize = 16;

/// Opérateur binaire numérique applicable à deux expressions de même type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum BinaryOperator {
    /// Addition (`a + b`).
    Add,
    /// Soustraction (`a - b`).
    Sub,
    /// Multiplication (`a * b`).
    Mul,
    /// Plus petite des deux valeurs.
    Min,
    /// Plus grande des deux valeurs.
    Max,
}

/// Expression produisant une valeur scalaire à l'exécution.
///
/// La sérialisation est non étiquetée : un littéral garde exactement la forme JSON
/// d'un [`ScalarValue`] (`{"kind":"int","data":5}`), ce qui laisse les définitions
/// antérieures à la version 0.2.0 valides sans migration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(untagged)]
pub enum ValueExpr {
    /// Valeur constante.
    Literal(ScalarValue),
    /// Lecture de la valeur courante d'un champ de composant.
    Field {
        /// Adresse du champ lu.
        field: FieldAddress,
    },
    /// Opération binaire entre deux sous-expressions.
    Binary {
        /// Opérateur appliqué.
        op: BinaryOperator,
        /// Opérande gauche.
        left: Box<ValueExpr>,
        /// Opérande droite.
        right: Box<ValueExpr>,
    },
}

impl From<ScalarValue> for ValueExpr {
    fn from(value: ScalarValue) -> Self {
        Self::Literal(value)
    }
}
