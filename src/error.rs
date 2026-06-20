// SPDX-FileCopyrightText: 2023-2026 Hubert Figuière
//
// SPDX-License-Identifier: GPL-3.0-or-later

/// An error context.
#[derive(Debug)]
pub enum ErrorContext {
    /// Error relate to diretory (of the project)
    Directory,
    /// Error relate to project file
    Project,
    /// Error relate to git repository
    Repository,
}

impl std::fmt::Display for ErrorContext {
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

/// Create an error from a string.
#[macro_export]
macro_rules! anyerror {
    ($msg:literal) => {
        $crate::AnyError::from($crate::Error::Any($msg.into()))
    };
    ($msg:expr) => {
        $crate::AnyError::from($crate::Error::Any($msg))
    };
}

#[derive(thiserror::Error)]
#[error("{}, {}", self.source, self.context)]
pub struct AnyError {
    #[source]
    source: Error,
    context: String,
}

impl AnyError {
    pub fn source(&self) -> &Error {
        &self.source
    }
}

/// Implement Debug manually to use the Display for the error,
/// unlike the default implementation.
impl std::fmt::Debug for AnyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{self}")
    }
}

impl<E> From<E> for AnyError
where
    E: std::error::Error + Into<Error>,
{
    fn from(source: E) -> Self {
        Self {
            source: source.into(),
            context: String::default(),
        }
    }
}

impl AnyError {
    pub fn context(context: String, source: Error) -> Self {
        Self { source, context }
    }
}

impl<T> Context<T, AnyError> for core::result::Result<T, AnyError> {
    fn context(self, context: &str) -> core::result::Result<T, AnyError> {
        match self {
            Ok(ok) => Ok(ok),
            Err(err) => Err(AnyError::context(context.into(), err.source)),
        }
    }

    fn with_context<F>(self, context: F) -> core::result::Result<T, AnyError>
    where
        F: FnOnce() -> String,
    {
        match self {
            Ok(ok) => Ok(ok),
            Err(err) => Err(AnyError::context(context(), err.source)),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid argument")]
    InvalidArgument,
    #[error("{0} already exist")]
    AlreadyExist(ErrorContext),
    #[error("Not found")]
    NotFound,
    #[error("Manifest error")]
    Manifest,
    #[error("Not implemented")]
    NotImplemented,
    #[error("IO error {0}")]
    Io(#[from] std::io::Error),
    #[error("Git error {0}")]
    Git(#[from] git2::Error),
    #[error("Toml serialization error {0}")]
    TomlSer(#[from] toml::ser::Error),
    #[error("Toml deserialization error {0}")]
    TomlDe(#[from] toml::de::Error),
    #[error("Serde JSON error {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("Strip prefix error {0}")]
    StripPrefixError(#[from] std::path::StripPrefixError),
    #[error("Any error {0}")]
    Any(String),
}

pub type Result<T> = std::result::Result<T, AnyError>;

/// This trait allow creating a result with `Error` and a context from
/// another error.
pub trait Context<T, E> {
    fn context(self, context: &str) -> core::result::Result<T, AnyError>;
    fn with_context<F>(self, context: F) -> core::result::Result<T, AnyError>
    where
        F: FnOnce() -> String;
}

impl<T, E> Context<T, E> for core::result::Result<T, E>
where
    E: core::error::Error + Into<Error> + 'static,
{
    fn context(self, context: &str) -> core::result::Result<T, AnyError> {
        match self {
            Ok(ok) => Ok(ok),
            Err(err) => Err(AnyError::context(context.into(), err.into())),
        }
    }

    fn with_context<F>(self, context: F) -> core::result::Result<T, AnyError>
    where
        F: FnOnce() -> String,
    {
        match self {
            Ok(ok) => Ok(ok),
            Err(err) => Err(AnyError::context(context(), err.into())),
        }
    }
}
