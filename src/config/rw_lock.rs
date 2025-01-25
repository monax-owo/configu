use std::{
  fs::{read_to_string, File},
  io::{BufReader, BufWriter, Read, Write},
  path::Path,
  sync::RwLock,
};

use anyhow::bail;
use serde::{Deserialize, Serialize};

use crate::{Configurable, RwLockConfig};

impl<T> RwLockConfig<T>
where
  T: for<'de> Deserialize<'de> + Serialize + Default,
{
  pub fn open<P>(path: Option<P>) -> Self
  where
    P: AsRef<Path>,
  {
    Self {
      file_path: path.map(|p| p.as_ref().to_path_buf()),
      data: T::default(),
    }
  }
}

impl<T> Configurable for RwLockConfig<std::sync::RwLock<T>>
where
  T: Serialize + for<'de> Deserialize<'de>,
{
  fn save(&self) -> anyhow::Result<()> {
    if let Some(file_path) = &self.file_path {
      let mut writer = BufWriter::new(File::create(file_path)?);

      let serialized = toml::to_string::<RwLock<T>>(&self.data)?;
      writer.write_all(serialized.as_bytes())?;

      Ok(())
    } else {
      bail!("Path is not specified");
    }
  }

  // TODO:パースに失敗したらファイル名をoldにして新しいconfig fileを作る
  fn load(&mut self) -> anyhow::Result<()> {
    if let Some(file_path) = &self.file_path {
      let file = File::open(file_path)?;
      let mut reader = BufReader::new(file);

      if read_to_string(file_path)?.is_empty() {
        self.save()?;
      }

      let content = {
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;
        buf
      };

      let deserialized = toml::from_str::<T>(&content)?;
      *self.data.write().unwrap() = deserialized;

      Ok(())
    } else {
      bail!("Path is not specified");
    }
  }
}
