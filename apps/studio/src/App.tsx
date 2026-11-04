import type { ProjectDef, WorldState } from "@hydrogen/bindings";
import { projectDefToGraph } from "@hydrogen/editor-core";
import { Badge, Button } from "@hydrogen/ui-kit";
import initWasm, { WasmRuntime } from "@hydrogen/wasm-core";
import type { Node } from "@xyflow/react";
import { useEffect, useMemo, useState } from "react";

import counterProjectJson from "../../../templates/app/project.json";
import rogueProjectJson from "../../../templates/game/project.json";
import { EventLog, type LogEntry } from "./components/EventLog";
import { InspectorPanel } from "./components/InspectorPanel";
import { NodalCanvas } from "./components/NodalCanvas";
import { PlayerActions } from "./components/PlayerActions";
import { WorldStateView } from "./components/WorldStateView";
import "./style.css";

interface TickReportJson {
  tick_index: number;
  rules_executed: number;
  events_processed: number;
  emitted_events: string[];
}

export function App() {
  const [activeProjectKey, setActiveProjectKey] = useState<"rogue" | "counter">("rogue");
  const [activeTab, setActiveTab] = useState<"world" | "graph">("world");
  const [selectedNodeId, setSelectedNodeId] = useState<string | null>(null);

  const [wasmReady, setWasmReady] = useState(false);
  const [runtime, setRuntime] = useState<WasmRuntime | null>(null);
  const [ticksCount, setTicksCount] = useState(0);
  const [simulatedWorld, setSimulatedWorld] = useState<WorldState | null>(null);
  const [statusMessage, setStatusMessage] = useState("Initialisation WASM...");
  const [hasSavedSlot, setHasSavedSlot] = useState(false);
  const [logs, setLogs] = useState<LogEntry[]>([]);

  const project: ProjectDef =
    activeProjectKey === "rogue"
      ? (rogueProjectJson as unknown as ProjectDef)
      : (counterProjectJson as unknown as ProjectDef);

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

  useEffect(() => {
    if (!wasmReady) return;
    try {
      const rt = new WasmRuntime(JSON.stringify(project));
      setRuntime(rt);
      setTicksCount(0);
      setSimulatedWorld(JSON.parse(rt.get_world_state_json()) as WorldState);
      setStatusMessage(`Projet '${project.display_name}' chargé.`);
      setLogs([]);
      const saved = localStorage.getItem(`hydrogen_slot_${project.project_id}`);
      setHasSavedSlot(saved !== null);
    } catch (err) {
      setStatusMessage(`Erreur chargement : ${String(err)}`);
    }
  }, [project, wasmReady]);

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
      const reportJson = runtime.tick();
      const report = JSON.parse(reportJson) as TickReportJson;
      const updatedWorld = JSON.parse(runtime.get_world_state_json()) as WorldState;
      const count = Number(runtime.get_tick_count());

      setSimulatedWorld(updatedWorld);
      setTicksCount(count);
      setStatusMessage(
        `Cycle ${count} : ${report.rules_executed} règle(s), ${report.events_processed} événement(s).`,
      );

      const entry: LogEntry = {
        id: `${count}-${Date.now()}`,
        tick: count,
        type: "tick",
        message: `Cycle complété — ${report.rules_executed} règle(s), ${report.events_processed} événement(s)${report.emitted_events.length > 0 ? ` [émis: ${report.emitted_events.join(", ")}]` : ""}`,
      };
      setLogs((prev) => [entry, ...prev.slice(0, 49)]);
    } catch (err) {
      setStatusMessage(`Erreur cycle : ${String(err)}`);
    }
  }

  function handleTriggerPlayerEvent(eventId: string) {
    if (!runtime) return;
    try {
      runtime.publish_event_by_id(eventId, undefined);
      const reportJson = runtime.tick();
      const report = JSON.parse(reportJson) as TickReportJson;
      const updatedWorld = JSON.parse(runtime.get_world_state_json()) as WorldState;
      const count = Number(runtime.get_tick_count());

      setSimulatedWorld(updatedWorld);
      setTicksCount(count);
      setStatusMessage(`Action joueur '${eventId}' exécutée au cycle ${count}.`);

      const entry: LogEntry = {
        id: `${count}-${Date.now()}`,
        tick: count,
        type: "action",
        message: `Action joueur déclenchée : '${eventId}' -> ${report.rules_executed} règle(s) cascade${report.emitted_events.length > 0 ? ` [émis: ${report.emitted_events.join(", ")}]` : ""}`,
      };
      setLogs((prev) => [entry, ...prev.slice(0, 49)]);
    } catch (err) {
      setStatusMessage(`Erreur action '${eventId}' : ${String(err)}`);
    }
  }

  function handleReset() {
    if (!wasmReady) return;
    try {
      const rt = new WasmRuntime(JSON.stringify(project));
      setRuntime(rt);
      setTicksCount(0);
      setSimulatedWorld(JSON.parse(rt.get_world_state_json()) as WorldState);
      setLogs([]);
      setStatusMessage("Simulation réinitialisée à l'état initial.");
    } catch (err) {
      setStatusMessage(`Erreur réinitialisation : ${String(err)}`);
    }
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
    if (!saved || !runtime) return;
    try {
      const data = JSON.parse(saved) as { tick: number; world: WorldState };
      runtime.restore_world_state(JSON.stringify(data.world), BigInt(data.tick));
      const restored = JSON.parse(runtime.get_world_state_json()) as WorldState;
      setSimulatedWorld(restored);
      setTicksCount(data.tick);
      setStatusMessage(`Slot restauré avec succès au cycle ${data.tick} via WASM.`);

      const entry: LogEntry = {
        id: `restore-${Date.now()}`,
        tick: data.tick,
        type: "event",
        message: `Restauration complète de l'état du monde depuis la sauvegarde (cycle ${data.tick})`,
      };
      setLogs((prev) => [entry, ...prev.slice(0, 49)]);
    } catch (err) {
      setStatusMessage(`Erreur restauration : ${String(err)}`);
    }
  }

  return (
    <div className="studio-root">
      <header className="studio-topbar">
        <div className="studio-title">
          <span>Hydrogen Studio</span>
          <Badge variant="accent">WASM Rust v0.2.0</Badge>
        </div>

        <div style={{ display: "flex", gap: "8px", alignItems: "center" }}>
          <select
            value={activeProjectKey}
            onChange={(e) => setActiveProjectKey(e.target.value as "rogue" | "counter")}
            style={selectStyle}
          >
            <option value="rogue">Rogue Core (Dungeon Crawler)</option>
            <option value="counter">Application Compteur</option>
          </select>

          <Button
            variant={activeTab === "world" ? "primary" : "secondary"}
            size="sm"
            onClick={() => setActiveTab("world")}
          >
            Vue Données
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
        <div className="studio-canvas" style={{ display: "flex", flexDirection: "column" }}>
          <PlayerActions
            events={project.events}
            onTriggerPlayerEvent={handleTriggerPlayerEvent}
            onTick={handleTick}
            onReset={handleReset}
          />

          <div style={{ flex: 1, minHeight: 0, position: "relative" }}>
            {activeTab === "world" ? (
              <WorldStateView world={simulatedWorld} tickCount={ticksCount} />
            ) : (
              <NodalCanvas nodes={flowNodes} edges={flowEdges} onNodeClick={setSelectedNodeId} />
            )}
          </div>

          <EventLog logs={logs} onClear={() => setLogs([])} />
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
