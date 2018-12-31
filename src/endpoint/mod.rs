pub mod method;

use self::method::{Method, MethodType::*};

#[derive(Debug)]
pub struct Endpoint {
  uri: String,
  get: Option<Method>,
}

type JsonMap = serde_json::Map<String, serde_json::Value>;

impl Endpoint {
  pub fn from_paths(paths: &JsonMap) -> Vec<Endpoint> {
    let paths: Vec<Endpoint> = paths.into_iter().fold(vec![], |mut acc, (uri, value)| {
      let get = Method::from_endpoint(&value, Get);

      let endpoint = Endpoint {
        uri: uri.to_owned(),
        get,
      };
      acc.push(endpoint);
      acc
    });
    paths
  }
}
