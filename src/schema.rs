use serde_json::Value;

#[derive(Debug)]
pub struct Param {}

#[derive(Debug)]
pub struct Get {
  params: Option<Vec<Param>>,
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
    let mut paths: Vec<Endpoint> = vec![];
    if let Value::Object(entity) = &json["paths"] {
      for (uri, value) in entity {
        let endpoint = Endpoint {
          uri: uri.to_owned(),
          get: read_get(&value["get"]),
        };
        paths.push(endpoint);
      }
    }

    Schema {
      paths,
      models: vec![],
    }
  }
}
type JsonMap = serde_json::Map<String, Value>;

fn read_params(value: &JsonMap) -> Option<Vec<Param>> {
  match &value["parameters"] {
    Value::Array(params) => params
      .into_iter()
      .map(|item| {
        let param = Param {};
        Some(param)
      })
      .collect(),
    _ => None,
  }
}

fn read_get(value: &Value) -> Option<Get> {
  match value {
    Value::Object(get) => {
      let params = read_params(get);

      Some(Get {
        params,
        responses: (),
      })
    }
    _ => None,
  }
}
