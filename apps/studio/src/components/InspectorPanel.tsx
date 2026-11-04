import type { ProjectDef, WorldState } from "@hydrogen/bindings";
import { Button, InspectorRow } from "@hydrogen/ui-kit";
import type React from "react";

interface InspectorPanelProps {
  selectedNodeId: string | null;
  project: ProjectDef;
  ticksCount: number;
  simulatedWorld: WorldState | null;
  onSaveSlot: () => void;
  onLoadSlot: () => void;
  hasSavedSlot: boolean;
}

export function InspectorPanel({
  selectedNodeId,
  project,
  ticksCount,
  simulatedWorld,
  onSaveSlot,
  onLoadSlot,
  hasSavedSlot,
}: InspectorPanelProps): React.JSX.Element {
  return (
    <aside className="studio-inspector">
      <div className="inspector-section">
        <h3>Nœud sélectionné</h3>
        <div className="tabular" style={{ color: "#38bdf8", fontWeight: 600 }}>
          {selectedNodeId ?? "Aucun nœud sélectionné (cliquez sur un nœud)"}
        </div>
      </div>

      <div className="inspector-section">
        <h3>Métriques du projet</h3>
        <div style={{ display: "flex", flexDirection: "column" }}>
          <InspectorRow label="Entités" value={project.entities.length} />
          <InspectorRow label="Composants" value={project.components.length} />
          <InspectorRow label="Règles" value={project.rules.length} />
          <InspectorRow label="Cycles joués" value={ticksCount} />
        </div>
      </div>

      <div className="inspector-section">
        <h3>Persistance (SaveStore)</h3>
        <div style={{ display: "flex", gap: "8px", marginTop: "8px" }}>
          <Button variant="secondary" size="sm" onClick={onSaveSlot}>
            Sauvegarder slot
          </Button>
          <Button variant="ghost" size="sm" disabled={!hasSavedSlot} onClick={onLoadSlot}>
            Restaurer slot
          </Button>
        </div>
      </div>

      <div className="inspector-section">
        <h3>État du monde (WorldState)</h3>
        <pre className="tabular">{JSON.stringify(simulatedWorld ?? project.entities, null, 2)}</pre>
      </div>
    </aside>
  );
}
