use crate::params::Params;

pub enum MethodType {
  Get,
  Put,
  Post,
  Patch,
  Delete,
}

#[derive(Debug)]
pub struct Method {
  params: Params,
  responses: (),
}

impl Method {
  pub fn from_endpoint(value: &serde_json::Value, method_type: MethodType) -> Option<Method> {
    let method_type = match method_type {
      MethodType::Get => "get",
      MethodType::Put => "put",
      MethodType::Post => "post",
      MethodType::Patch => "patch",
      MethodType::Delete => "delete",
    };

    match &value.get(method_type) {
      Some(serde_json::Value::Object(get)) => {
        let params = Params::from_method(get);
        Some(Method {
          params,
          responses: (),
        })
      }
      _ => None,
    }
  }
}
