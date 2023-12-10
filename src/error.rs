// SPDX-FileCopyrightText: 2023 Hubert Figuière
//
// SPDX-License-Identifier: GPL-3.0-or-later

/// An error context.
#[derive(Debug)]
pub enum Context {
    /// Error relate to diretory (of the project)
    Directory,
    /// Error relate to project file
    Project,
    /// Error relate to git repository
    Repository,
}

impl std::fmt::Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Directory => "Directory",
                Self::Project => "Project",
                Self::Repository => "Repository",
            }
        )
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid argument")]
    InvalidArgument,
    #[error("{0} already exist")]
    AlreadyExist(Context),
    #[error("Not found")]
    NotFound,
    #[error("Manifest error")]
    Manifest,
    #[error("IO error {0}")]
    IoError(#[from] std::io::Error),
    #[error("Git error {0}")]
    Git(#[from] git2::Error),
    #[error("Toml serialization error {0}")]
    TomlSer(#[from] toml::ser::Error),
    #[error("Toml deserialization error {0}")]
    TomlDe(#[from] toml::de::Error),
    #[error("Serde JSON error {0}")]
    SerdeJson(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
