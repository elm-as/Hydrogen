import type { EventDef } from "@hydrogen/bindings";
import { Badge, Button } from "@hydrogen/ui-kit";
import type React from "react";

interface PlayerActionsProps {
  events: EventDef[];
  onTriggerPlayerEvent: (eventId: string) => void;
  onTick: () => void;
  onReset: () => void;
}

export function PlayerActions({
  events,
  onTriggerPlayerEvent,
  onTick,
  onReset,
}: PlayerActionsProps): React.JSX.Element {
  const playerEvents = events.filter((e) => e.source === "player");

  return (
    <div style={containerStyle}>
      <div style={sectionHeaderStyle}>
        <span style={titleStyle}>Commandes & Actions Joueur</span>
        <Badge variant="accent">{playerEvents.length} action(s)</Badge>
      </div>

      <div style={actionsRowStyle}>
        <Button variant="primary" onClick={onTick}>
          Avancer d'un Cycle (Tick)
        </Button>

        {playerEvents.map((evt) => (
          <Button key={evt.id} variant="secondary" onClick={() => onTriggerPlayerEvent(evt.id)}>
            {formatPlayerActionLabel(evt.id)}
          </Button>
        ))}

        <Button variant="ghost" onClick={onReset}>
          Réinitialiser
        </Button>
      </div>
    </div>
  );
}

function formatPlayerActionLabel(eventId: string): string {
  switch (eventId) {
    case "player_attack":
      return "Attaquer (Joueur)";
    case "player_drink_potion":
      return "Boire une Potion";
    case "player_increment":
      return "Incrémenter (+10 Joueur)";
    default: {
      const clean = eventId.replace(/^player_/, "").replace(/_/g, " ");
      return clean.charAt(0).toUpperCase() + clean.slice(1);
    }
  }
}

const containerStyle: React.CSSProperties = {
  backgroundColor: "#161b22",
  border: "1px solid #30363d",
  borderRadius: "6px",
  padding: "12px 16px",
  display: "flex",
  flexDirection: "column",
  gap: "10px",
};

const sectionHeaderStyle: React.CSSProperties = {
  display: "flex",
  justifyContent: "space-between",
  alignItems: "center",
};

const titleStyle: React.CSSProperties = {
  fontSize: "12px",
  fontWeight: 600,
  color: "#8b949e",
  textTransform: "uppercase",
  letterSpacing: "0.5px",
};

const actionsRowStyle: React.CSSProperties = {
  display: "flex",
  flexWrap: "wrap",
  gap: "8px",
  alignItems: "center",
};
