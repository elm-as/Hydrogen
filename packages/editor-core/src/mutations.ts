//! Mutations pures et immuables d'une définition de projet (`ProjectDef`).

import type { ProjectDef, RuleDef, ScalarValue } from "@hydrogen/bindings";

export function updateEntityComponentField(
  project: ProjectDef,
  target: { entityId: string; compId: string; fieldId: string },
  newValue: ScalarValue,
): ProjectDef {
  return {
    ...project,
    entities: project.entities.map((entity) => {
      if (entity.id !== target.entityId) {
        return entity;
      }

      const compFields = entity.components[target.compId];
      if (!compFields) {
        return entity;
      }

      return {
        ...entity,
        components: {
          ...entity.components,
          [target.compId]: {
            ...compFields,
            [target.fieldId]: newValue,
          },
        },
      };
    }),
  };
}

export function appendRule(project: ProjectDef, rule: RuleDef): ProjectDef {
  return {
    ...project,
    rules: [...project.rules, rule],
  };
}
