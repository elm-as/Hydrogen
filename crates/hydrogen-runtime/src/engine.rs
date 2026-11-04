//! Orchestrateur central d'exécution déclarative (`Runtime`).

use hydrogen_core::{EventBus, RuntimeEvent, WorldState};
use hydrogen_schema::{ProjectDef, RuleDef, RuleTrigger};
use serde::{Deserialize, Serialize};

use crate::action::apply_action;
use crate::condition::evaluate_condition;
use crate::config::RuntimeConfig;
use crate::error::RuntimeError;
use crate::world_builder::build_initial_world;

/// Résumé métrique d'un cycle d'exécution (`tick`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TickReport {
    /// Numéro séquentiel du cycle (commence à 1).
    pub tick_index: u64,
    /// Nombre de règles exécutées durant ce cycle.
    pub rules_executed: usize,
    /// Nombre d'événements traités durant ce cycle.
    pub events_processed: usize,
}

/// Moteur d'exécution autonome Hydrogen.
#[derive(Debug, Clone)]
pub struct Runtime {
    project: ProjectDef,
    world: WorldState,
    event_bus: EventBus,
    config: RuntimeConfig,
    tick_count: u64,
}

impl Runtime {
    /// Initialise un nouveau runtime à partir d'un projet et d'une configuration.
    ///
    /// # Errors
    /// Retourne [`RuntimeError`] si le projet est invalide au chargement.
    pub fn new(project: ProjectDef, config: RuntimeConfig) -> Result<Self, RuntimeError> {
        let world = build_initial_world(&project)?;
        Ok(Self {
            project,
            world,
            event_bus: EventBus::new(),
            config,
            tick_count: 0,
        })
    }

    /// Exécute un cycle complet (règles `OnTick` puis cascade d'événements) de façon transactionnelle.
    ///
    /// # Errors
    /// En cas d'erreur, l'état n'est pas modifié et [`RuntimeError`] est retournée.
    pub fn tick(&mut self) -> Result<TickReport, RuntimeError> {
        let mut working_world = self.world.clone();
        let mut working_bus = self.event_bus.clone();

        let rules_on_tick =
            execute_tick_rules(&self.project.rules, &mut working_world, &mut working_bus)?;

        let cascade_result = drain_events(
            &self.project.rules,
            &mut StepContext {
                world: &mut working_world,
                bus: &mut working_bus,
                max_events: self.config.max_events_per_tick,
            },
        )?;

        self.world = working_world;
        self.event_bus = working_bus;
        self.tick_count += 1;

        Ok(TickReport {
            tick_index: self.tick_count,
            rules_executed: rules_on_tick + cascade_result.rules_executed,
            events_processed: cascade_result.events_processed,
        })
    }

    /// Publie un événement externe dans la file du runtime.
    pub fn publish_event(&mut self, event: RuntimeEvent) {
        self.event_bus.publish(event);
    }

    /// Retourne l'état courant du monde en lecture seule.
    pub fn world(&self) -> &WorldState {
        &self.world
    }

    /// Retourne la définition du projet en cours d'exécution.
    pub fn project(&self) -> &ProjectDef {
        &self.project
    }

    /// Retourne la configuration d'exécution.
    pub const fn config(&self) -> &RuntimeConfig {
        &self.config
    }

    /// Retourne le nombre total de cycles complétés.
    pub const fn tick_count(&self) -> u64 {
        self.tick_count
    }
}

struct CascadeReport {
    events_processed: usize,
    rules_executed: usize,
}

struct StepContext<'a> {
    world: &'a mut WorldState,
    bus: &'a mut EventBus,
    max_events: usize,
}

fn execute_tick_rules(
    rules: &[RuleDef],
    world: &mut WorldState,
    bus: &mut EventBus,
) -> Result<usize, RuntimeError> {
    let mut executed = 0;
    for rule in rules {
        if rule.trigger != RuleTrigger::OnTick {
            continue;
        }

        if !is_condition_met(rule, world) {
            continue;
        }

        apply_rule_actions(rule, world, bus)?;
        executed += 1;
    }
    Ok(executed)
}

fn drain_events(
    rules: &[RuleDef],
    context: &mut StepContext<'_>,
) -> Result<CascadeReport, RuntimeError> {
    let mut events_count = 0;
    let mut rules_count = 0;

    while let Some(event) = context.bus.pop_next() {
        events_count += 1;
        if events_count > context.max_events {
            return Err(RuntimeError::EventLoopLimitExceeded {
                limit: context.max_events,
            });
        }

        for rule in rules {
            if rule.trigger != RuleTrigger::OnEvent(event.event_id.clone()) {
                continue;
            }

            if !is_condition_met(rule, context.world) {
                continue;
            }

            apply_rule_actions(rule, context.world, context.bus)?;
            rules_count += 1;
        }
    }

    Ok(CascadeReport {
        events_processed: events_count,
        rules_executed: rules_count,
    })
}

fn is_condition_met(rule: &RuleDef, world: &WorldState) -> bool {
    rule.condition
        .as_ref()
        .map_or(true, |cond| evaluate_condition(cond, world))
}

fn apply_rule_actions(
    rule: &RuleDef,
    world: &mut WorldState,
    bus: &mut EventBus,
) -> Result<(), RuntimeError> {
    for action in &rule.actions {
        apply_action(action, world, bus)?;
    }
    Ok(())
}
