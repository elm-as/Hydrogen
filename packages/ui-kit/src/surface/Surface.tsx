import type React from "react";
import { THEME_TOKENS } from "../theme/tokens";

export interface SurfaceProps {
  title?: string;
  headerAction?: React.ReactNode;
  children: React.ReactNode;
  footer?: React.ReactNode;
  style?: React.CSSProperties;
}

export function Surface({
  title,
  headerAction,
  children,
  footer,
  style,
}: SurfaceProps): React.JSX.Element {
  return (
    <section
      style={{
        backgroundColor: THEME_TOKENS.colors.surface,
        border: `1px solid ${THEME_TOKENS.colors.border}`,
        borderRadius: THEME_TOKENS.radius.md,
        display: "flex",
        flexDirection: "column",
        overflow: "hidden",
        ...style,
      }}
    >
      {(title || headerAction) && (
        <header
          style={{
            display: "flex",
            alignItems: "center",
            justifyContent: "space-between",
            padding: "8px 12px",
            borderBottom: `1px solid ${THEME_TOKENS.colors.border}`,
            backgroundColor: THEME_TOKENS.colors.surfaceElevated,
          }}
        >
          {title && (
            <h3
              style={{
                margin: 0,
                fontSize: "12px",
                fontWeight: 600,
                color: THEME_TOKENS.colors.textPrimary,
                fontFamily: THEME_TOKENS.typography.fontSans,
                textTransform: "uppercase",
                letterSpacing: "0.05em",
              }}
            >
              {title}
            </h3>
          )}
          {headerAction}
        </header>
      )}

      <div style={{ padding: "8px 12px", flex: 1 }}>{children}</div>

      {footer && (
        <footer
          style={{
            padding: "6px 12px",
            borderTop: `1px solid ${THEME_TOKENS.colors.border}`,
            backgroundColor: THEME_TOKENS.colors.surfaceElevated,
            fontSize: "11px",
            color: THEME_TOKENS.colors.textMuted,
          }}
        >
          {footer}
        </footer>
      )}
    </section>
  );
}
