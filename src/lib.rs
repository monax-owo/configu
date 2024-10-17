pub(crate) mod app_config;
pub(crate) mod builder;

pub use self::app_config::*;
pub use self::builder::*;

pub trait Configurable {
  /// selfの内容をファイルに書き込むメソッド
  fn save(&self) -> anyhow::Result<()>;
  /// ファイルの内容をselfに書き込むメソッド
  fn load(&mut self) -> anyhow::Result<()>;
}
