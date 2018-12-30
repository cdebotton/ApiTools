use super::params::Params;
use serde_json::Value;

#[derive(Debug)]
pub struct Get {
  params: Params,
  responses: (),
}

#[derive(Debug)]
pub struct Endpoint {
  uri: String,
  get: Option<Get>,
}

#[derive(Debug)]
pub struct Schema {
  pub paths: Vec<Endpoint>,
  pub models: Vec<(String, Value)>,
}

impl Schema {
  pub fn new(json: Value) -> Schema {
    let paths = &json.get("paths").unwrap().as_object().unwrap();
    let paths: Vec<Endpoint> = paths.into_iter().fold(vec![], |mut acc, (uri, value)| {
      let get = match &value.get("get") {
        Some(Value::Object(get)) => read_get(get),
        _ => None,
      };

      let endpoint = Endpoint {
        uri: uri.to_owned(),
        get,
      };
      acc.push(endpoint);
      acc
    });

    Schema {
      paths,
      models: vec![],
    }
  }
}

type JsonMap = serde_json::Map<String, Value>;

fn read_get(value: &JsonMap) -> Option<Get> {
  let params = Params::from_method(&value);

  Some(Get {
    params,
    responses: (),
  })
}
