#![feature(await_macro, async_await, futures_api)]

#[macro_use]
extern crate tokio;
extern crate hyper;
extern crate hyper_tls;
extern crate serde_json;
extern crate base64;

#[macro_use]
extern crate dotenv_codegen;

mod api;
mod schema;
mod params;
mod dialect;

pub fn main() {
    tokio::run_async(async move {
        let url = dotenv!("API_URL").to_string();
        let token = api::Token {
            id: dotenv!("API_ID").to_string(),
            secret: dotenv!("API_SECRET").to_string(),
        };

        let schema = await!(api::fetch_schema(url, token));

        println!("{:?}", schema.paths);
    });
}
