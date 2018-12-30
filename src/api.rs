use super::schema::Schema;
use hyper::{Body, Client, Request, Uri};
use hyper_tls::HttpsConnector;
use tokio::prelude::*;

pub struct Token {
  pub id: String,
  pub secret: String,
}

pub async fn fetch_schema(url: String, token: Token) -> Schema {
  let token = {
    let token = format!("{}:{}", token.id, token.secret);
    base64::encode(&token)
  };

  let authorization = format!("basic {}", token);
  let uri: Uri = url.parse().unwrap();
  let https = HttpsConnector::new(4).unwrap();
  let client = Client::builder().build::<_, hyper::Body>(https);
  let request = Request::get(uri)
    .header("Authorization", authorization)
    .body(Body::empty())
    .unwrap();

  let response = await!(client.request(request)).unwrap();
  let json = await!(response.into_body().concat2()).unwrap();
  let value: serde_json::Value = serde_json::from_slice(&json).unwrap();
  Schema::new(value)
}
