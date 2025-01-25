pub(crate) mod config;

use std::path::PathBuf;

pub trait Configurable {
  /// selfの内容をファイルに書き込むメソッド
  fn save(&self) -> anyhow::Result<()>;
  /// ファイルの内容をselfに書き込むメソッド
  fn load(&mut self) -> anyhow::Result<()>;
}

#[derive(Debug)]
pub struct Config<T = ()> {
  pub file_path: PathBuf,
  pub(crate) config: T,
}

#[derive(Debug)]
pub struct RwLockConfig<T = ()> {
  pub file_path: PathBuf,
  pub(crate) config: T,
}
