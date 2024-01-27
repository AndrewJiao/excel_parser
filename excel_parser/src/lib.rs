use std::env::VarError;
use thiserror::Error;

pub mod template;
pub mod output;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("xsl error : {0}")]
    XslError(#[from] calamine::XlsxError),

    #[error("de error : {0}")]
    DeError(#[from] calamine::DeError),

    #[error("var error : {0}")]
    VarError(#[from] VarError),

    #[error("serde json error : {0}")]
    JsonError(#[from] serde_json::Error),
}
