//! Types fondamentaux du graphe nodal abstrait (@hydrogen/editor-core).

export type NodalNodeType = "entity" | "component" | "rule" | "event";

export interface NodalPosition {
  x: number;
  y: number;
}

export interface NodalNode {
  id: string;
  type: NodalNodeType;
  label: string;
  subtitle: string;
  position: NodalPosition;
  metadata: Record<string, string | number | boolean>;
}

export interface NodalEdge {
  id: string;
  source: string;
  target: string;
  label: string;
}

export interface NodalGraph {
  nodes: NodalNode[];
  edges: NodalEdge[];
}
