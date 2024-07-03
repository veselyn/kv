use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("internal config error: {0}")]
    Other(#[from] config::ConfigError),
}
