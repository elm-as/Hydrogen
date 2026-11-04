import { describe, expect, it } from "vitest";
import type { ProjectDef } from "../src/index.js";

describe("@hydrogen/bindings generated types", () => {
  it("accepts a strongly-typed ProjectDef matching the Rust schema", () => {
    const sampleProject: ProjectDef = {
      schema_version: { major: 0, minor: 1, patch: 0 },
      project_id: "sample_counter",
      display_name: "Sample Counter",
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
          id: "increment_on_tick",
          trigger: { kind: "on_tick" },
          condition: {
            target: {
              entity_id: "main_counter",
              component_id: "counter",
              field_id: "value",
            },
            operator: "less_than",
            expected: { kind: "int", data: 100 },
          },
          actions: [
            {
              kind: "add_numeric",
              target: {
                entity_id: "main_counter",
                component_id: "counter",
                field_id: "value",
              },
              delta: { kind: "int", data: 5 },
            },
          ],
        },
      ],
    };

    expect(sampleProject.project_id).toBe("sample_counter");
    expect(sampleProject.rules).toHaveLength(1);
  });
});
