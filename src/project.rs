// SPDX-FileCopyrightText: 2023-2025 Hubert Figuière
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use crate::repo;
use crate::{Error, ErrorContext, Result};
use serde::{Deserialize, Serialize};

const PROJECT_FILE: &str = "flathub-project.toml";

#[derive(Deserialize, Serialize)]
/// The config data of the project.
pub(crate) struct Config {
    /// The package id.
    id: String,
    /// The path to the manifest relative to the project.
    manifest: String,
}

/// A Project is what lead to building a Flatpak.
pub(crate) struct Project {
    /// The directory where the project is located.
    pub(crate) path: PathBuf,
    /// The configuration, as saved in `PROJECT_FILE`.
    config: Config,
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
        let config = Self::load_config(&project_file);
        if config.is_err() {
            return false;
        }

        true
    }

    pub fn create<P>(dir: P, project_id: &str, existing: bool) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        if !existing && Self::exists(&dir) {
            return Err(Error::AlreadyExist(ErrorContext::Project).into());
        }
        if !existing && (dir.as_ref().try_exists()? && repo::check_repo_exist(&dir)) {
            return Err(Error::AlreadyExist(ErrorContext::Repository).into());
        }
        let mut manifest = String::from(project_id);
        manifest.push_str(".json");
        let config = Config {
            id: project_id.to_string(),
            manifest,
        };
        let proj = Self {
            path: dir.as_ref().to_path_buf(),
            config,
        };
        std::fs::create_dir_all(&dir)?;
        let project_file = dir.as_ref().join(PROJECT_FILE);
        proj.create_project_file(project_file)?;

        // Create the git repo
        let repo = git2::Repository::init(&dir)?;
        let mut index = repo.index()?;
        // Add the project file to the repo.
        index.add_path(&PathBuf::from(PROJECT_FILE))?;
        index.write()?;

        Ok(proj)
    }

    pub fn open<P>(dir: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        if !Self::exists(&dir) {
            return Err(Error::NotFound.into());
        }
        let project_file = dir.as_ref().join(PROJECT_FILE);
        let config = Self::load_config(&project_file)?;
        Ok(Self {
            path: dir.as_ref().to_path_buf(),
            config,
        })
    }

    /// Return the repo for the project.
    pub fn repo(&self) -> Result<git2::Repository> {
        let repo = git2::Repository::init(&self.path)?;

        Ok(repo)
    }

    pub fn id(&self) -> &str {
        &self.config.id
    }

    fn create_project_file<P: AsRef<Path>>(&self, project_file: P) -> Result<()> {
        // Create the project file.
        let mut file = std::fs::File::create(project_file)?;
        let toml = toml::to_string(&self.config)?;
        file.write_all(toml.as_bytes())?;

        Ok(())
    }

    fn load_config(project_file: &Path) -> Result<Config> {
        let mut toml = String::default();
        let mut file = std::fs::File::open(project_file)?;
        file.read_to_string(&mut toml)?;
        let config: Config = toml::from_str(&toml)?;

        Ok(config)
    }
}
