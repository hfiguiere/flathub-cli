// SPDX-FileCopyrightText: 2023 Hubert Figuière
//
// SPDX-License-Identifier: GPL-3.0-or-later

use clap::Parser;

use crate::manifest;
use crate::project::Project;
use crate::{Error, Result};

#[derive(Parser)]
pub struct Args {
    /// Command
    command: String,
}

fn create_manifest() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let project = Project::open(current_dir)?;

    let manifest = manifest::Manifest::prompt_with_id(Some(project.id())).ok_or(Error::Manifest)?;

    let repo = project.repo()?;
    manifest.generate(&repo, project.path)?;

    Ok(())
}

pub fn run(args: Args) -> Result<()> {
    match args.command.as_str() {
        "create" => create_manifest(),
        _ => Err(Error::InvalidArgument),
    }
}
