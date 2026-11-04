# ADR 0001 — Choix de la stack initiale, de l'orchestrateur et du framework d'édition

- **Statut** : Accepté
- **Date** : 2026-11-04

## Contexte

Hydrogen est un moteur déclaratif conçu pour propulser à la fois des jeux, des applications métiers et des outils logiciels à partir de définitions communes. Il nécessite :
1. Un cœur déterministe sans I/O capable de tourner en natif (Desktop/CLI) et dans le navigateur (WASM) sans duplication de logique.
2. Un éditeur visuel nodal (**Hydrogen Studio**) fluide sur des graphes complexes.
3. Un monorepo hybride (Rust + TypeScript) gouverné par des règles de qualité strictes (`ELMAS.md`).

## Décisions

1. **Cœur en Rust (Workspace Cargo, édition 2021)** :
   - Garantit la sûreté mémoire sans ramasse-miettes, des performances prévisibles pour une boucle d'exécution temps réel, et une compilation directe vers WebAssembly (`wasm32-unknown-unknown`) ainsi qu'une intégration native avec Tauri v2.
2. **Orchestration par `just` (`justfile`) + `pnpm` workspaces** :
   - `just` fournit un point d'entrée déclaratif unique, lisible et sans surcharge pour coordonner les commandes Cargo (`clippy`, `nextest`, `deny`) et pnpm (`biome`, `tsc`, `vitest`).
3. **UI de l'éditeur avec React 19 + TypeScript strict + `@xyflow/react`** :
   - L'éditeur nodal constitue la surface critique de `apps/studio`. `@xyflow/react` est l'implémentation de référence de la suite xyflow (support mature des sous-graphes, virtualisation des nœuds, sélection multiple et personnalisation fine des arêtes).
   - Conformément à `ELMAS.md`, toute la logique de graphe pure sera isolée dans `packages/editor-core` et toute la logique d'exécution/validation reste dans les crates Rust.
4. **Qualité & Formatage unifiés par Biome (TS) et Clippy/Rustfmt/Cargo-Deny (Rust)** :
   - Biome remplace ESLint + Prettier par un binaire Rust unique ultra-rapide, configuré pour interdire `any` et imposer le typage strict.

## Conséquences

- Zéro logique métier autorisée dans les composants React : l'UI consomme uniquement les types générés dans `packages/bindings` et invoque le cœur Rust (via IPC Tauri ou WASM).
- Tout fichier du dépôt (Rust ou TypeScript) est soumis à la barrière bloquante de 350 lignes maximum en CI.
