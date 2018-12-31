mod get;

use self::get::Get;

#[derive(Debug)]
pub struct Endpoint {
  uri: String,
  get: Option<Get>,
}

type JsonMap = serde_json::Map<String, serde_json::Value>;

impl Endpoint {
  pub fn from_paths(paths: &JsonMap) -> Vec<Endpoint> {
    let paths: Vec<Endpoint> = paths.into_iter().fold(vec![], |mut acc, (uri, value)| {
      let get = Get::from_endpoint(&value);

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
