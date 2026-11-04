//! Événements typés et file d'attente déterministe en mémoire.

use std::collections::{BTreeMap, VecDeque};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::identifier::Identifier;
use crate::value::ScalarValue;

/// Instance d'événement émise lors de l'exécution d'un projet Hydrogen.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
pub struct RuntimeEvent {
    /// Identifiant du schéma d'événement.
    pub event_id: Identifier,
    /// Charge utile structurée (`nom_champ -> valeur`).
    pub payload: BTreeMap<Identifier, ScalarValue>,
}

impl RuntimeEvent {
    /// Crée un événement sans charge utile.
    pub fn new(event_id: Identifier) -> Self {
        Self {
            event_id,
            payload: BTreeMap::new(),
        }
    }

    /// Crée un événement accompagné de sa charge utile.
    pub fn with_payload(event_id: Identifier, payload: BTreeMap<Identifier, ScalarValue>) -> Self {
        Self { event_id, payload }
    }
}

/// File FIFO déterministe stockant les événements en attente de traitement par le moteur.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EventBus {
    queue: VecDeque<RuntimeEvent>,
}

impl EventBus {
    /// Crée une file d'événements vide.
    pub fn new() -> Self {
        Self::default()
    }

    /// Empile un nouvel événement en fin de file.
    pub fn publish(&mut self, event: RuntimeEvent) {
        self.queue.push_back(event);
    }

    /// Dépile le prochain événement à traiter dans l'ordre FIFO.
    pub fn pop_next(&mut self) -> Option<RuntimeEvent> {
        self.queue.pop_front()
    }

    /// Indique si la file d'événements est vide.
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// Retourne le nombre d'événements en attente.
    pub fn len(&self) -> usize {
        self.queue.len()
    }
}
