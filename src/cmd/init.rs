use std::path::PathBuf;

use clap::Parser;

use crate::project::Project;
use crate::{Error, Result};

#[derive(Parser)]
pub struct Args {
    /// Path
    path: String,
}

/// Run the init command.
pub fn run(args: Args) -> Result<()> {
    let current_dir = std::env::current_dir()?;

    let target_dir = PathBuf::from(&current_dir).join(args.path);
    if Project::exists(&target_dir) {
        return Err(Error::AlreadyExist);
    }
    let project = Project::create(&target_dir);
    println!("Created project adnd git repository at {target_dir:?}");
    Ok(())
}
