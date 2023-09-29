use clap::Parser;

use crate::flathub;
use crate::{Error, Result};

#[derive(Parser)]
pub struct Args {
    #[arg(short, long)]
    /// Use ssh to clone.
    ssh: bool,
    /// Package to clone. app-id or git repository.
    package: String,
}

pub fn run(args: Args) -> Result<()> {
    let package = &args.package;

    // if is a git URL
    // XXX this is broken by design
    let (url, dirname) = if package.starts_with("http://")
        || package.starts_with("https://")
        || package.starts_with("git@")
    {
        // find the directory name
        if let Some((_, dir)) = package.rsplit_once('/') {
            (package.to_owned(), dir)
        } else {
            return Err(Error::AlreadyExist);
        }
    } else {
        let url = flathub::repo_for_package(package);
        (url, package.as_str())
    };
    let current_dir = std::env::current_dir()?;
    let dest = current_dir.join(dirname);
    if dest.try_exists()? {
        return Err(Error::AlreadyExist);
    }
    let _repo = git2::Repository::clone(&url, &dest)?;
    println!("Cloned {package} into {dest:?}");
    Ok(())
}
