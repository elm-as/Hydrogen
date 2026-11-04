import { Badge, Button } from "@hydrogen/ui-kit";
import type React from "react";

export interface LogEntry {
  id: string;
  tick: number;
  message: string;
  type: "tick" | "event" | "action";
}

interface EventLogProps {
  logs: LogEntry[];
  onClear: () => void;
}

export function EventLog({ logs, onClear }: EventLogProps): React.JSX.Element {
  return (
    <div style={containerStyle}>
      <div style={headerStyle}>
        <div style={{ display: "flex", alignItems: "center", gap: "8px" }}>
          <span style={titleStyle}>Journal d'Exécution Moteur</span>
          <Badge variant="accent">{logs.length} entrée(s)</Badge>
        </div>
        {logs.length > 0 && (
          <Button variant="ghost" size="sm" onClick={onClear}>
            Effacer
          </Button>
        )}
      </div>

      <div style={logListStyle}>
        {logs.length === 0 ? (
          <div style={emptyStyle}>
            <span>En attente du premier cycle moteur ou d'une action joueur...</span>
          </div>
        ) : (
          logs.map((entry) => <LogItem key={entry.id} entry={entry} />)
        )}
      </div>
    </div>
  );
}

function LogItem({ entry }: { entry: LogEntry }): React.JSX.Element {
  return (
    <div style={itemStyle}>
      <span style={tickBadgeStyle} className="tabular">
        T{entry.tick}
      </span>
      <span style={getTypeStyle(entry.type)}>[{entry.type}]</span>
      <span style={messageStyle}>{entry.message}</span>
    </div>
  );
}

function getTypeStyle(type: LogEntry["type"]): React.CSSProperties {
  switch (type) {
    case "event":
      return { color: "#e3b341", fontWeight: 600, fontSize: "11px" };
    case "action":
      return { color: "#38bdf8", fontWeight: 600, fontSize: "11px" };
    case "tick":
      return { color: "#56d364", fontWeight: 600, fontSize: "11px" };
  }
}

const containerStyle: React.CSSProperties = {
  backgroundColor: "#0d1117",
  border: "1px solid #30363d",
  borderRadius: "6px",
  display: "flex",
  flexDirection: "column",
  height: "180px",
  flexShrink: 0,
};

const headerStyle: React.CSSProperties = {
  display: "flex",
  justifyContent: "space-between",
  alignItems: "center",
  padding: "8px 12px",
  borderBottom: "1px solid #21262d",
};

const titleStyle: React.CSSProperties = {
  fontSize: "12px",
  fontWeight: 600,
  color: "#8b949e",
  textTransform: "uppercase",
  letterSpacing: "0.5px",
};

const logListStyle: React.CSSProperties = {
  padding: "8px 12px",
  overflowY: "auto",
  display: "flex",
  flexDirection: "column",
  gap: "4px",
  flex: 1,
  fontFamily: "ui-monospace, SFMono-Regular, Menlo, monospace",
  fontSize: "12px",
};

const emptyStyle: React.CSSProperties = {
  color: "#6e7681",
  fontStyle: "italic",
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
  height: "100%",
};

const itemStyle: React.CSSProperties = {
  display: "flex",
  alignItems: "baseline",
  gap: "8px",
  lineHeight: "1.4",
};

const tickBadgeStyle: React.CSSProperties = {
  color: "#8b949e",
  fontSize: "11px",
  minWidth: "28px",
};

const messageStyle: React.CSSProperties = {
  color: "#f0f6fc",
};
