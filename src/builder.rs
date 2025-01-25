use std::{
  fs::{create_dir_all, File},
  path::{Path, PathBuf},
  sync::RwLock,
};

use anyhow::{bail, Context};
use serde::{Deserialize, Serialize};

use crate::{Config, Configurable};

#[derive(Debug)]
pub struct ConfigBuilder<T = ()> {
  file_path: PathBuf,
  data: T,
}

impl ConfigBuilder<()> {
  pub(crate) fn new<P>(path: P) -> Self
  where
    P: AsRef<Path>,
  {
    Self {
      file_path: path.as_ref().to_path_buf(),
      data: (),
    }
  }
}

impl<T> ConfigBuilder<T>
where
  T: for<'de> Deserialize<'de> + Serialize,
{
  /// set data to builder and return builder.
  pub fn data<U>(self, data: U) -> ConfigBuilder<U>
  where
    U: for<'de> Deserialize<'de> + Serialize,
  {
    ConfigBuilder {
      file_path: self.file_path,
      data,
    }
  }

  /// Building Self.
  /// # Errors
  /// This function will return an error if build failed.
  pub fn build(self) -> anyhow::Result<Config<T>> {
    let file_path = self.file_path;
    {
      let parent = file_path.parent().context("no parent")?;
      if !parent.exists() {
        create_dir_all(parent)?;
      }
    }
    if !file_path.exists() {
      File::create(&file_path)?;
    }
    if !file_path.is_file() {
      bail!("path is not file")
    }

    let mut conf = Config {
      file_path,
      config: self.data,
    };
    Config::load(&mut conf)?;

    Ok(conf)
  }
}
