mod cmd;
mod error;
mod flathub;
mod project;
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
    /// Init a flatpak project.
    Init(cmd::init::Args),
    /// Manage manifest.
    Manifest,
    /// Clone a flathub package.
    Clone(cmd::clone::Args),
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
}

fn main() -> Result<()> {
    let args = Args::parse();
    match args.command {
        Commands::Init(args) => cmd::init::run(args),
        Commands::Clone(args) => cmd::clone::run(args),
        _ => {
            println!("Currently unimplemented.");
            Err(Error::InvalidArgument)
        }
    }
}
