use std::path::PathBuf;

use clap::Parser;

use crate::repo;
use crate::{Error, Result};

#[derive(Parser)]
pub struct Args {
    /// Path
    path: String,
}

pub fn run(args: Args) -> Result<()> {
    let current_dir = std::env::current_dir()?;

    let target_dir = PathBuf::from(&current_dir).join(args.path);
    if target_dir != current_dir && !target_dir.try_exists()? {
        std::fs::create_dir_all(&target_dir)?;
    }
    let target_dir = target_dir.canonicalize()?;
    if repo::check_repo_exist(&target_dir) {
        return Err(Error::AlreadyExist);
    }
    let repo = git2::Repository::init(&target_dir)?;
    println!("Created git repository at {target_dir:?}");
    Ok(())
}
