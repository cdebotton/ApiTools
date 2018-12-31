use crate::params::{Param, Params};
use crate::response::Response;
use crate::schema::Schema;
use crate::types::Types;
use std::fmt;

pub trait TypeScriptDialect {}

impl TypeScriptDialect for Schema {}
impl TypeScriptDialect for Param {}
impl TypeScriptDialect for Params {}
impl TypeScriptDialect for Response {}
impl TypeScriptDialect for Types {}

impl fmt::Display for Schema
where
  Self: TypeScriptDialect,
{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Typescript!")
  }
}

// impl fmt::Display for Types
// where
//   Param: TypescriptDialect,
// {
//   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//     let type_value = match self {
//       Types::Boolean => "boolean",
//       // Types::Enum(enum_value) => enum_value.join(" | "),
//       Types::Null => "null",
//       Types::Number => "number",
//       Types::String => "string",
//       _ => "unknown",
//     };

//     write!(f, "{}", type_value)
//   }
// }

// impl TypescriptDialect for Params {
//   fn to_typescript(&self) -> String {
//     match self {
//       Params::Contents(contents) => contents.iter().map(|param| param.to_typescript()).collect(),
//       Params::Empty => "".to_string(),
//     }
//   }
// }

// impl TypescriptDialect for Response {
//   fn to_typescript(&self) -> String {
//     match self {
//       Response::Array(responses) => "".to_string(),
//       Response::One(response) => "".to_string(),
//     }
//   }
// }
