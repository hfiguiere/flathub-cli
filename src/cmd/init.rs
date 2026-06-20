// SPDX-FileCopyrightText: 2023-2026 Hubert Figuière
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::path::PathBuf;

use clap::Parser;

use crate::project::Project;
use crate::{error::Context, AnyError, Error, ErrorContext, Result};

#[derive(Parser)]
pub struct Args {
    #[arg(short, long, help = "Initialize project even if git repository exists")]
    /// Initialize if existing.
    existing: bool,
    #[arg(short, long, help = "The application id. Otherwise it is extrapolated")]
    /// The application id.
    id: Option<String>,
    /// Path
    path: String,
}

/// Run the init command.
pub fn run(args: Args) -> Result<()> {
    let current_dir = std::env::current_dir()?;

    let target_dir = PathBuf::from(&current_dir).join(&args.path);
    if !args.existing && Project::exists(&target_dir) {
        return Err(AnyError::context(
            "use --existing to override".into(),
            Error::AlreadyExist(ErrorContext::Project),
        ));
    }
    let id = match args.id {
        Some(id) => id,
        _ => target_dir
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string(),
    };
    Project::create(&target_dir, &id, args.existing).context("use --existing to override")?;

    println!("Created project and git repository at {:?}", &args.path);
    Ok(())
}
