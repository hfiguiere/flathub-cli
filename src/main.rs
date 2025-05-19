// SPDX-FileCopyrightText: 2023 Hubert Figuière
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod builder;
mod cmd;
mod error;
mod flathub;
mod manifest;
mod project;
mod repo;

use clap::{Parser, Subcommand};

use error::{Context as ErrorContext, Error, Result};

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
    Manifest(cmd::manifest::Args),
    /// Clone a flathub package.
    Clone(cmd::clone::Args),
    /*
    /// Build the package.
    Build,
    Submit,
    /// Run the linter without building.
    Lint,
    */
    /// Cleanup build artifacts.
    Cleanup(cmd::cleanup::Args),
    /*
    /// Manage modules.
    Module,
    /// Configure `flathub-cli`
    Configure,
    /// Update `flathub-cli` configuration
    Update,
     */
}

fn main() -> Result<()> {
    let args = Args::parse();
    match args.command {
        Commands::Init(args) => cmd::init::run(args),
        Commands::Cleanup(args) => cmd::cleanup::run(args),
        Commands::Clone(args) => cmd::clone::run(args),
        Commands::Manifest(args) => cmd::manifest::run(args),
    }
}
