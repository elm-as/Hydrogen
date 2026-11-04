import type { ProjectDef } from "@hydrogen/bindings";

export const COUNTER_PROJECT: ProjectDef = {
  schema_version: { major: 0, minor: 1, patch: 0 },
  project_id: "counter_app",
  display_name: "Application Compteur",
  components: [
    {
      id: "counter",
      fields: {
        value: {
          kind: "int",
          default_value: { kind: "int", data: 0 },
        },
      },
    },
  ],
  entities: [
    {
      id: "main_counter",
      components: {
        counter: {
          value: { kind: "int", data: 10 },
        },
      },
    },
  ],
  events: [
    {
      id: "threshold_reached",
      source: "internal",
      payload_fields: {},
    },
  ],
  rules: [
    {
      id: "inc_counter",
      trigger: { kind: "on_tick" },
      condition: null,
      actions: [
        {
          kind: "add_numeric",
          target: { entity_id: "main_counter", component_id: "counter", field_id: "value" },
          delta: { kind: "int", data: 1 },
        },
      ],
    },
  ],
};

export const ROGUE_PROJECT: ProjectDef = {
  schema_version: { major: 0, minor: 1, patch: 0 },
  project_id: "rogue_core",
  display_name: "Rogue Core (Dungeon Crawler)",
  components: [
    {
      id: "health",
      fields: {
        current: { kind: "int", default_value: { kind: "int", data: 100 } },
        max: { kind: "int", default_value: { kind: "int", data: 100 } },
      },
    },
    {
      id: "stats",
      fields: {
        attack: { kind: "int", default_value: { kind: "int", data: 10 } },
        defense: { kind: "int", default_value: { kind: "int", data: 2 } },
        is_alive: { kind: "bool", default_value: { kind: "bool", data: true } },
      },
    },
    {
      id: "inventory",
      fields: {
        gold: { kind: "int", default_value: { kind: "int", data: 0 } },
        potions: { kind: "int", default_value: { kind: "int", data: 1 } },
      },
    },
    {
      id: "dungeon",
      fields: {
        floor: { kind: "int", default_value: { kind: "int", data: 1 } },
        turn: { kind: "int", default_value: { kind: "int", data: 0 } },
      },
    },
  ],
  entities: [
    {
      id: "hero",
      components: {
        health: {
          current: { kind: "int", data: 100 },
          max: { kind: "int", data: 100 },
        },
        stats: {
          attack: { kind: "int", data: 15 },
          defense: { kind: "int", data: 4 },
          is_alive: { kind: "bool", data: true },
        },
        inventory: {
          gold: { kind: "int", data: 0 },
          potions: { kind: "int", data: 2 },
        },
      },
    },
    {
      id: "goblin",
      components: {
        health: {
          current: { kind: "int", data: 30 },
          max: { kind: "int", data: 30 },
        },
        stats: {
          attack: { kind: "int", data: 6 },
          defense: { kind: "int", data: 1 },
          is_alive: { kind: "bool", data: true },
        },
      },
    },
    {
      id: "game_state",
      components: {
        dungeon: {
          floor: { kind: "int", data: 1 },
          turn: { kind: "int", data: 0 },
        },
      },
    },
  ],
  events: [
    { id: "turn_advance", source: "internal", payload_fields: {} },
    { id: "hero_attack", source: "internal", payload_fields: {} },
    { id: "goblin_defeated", source: "internal", payload_fields: {} },
    { id: "loot_collected", source: "internal", payload_fields: {} },
  ],
  rules: [
    {
      id: "rule_tick_turn",
      trigger: { kind: "on_tick" },
      condition: null,
      actions: [
        {
          kind: "add_numeric",
          target: { entity_id: "game_state", component_id: "dungeon", field_id: "turn" },
          delta: { kind: "int", data: 1 },
        },
        { kind: "emit_event", event_id: "turn_advance", payload: {} },
      ],
    },
    {
      id: "rule_hero_attack_on_turn",
      trigger: { kind: "on_event", event_id: "turn_advance" },
      condition: {
        target: { entity_id: "goblin", component_id: "stats", field_id: "is_alive" },
        operator: "equal",
        expected: { kind: "bool", data: true },
      },
      actions: [
        {
          kind: "add_numeric",
          target: { entity_id: "goblin", component_id: "health", field_id: "current" },
          delta: { kind: "int", data: -15 },
        },
        { kind: "emit_event", event_id: "hero_attack", payload: {} },
      ],
    },
    {
      id: "rule_check_goblin_death",
      trigger: { kind: "on_event", event_id: "hero_attack" },
      condition: {
        target: { entity_id: "goblin", component_id: "health", field_id: "current" },
        operator: "less_or_equal",
        expected: { kind: "int", data: 0 },
      },
      actions: [
        {
          kind: "set_field",
          target: { entity_id: "goblin", component_id: "stats", field_id: "is_alive" },
          value: { kind: "bool", data: false },
        },
        { kind: "emit_event", event_id: "goblin_defeated", payload: {} },
      ],
    },
    {
      id: "rule_reward_on_goblin_death",
      trigger: { kind: "on_event", event_id: "goblin_defeated" },
      condition: null,
      actions: [
        {
          kind: "add_numeric",
          target: { entity_id: "hero", component_id: "inventory", field_id: "gold" },
          delta: { kind: "int", data: 50 },
        },
        { kind: "emit_event", event_id: "loot_collected", payload: {} },
      ],
    },
  ],
};
