use std::{
  fs::{read_to_string, File},
  io::{BufReader, BufWriter, Read, Write},
  path::Path,
  sync::RwLock,
};

use serde::{Deserialize, Serialize};

use crate::{Configurable, RwLockConfig};

impl<T> RwLockConfig<T>
where
  T: for<'de> Deserialize<'de> + Serialize,
{
  pub fn open<P>(path: P, data: T) -> Self
  where
    P: AsRef<Path>,
  {
    Self {
      file: path.as_ref().to_path_buf(),
      data,
    }
  }
}

impl<T> Configurable for RwLockConfig<std::sync::RwLock<T>>
where
  T: Serialize + for<'de> Deserialize<'de>,
{
  fn save(&self) -> anyhow::Result<()> {
    let mut writer = BufWriter::new(File::create(&self.file)?);

    let serialized = toml::to_string::<RwLock<T>>(&self.data)?;
    writer.write_all(serialized.as_bytes())?;

    Ok(())
  }

  // TODO:パースに失敗したらファイル名をoldにして新しいconfig fileを作る
  fn load(&mut self) -> anyhow::Result<()> {
    let file = File::open(&self.file)?;
    let mut reader = BufReader::new(file);

    if read_to_string(&self.file)?.is_empty() {
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
  }
}
