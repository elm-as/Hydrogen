//! Configuration d'exécution du moteur `hydrogen-runtime`.

const DEFAULT_MAX_EVENTS_PER_TICK: usize = 1024;

/// Paramètres régissant l'exécution des cycles du moteur déclaratif.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeConfig {
    /// Nombre maximal d'événements autorisés à être traités en cascade au cours d'un même cycle.
    pub max_events_per_tick: usize,
}

impl RuntimeConfig {
    /// Crée une nouvelle configuration avec une limite d'événements explicite.
    pub const fn with_max_events(max_events_per_tick: usize) -> Self {
        Self {
            max_events_per_tick,
        }
    }
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            max_events_per_tick: DEFAULT_MAX_EVENTS_PER_TICK,
        }
    }
}
