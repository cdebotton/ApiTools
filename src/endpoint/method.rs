use crate::params::Params;

pub enum HttpVerb {
  Get,
  Put,
  Post,
  Patch,
  Delete,
}

impl HttpVerb {
  fn to_string(self) -> &'static str {
    match self {
      HttpVerb::Get => "get",
      HttpVerb::Put => "put",
      HttpVerb::Post => "post",
      HttpVerb::Patch => "patch",
      HttpVerb::Delete => "delete",
    }
  }
}

#[derive(Debug)]
pub(in crate::endpoint) struct Method {
  params: Params,
  responses: (),
}

impl Method {
  pub(in crate::endpoint) fn from_endpoint(
    value: &serde_json::Value,
    http_verb: HttpVerb,
  ) -> Option<Method> {
    match &value.get(http_verb.to_string()) {
      Some(serde_json::Value::Object(method)) => {
        let params = Params::from_method(method);
        Some(Method {
          params,
          responses: (),
        })
      }
      _ => None,
    }
  }
}
