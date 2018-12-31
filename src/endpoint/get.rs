use crate::params::Params;

#[derive(Debug)]
pub struct Get {
  params: Params,
  responses: (),
}

impl Get {
  pub fn from_endpoint(value: &serde_json::Value) -> Option<Get> {
    match &value.get("get") {
      Some(serde_json::Value::Object(get)) => {
        let params = Params::from_method(get);
        Some(Get {
          params,
          responses: (),
        })
      }
      _ => None,
    }
  }
}
