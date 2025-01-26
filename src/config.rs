use std::{
  fs::{read_to_string, File},
  io::{BufReader, BufWriter, Read, Write},
  ops::{Deref, DerefMut},
  path::Path,
};

use serde::{Deserialize, Serialize};

use crate::{Config, Configurable, Error, Result};

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
  fn save(&self) -> Result<()> {
    if let Some(file_path) = &self.file_path {
      let mut writer = BufWriter::new(File::create(file_path)?);

      let serialized = toml::to_string(&self.data).map_err(|e| Error::Serialize(e.to_string()))?;
      writer.write_all(serialized.as_bytes())?;
      Ok(())
    } else {
      Err(Error::PathNotSpecified)
    }
  }

  // TODO:パースに失敗したらファイル名をoldにして新しいconfig fileを作る
  fn load(&mut self) -> Result<()> {
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

      let deserialized = toml::from_str(&content).map_err(|e| Error::Deserialize(e.to_string()))?;
      self.data = deserialized;

      Ok(())
    } else {
      Err(Error::PathNotSpecified)
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
