# ADR 0005 — Architecture du module WebAssembly (`hydrogen-wasm`) et du Playground Web

- **Statut** : Accepté
- **Date** : 2026-11-04

## Contexte

L'étape 5 a pour objectif d'exécuter les définitions déclaratives Hydrogen directement dans le navigateur sans serveur intermédiaire, en compilant le moteur en WebAssembly.
Le module WebAssembly doit :
1. Être une passerelle pure et légère au-dessus de `hydrogen-core`, `hydrogen-schema` et `hydrogen-runtime`.
2. Assurer des échanges mémoire sûrs entre l'environnement JavaScript et le runtime Rust sans risque de fuite de mémoire ou de désynchronisation de pointeurs.
3. Alimenter une application web autonome (`apps/playground`) développée en TypeScript strict avec Vite.

## Décisions

1. **Passerelle WASM basée sur des chaînes JSON typées via `wasm-bindgen`** :
   - Plutôt que d'exposer des structs complexes ou des pointeurs opaques non typés côté JS, `WasmRuntime` expose une API déterministe par chaînes JSON :
     - `new(project_json: &str) -> Result<WasmRuntime, JsValue>`
     - `tick() -> Result<String, JsValue>` (rapport métrique JSON)
     - `publish_event(event_json: &str) -> Result<(), JsValue>`
     - `get_world_state_json() -> Result<String, JsValue>`
     - `get_tick_count() -> u64`
   - Les types TypeScript de ces échanges sont directement garantis par le package `@hydrogen/bindings` (`ProjectDef`, `RuntimeEvent`, `WorldState`, etc.).
2. **Profil de compilation `wasm-release` ultra-optimisé** :
   - Hérite du profil `release` avec `opt-level = "z"`, `lto = true`, `panic = "abort"`, `strip = true` et `codegen-units = 1`.
3. **Application web `apps/playground` avec Vite et TypeScript strict** :
   - Interface minimaliste sans fioriture : bouton pour déclencher les ticks, formulaire d'émission d'événements, inspecteur d'état en direct avec police tabulaire (`tabular-nums`) respectant `ELMAS.md`.

## Conséquences

- Le même fichier de définition (`project.json`) s'exécute à l'identique dans la CLI native et dans le navigateur, garantissant une cohérence absolue entre les plateformes.
- L'interface web ne contient aucune logique métier : elle délègue 100 % de l'évaluation au module WebAssembly.

