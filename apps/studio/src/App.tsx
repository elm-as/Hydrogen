import type { ProjectDef, WorldState } from "@hydrogen/bindings";
import { projectDefToGraph } from "@hydrogen/editor-core";
import { Badge, Button } from "@hydrogen/ui-kit";
import initWasm, { WasmRuntime } from "@hydrogen/wasm-core";
import type { Node } from "@xyflow/react";
import { useEffect, useMemo, useState } from "react";

import { InspectorPanel } from "./components/InspectorPanel";
import { NodalCanvas } from "./components/NodalCanvas";
import { RogueGameView } from "./components/RogueGameView";
import { COUNTER_PROJECT, ROGUE_PROJECT } from "./templates/projects";
import "./style.css";

export function App() {
  const [activeProjectKey, setActiveProjectKey] = useState<"rogue" | "counter">("rogue");
  const [activeTab, setActiveTab] = useState<"game" | "graph">("game");
  const [selectedNodeId, setSelectedNodeId] = useState<string | null>(null);

  const [wasmReady, setWasmReady] = useState(false);
  const [runtime, setRuntime] = useState<WasmRuntime | null>(null);
  const [ticksCount, setTicksCount] = useState(0);
  const [simulatedWorld, setSimulatedWorld] = useState<WorldState | null>(null);
  const [statusMessage, setStatusMessage] = useState("Initialisation WASM...");
  const [hasSavedSlot, setHasSavedSlot] = useState(false);

  const project: ProjectDef = activeProjectKey === "rogue" ? ROGUE_PROJECT : COUNTER_PROJECT;

  // Initialisation du runtime WASM
  useEffect(() => {
    async function boot() {
      try {
        await initWasm();
        setWasmReady(true);
      } catch (err) {
        setStatusMessage(`Erreur WASM: ${String(err)}`);
      }
    }
    boot();
  }, []);

  // Recréation du runtime lors du changement de projet
  useEffect(() => {
    if (!wasmReady) return;
    try {
      const rt = new WasmRuntime(JSON.stringify(project));
      setRuntime(rt);
      setTicksCount(0);
      setSimulatedWorld(JSON.parse(rt.get_world_state_json()) as WorldState);
      setStatusMessage(`Projet '${project.display_name}' chargé.`);
      const saved = localStorage.getItem(`hydrogen_slot_${project.project_id}`);
      setHasSavedSlot(saved !== null);
    } catch (err) {
      setStatusMessage(`Erreur de chargement : ${String(err)}`);
    }
  }, [project, wasmReady]);

  // Conversion en graphe nodal
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

  function handleTick() {
    if (!runtime) return;
    try {
      runtime.tick();
      const updatedWorld = JSON.parse(runtime.get_world_state_json()) as WorldState;
      const count = Number(runtime.get_tick_count());
      setSimulatedWorld(updatedWorld);
      setTicksCount(count);
      setStatusMessage(`Cycle ${count} exécuté par le cœur Rust.`);
    } catch (err) {
      setStatusMessage(`Erreur cycle : ${String(err)}`);
    }
  }

  function handleReset() {
    if (!wasmReady) return;
    const rt = new WasmRuntime(JSON.stringify(project));
    setRuntime(rt);
    setTicksCount(0);
    setSimulatedWorld(JSON.parse(rt.get_world_state_json()) as WorldState);
    setStatusMessage("Partie réinitialisée.");
  }

  function handleSaveSlot() {
    if (!simulatedWorld) return;
    const payload = JSON.stringify({ tick: ticksCount, world: simulatedWorld });
    localStorage.setItem(`hydrogen_slot_${project.project_id}`, payload);
    setHasSavedSlot(true);
    setStatusMessage(`Slot de sauvegarde enregistré (cycle ${ticksCount}).`);
  }

  function handleLoadSlot() {
    const saved = localStorage.getItem(`hydrogen_slot_${project.project_id}`);
    if (!saved || !wasmReady) return;
    try {
      const data = JSON.parse(saved) as { tick: number; world: WorldState };
      const rt = new WasmRuntime(JSON.stringify(project));
      for (let i = 0; i < data.tick; i += 1) {
        rt.tick();
      }
      setRuntime(rt);
      setTicksCount(data.tick);
      setSimulatedWorld(data.world);
      setStatusMessage(`Slot restauré au cycle ${data.tick}.`);
    } catch (err) {
      setStatusMessage(`Erreur restauration : ${String(err)}`);
    }
  }

  return (
    <div className="studio-root">
      <header className="studio-topbar">
        <div className="studio-title">
          <span>Hydrogen Studio</span>
          <Badge variant="accent">WASM Rust v0.1.0</Badge>
        </div>

        <div style={{ display: "flex", gap: "8px", alignItems: "center" }}>
          <select
            value={activeProjectKey}
            onChange={(e) => setActiveProjectKey(e.target.value as "rogue" | "counter")}
            style={selectStyle}
          >
            <option value="rogue">Rogue Core (Mini-Jeu)</option>
            <option value="counter">Application Compteur</option>
          </select>

          <Button
            variant={activeTab === "game" ? "primary" : "secondary"}
            size="sm"
            onClick={() => setActiveTab("game")}
          >
            Vue Jeu
          </Button>
          <Button
            variant={activeTab === "graph" ? "primary" : "secondary"}
            size="sm"
            onClick={() => setActiveTab("graph")}
          >
            Graphe Nodal
          </Button>
        </div>
      </header>

      <div className="studio-workspace">
        <div className="studio-canvas">
          {activeTab === "game" ? (
            <RogueGameView
              world={simulatedWorld}
              tickCount={ticksCount}
              onTick={handleTick}
              onReset={handleReset}
            />
          ) : (
            <NodalCanvas nodes={flowNodes} edges={flowEdges} onNodeClick={setSelectedNodeId} />
          )}
        </div>

        <InspectorPanel
          selectedNodeId={selectedNodeId}
          project={project}
          ticksCount={ticksCount}
          simulatedWorld={simulatedWorld}
          onSaveSlot={handleSaveSlot}
          onLoadSlot={handleLoadSlot}
          hasSavedSlot={hasSavedSlot}
        />
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

const selectStyle: React.CSSProperties = {
  backgroundColor: "#161b22",
  color: "#f0f6fc",
  border: "1px solid #30363d",
  borderRadius: "4px",
  padding: "4px 8px",
  fontSize: "12px",
  fontWeight: 600,
  outline: "none",
  cursor: "pointer",
};
