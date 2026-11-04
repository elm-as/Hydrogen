import type React from "react";
import { THEME_TOKENS } from "../theme/tokens";

export type BadgeVariant = "neutral" | "success" | "warning" | "danger" | "accent";

export interface BadgeProps extends React.HTMLAttributes<HTMLSpanElement> {
  variant?: BadgeVariant;
}

function resolveBadgeStyles(variant: BadgeVariant): React.CSSProperties {
  switch (variant) {
    case "success":
      return {
        backgroundColor: "rgba(35, 134, 54, 0.2)",
        color: THEME_TOKENS.colors.successText,
        borderColor: "rgba(63, 185, 80, 0.4)",
      };
    case "warning":
      return {
        backgroundColor: "rgba(210, 153, 34, 0.2)",
        color: THEME_TOKENS.colors.warningText,
        borderColor: "rgba(240, 136, 62, 0.4)",
      };
    case "danger":
      return {
        backgroundColor: "rgba(218, 54, 51, 0.2)",
        color: THEME_TOKENS.colors.dangerText,
        borderColor: "rgba(248, 81, 73, 0.4)",
      };
    case "accent":
      return {
        backgroundColor: "rgba(56, 189, 248, 0.15)",
        color: THEME_TOKENS.colors.accent,
        borderColor: "rgba(56, 189, 248, 0.4)",
      };
    default:
      return {
        backgroundColor: "rgba(110, 118, 129, 0.15)",
        color: THEME_TOKENS.colors.textSecondary,
        borderColor: THEME_TOKENS.colors.border,
      };
  }
}

export function Badge({
  variant = "neutral",
  children,
  style,
  ...restProps
}: BadgeProps): React.JSX.Element {
  const badgeStyle: React.CSSProperties = {
    display: "inline-flex",
    alignItems: "center",
    padding: "2px 6px",
    borderRadius: THEME_TOKENS.radius.sm,
    fontSize: "11px",
    fontWeight: 600,
    fontFamily: THEME_TOKENS.typography.fontMono,
    fontVariantNumeric: THEME_TOKENS.typography.tabularNums,
    borderWidth: "1px",
    borderStyle: "solid",
    lineHeight: "14px",
    ...resolveBadgeStyles(variant),
    ...style,
  };

  return (
    <span style={badgeStyle} {...restProps}>
      {children}
    </span>
  );
}
