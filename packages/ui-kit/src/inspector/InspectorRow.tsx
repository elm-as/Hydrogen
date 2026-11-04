import type React from "react";
import { THEME_TOKENS } from "../theme/tokens";

export interface InspectorRowProps {
  label: string;
  value: React.ReactNode;
  description?: string;
  action?: React.ReactNode;
  isMonospace?: boolean;
}

export function InspectorRow({
  label,
  value,
  description,
  action,
  isMonospace = true,
}: InspectorRowProps): React.JSX.Element {
  return (
    <div
      style={{
        display: "flex",
        alignItems: "center",
        justifyContent: "space-between",
        padding: "6px 8px",
        borderBottom: `1px solid ${THEME_TOKENS.colors.border}`,
        fontSize: "12px",
        gap: "8px",
      }}
    >
      <div style={{ display: "flex", flexDirection: "column", gap: "2px" }}>
        <span style={{ color: THEME_TOKENS.colors.textSecondary, fontWeight: 500 }}>{label}</span>
        {description && (
          <span style={{ color: THEME_TOKENS.colors.textMuted, fontSize: "10px" }}>
            {description}
          </span>
        )}
      </div>

      <div style={{ display: "flex", alignItems: "center", gap: "6px" }}>
        <span
          style={{
            color: THEME_TOKENS.colors.textPrimary,
            fontFamily: isMonospace
              ? THEME_TOKENS.typography.fontMono
              : THEME_TOKENS.typography.fontSans,
            fontVariantNumeric: THEME_TOKENS.typography.tabularNums,
            fontWeight: 600,
          }}
        >
          {value}
        </span>
        {action}
      </div>
    </div>
  );
}
