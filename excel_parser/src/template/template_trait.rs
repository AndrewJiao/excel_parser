use std::fmt::Debug;

use thiserror::Error;

#[derive(Debug,Error)]
pub enum  ParserError{
    #[error("io error:{0}")]
    IoError(#[from] std::io::Error),

    #[error("xsl error : {0}")]
    XslError(#[from] calamine::XlsxError),

    #[error("de error : {0}")]
    DeError(#[from] calamine::DeError),
}

