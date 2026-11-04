# ADR 0009 — Expressions déclaratives de calcul, événements joueur et architecture Studio data-driven

- **Statut** : Accepté
- **Date** : 2026-11-04

## Contexte

Dans la version initiale (v0.1.0), le projet pilote `rogue-core` appliquait des dégâts arbitraires en dur (`delta: -15`) sans calculer la différence entre l'attaque de l'attaquant et la défense de la cible. De plus, Hydrogen Studio présentait une dette technique importante :
1. Une vue dédiée codée en dur (`RogueGameView.tsx`) avec des statistiques hardcodées dans le JSX.
2. Une duplication des fichiers modèles dans `apps/studio/src/templates/projects.ts`.
3. Une fausse restauration d'état (rejeu de ticks en boucle au lieu d'une injection d'état).
4. Aucune distinction entre événements de cascade interne et actions interactives déclenchées par le joueur.

## Décisions

1. **Arbre d'expressions déclaratives (`ValueExpr`)** :
   - Schéma v0.2.0 introduisant `ValueExpr` : littéraux scalaires, adresses de champs (`FieldAddress`) et opérations binaires (`BinaryOperator` : `add`, `sub`, `mul`, `min`, `max`).
   - Désérialisation `#[serde(untagged)]` garantissant une compatibilité descendante totale avec les scalaires v0.1.0.
   - Contrôle statique de type pur (`ensure_expr_kind`) interdisant les mélanges implicites Int/Float et bornant la récursion à `MAX_EXPRESSION_DEPTH = 16`.
   - Évaluation pure dans le runtime (`evaluate_expr`) avec arithmétique entière protégée (`checked_add`, `checked_sub`, `checked_mul`) et validation de finitude des flottants.

2. **Événements Joueur et Déclenchement Externe (`EventSource`)** :
   - Ajout de `source: EventSource` (`internal` par défaut, ou `player`) sur `EventDef`.
   - Exposition de `publish_event_by_id(event_id, payload_json)` et `restore_world_state` dans `WasmRuntime`.
   - Enrichissement de `TickReport` avec `emitted_events: Vec<Identifier>` pour tracer les événements émis durant chaque cycle.

3. **Modélisation réelle de `rogue-core`** :
   - Formule de dégâts calculée dynamiquement par le moteur : `max(0, goblin.health.current - max(1, hero.stats.attack - goblin.stats.defense))`.
   - Ajout des événements joueur `player_attack` et `player_drink_potion` (`potions -= 1`, `hp = min(max_hp, hp + 30)`).

4. **Refonte Data-Driven du Studio** :
   - Suppression totale de `RogueGameView.tsx` et `templates/projects.ts`.
   - Importation directe des fichiers JSON officiels (`templates/game/project.json`, `templates/app/project.json`).
   - Création de composants génériques : `WorldStateView` (inspection réactive de toutes les entités, barres de PV et badges de vie), `PlayerActions` (génération automatique de boutons pour chaque événement joueur), et `EventLog` (journal en direct des cascades).
   - Restauration réelle des sauvegardes via `runtime.restore_world_state`.

## Conséquences

- Toute modification de caractéristique (`attack`, `defense`) dans `project.json` altère immédiatement les calculs de combat dans le moteur sans toucher au code Rust ou TypeScript.
- Hydrogen Studio devient un véritable environnement d'exécution générique pour n'importe quelle application ou jeu déclaratif.
- L'ensemble de la base de code respecte rigoureusement la charte `ELMAS.md` (aucun fichier > 350 lignes, 3 arguments max, zéro unwrap en code de production).
