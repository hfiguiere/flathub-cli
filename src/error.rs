// SPDX-FileCopyrightText: 2023 Hubert Figuière
//
// SPDX-License-Identifier: GPL-3.0-or-later

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid argument")]
    InvalidArgument,
    #[error("Already exist")]
    AlreadyExist,
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
}

pub type Result<T> = std::result::Result<T, Error>;
