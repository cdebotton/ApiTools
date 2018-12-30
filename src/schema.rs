use super::params::{Param, ParamLocation};
use serde_json::Value;

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
        let get = match &value.get("get") {
          Some(Value::Object(get)) => read_get(get),
          _ => None,
        };

        let endpoint = Endpoint {
          uri: uri.to_owned(),
          get,
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
  match &value.get("parameters") {
    Some(Value::Array(params)) => params
      .into_iter()
      .map(|item| {
        let nullable = if let Value::Bool(required) = item.get("required").unwrap() {
          *required != true
        } else {
          true
        };

        let location = match item.get("in").unwrap() {
          Value::String(string) if string == "path" => ParamLocation::Uri,
          Value::String(string) if string == "query" => ParamLocation::QueryString,
          _ => panic!("Invalid path location"),
        };

        let param = Param {
          name: item.get("name").unwrap().to_string(),
          description: item.get("description").unwrap().to_string(),
          nullable,
          location,
        };
        Some(param)
      })
      .collect(),
    _ => None,
  }
}

fn read_get(value: &JsonMap) -> Option<Get> {
  let params = read_params(value);

  Some(Get {
    params,
    responses: (),
  })
}
