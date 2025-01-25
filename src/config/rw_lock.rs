use std::{
  fs::{read_to_string, File},
  io::{BufReader, BufWriter, Read, Write},
  sync::RwLock,
};

use serde::{Deserialize, Serialize};

use crate::{Config, Configurable};

impl<T> Configurable<RwLock<()>> for Config<std::sync::RwLock<T>>
where
  T: Serialize + for<'de> Deserialize<'de>,
{
  fn save(&self) -> anyhow::Result<()> {
    let mut writer = BufWriter::new(File::create(&self.file_path)?);

    let serialized = toml::to_string::<RwLock<T>>(&self.config)?;
    writer.write_all(serialized.as_bytes())?;

    Ok(())
  }

  // TODO:パースに失敗したらファイル名をoldにして新しいconfig fileを作る
  fn load(&mut self) -> anyhow::Result<()> {
    let file = File::open(&self.file_path)?;
    let mut reader = BufReader::new(file);

    if read_to_string(&self.file_path)?.is_empty() {
      self.save()?;
    }

    let content = {
      let mut buf = String::new();
      reader.read_to_string(&mut buf)?;
      buf
    };

    let deserialized = toml::from_str::<T>(&content)?;
    *self.config.write().unwrap() = deserialized;

    Ok(())
  }
}
