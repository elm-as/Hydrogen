# ADR 0004 — Architecture de l'interface en ligne de commande (`hydrogen-cli`)

- **Statut** : Accepté
- **Date** : 2026-11-04

## Contexte

L'étape 4 introduit l'outil en ligne de commande `hydrogen-cli` permettant de :
1. Initialiser de nouveaux projets Hydrogen déclaratifs (`hydrogen new`).
2. Valider la conformité statique et sémantique d'un fichier de projet sans l'exécuter (`hydrogen validate`).
3. Exécuter un projet déclaratif sur la ligne de commande pendant un nombre défini de cycles (`hydrogen run`).

Conformément à `ELMAS.md`, toute nouvelle dépendance doit être justifiée, les erreurs doivent être typées, et aucun `unwrap()` n'est toléré dans le code de production.

## Décisions

1. **Dépendance CLI : `clap` (feature `derive`)** :
   - `clap` est le standard de l'écosystème Rust pour le parsing d'arguments en ligne de commande. Il génère automatiquement l'aide formatée (`--help`), la gestion des versions (`--version`) et les sous-commandes typées.
2. **Codes de sortie standardisés** :
   - `0` : Exécution réussie.
   - `1` : Erreur de syntaxe, de schéma ou de validation statique (`SchemaError`).
   - `2` : Erreur d'exécution au runtime (`RuntimeError`).
   - `3` : Erreur d'entrée/sortie ou de chemin de fichier (`std::io::Error`).
3. **Templates embarqués dans le binaire (`include_str!`)** :
   - Le modèle de projet initial est embarqué au moment de la compilation pour que `hydrogen new` fonctionne de manière 100 % autonome, sans dépendance relative à un dossier de templates sur le système de fichiers hôte.
   - Un miroir du template est également conservé dans `templates/app/project.json` pour la documentation et les tests.
4. **Sortie déterministe et lisible** :
   - La commande `run` affiche les métriques de cycle et l'état final formaté en JSON.
   - Les messages d'erreurs sont acheminés vers `stderr` sans panic.

## Conséquences

- `hydrogen-cli` devient le premier consommateur natif de `hydrogen-schema` et `hydrogen-runtime`.
- La CLI est prête à être empaquetée ou distribuée comme binaire autonome.

