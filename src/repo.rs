// SPDX-FileCopyrightText: 2023 Hubert Figuière
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{Error, Result};

/// Check if the git repository at `repo` exists.
pub fn check_repo_exist<P>(repo: P) -> bool
where
    P: AsRef<std::path::Path>,
{
    git2::Repository::open(repo).is_ok()
}

/// Convenience to add a path to a git repo.
/// Will ensure the path is relative.
///
/// # Panic
/// Will panic is the repo is bare.
pub fn add_path_to_repo<P>(repo: &git2::Repository, path: P) -> Result<()>
where
    P: AsRef<std::path::Path>,
{
    assert!(!repo.is_bare());

    let base_path = repo.path().parent().ok_or(Error::InvalidArgument)?;
    let relative_path = path
        .as_ref()
        .strip_prefix(base_path)
        .or(Err(Error::InvalidArgument))?;
    let mut index = repo.index()?;
    index.add_path(relative_path)?;
    index.write()?;

    Ok(())
}

pub(crate) fn add_submodule_to_repo<P>(repo: &git2::Repository, url: &str, path: P) -> Result<()>
where
    P: AsRef<std::path::Path>,
{
    let mut submodule = repo.submodule(url, path.as_ref(), false)?;
    let _ = submodule.open()?;
    submodule.clone(None)?;
    submodule.add_finalize()?;

    Ok(())
}
