use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Init a flatpak package repository.
    Init,
    /// Build the package.
    Build,
    Submit,
    /// Run the linter without building.
    Lint,
    /// Clean build artifacts.
    Clean,
    /// Add a module.
    Add,
    /// Configure `flathub-cli`
    Configure,
    /// Update `flathub-cli` configuration
    Update,
    /// Create a manifest.
    CreateManifest,
}

fn main() {
    let args = Args::parse();
    match args.command {
        _ => {
            println!("Currently unimplemented.")
        }
    }
}
