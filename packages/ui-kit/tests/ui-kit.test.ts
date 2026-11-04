import React from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it } from "vitest";

import { Badge, Button, InspectorRow, Surface, THEME_TOKENS } from "../src/index.js";

describe("@hydrogen/ui-kit", () => {
  it("exposes theme tokens with tabular-nums typography and low-saturation palette", () => {
    expect(THEME_TOKENS.typography.tabularNums).toBe("tabular-nums");
    expect(THEME_TOKENS.colors.background).toBe("#0d1117");
    expect(THEME_TOKENS.colors.border).toBe("#30363d");
  });

  it("renders Button with proper variant styles and attributes", () => {
    const html = renderToStaticMarkup(
      React.createElement(Button, { variant: "primary", size: "sm" }, "Lancer"),
    );
    expect(html).toContain("button");
    expect(html).toContain("Lancer");
    expect(html).toContain("tabular-nums");
  });

  it("renders Button in disabled state when isLoading is true", () => {
    const html = renderToStaticMarkup(React.createElement(Button, { isLoading: true }, "Valider"));
    expect(html).toContain("disabled");
    expect(html).toContain("...");
  });

  it("renders Badge with tabular numbers and variant classes", () => {
    const html = renderToStaticMarkup(
      React.createElement(Badge, { variant: "success" }, "TICK #42"),
    );
    expect(html).toContain("span");
    expect(html).toContain("TICK #42");
    expect(html).toContain("tabular-nums");
  });

  it("renders InspectorRow displaying label, tabular value, and description", () => {
    const html = renderToStaticMarkup(
      React.createElement(InspectorRow, {
        label: "Entités",
        value: 128,
        description: "Nombre d'entités actives",
      }),
    );
    expect(html).toContain("Entités");
    expect(html).toContain("128");
    expect(html).toContain("entités actives");
  });

  it("renders Surface container with header title and children content", () => {
    const html = renderToStaticMarkup(
      React.createElement(
        Surface,
        { title: "Panneau de contrôle", footer: "v0.1.0" },
        React.createElement("p", null, "Contenu de test"),
      ),
    );
    expect(html).toContain("Panneau de contrôle");
    expect(html).toContain("Contenu de test");
    expect(html).toContain("v0.1.0");
  });
});
