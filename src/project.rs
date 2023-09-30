use std::path::{Path, PathBuf};

use crate::repo;
use crate::{Error, Result};

const PROJECT_FILE: &str = "flathub-project.toml";

/// A Project is what lead to building a Flatpak.
pub(crate) struct Project {
    /// The directory where the project is located.
    path: PathBuf,
}

impl Project {
    /// Tell if a project in `dir` exists.
    pub fn exists<P>(dir: P) -> bool
    where
        P: AsRef<Path>,
    {
        if !dir.as_ref().try_exists().unwrap_or(false) {
            return false;
        }
        let project_file = dir.as_ref().join(PROJECT_FILE);
        if !project_file.is_file() {
            return false;
        }
        // XXX check the file is valid
        true
    }

    pub fn create<P>(dir: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        if Self::exists(&dir) {
            return Err(Error::AlreadyExist);
        }
        if dir.as_ref().try_exists()? && repo::check_repo_exist(&dir) {
            return Err(Error::AlreadyExist);
        }
        let proj = Self {
            path: dir.as_ref().to_path_buf(),
        };
        std::fs::create_dir_all(&dir)?;
        let project_file = dir.as_ref().join(PROJECT_FILE);
        let repo = git2::Repository::init(&dir)?;
        std::fs::File::create(project_file)?;
        let mut index = repo.index()?;
        index.add_path(&PathBuf::from(PROJECT_FILE))?;
        index.write()?;

        Ok(proj)
    }

    pub fn open<P>(dir: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        if !Self::exists(&dir) {
            return Err(Error::NotFound);
        }
        Ok(Self {
            path: dir.as_ref().to_path_buf(),
        })
    }
}
