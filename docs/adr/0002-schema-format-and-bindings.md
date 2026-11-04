# ADR 0002 — Format de définition (JSON) et génération des types TypeScript (`ts-rs`)

- **Statut** : Accepté
- **Date** : 2026-11-04

## Contexte

L'étape 2 introduit le modèle fondamental (`hydrogen-core`) et le schéma déclaratif (`hydrogen-schema`) qui décrivent tout projet Hydrogen (entités, composants, événements, règles). Conformément à `ELMAS.md`, toute nouvelle dépendance doit être justifiée par écrit et une seule source de vérité doit alimenter à la fois le moteur Rust et l'éditeur TypeScript (`packages/bindings`).

## Décisions

1. **Format de définition canonique : JSON (`serde` + `serde_json`)** :
   - Le JSON est nativement supporté par le navigateur (WASM), par l'IPC de Tauri v2 et par l'éditeur nodal React (`@xyflow/react`), évitant toute couche de transcodage intermédiaire entre l'UI et le cœur Rust.
   - Le champ `schema_version` (format `MAJOR.MINOR.PATCH`) est obligatoire à la racine de toute définition pour garantir la compatibilité ascendante et le rejet explicite des versions non supportées.
2. **Génération automatique des types TypeScript via `ts-rs`** :
   - Les structures de `hydrogen-core` et `hydrogen-schema` dérivent `ts_rs::TS`.
   - Un test dédié dans `hydrogen-schema` génère les fichiers `.ts` dans `packages/bindings/src/generated/`, garantissant à chaque exécution de la suite de tests que les bindings TypeScript sont synchronisés avec le code Rust.
3. **Justification des dépendances ajoutées** :
   - `thiserror` (`hydrogen-core`, `hydrogen-schema`) : dérivation standard `std::error::Error` sans `unwrap()` ni boilerplate manuel.
   - `serde` & `serde_json` (`hydrogen-core`, `hydrogen-schema`) : sérialisation/désérialisation déterministe des définitions.
   - `ts-rs` (`hydrogen-core`, `hydrogen-schema`) : génération sans runtime des types TypeScript depuis les structures Rust.
   - `proptest` (`dev-dependencies` de `hydrogen-schema`) : tests property-based pour vérifier le roundtrip JSON et les invariants du validateur face à des entrées arbitraires.

## Conséquences

- Les collections sérialisées dans les définitions et l'état utilisent `BTreeMap` afin de garantir un ordre de clés déterministe (indispensable pour des diffs Git propres et des tests reproductibles).
- Aucune édition manuelle n'est autorisée dans `packages/bindings/src/generated/`.

