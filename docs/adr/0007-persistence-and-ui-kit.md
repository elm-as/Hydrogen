# ADR 0007 — Architecture de persistance (hydrogen-data) et kit UI partagé (ui-kit)

- **Statut** : Accepté
- **Date** : 2026-11-04

## Contexte

L'étape 7 consolide la plateforme Hydrogen en fournissant deux briques fondamentales réutilisables :
1. Une couche de persistance universelle (`crates/hydrogen-data`) pour sauvegarder, restaurer et lister les instantanés de simulation (`WorldState`) de manière fiable et versionnée.
2. Un kit de composants d'interface utilisateur partagés (`packages/ui-kit`) répondant aux exigences strictes de la charte ElmasCore (densité d'information, chiffres tabulaires, sobriété technique, zéro шаблон SaaS générique).

## Décisions

1. **Abstraction de persistance par trait (`SaveStore`)** :
   - Définition du trait `SaveStore` exposant `save`, `load`, `delete`, `list_slots` et `exists`.
   - Structure d'instantané `SaveSnapshot` contenant :
     - `version: SchemaVersion` (vérification de compatibilité de schéma à la désérialisation).
     - `project_name: String`
     - `tick: u64` (horodatage logique).
     - `world_state: WorldState` (état exact des entités et composants).
     - `metadata: BTreeMap<String, String>` (horodatage réel, libellé, étiquettes).
   - Sécurisation stricte des identifiants de slots via `validate_slot_name` (rejet immédiat de toute traversée de chemin `../`, séparateurs ou caractères spéciaux).

2. **Adaptateurs de persistance concrets** :
   - `FsStore` : persistance locale sur disque par fichiers JSON atomiques. L'écriture s'effectue dans un fichier `.tmp` synchronisé via `sync_all()`, puis renommé de façon atomique vers `.json`.
   - `MemoryStore` : stockage en mémoire vive (`BTreeMap`), sans entrée/sortie, portable en environnement WASM et déterministe pour les suites de tests.

3. **Kit UI partagé (`packages/ui-kit`)** :
   - Bibliothèque de composants React 19 / TypeScript stricts sans dépendance CSS lourde ni runtime CSS-in-JS.
   - Typographie ancrée dans la charte ElmasCore : chiffres tabulaires obligatoires (`tabular-nums`), palette sombre sobre et lisible (slate, charcoal, cyan technique, sans dégradés décoratifs).
   - Composants disponibles :
     - `Button` : variantes primaires, secondaires, danger, ghost avec gestion native du chargement et des états désactivés.
     - `Badge` : indicateurs d'états d'exécution, numéros de ticks et versions de schémas.
     - `InspectorRow` : ligne clé/valeur haute densité pour l'inspection des entités et métriques.
     - `Surface` : conteneur encadré avec en-tête et pied de page.
   - Intégration et validation dans `apps/studio`.

## Conséquences

- Les simulations Hydrogen peuvent être sauvegardées et rechargées de façon transactionnelle sans risque de corruption partielle de fichier.
- Le kit UI élimine la duplication des styles de formulaires et d'inspection entre le studio desktop et les futures applications.
