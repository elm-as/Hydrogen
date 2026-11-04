//! Modèle déclaratif des règles métier (déclencheur, condition pure, actions).

use std::collections::BTreeMap;

use hydrogen_core::{FieldAddress, Identifier, ScalarValue};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Déclencheur d'une règle déclarative.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(tag = "kind", content = "event_id", rename_all = "snake_case")]
pub enum RuleTrigger {
    /// Exécutée à chaque cycle (`tick`) de la boucle d'exécution.
    OnTick,
    /// Exécutée lorsqu'un événement portant cet identifiant est reçu.
    OnEvent(Identifier),
}

/// Opérateur de comparaison entre la valeur d'un champ et une constante scalaire.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum ComparisonOperator {
    /// Égalité stricte (`==`).
    Equal,
    /// Différence (`!=`).
    NotEqual,
    /// Strictement supérieur (`>`).
    GreaterThan,
    /// Supérieur ou égal (`>=`).
    GreaterOrEqual,
    /// Strictement inférieur (`<`).
    LessThan,
    /// Inférieur ou égal (`<=`).
    LessOrEqual,
}

/// Prédicat pur vérifiant l'état d'un champ de composant avant l'exécution d'une règle.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
pub struct RuleCondition {
    /// Adresse du champ inspecté (`entité.composant.champ`).
    pub target: FieldAddress,
    /// Opérateur de comparaison appliqué.
    pub operator: ComparisonOperator,
    /// Valeur de référence comparée au champ.
    pub expected: ScalarValue,
}

/// Effet déclaratif produit lorsqu'une règle est déclenchée et que sa condition est vraie.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum RuleAction {
    /// Remplace la valeur d'un champ par une nouvelle valeur constante.
    SetField {
        /// Adresse du champ à modifier.
        target: FieldAddress,
        /// Nouvelle valeur affectée au champ.
        value: ScalarValue,
    },
    /// Ajoute un delta numérique (`Int` ou `Float`) à un champ numérique existant.
    AddNumeric {
        /// Adresse du champ numérique à incrémenter/décrémenter.
        target: FieldAddress,
        /// Delta numérique (`Int` ou `Float`) à additionner.
        delta: ScalarValue,
    },
    /// Émet un nouvel événement dans le bus d'événements du runtime.
    EmitEvent {
        /// Identifiant de l'événement émis.
        event_id: Identifier,
        /// Charge utile attachée à l'événement émis.
        payload: BTreeMap<Identifier, ScalarValue>,
    },
}

/// Règle déclarative complète reliant un déclencheur, une condition optionnelle et des actions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
pub struct RuleDef {
    /// Identifiant unique de la règle.
    pub id: Identifier,
    /// Événement ou cycle déclenchant l'évaluation de la règle.
    pub trigger: RuleTrigger,
    /// Condition optionnelle devant être satisfaite pour exécuter les actions.
    pub condition: Option<RuleCondition>,
    /// Liste ordonnée des actions exécutées lorsque la règle s'applique.
    pub actions: Vec<RuleAction>,
}
