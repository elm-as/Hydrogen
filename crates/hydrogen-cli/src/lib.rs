//! Bibliothèque sous-jacente de l'interface en ligne de commande `hydrogen-cli`.

pub mod args;
pub mod commands;
pub mod error;

pub use args::{CliArgs, Commands};
pub use error::CliError;

/// Point d'entrée logique dispatchant la sous-commande reçue vers son module dédié.
///
/// # Errors
/// Retourne [`CliError`] avec son code d'erreur associé en cas d'échec de la commande.
pub fn run_cli(args: CliArgs) -> Result<(), CliError> {
    match args.command {
        Commands::New { path } => commands::execute_new(&path),
        Commands::Validate { path } => commands::execute_validate(&path),
        Commands::Run { path, ticks } => commands::execute_run(&path, ticks),
    }
}
