import type { ProjectDef } from "@hydrogen/bindings";
import { describe, expect, it } from "vitest";
import { projectDefToGraph, updateEntityComponentField } from "../src/index.js";

const SAMPLE_PROJECT: ProjectDef = {
  schema_version: { major: 0, minor: 1, patch: 0 },
  project_id: "test_project",
  display_name: "Test Project",
  components: [
    {
      id: "stats",
      fields: {
        score: {
          kind: "int",
          default_value: { kind: "int", data: 0 },
        },
      },
    },
  ],
  entities: [
    {
      id: "player_1",
      components: {
        stats: {
          score: { kind: "int", data: 42 },
        },
      },
    },
  ],
  events: [
    {
      id: "game_over",
      payload_fields: {},
    },
  ],
  rules: [
    {
      id: "increment_score",
      trigger: { kind: "on_tick" },
      condition: null,
      actions: [
        {
          kind: "add_numeric",
          target: {
            entity_id: "player_1",
            component_id: "stats",
            field_id: "score",
          },
          delta: { kind: "int", data: 1 },
        },
        {
          kind: "emit_event",
          event_id: "game_over",
          payload: {},
        },
      ],
    },
  ],
};

describe("@hydrogen/editor-core", () => {
  it("converts ProjectDef into a typed NodalGraph", () => {
    const graph = projectDefToGraph(SAMPLE_PROJECT);

    expect(graph.nodes).toHaveLength(4);
    expect(graph.nodes.map((n) => n.type)).toEqual(["entity", "component", "event", "rule"]);

    expect(graph.edges.length).toBeGreaterThanOrEqual(3);
    const entityEdge = graph.edges.find((e) => e.source === "entity:player_1");
    expect(entityEdge?.target).toBe("comp:stats");

    const ruleActionEdge = graph.edges.find(
      (e) => e.source === "rule:increment_score" && e.target === "event:game_over",
    );
    expect(ruleActionEdge).toBeDefined();
  });

  it("updates entity component field immutably", () => {
    const updated = updateEntityComponentField(
      SAMPLE_PROJECT,
      { entityId: "player_1", compId: "stats", fieldId: "score" },
      { kind: "int", data: 99 },
    );

    expect(SAMPLE_PROJECT.entities[0]?.components.stats?.score).toEqual({
      kind: "int",
      data: 42,
    });
    expect(updated.entities[0]?.components.stats?.score).toEqual({
      kind: "int",
      data: 99,
    });
  });
});
