#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid argument")]
    InvalidArgument,
    #[error("Already exist")]
    AlreadyExist,
    #[error("IO error {0}")]
    IoError(#[from] std::io::Error),
    #[error("Git error {0}")]
    Git(#[from] git2::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
