use super::dialect::typescript::Typescript;
use serde_json::Value;

#[derive(Debug)]
pub enum ParamLocation {
  Uri,
  QueryString,
}

#[derive(Debug)]
pub struct Param {
  pub description: String,
  pub name: String,
  pub nullable: bool,
  pub location: ParamLocation,
}

impl Typescript for Param {
  fn to_typescript(&self) -> String {
    let nullable = if self.nullable { "?" } else { "" };
    format!("{}: {}{};", self.name, nullable, "type")
  }
}

#[derive(Debug)]
pub enum Params {
  Contents(Vec<Param>),
  Empty,
}

type JsonMap = serde_json::Map<String, Value>;

impl Params {
  pub fn from_method(value: &JsonMap) -> Params {
    match &value.get("parameters") {
      Some(Value::Array(params)) => {
        let contents: Vec<Param> = params
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

            Param {
              name: item.get("name").unwrap().to_string(),
              description: item.get("description").unwrap().to_string(),
              nullable,
              location,
            }
          })
          .collect();
        Params::Contents(contents)
      }
      _ => Params::Empty,
    }
  }
}