pub(crate) mod config;

use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
  #[error("configuration file was not found: {0}")]
  NotFound(PathBuf),
  #[error("failed to parse")]
  Parse(String),
}

pub type Result<T> = core::result::Result<T, Error>;

pub trait Configurable {
  /// selfの内容をファイルに書き込むメソッド
  fn save(&self) -> anyhow::Result<()>;
  /// ファイルの内容をselfに書き込むメソッド
  fn load(&mut self) -> anyhow::Result<()>;
}

#[derive(Debug)]
pub struct Config<T = ()> {
  pub file_path: Option<PathBuf>,
  pub(crate) data: T,
}

#[derive(Debug)]
pub struct RwLockConfig<T = ()> {
  pub file_path: Option<PathBuf>,
  pub(crate) data: T,
}
