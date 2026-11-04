import type { ProjectDef, WorldState } from "@hydrogen/bindings";
import initWasm, { WasmRuntime } from "@hydrogen/wasm-core";

const DEFAULT_PROJECT: ProjectDef = {
  schema_version: { major: 0, minor: 1, patch: 0 },
  project_id: "wasm_demo",
  display_name: "Hydrogen Web Playground",
  components: [
    {
      id: "counter",
      fields: {
        value: {
          kind: "int",
          default_value: { kind: "int", data: 0 },
        },
      },
    },
  ],
  entities: [
    {
      id: "player",
      components: {
        counter: {
          value: { kind: "int", data: 0 },
        },
      },
    },
  ],
  events: [
    {
      id: "step_reached",
      payload_fields: {},
    },
  ],
  rules: [
    {
      id: "increment_counter",
      trigger: { kind: "on_tick" },
      condition: null,
      actions: [
        {
          kind: "add_numeric",
          target: {
            entity_id: "player",
            component_id: "counter",
            field_id: "value",
          },
          delta: { kind: "int", data: 1 },
        },
      ],
    },
  ],
};

let runtime: WasmRuntime | null = null;
let currentWorld: WorldState | null = null;

async function bootstrap() {
  const root = document.getElementById("app");
  if (!root) {
    return;
  }

  root.innerHTML = renderContainerHtml();
  await initWasm();

  runtime = new WasmRuntime(JSON.stringify(DEFAULT_PROJECT));
  refreshUi();
  attachEventListeners();
}

function renderContainerHtml(): string {
  return `
    <main class="playground-container">
      <header class="header">
        <h1>Hydrogen Playground (WebAssembly)</h1>
        <p>Exécution déterministe d'un projet déclaratif dans le navigateur via Rust / WASM</p>
      </header>

      <section class="panel">
        <div class="controls">
          <button id="btn-tick">Exécuter 1 tick</button>
          <button id="btn-tick-10">Exécuter 10 ticks</button>
          <button id="btn-reset">Réinitialiser</button>
        </div>

        <div class="metrics">
          <div>Cycles (ticks) : <span id="metric-ticks" class="tabular metric-value">0</span></div>
          <div>Statut : <span id="metric-status" class="tabular metric-value">Prêt (WASM actif)</span></div>
        </div>
      </section>

      <section class="panel">
        <h2 style="font-size: 1rem; margin-bottom: 0.75rem;">État du monde (WorldState)</h2>
        <pre id="world-output" class="tabular">Chargement...</pre>
      </section>
    </main>
  `;
}

function attachEventListeners() {
  const btnTick = document.getElementById("btn-tick");
  const btnTick10 = document.getElementById("btn-tick-10");
  const btnReset = document.getElementById("btn-reset");

  btnTick?.addEventListener("click", () => runTicks(1));
  btnTick10?.addEventListener("click", () => runTicks(10));
  btnReset?.addEventListener("click", () => resetRuntime());
}

function runTicks(count: number) {
  if (!runtime) {
    return;
  }

  try {
    for (let i = 0; i < count; i += 1) {
      runtime.tick();
    }
    refreshUi();
  } catch (error) {
    updateStatusText(`Erreur : ${String(error)}`);
  }
}

function resetRuntime() {
  runtime = new WasmRuntime(JSON.stringify(DEFAULT_PROJECT));
  refreshUi();
}

function refreshUi() {
  if (!runtime) {
    return;
  }

  const ticksElem = document.getElementById("metric-ticks");
  const worldElem = document.getElementById("world-output");

  const tickCount = runtime.get_tick_count();
  const rawWorld = runtime.get_world_state_json();
  currentWorld = JSON.parse(rawWorld) as WorldState;

  if (ticksElem) {
    ticksElem.textContent = String(tickCount);
  }
  if (worldElem) {
    worldElem.textContent = JSON.stringify(currentWorld, null, 2);
  }
}

function updateStatusText(message: string) {
  const statusElem = document.getElementById("metric-status");
  if (statusElem) {
    statusElem.textContent = message;
  }
}

void bootstrap();
