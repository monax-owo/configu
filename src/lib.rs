pub(crate) mod builder;
pub(crate) mod config;

pub use self::builder::*;
pub use self::config::*;

pub trait Configurable {
  /// selfの内容をファイルに書き込むメソッド
  fn save(&self) -> anyhow::Result<()>;
  /// ファイルの内容をselfに書き込むメソッド
  fn load(&mut self) -> anyhow::Result<()>;
}
