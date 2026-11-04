//! Conversion pure d'une définition de projet déclarative (`ProjectDef`) en graphe nodal.

import type { ProjectDef, RuleDef } from "@hydrogen/bindings";
import type { NodalEdge, NodalGraph, NodalNode } from "./types.js";

const COLUMN_X = {
  entity: 40,
  component: 320,
  event: 600,
  rule: 880,
} as const;

const ROW_HEIGHT = 120;

export function projectDefToGraph(project: ProjectDef): NodalGraph {
  const nodes: NodalNode[] = [];
  const edges: NodalEdge[] = [];

  appendEntityNodes(project, nodes, edges);
  appendComponentNodes(project, nodes);
  appendEventNodes(project, nodes);
  appendRuleNodes(project, nodes, edges);

  return { nodes, edges };
}

function appendEntityNodes(project: ProjectDef, nodes: NodalNode[], edges: NodalEdge[]) {
  project.entities.forEach((entity, index) => {
    nodes.push({
      id: `entity:${entity.id}`,
      type: "entity",
      label: entity.id,
      subtitle: `${Object.keys(entity.components).length} composant(s)`,
      position: { x: COLUMN_X.entity, y: 60 + index * ROW_HEIGHT },
      metadata: { entity_id: entity.id },
    });

    Object.keys(entity.components).forEach((compId) => {
      edges.push({
        id: `edge:entity:${entity.id}->comp:${compId}`,
        source: `entity:${entity.id}`,
        target: `comp:${compId}`,
        label: "attache",
      });
    });
  });
}

function appendComponentNodes(project: ProjectDef, nodes: NodalNode[]) {
  project.components.forEach((comp, index) => {
    nodes.push({
      id: `comp:${comp.id}`,
      type: "component",
      label: comp.id,
      subtitle: `${Object.keys(comp.fields).length} champ(s)`,
      position: { x: COLUMN_X.component, y: 60 + index * ROW_HEIGHT },
      metadata: { comp_id: comp.id },
    });
  });
}

function appendEventNodes(project: ProjectDef, nodes: NodalNode[]) {
  project.events.forEach((evt, index) => {
    nodes.push({
      id: `event:${evt.id}`,
      type: "event",
      label: evt.id,
      subtitle: `${Object.keys(evt.payload_fields).length} charge(s)`,
      position: { x: COLUMN_X.event, y: 60 + index * ROW_HEIGHT },
      metadata: { event_id: evt.id },
    });
  });
}

function appendRuleNodes(project: ProjectDef, nodes: NodalNode[], edges: NodalEdge[]) {
  project.rules.forEach((rule, index) => {
    nodes.push({
      id: `rule:${rule.id}`,
      type: "rule",
      label: rule.id,
      subtitle: formatTriggerSubtitle(rule),
      position: { x: COLUMN_X.rule, y: 60 + index * ROW_HEIGHT },
      metadata: { rule_id: rule.id },
    });

    connectRuleEdges(rule, edges);
  });
}

function formatTriggerSubtitle(rule: RuleDef): string {
  if (rule.trigger.kind === "on_tick") {
    return "Déclencheur: Horloge (tick)";
  }
  return `Déclencheur: Événement '${rule.trigger.event_id}'`;
}

function connectRuleEdges(rule: RuleDef, edges: NodalEdge[]) {
  if (rule.trigger.kind === "on_event") {
    edges.push({
      id: `edge:event:${rule.trigger.event_id}->rule:${rule.id}`,
      source: `event:${rule.trigger.event_id}`,
      target: `rule:${rule.id}`,
      label: "déclenche",
    });
  }

  rule.actions.forEach((action, actionIndex) => {
    if (action.kind === "emit_event") {
      edges.push({
        id: `edge:rule:${rule.id}->event:${action.event_id}:${actionIndex}`,
        source: `rule:${rule.id}`,
        target: `event:${action.event_id}`,
        label: "émet",
      });
      return;
    }

    edges.push({
      id: `edge:rule:${rule.id}->target:${action.target.entity_id}:${actionIndex}`,
      source: `rule:${rule.id}`,
      target: `entity:${action.target.entity_id}`,
      label: action.kind === "add_numeric" ? "ajoute" : "affecte",
    });
  });
}
