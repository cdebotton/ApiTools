use serde_json::Value as JsonValue;

type Model = JsonValue;

pub enum Response {
  Array(Vec<Model>),
  One(Model),
}
