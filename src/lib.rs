pub(crate) mod config;

use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("configuration file path was not provided")]
  PathNotSpecified,
  #[error("configuration file was not found: {0}")]
  NotFound(PathBuf),
  #[error("failed to create configuration file: {0}")]
  Create(#[source] std::io::Error),
  #[error("failed to open configuration file: {0}")]
  Open(#[source] std::io::Error),
  #[error("failed to read configuration file: {0}")]
  Read(#[source] std::io::Error),
  #[error("failed to write configuration file: {0}")]
  Write(#[source] std::io::Error),
  #[error("failed to serialize: {0}")]
  Serialize(String),
  #[error("failed to deserialize: {0}")]
  Deserialize(String),
}

pub type Result<T> = core::result::Result<T, Error>;

pub trait Configurable {
  /// selfの内容をファイルに書き込むメソッド
  fn save(&self) -> Result<()>;
  /// ファイルの内容をselfに書き込むメソッド
  fn load(&mut self) -> Result<()>;
}

#[derive(Debug)]
pub struct Config<T = ()> {
  pub file_path: Option<PathBuf>,
  pub(crate) data: T,
}
