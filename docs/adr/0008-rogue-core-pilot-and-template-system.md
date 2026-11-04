# ADR 0008 — Architecture du projet pilote rogue-core et support multi-templates CLI

- **Statut** : Accepté
- **Date** : 2026-11-04

## Contexte

Pour valider l'intégralité de la chaîne logicielle d'Hydrogen (cœur sans I/O, validation de schéma, moteur d'exécution déterministe, persistance par slots et outillage CLI/Studio) sur un cas concret, nous implémentons le premier projet pilote officiel : **`rogue-core`**.

## Décisions

1. **Modèle de jeu déclaratif (`templates/game/project.json`)** :
   - **Composants** :
     - `health` : points de vie actuels et maximaux (`current`, `max`).
     - `stats` : caractéristiques d'attaque, défense et état vital (`is_alive`).
     - `inventory` : accumulation d'or et consommables (`gold`, `potions`).
     - `dungeon` : suivi de l'étage et du numéro de tour (`floor`, `turn`).
   - **Entités** : `hero`, `goblin`, `game_state`.
   - **Événements** : `turn_advance`, `hero_attack`, `goblin_defeated`, `loot_collected`.
   - **Règles en cascade** :
     - `rule_tick_turn` : incrémente le tour à chaque tick et émet `turn_advance`.
     - `rule_hero_attack_on_turn` : conditionnée à `goblin.stats.is_alive == true`, applique les dégâts (-15 PV) et émet `hero_attack`.
     - `rule_check_goblin_death` : déclenchée sur `hero_attack`, teste `current <= 0`, bascule `is_alive` à `false` et émet `goblin_defeated`.
     - `rule_reward_on_goblin_death` : déclenchée sur `goblin_defeated`, crédite +50 pièces d'or sur l'inventaire du héros.

2. **Évolution de la CLI (`hydrogen new --template <NAME>`)** :
   - Ajout du drapeau `--template` / `-t` sur la commande `hydrogen new` (valeurs supportées : `app`, `game`).
   - Contrôle strict et erreur typée `CliError::UnknownTemplate` pour tout argument non reconnu.

3. **Validation de boucle et voyage dans le temps (`rogue_simulation_test.rs`)** :
   - Exécution successive des tours avec assertion de l'état du monde.
   - Sauvegarde d'instantané atomique (`FsStore`) à l'issue du Tour 1.
   - Poursuite de la partie jusqu'à l'élimination du gobelin au Tour 2 et vérification de la cascade.
   - Restauration de la sauvegarde du Tour 1 via `runtime.restore_world_state` : validation d'un rejeu 100% déterministe depuis le point de reprise.

## Conséquences

- Hydrogen dispose d'une suite de référence éprouvant l'ensemble des couches techniques en interaction réelle.
- Le modèle de jeu est immédiatement utilisable via `hydrogen new <dossier> --template game` ou directement exécutable avec `hydrogen run`.
