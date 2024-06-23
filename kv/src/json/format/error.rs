use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("input contains nul character: {0}")]
    NulInput(#[from] std::ffi::NulError),
    #[error("output is not valid utf-8: {0}")]
    InvalidOutput(#[from] std::str::Utf8Error),
    #[error("opening memstream file")]
    OpenMemstreamFile,
    #[error("closing memstream file: {0}")]
    CloseMemstreamFile(libc::c_int),
}
