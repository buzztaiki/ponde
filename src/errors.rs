use std::io;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io error: {0}")]
    Io(#[from] io::Error),
    #[error("yaml load error: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("error: {0}")]
    Error(String),
}
