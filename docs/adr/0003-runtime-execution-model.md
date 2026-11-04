# ADR 0003 — Modèle d'exécution du runtime, transactionnalité et benchmarks

- **Statut** : Accepté
- **Date** : 2026-11-04

## Contexte

L'étape 3 introduit le moteur d'exécution (`hydrogen-runtime`) chargé de faire évoluer l'état du monde (`WorldState`) à partir d'une définition (`ProjectDef`) validée. Le moteur doit garantir :
1. Un déterminisme strict quel que soit l'environnement (natif, CLI, WASM).
2. L'absence totale d'état corrompu en cas d'erreur lors d'un cycle.
3. La prévention des boucles infinies de cascades d'événements.
4. L'absence de panics sur les opérations arithmétiques.
5. Une mesure systématique des performances via `criterion` avant toute optimisation prématurée.

## Décisions

1. **Transactionnalité stricte de `Runtime::tick()`** :
   - Chaque cycle d'exécution travaille sur une copie de travail de l'état du monde (`WorldState`) et de la file d'événements (`EventBus`).
   - Si une action échoue (dépassement arithmétique, limite d'événements atteinte), l'erreur typée `RuntimeError` est retournée et l'état original est préservé intact.
2. **Cascade d'événements dans le même cycle avec budget fini (`max_events_per_tick`)** :
   - Les règles `OnTick` s'exécutent en premier, dans l'ordre de leur déclaration.
   - Les événements émis (`RuleAction::EmitEvent`) alimentent l'`EventBus` et sont dépilés en FIFO au sein du même cycle.
   - Un budget maximal `RuntimeConfig::max_events_per_tick` (par défaut 1024) est imposé. S'il est dépassé, l'exécution s'interrompt avec `RuntimeError::EventLoopLimitExceeded`.
3. **Arithmétique sécurisée (`checked_add`)** :
   - `RuleAction::AddNumeric` utilise `checked_add` pour les entiers `i64` et vérifie la finitude pour les flottants `f64`. Tout dépassement produit `RuntimeError::ArithmeticOverflow`.
4. **Initialisation automatique des champs par défaut** :
   - Lors de la construction de l'état initial (`build_initial_world`), les champs de composants non explicitement renseignés dans `EntityDef` reçoivent automatiquement la `default_value` définie dans `ComponentFieldDef`.
5. **Benchmarks de référence (`criterion`)** :
   - Intégration de `criterion` en `dev-dependencies` dans `hydrogen-runtime` pour mesurer le coût d'un tick (1, 100 et 1000 règles) ainsi que des cascades d'événements.

## Conséquences

- Zéro panic possible en runtime : toutes les anomalies sont typées dans `RuntimeError`.
- La garantie transactionnelle permet au Studio et au web (WASM) de signaler une erreur à l'utilisateur sans redémarrer le projet ni corrompre l'état.

