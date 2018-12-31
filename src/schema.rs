use crate::endpoint::Endpoint;
use serde_json::Value;

#[derive(Debug)]
pub struct Schema {
  pub paths: Vec<Endpoint>,
  pub models: Vec<(String, Value)>,
}

impl Schema {
  pub fn new(json: Value) -> Schema {
    let paths = &json.get("paths").unwrap().as_object().unwrap();
    let paths = Endpoint::from_paths(paths);

    Schema {
      paths,
      models: vec![],
    }
  }
}
