use hmac::digest::crypto_common;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OssError{
  #[error("reqwest error: {0}")]
  Request(#[from] reqwest::Error),

  #[error("url parse error: {0}")]
  UrlParse(#[from] url::ParseError),

  #[error("{0}")]
  #[cfg(test)]
  Dotenv(#[from] dotenv::Error),

  #[error("var error: {0}")]
  VarError(#[from] std::env::VarError),

  #[error("input error: {0}")]
  Input(String),

  #[error("io error: {0}")]
  Io(#[from] std::io::Error),

  #[error("QuickXml error: {0}")]
  QuickXml(#[from] quick_xml::Error),

  #[error("chrono error: {0}")]
  Chrono(#[from] chrono::ParseError),

  #[error("ParseIntError: {0}")]
  ParseIntError(#[from] std::num::ParseIntError),

  #[error("hmac InvalidLength: {0}")]
  InvalidLength(#[from] crypto_common::InvalidLength),

  #[error(transparent)]
  Other(#[from] anyhow::Error),
}

pub type OssResult<T> = Result<T,OssError>;