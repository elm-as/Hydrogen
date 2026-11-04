# ADR 0006 — Architecture de Hydrogen Studio (Tauri v2 + React + @xyflow/react)

- **Statut** : Accepté
- **Date** : 2026-11-04

## Contexte

L'étape 6 introduit **Hydrogen Studio**, l'éditeur visuel de référence permettant de concevoir, manipuler et tester des projets déclaratifs Hydrogen sans écrire de code.
L'éditeur doit satisfaire les critères stricts d'`ELMAS.md` :
1. Aucune logique métier ni de conversion dans l'UI : toute la logique de graphe doit être pure et isolée dans `packages/editor-core`.
2. Communication IPC typée et sécurisée entre le frontend et le backend desktop via Tauri v2.
3. Rendu fluide et interactif du graphe nodal propulsé par `@xyflow/react` et React 19.

## Décisions

1. **Isolation de la logique du graphe dans `packages/editor-core`** :
   - `packages/editor-core` est une bibliothèque TypeScript pure, sans dépendance React ni DOM.
   - Elle fournit le convertisseur bidirectionnel canonique :
     - `projectDefToGraph(project: ProjectDef): NodalGraph`
     - `graphToProjectDef(graph: NodalGraph, baseProject: ProjectDef): ProjectDef`
   - Le graphe modélise les entités, les composants, les événements et les règles sous forme de nœuds typés et d'arêtes sémantiques.
2. **Couche desktop : Tauri v2 (`apps/studio/src-tauri`)** :
   - Crate natif intégrant les commandes IPC :
     - `load_project(path: String) -> Result<ProjectDef, String>`
     - `save_project(path: String, project: ProjectDef) -> Result<(), String>`
     - `validate_project(project: ProjectDef) -> Result<(), String>`
     - `run_project_ticks(project: ProjectDef, ticks: u64) -> Result<StudioRunResult, String>`
   - Le backend natif réutilise directement `hydrogen-schema` et `hydrogen-runtime` sans duplication.
3. **Frontend Studio avec React 19 et `@xyflow/react`** :
   - Canvas nodal affichant les nœuds de composants, entités, déclencheurs et règles.
   - Inspecteur latéral de sélection affichant les propriétés en police tabulaire (`tabular-nums`).
   - Barre d'action avec validation en direct et simulation pas à pas.

## Conséquences

- Séparation stricte des responsabilités : Studio est une coquille de présentation et d'orchestration au-dessus de `editor-core` et du moteur natif Rust.
- La bibliothèque `editor-core` est réutilisable pour d'autres environnements (WASM web, CLI ou plugins futurs).

