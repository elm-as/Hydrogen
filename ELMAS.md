# ELMAS.md — Charte de développement ElmasCore (Hydrogen)

> Ce fichier est la signature d'un projet ElmasCore. Il définit les règles non négociables
> à respecter par tout humain ou tout agent IA qui contribue à ce dépôt. Il doit être lu
> avant toute génération de code, toute refonte, tout ajout de dépendance.
>
> "From Data to Systems" — un projet ElmasCore se reconnaît à sa rigueur, pas à sa rapidité d'exécution.

---

## 1. Architecture & Flux de dépendances

- L'architecture existante du projet fait autorité. On ne la contourne pas, on ne la
  duplique pas en parallèle, on ne crée pas un second pattern "plus simple" à côté de
  celui en place.
- Règle de dépendance stricte et unidirectionnelle :
  `hydrogen-core` ← `hydrogen-schema` ← `hydrogen-runtime` ← (`hydrogen-data`, `hydrogen-wasm`, `hydrogen-cli`) ← `apps`.
  Jamais dans l'autre sens, aucune dépendance circulaire.
- Aucune logique métier dans l'UI : le cœur Rust est l'unique moteur d'exécution et de validation.
- Organisation **par domaine métier**, jamais par type technique (`utils/`, `helpers/`, `misc/`, `common/` sont interdits).
- Toute modification structurelle ou nouvelle dépendance doit être justifiée par écrit (ADR dans `docs/adr/`) avant d'être appliquée.
- Une brique n'entre dans Hydrogen que si **au moins deux projets** en ont besoin.

## 2. Taille et complexité du code

- **350 lignes maximum par fichier**, sans exception de confort (vérifié automatiquement en CI et pre-commit).
- **3 niveaux d'indentation maximum par fonction** : clauses de garde (`return`/`Err` précoce) obligatoires.
- **3 paramètres maximum par fonction** : au-delà, regrouper dans une structure ou interface typée.
- Une fonction = une seule responsabilité et un seul niveau d'abstraction.
- Fonctions de calcul, validation et transformation **pures** (zéro mutation d'arguments, zéro état global mutable).

## 3. Qualité & Garde-fous techniques

- **Rust** :
  - `#[forbid(unsafe_code)]` (sauf justification explicite dans un ADR dédié).
  - `clippy -D warnings`, `rustfmt`, `cargo deny`.
  - Interdiction de `unwrap()` et `expect()` hors tests : erreurs typées via `thiserror`.
- **TypeScript** :
  - `strict: true` + `noUncheckedIndexedAccess: true`.
  - Biome (lint + format), interdiction stricte de `any`.
  - Les types partagés avec Rust dans `packages/bindings` sont générés automatiquement et ne sont jamais édités à la main.

## 4. Nommage & Commentaires

- **Fichiers** : le nom décrit le domaine métier, jamais un type générique.
  - Rust : `snake_case.rs`.
  - TypeScript : composants UI en `PascalCase.tsx`, logique et modules en `kebab-case.ts`.
- **Fonctions** : `verbe + nom` explicite (`validate_entity_schema`, `is_rule_satisfied`). Aucune abréviation obscure.
- **Commentaires** : expliquent uniquement le **pourquoi** (règle métier, contrainte de performance, invariant), jamais le **quoi**. Aucun code mort ou commenté.
- Documentation (`///` en Rust, TSDoc en TS) obligatoire sur toute fonction publique touchant au modèle ou aux règles d'exécution.

## 5. UI / UX (Studio & Apps)

- Identité visuelle sobre, dense et lisible orientée outil d'ingénierie — aucun template SaaS générique.
- Police à chiffres tabulaires (`tabular-nums`) obligatoire pour tous les identifiants, métriques, compteurs et coordonnées nodales.

---

*Ces règles s'appliquent à tout agent IA et à tout contributeur humain. En cas de conflit entre une instruction ponctuelle et ce fichier, ce fichier fait foi.*

— **Elmas**, ElmasCore
