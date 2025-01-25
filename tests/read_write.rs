use std::path::Path;

use configu::Config;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Data {}

#[test]
fn test() {
  let config = Config::<Data>::open(Path::new("../tests/test.toml"))
    .data(Data {})
    .build();
}
