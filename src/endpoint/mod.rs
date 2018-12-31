pub mod method;

use self::method::{HttpVerb::*, Method};

#[derive(Debug)]
pub struct Endpoint {
  uri: String,
  get: Option<Method>,
  put: Option<Method>,
  post: Option<Method>,
  patch: Option<Method>,
  delete: Option<Method>,
}

type JsonMap = serde_json::Map<String, serde_json::Value>;

impl Endpoint {
  pub fn from_paths(paths: &JsonMap) -> Vec<Endpoint> {
    let paths: Vec<Endpoint> = paths.into_iter().fold(vec![], |mut acc, (uri, value)| {
      let get = Method::from_endpoint(&value, Get);
      let put = Method::from_endpoint(&value, Put);
      let post = Method::from_endpoint(&value, Post);
      let patch = Method::from_endpoint(&value, Patch);
      let delete = Method::from_endpoint(&value, Delete);

      let endpoint = Endpoint {
        uri: uri.to_owned(),
        get,
        put,
        post,
        patch,
        delete,
      };
      acc.push(endpoint);
      acc
    });
    paths
  }
}
