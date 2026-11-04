// Tokens de design sobres et denses conformes à la charte ElmasCore.

export const THEME_TOKENS = {
  colors: {
    background: "#0d1117",
    surface: "#161b22",
    surfaceElevated: "#21262d",
    border: "#30363d",
    borderActive: "#58a6ff",
    textPrimary: "#f0f6fc",
    textSecondary: "#8b949e",
    textMuted: "#6e7681",
    accent: "#38bdf8",
    success: "#238636",
    successText: "#3fb950",
    warning: "#d29922",
    warningText: "#f0883e",
    danger: "#da3633",
    dangerText: "#f85149",
  },
  typography: {
    fontMono: 'ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", monospace',
    fontSans: '-apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif',
    tabularNums: "tabular-nums",
  },
  radius: {
    sm: "3px",
    md: "4px",
  },
} as const;

export type ThemeTokens = typeof THEME_TOKENS;
