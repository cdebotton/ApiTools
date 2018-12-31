use serde_json::Value as JsonValue;

#[derive(Debug)]
pub enum Types {
  Null,
  Number,
  Boolean,
  String,
  Enum(Vec<String>),
  Array(Box<Types>),
  Object,
}

impl Types {
  pub fn from_schema(schema: &JsonValue) -> Self {
    let type_value = schema.get("type");
    let ref_value = schema.get("$ref");
    let enum_value = schema.get("enum");

    if let Some(JsonValue::Array(enum_array)) = enum_value {
      return Types::Enum(
        enum_array
          .into_iter()
          .map(|item| item.to_string())
          .collect(),
      );
    }

    match (type_value, ref_value) {
      (Some(JsonValue::String(name)), _) if name == "string" => Types::String,
      (Some(JsonValue::String(name)), _) if name == "number" => Types::Number,
      (Some(JsonValue::String(name)), _) if name == "boolean" => Types::Boolean,
      (Some(JsonValue::String(name)), _) if name == "null" => Types::Null,
      _ => {
        println!("{:?}", ref_value);
        Types::Null
      }
    }
  }
}
