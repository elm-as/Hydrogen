# Hydrogen

Socle déclaratif low-code / no-code ElmasCore pour applications, jeux et outils logiciels.
Hydrogen exécute des fichiers de définition versionnés via un cœur en Rust (natif via Tauri v2 et Web via WebAssembly) et fournit un éditeur nodal visuel (**Hydrogen Studio**).

## Architecture du Monorepo

```text
hydrogen/
├─ ELMAS.md                  # Charte de développement non négociable
├─ crates/
│  ├─ hydrogen-core/         # Types fondamentaux, erreurs, ECS/état, événements (sans I/O)
│  ├─ hydrogen-schema/       # Schémas de définition, validation et versioning
│  ├─ hydrogen-runtime/      # Exécution des définitions (règles, flux, boucle d'état)
│  ├─ hydrogen-data/         # Persistance et sauvegarde (traits + adaptateurs)
│  ├─ hydrogen-wasm/         # Bindings WebAssembly du moteur
│  └─ hydrogen-cli/          # CLI (`hydrogen new | validate | run | build`)
├─ packages/
│  ├─ bindings/              # Types TypeScript générés depuis Rust (lecture seule)
│  ├─ editor-core/           # Logique pure du graphe nodal
│  └─ ui-kit/                # Composants d'interface partagés
├─ apps/
│  ├─ studio/                # Éditeur visuel desktop (Tauri v2 + React + @xyflow/react)
│  └─ playground/            # Exécution web minimale propulsée par WASM
├─ templates/                # Modèles de projets pour `hydrogen new`
└─ docs/                     # Architecture Decision Records (ADR) et guides
```

### Sens des dépendances

`hydrogen-core` ← `hydrogen-schema` ← `hydrogen-runtime` ← (`hydrogen-data`, `hydrogen-wasm`, `hydrogen-cli`) ← `apps`.

## Prérequis

- **Rust** (stable, édition 2021+) + `cargo-nextest`, `cargo-deny`
- **Node.js** >= 22 + **pnpm** >= 10
- **just** (orchestrateur de commandes)

## Commandes principales

Toutes les opérations du monorepo sont centralisées dans le `justfile` (ou via les scripts `pnpm` équivalents) :

```bash
# Vérifier le formatage, le lint, les types et la limite de 350 lignes par fichier
just check

# Lancer tous les tests (Rust + TypeScript)
just test

# Vérifier uniquement la limite des 350 lignes par fichier
pnpm check:lines
```
