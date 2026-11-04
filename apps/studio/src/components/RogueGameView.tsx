import type { WorldState } from "@hydrogen/bindings";
import { Badge, Button } from "@hydrogen/ui-kit";
import type React from "react";

interface RogueGameViewProps {
  world: WorldState | null;
  tickCount: number;
  onTick: () => void;
  onReset: () => void;
}

function extractInt(world: WorldState | null, entity: string, comp: string, field: string): number {
  const val = world?.entities[entity]?.components[comp]?.[field];
  return val?.kind === "int" ? val.data : 0;
}

function extractBool(
  world: WorldState | null,
  entity: string,
  comp: string,
  field: string,
): boolean {
  const val = world?.entities[entity]?.components[comp]?.[field];
  return val?.kind === "bool" ? val.data : false;
}

export function RogueGameView({
  world,
  tickCount,
  onTick,
  onReset,
}: RogueGameViewProps): React.JSX.Element {
  const heroHp = extractInt(world, "hero", "health", "current");
  const heroMaxHp = Math.max(1, extractInt(world, "hero", "health", "max"));
  const heroGold = extractInt(world, "hero", "inventory", "gold");
  const heroPotions = extractInt(world, "hero", "inventory", "potions");

  const goblinHp = Math.max(0, extractInt(world, "goblin", "health", "current"));
  const goblinMaxHp = Math.max(1, extractInt(world, "goblin", "health", "max"));
  const isGoblinAlive = extractBool(world, "goblin", "stats", "is_alive");

  const turn = extractInt(world, "game_state", "dungeon", "turn");
  const floor = extractInt(world, "game_state", "dungeon", "floor");

  const goblinPct = Math.round((goblinHp / goblinMaxHp) * 100);
  const heroPct = Math.round((heroHp / heroMaxHp) * 100);

  return (
    <div style={containerStyle}>
      <div style={headerCardStyle}>
        <div>
          <h2 style={{ margin: "0 0 4px 0", fontSize: "16px" }}>Donjon — Étage {floor}</h2>
          <span style={{ color: "#8b949e", fontSize: "12px" }}>
            Tour :{" "}
            <strong style={{ color: "#f0f6fc" }} className="tabular">
              {turn}
            </strong>{" "}
            | Cycles moteur :{" "}
            <strong style={{ color: "#f0f6fc" }} className="tabular">
              {tickCount}
            </strong>
          </span>
        </div>
        <div style={{ display: "flex", gap: "8px" }}>
          <Button variant="primary" onClick={onTick}>
            Jouer 1 Tour (Attaquer)
          </Button>
          <Button variant="secondary" onClick={onReset}>
            Recommencer
          </Button>
        </div>
      </div>

      <div style={arenaGridStyle}>
        {/* Carte Héros */}
        <div style={cardStyle}>
          <div style={cardHeaderStyle}>
            <span style={{ fontWeight: 600 }}>Héros du Donjon</span>
            <Badge variant="accent">Niveau 1</Badge>
          </div>

          <div style={{ marginBottom: "12px" }}>
            <div style={statRowStyle}>
              <span>Points de vie</span>
              <span className="tabular">
                {heroHp} / {heroMaxHp} PV
              </span>
            </div>
            <div style={barBackgroundStyle}>
              <div style={{ ...barFillStyle, width: `${heroPct}%`, backgroundColor: "#38bdf8" }} />
            </div>
          </div>

          <div style={statGridStyle}>
            <div style={statBoxStyle}>
              <span style={statLabelStyle}>Attaque</span>
              <span style={statValStyle}>15</span>
            </div>
            <div style={statBoxStyle}>
              <span style={statLabelStyle}>Défense</span>
              <span style={statValStyle}>4</span>
            </div>
            <div style={statBoxStyle}>
              <span style={statLabelStyle}>Or accumulé</span>
              <span style={{ ...statValStyle, color: "#fbbf24" }}>{heroGold} po</span>
            </div>
            <div style={statBoxStyle}>
              <span style={statLabelStyle}>Potions</span>
              <span style={statValStyle}>{heroPotions}</span>
            </div>
          </div>
        </div>

        {/* Carte Monstre */}
        <div style={{ ...cardStyle, borderColor: isGoblinAlive ? "#f87171" : "#30363d" }}>
          <div style={cardHeaderStyle}>
            <span style={{ fontWeight: 600 }}>Gobelin des Profondeurs</span>
            <Badge variant={isGoblinAlive ? "danger" : "neutral"}>
              {isGoblinAlive ? "EN VIE" : "VAINCU"}
            </Badge>
          </div>

          <div style={{ marginBottom: "12px" }}>
            <div style={statRowStyle}>
              <span>Points de vie</span>
              <span className="tabular">
                {goblinHp} / {goblinMaxHp} PV
              </span>
            </div>
            <div style={barBackgroundStyle}>
              <div
                style={{
                  ...barFillStyle,
                  width: `${goblinPct}%`,
                  backgroundColor: isGoblinAlive ? "#f87171" : "#6e7681",
                }}
              />
            </div>
          </div>

          <div style={statGridStyle}>
            <div style={statBoxStyle}>
              <span style={statLabelStyle}>Attaque</span>
              <span style={statValStyle}>6</span>
            </div>
            <div style={statBoxStyle}>
              <span style={statLabelStyle}>Défense</span>
              <span style={statValStyle}>1</span>
            </div>
            <div style={statBoxStyle}>
              <span style={statLabelStyle}>Récompense</span>
              <span style={{ ...statValStyle, color: "#fbbf24" }}>+50 or</span>
            </div>
            <div style={statBoxStyle}>
              <span style={statLabelStyle}>État</span>
              <span style={{ ...statValStyle, color: isGoblinAlive ? "#f87171" : "#3fb950" }}>
                {isGoblinAlive ? "Hostile" : "K.O."}
              </span>
            </div>
          </div>
        </div>
      </div>

      <div style={logContainerStyle}>
        <div style={{ fontWeight: 600, fontSize: "12px", color: "#8b949e", marginBottom: "4px" }}>
          Journal de combat (évalué de manière transactionnelle par le moteur Rust) :
        </div>
        <div style={{ fontSize: "12px", color: "#f0f6fc", lineHeight: "1.6" }}>
          {turn === 0 &&
            "Le combat n'a pas encore commencé. Cliquez sur 'Jouer 1 Tour' pour lancer la première attaque."}
          {turn === 1 &&
            "Tour 1 : Le héros frappe le gobelin et lui inflige 15 points de dégâts (30 -> 15 PV)."}
          {turn === 2 &&
            "Tour 2 : Coup fatal ! Le gobelin subit 15 dégâts et tombe à 0 PV. Événement 'goblin_defeated' émis -> règle de récompense déclenchée (+50 pièces d'or au héros) !"}
          {turn >= 3 &&
            `Tour ${turn} : Le gobelin est déjà vaincu. La condition is_alive est fausse, le tour avance sans dégâts supplémentaires.`}
        </div>
      </div>
    </div>
  );
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

const headerCardStyle: React.CSSProperties = {
  backgroundColor: "#161b22",
  border: "1px solid #30363d",
  borderRadius: "6px",
  padding: "16px",
  display: "flex",
  alignItems: "center",
  justifyContent: "space-between",
};

const arenaGridStyle: React.CSSProperties = {
  display: "grid",
  gridTemplateColumns: "1fr 1fr",
  gap: "16px",
};

const cardStyle: React.CSSProperties = {
  backgroundColor: "#161b22",
  border: "1px solid #30363d",
  borderRadius: "6px",
  padding: "16px",
  display: "flex",
  flexDirection: "column",
};

const cardHeaderStyle: React.CSSProperties = {
  display: "flex",
  alignItems: "center",
  justifyContent: "space-between",
  marginBottom: "12px",
  fontSize: "14px",
};

const statRowStyle: React.CSSProperties = {
  display: "flex",
  justifyContent: "space-between",
  fontSize: "11px",
  color: "#8b949e",
  marginBottom: "4px",
};

const barBackgroundStyle: React.CSSProperties = {
  width: "100%",
  height: "8px",
  backgroundColor: "#21262d",
  borderRadius: "4px",
  overflow: "hidden",
};

const barFillStyle: React.CSSProperties = {
  height: "100%",
  transition: "width 0.25s ease, background-color 0.25s ease",
};

const statGridStyle: React.CSSProperties = {
  display: "grid",
  gridTemplateColumns: "1fr 1fr",
  gap: "8px",
};

const statBoxStyle: React.CSSProperties = {
  backgroundColor: "#0d1117",
  border: "1px solid #21262d",
  borderRadius: "4px",
  padding: "8px",
  display: "flex",
  flexDirection: "column",
  gap: "2px",
};

const statLabelStyle: React.CSSProperties = {
  fontSize: "10px",
  color: "#8b949e",
  textTransform: "uppercase",
};

const statValStyle: React.CSSProperties = {
  fontSize: "13px",
  fontWeight: 600,
  fontVariantNumeric: "tabular-nums",
};

const logContainerStyle: React.CSSProperties = {
  backgroundColor: "#161b22",
  border: "1px solid #30363d",
  borderRadius: "6px",
  padding: "14px",
};
