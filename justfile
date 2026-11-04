# Orchestrateur du monorepo Hydrogen (Rust + pnpm)

set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

# Affiche la liste des recettes disponibles
default:
    @just --list

# Vérifie la limite des 350 lignes par fichier (ELMAS.md)
check-lines:
    pnpm check:lines

# Vérifie le formatage, le lint et le typage TypeScript
check-ts:
    pnpm check:ts

# Vérifie le formatage, Clippy (-D warnings) et les licences Rust
check-rust:
    cargo fmt --all -- --check
    cargo clippy --workspace --all-targets --all-features -- -D warnings
    cargo deny check

# Exécute l'ensemble des vérifications bloquantes (Qualité + Rust + TS)
check: check-lines check-ts check-rust

# Lance les tests unitaires et property-based Rust
test-rust:
    cargo nextest run --workspace --all-features

# Lance les tests unitaires TypeScript (Vitest)
test-ts:
    pnpm test:ts

# Lance toute la suite de tests du monorepo
test: test-rust test-ts

# Lance les benchmarks Criterion sur le runtime
bench:
    cargo bench -p hydrogen-runtime

# Compile le cœur Rust en WebAssembly
build-wasm:
    wasm-pack build crates/hydrogen-wasm --target web --out-dir ../../packages/wasm-core --release

# Formate l'ensemble du code (Rust + TypeScript)
fmt:
    cargo fmt --all
    pnpm format
