use configu::{Config, Configurable};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
struct Data {
  value: i32,
}

#[test]
fn config() {
  let path = std::env::current_dir().unwrap().join("tests/config.toml");
  let mut config = Config::open(Some(path));

  println!("data: {:#?}", *config);

  *config = Data { value: 1 };

  println!("data: {:#?}", *config);

  config.save().unwrap();
}
