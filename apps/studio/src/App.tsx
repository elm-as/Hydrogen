import type { ProjectDef, WorldState } from "@hydrogen/bindings";
import { projectDefToGraph, updateEntityComponentField } from "@hydrogen/editor-core";
import { Background, Controls, type Node, ReactFlow } from "@xyflow/react";
import { useMemo, useState } from "react";
import "./style.css";

const INITIAL_PROJECT: ProjectDef = {
  schema_version: { major: 0, minor: 1, patch: 0 },
  project_id: "studio_sample",
  display_name: "Projet Studio Demo",
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
      id: "hero",
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
          target: { entity_id: "hero", component_id: "counter", field_id: "value" },
          delta: { kind: "int", data: 1 },
        },
      ],
    },
  ],
};

export function App() {
  const [project, setProject] = useState<ProjectDef>(INITIAL_PROJECT);
  const [selectedNodeId, setSelectedNodeId] = useState<string | null>(null);
  const [ticksCount, setTicksCount] = useState(0);
  const [simulatedWorld, setSimulatedWorld] = useState<WorldState | null>(null);
  const [statusMessage, setStatusMessage] = useState("Studio prêt.");

  const { flowNodes, flowEdges } = useMemo(() => {
    const graph = projectDefToGraph(project);
    const nodes: Node[] = graph.nodes.map((n) => ({
      id: n.id,
      position: n.position,
      data: { label: `${n.label}\n(${n.subtitle})` },
      style: getNodeStyle(n.type),
    }));
    const edges = graph.edges.map((e) => ({
      id: e.id,
      source: e.source,
      target: e.target,
      label: e.label,
      animated: e.label === "déclenche",
      style: { stroke: "#38bdf8" },
    }));
    return { flowNodes: nodes, flowEdges: edges };
  }, [project]);

  function handleIncrementInitial() {
    const currentVal = project.entities[0]?.components.counter?.value;
    const currentNum = currentVal?.kind === "int" ? currentVal.data : 0;
    const updated = updateEntityComponentField(
      project,
      { entityId: "hero", compId: "counter", fieldId: "value" },
      { kind: "int", data: currentNum + 5 },
    );
    setProject(updated);
    setStatusMessage("Valeur initiale du héros augmentée de +5.");
  }

  function handleSimulateTick() {
    setTicksCount((prev) => prev + 1);
    const baseValue = project.entities[0]?.components.counter?.value;
    const num = baseValue?.kind === "int" ? baseValue.data : 0;

    const mockWorld: WorldState = {
      entities: {
        hero: {
          id: "hero",
          components: {
            counter: {
              value: { kind: "int", data: num + ticksCount + 1 },
            },
          },
        },
      },
    };
    setSimulatedWorld(mockWorld);
    setStatusMessage(`Cycle ${ticksCount + 1} simulé avec succès.`);
  }

  return (
    <div className="studio-root">
      <header className="studio-topbar">
        <div className="studio-title">
          <span>Hydrogen Studio</span>
          <span className="studio-badge tabular">v0.1.0 (Tauri v2 + React)</span>
        </div>
        <div className="studio-actions">
          <button type="button" onClick={handleIncrementInitial}>
            Modifier héros (+5)
          </button>
          <button type="button" className="primary" onClick={handleSimulateTick}>
            Simuler 1 cycle
          </button>
        </div>
      </header>

      <div className="studio-workspace">
        <div className="studio-canvas">
          <ReactFlow
            nodes={flowNodes}
            edges={flowEdges}
            onNodeClick={(_, node) => setSelectedNodeId(node.id)}
            fitView
          >
            <Background color="#1e293b" gap={16} />
            <Controls />
          </ReactFlow>
        </div>

        <aside className="studio-inspector">
          <div className="inspector-section">
            <h3>Nœud sélectionné</h3>
            <div className="tabular" style={{ color: "#38bdf8", fontWeight: 600 }}>
              {selectedNodeId ?? "Aucun nœud sélectionné (cliquez sur un nœud)"}
            </div>
          </div>

          <div className="inspector-section">
            <h3>Métriques du projet</h3>
            <div
              className="tabular"
              style={{ fontSize: "0.85rem", display: "grid", gap: "0.25rem" }}
            >
              <div>Entités : {project.entities.length}</div>
              <div>Composants : {project.components.length}</div>
              <div>Règles : {project.rules.length}</div>
              <div>Cycles joués : {ticksCount}</div>
            </div>
          </div>

          <div className="inspector-section">
            <h3>État de simulation (WorldState)</h3>
            <pre className="tabular">
              {JSON.stringify(simulatedWorld ?? project.entities, null, 2)}
            </pre>
          </div>
        </aside>
      </div>

      <footer className="status-bar">
        <span>{statusMessage}</span>
        <span className="tabular">{project.project_id}</span>
      </footer>
    </div>
  );
}

function getNodeStyle(type: string): React.CSSProperties {
  const base: React.CSSProperties = {
    background: "#141822",
    color: "#e2e8f0",
    border: "1px solid #262c3e",
    borderRadius: "6px",
    padding: "10px",
    fontSize: "12px",
    minWidth: "140px",
    whiteSpace: "pre-wrap",
  };

  switch (type) {
    case "entity":
      return { ...base, borderColor: "#38bdf8" };
    case "component":
      return { ...base, borderColor: "#34d399" };
    case "rule":
      return { ...base, borderColor: "#fbbf24" };
    default:
      return base;
  }
}
