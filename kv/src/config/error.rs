use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("getting os specific data dir")]
    GetDataDir,
}
