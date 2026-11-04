//! Définition des arguments et sous-commandes de la CLI via `clap`.

use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// Interface en ligne de commande du moteur déclaratif Hydrogen.
#[derive(Debug, Parser)]
#[command(
    name = "hydrogen",
    author = "Elmas <elmas@elmascore.dev>",
    version,
    about = "Moteur déclaratif low-code / no-code pour applications, jeux et outils"
)]
pub struct CliArgs {
    /// Commande à exécuter.
    #[command(subcommand)]
    pub command: Commands,
}

/// Sous-commandes disponibles dans l'outil `hydrogen`.
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Initialise un nouveau projet Hydrogen à partir du template officiel.
    New {
        /// Chemin du dossier cible à créer ou initialiser.
        path: PathBuf,

        /// Modèle de départ ('app' ou 'game').
        #[arg(short, long, default_value = "app")]
        template: String,
    },

    /// Valide la syntaxe et la sémantique d'un fichier de définition de projet.
    Validate {
        /// Chemin vers le fichier JSON de définition (défaut: 'project.json').
        #[arg(default_value = "project.json")]
        path: PathBuf,
    },

    /// Exécute un projet Hydrogen sur un nombre donné de cycles d'horloge (`ticks`).
    Run {
        /// Chemin vers le fichier JSON de définition (défaut: 'project.json').
        #[arg(default_value = "project.json")]
        path: PathBuf,

        /// Nombre de cycles d'exécution (`ticks`) à jouer.
        #[arg(short, long, default_value_t = 1)]
        ticks: u64,
    },
}
