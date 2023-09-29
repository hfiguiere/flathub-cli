mod clone;
mod cmd;
mod error;
mod flathub;
mod repo;

use clap::{Parser, Subcommand};

use error::{Error, Result};

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Init a flatpak package repository.
    Init(cmd::init::Args),
    /// Clone a flathub package.
    Clone(clone::Args),
    /// Build the package.
    Build,
    Submit,
    /// Run the linter without building.
    Lint,
    /// Clean build artifacts.
    Clean,
    /// Manage modules.
    Module,
    /// Configure `flathub-cli`
    Configure,
    /// Update `flathub-cli` configuration
    Update,
    /// Create a manifest.
    CreateManifest,
}

fn main() -> Result<()> {
    let args = Args::parse();
    match args.command {
        Commands::Init(args) => cmd::init::run(args),
        Commands::Clone(args) => clone::run(args),
        _ => {
            println!("Currently unimplemented.");
            Err(Error::InvalidArgument)
        }
    }
}
