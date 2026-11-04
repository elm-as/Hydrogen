import type React from "react";
import { THEME_TOKENS } from "../theme/tokens";

export type ButtonVariant = "primary" | "secondary" | "danger" | "ghost";
export type ButtonSize = "sm" | "md";

export interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: ButtonVariant;
  size?: ButtonSize;
  isLoading?: boolean;
}

function resolveVariantStyles(variant: ButtonVariant): React.CSSProperties {
  switch (variant) {
    case "primary":
      return {
        backgroundColor: THEME_TOKENS.colors.accent,
        color: "#051626",
        border: `1px solid ${THEME_TOKENS.colors.accent}`,
      };
    case "danger":
      return {
        backgroundColor: THEME_TOKENS.colors.danger,
        color: "#ffffff",
        border: `1px solid ${THEME_TOKENS.colors.danger}`,
      };
    case "ghost":
      return {
        backgroundColor: "transparent",
        color: THEME_TOKENS.colors.textSecondary,
        border: "1px solid transparent",
      };
    default:
      return {
        backgroundColor: THEME_TOKENS.colors.surfaceElevated,
        color: THEME_TOKENS.colors.textPrimary,
        border: `1px solid ${THEME_TOKENS.colors.border}`,
      };
  }
}

export function Button({
  variant = "secondary",
  size = "md",
  isLoading = false,
  disabled,
  children,
  style,
  ...restProps
}: ButtonProps): React.JSX.Element {
  const isActionDisabled = disabled || isLoading;
  const padding = size === "sm" ? "4px 8px" : "6px 12px";
  const fontSize = size === "sm" ? "11px" : "12px";

  const baseStyle: React.CSSProperties = {
    display: "inline-flex",
    alignItems: "center",
    justifyContent: "center",
    gap: "6px",
    fontFamily: THEME_TOKENS.typography.fontSans,
    fontSize,
    fontWeight: 600,
    fontVariantNumeric: THEME_TOKENS.typography.tabularNums,
    borderRadius: THEME_TOKENS.radius.md,
    cursor: isActionDisabled ? "not-allowed" : "pointer",
    opacity: isActionDisabled ? 0.5 : 1,
    padding,
    transition: "background-color 0.15s ease, border-color 0.15s ease",
    outline: "none",
    ...resolveVariantStyles(variant),
    ...style,
  };

  return (
    <button type="button" disabled={isActionDisabled} style={baseStyle} {...restProps}>
      {isLoading ? "..." : children}
    </button>
  );
}
