use std::io;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io error: {0}")]
    Io(#[from] io::Error),
    #[error("yaml load error: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("unsuported device configuration")]
    DeviceConfigUnsupported,
    #[error("invalid device configuration value")]
    DeviceConfigInvalid,
    #[error("error: {0}")]
    Message(String),
}

impl From<input::DeviceConfigError> for Error {
    fn from(value: input::DeviceConfigError) -> Self {
        match value {
            input::DeviceConfigError::Unsupported => Self::DeviceConfigUnsupported,
            input::DeviceConfigError::Invalid => Self::DeviceConfigInvalid,
        }
    }
}
