//! Orchestrateur central d'exécution déclarative (`Runtime`).

use hydrogen_core::{EventBus, Identifier, RuntimeEvent, WorldState};
use hydrogen_schema::{ProjectDef, RuleDef, RuleTrigger};
use serde::{Deserialize, Serialize};

use crate::action::apply_action;
use crate::condition::evaluate_condition;
use crate::config::RuntimeConfig;
use crate::error::RuntimeError;
use crate::world_builder::build_initial_world;

/// Résumé métrique d'un cycle d'exécution (`tick`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TickReport {
    /// Numéro séquentiel du cycle (commence à 1).
    pub tick_index: u64,
    /// Nombre de règles exécutées durant ce cycle.
    pub rules_executed: usize,
    /// Nombre d'événements traités durant ce cycle.
    pub events_processed: usize,
    /// Identifiants des événements émis durant ce cycle.
    pub emitted_events: Vec<Identifier>,
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
        let mut emitted_events = Vec::new();

        let rules_on_tick = {
            let mut ctx = ExecutionContext {
                world: &mut working_world,
                bus: &mut working_bus,
                emitted_events: &mut emitted_events,
            };
            execute_tick_rules(&self.project.rules, &mut ctx)?
        };

        let cascade_result = {
            let mut ctx = ExecutionContext {
                world: &mut working_world,
                bus: &mut working_bus,
                emitted_events: &mut emitted_events,
            };
            drain_events(
                &self.project.rules,
                self.config.max_events_per_tick,
                &mut ctx,
            )?
        };

        self.world = working_world;
        self.event_bus = working_bus;
        self.tick_count += 1;

        Ok(TickReport {
            tick_index: self.tick_count,
            rules_executed: rules_on_tick + cascade_result.rules_executed,
            events_processed: cascade_result.events_processed,
            emitted_events,
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

    /// Restaure l'état du monde et le compteur de cycles (ex. depuis une sauvegarde).
    pub fn restore_world_state(&mut self, world: WorldState, tick_count: u64) {
        self.world = world;
        self.tick_count = tick_count;
        self.event_bus = EventBus::new();
    }
}

struct CascadeReport {
    events_processed: usize,
    rules_executed: usize,
}

struct ExecutionContext<'a> {
    world: &'a mut WorldState,
    bus: &'a mut EventBus,
    emitted_events: &'a mut Vec<Identifier>,
}

fn execute_tick_rules(
    rules: &[RuleDef],
    ctx: &mut ExecutionContext<'_>,
) -> Result<usize, RuntimeError> {
    let mut executed = 0;
    for rule in rules {
        if rule.trigger != RuleTrigger::OnTick {
            continue;
        }

        if !is_condition_met(rule, ctx.world)? {
            continue;
        }

        apply_rule_actions(rule, ctx)?;
        executed += 1;
    }
    Ok(executed)
}

fn drain_events(
    rules: &[RuleDef],
    max_events: usize,
    ctx: &mut ExecutionContext<'_>,
) -> Result<CascadeReport, RuntimeError> {
    let mut events_count = 0;
    let mut rules_count = 0;

    while let Some(event) = ctx.bus.pop_next() {
        events_count += 1;
        if events_count > max_events {
            return Err(RuntimeError::EventLoopLimitExceeded { limit: max_events });
        }

        for rule in rules {
            if rule.trigger != RuleTrigger::OnEvent(event.event_id.clone()) {
                continue;
            }

            if !is_condition_met(rule, ctx.world)? {
                continue;
            }

            apply_rule_actions(rule, ctx)?;
            rules_count += 1;
        }
    }

    Ok(CascadeReport {
        events_processed: events_count,
        rules_executed: rules_count,
    })
}

fn is_condition_met(rule: &RuleDef, world: &WorldState) -> Result<bool, RuntimeError> {
    rule.condition
        .as_ref()
        .map_or(Ok(true), |cond| evaluate_condition(cond, world))
}

fn apply_rule_actions(rule: &RuleDef, ctx: &mut ExecutionContext<'_>) -> Result<(), RuntimeError> {
    for action in &rule.actions {
        if let Some(event_id) = apply_action(action, ctx.world, ctx.bus)? {
            ctx.emitted_events.push(event_id);
        }
    }
    Ok(())
}
