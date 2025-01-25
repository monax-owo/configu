mod rw_lock;

use std::{
  fs::{read_to_string, File},
  io::{BufReader, BufWriter, Read, Write},
  ops::{Deref, DerefMut},
  path::Path,
};

use anyhow::bail;
use serde::{Deserialize, Serialize};

use crate::{Config, Configurable};

impl<T> Config<T>
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

impl<T> Configurable for Config<T>
where
  T: for<'de> Deserialize<'de> + Serialize + Default,
{
  fn save(&self) -> anyhow::Result<()> {
    if let Some(file_path) = &self.file_path {
      let mut writer = BufWriter::new(File::create(file_path)?);

      let serialized = toml::to_string(&self.data)?;
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

      if read_to_string(file_path)?.trim().is_empty() {
        self.save()?;
      }

      let content = {
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;
        buf
      };

      let deserialized = toml::from_str::<T>(&content)?;
      self.data = deserialized;

      Ok(())
    } else {
      bail!("Path is not specified");
    }
  }
}

impl<T> Deref for Config<T>
where
  T: for<'de> Deserialize<'de> + Serialize,
{
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.data
  }
}

impl<T> DerefMut for Config<T>
where
  T: for<'de> Deserialize<'de> + Serialize,
{
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.data
  }
}
