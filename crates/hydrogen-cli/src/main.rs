//! Point d'entrée de l'exécutable `hydrogen`.

use clap::Parser;
use hydrogen_cli::{run_cli, CliArgs};

fn main() {
    let args = CliArgs::parse();
    if let Err(err) = run_cli(args) {
        eprintln!("[Erreur Hydrogen] {err}");
        std::process::exit(err.exit_code());
    }
}
