import type { EntityState, ScalarValue, WorldState } from "@hydrogen/bindings";
import { Badge } from "@hydrogen/ui-kit";
import type React from "react";

interface WorldStateViewProps {
  world: WorldState | null;
  tickCount: number;
}

type ComponentMap = EntityState["components"];

export function WorldStateView({ world, tickCount }: WorldStateViewProps): React.JSX.Element {
  if (!world || Object.keys(world.entities).length === 0) {
    return (
      <div style={emptyContainerStyle}>
        <span style={{ color: "#8b949e" }}>Aucune entité présente dans le monde.</span>
      </div>
    );
  }

  const entityEntries = Object.entries(world.entities);

  return (
    <div style={containerStyle}>
      <div style={headerStyle}>
        <div>
          <h2 style={{ margin: 0, fontSize: "16px", color: "#f0f6fc" }}>État Actif du Monde</h2>
          <span style={{ fontSize: "12px", color: "#8b949e" }}>
            Cycle d'exécution :{" "}
            <strong style={{ color: "#58a6ff" }} className="tabular">
              {tickCount}
            </strong>
          </span>
        </div>
        <Badge variant="accent">{entityEntries.length} entité(s)</Badge>
      </div>

      <div style={gridStyle}>
        {entityEntries.map(([entityId, entityState]) => {
          if (!entityState) return null;
          return (
            <EntityCard key={entityId} entityId={entityId} components={entityState.components} />
          );
        })}
      </div>
    </div>
  );
}

interface EntityCardProps {
  entityId: string;
  components: ComponentMap;
}

function EntityCard({ entityId, components }: EntityCardProps): React.JSX.Element {
  const isAlive = extractBoolField(components, "stats", "is_alive");
  const healthCurrent = extractIntField(components, "health", "current");
  const healthMax = extractIntField(components, "health", "max");
  const hasHealthBar = healthCurrent !== null && healthMax !== null && healthMax > 0;

  return (
    <div style={cardStyle}>
      <div style={cardHeaderStyle}>
        <div style={{ display: "flex", alignItems: "center", gap: "8px" }}>
          <span style={entityTitleStyle} className="tabular">
            {entityId}
          </span>
          {isAlive !== null && (
            <Badge variant={isAlive ? "success" : "danger"}>{isAlive ? "En vie" : "K.O."}</Badge>
          )}
        </div>
        <span style={{ fontSize: "11px", color: "#8b949e" }}>
          {Object.keys(components).length} comp.
        </span>
      </div>

      {hasHealthBar && <HealthProgressBar current={healthCurrent} max={healthMax} />}

      <div style={componentsListStyle}>
        {Object.entries(components).map(([compId, fields]) => {
          if (!fields) return null;
          return (
            <div key={compId} style={compBlockStyle}>
              <span style={compNameStyle}>{compId}</span>
              <div style={fieldsGridStyle}>
                {Object.entries(fields).map(([fieldId, scalar]) => {
                  if (!scalar) return null;
                  return <FieldRow key={fieldId} fieldId={fieldId} value={scalar} />;
                })}
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
}

function HealthProgressBar({ current, max }: { current: number; max: number }): React.JSX.Element {
  const clamped = Math.max(0, Math.min(current, max));
  const pct = Math.round((clamped / max) * 100);
  const color = pct > 50 ? "#3fb950" : pct > 20 ? "#d29922" : "#f85149";

  return (
    <div style={{ marginBottom: "12px" }}>
      <div style={healthLabelRowStyle}>
        <span style={{ fontSize: "11px", color: "#8b949e" }}>Points de vie</span>
        <span style={{ fontSize: "12px", fontWeight: 600 }} className="tabular">
          {clamped} / {max} PV ({pct}%)
        </span>
      </div>
      <div style={progressTrackStyle}>
        <div style={{ ...progressFillStyle, width: `${pct}%`, backgroundColor: color }} />
      </div>
    </div>
  );
}

function FieldRow({ fieldId, value }: { fieldId: string; value: ScalarValue }): React.JSX.Element {
  return (
    <div style={fieldRowStyle}>
      <span style={fieldKeyStyle}>{fieldId}</span>
      <span style={fieldValStyle} className="tabular">
        {formatScalar(value)}
      </span>
    </div>
  );
}

function formatScalar(value: ScalarValue): string {
  switch (value.kind) {
    case "int":
      return String(value.data);
    case "float":
      return value.data.toFixed(2);
    case "bool":
      return value.data ? "true" : "false";
    case "text":
      return `"${value.data}"`;
  }
}

function extractBoolField(components: ComponentMap, comp: string, field: string): boolean | null {
  const val = components[comp]?.[field];
  if (!val || val.kind !== "bool") return null;
  return val.data;
}

function extractIntField(components: ComponentMap, comp: string, field: string): number | null {
  const val = components[comp]?.[field];
  if (!val || val.kind !== "int") return null;
  return val.data;
}

const containerStyle: React.CSSProperties = {
  display: "flex",
  flexDirection: "column",
  gap: "16px",
  padding: "20px",
  height: "100%",
  boxSizing: "border-box",
  overflowY: "auto",
};

const emptyContainerStyle: React.CSSProperties = {
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
  height: "100%",
};

const headerStyle: React.CSSProperties = {
  display: "flex",
  justifyContent: "space-between",
  alignItems: "center",
  borderBottom: "1px solid #21262d",
  paddingBottom: "12px",
};

const gridStyle: React.CSSProperties = {
  display: "grid",
  gridTemplateColumns: "repeat(auto-fill, minmax(280px, 1fr))",
  gap: "14px",
};

const cardStyle: React.CSSProperties = {
  background: "#161b22",
  border: "1px solid #30363d",
  borderRadius: "6px",
  padding: "14px",
  display: "flex",
  flexDirection: "column",
};

const cardHeaderStyle: React.CSSProperties = {
  display: "flex",
  justifyContent: "space-between",
  alignItems: "center",
  marginBottom: "10px",
};

const entityTitleStyle: React.CSSProperties = {
  fontWeight: 600,
  fontSize: "14px",
  color: "#f0f6fc",
};

const healthLabelRowStyle: React.CSSProperties = {
  display: "flex",
  justifyContent: "space-between",
  alignItems: "center",
  marginBottom: "4px",
};

const progressTrackStyle: React.CSSProperties = {
  height: "6px",
  backgroundColor: "#21262d",
  borderRadius: "3px",
  overflow: "hidden",
};

const progressFillStyle: React.CSSProperties = {
  height: "100%",
  transition: "width 0.2s ease, background-color 0.2s ease",
};

const componentsListStyle: React.CSSProperties = {
  display: "flex",
  flexDirection: "column",
  gap: "8px",
};

const compBlockStyle: React.CSSProperties = {
  backgroundColor: "#0d1117",
  border: "1px solid #21262d",
  borderRadius: "4px",
  padding: "8px",
};

const compNameStyle: React.CSSProperties = {
  fontSize: "11px",
  fontWeight: 600,
  color: "#79c0ff",
  display: "block",
  marginBottom: "6px",
  textTransform: "uppercase",
  letterSpacing: "0.5px",
};

const fieldsGridStyle: React.CSSProperties = {
  display: "flex",
  flexDirection: "column",
  gap: "4px",
};

const fieldRowStyle: React.CSSProperties = {
  display: "flex",
  justifyContent: "space-between",
  alignItems: "center",
  fontSize: "12px",
};

const fieldKeyStyle: React.CSSProperties = {
  color: "#8b949e",
};

const fieldValStyle: React.CSSProperties = {
  color: "#f0f6fc",
  fontWeight: 500,
};
