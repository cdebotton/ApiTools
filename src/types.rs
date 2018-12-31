use serde_json::Value as JsonValue;

#[derive(Debug)]
pub enum Types {
  Null,
  Number,
  Boolean,
  String,
  Enum,
  Array(Box<Types>),
  Object,
}

pub fn from_schema(schema: &JsonValue) -> Types {
  let type_name = schema.get("type");
  let ref_name = schema.get("$ref");

  match (type_name, ref_name) {
    (Some(JsonValue::String(name)), _) if name == "string" => Types::String,
    (Some(JsonValue::String(name)), _) if name == "number" => Types::Number,
    (Some(JsonValue::String(name)), _) if name == "boolean" => Types::Boolean,
    (Some(JsonValue::String(name)), _) if name == "null" => Types::Null,
    _ => Types::Null,
  }
}
