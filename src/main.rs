#![feature(await_macro, async_await, futures_api)]

#[macro_use]
extern crate tokio;
extern crate hyper;
extern crate hyper_tls;
extern crate serde_json;
extern crate base64;
extern crate clap;

#[macro_use]
extern crate dotenv_codegen;

mod api;
mod schema;
mod params;
mod dialect;
mod types;
mod response;
mod endpoint;
mod config;

use crate::config::{Config, Source as ConfigSource, Generator};
use crate::dialect::typescript::TypescriptDialect;
use crate::dialect::graphql::GraphQLDialect;

pub fn main() {
    let config = Config::read_env();

    tokio::run_async(async move {
        let schema = match config.source {
            ConfigSource::Http(config) => {
                await!(api::fetch_schema(config.url, config.token.unwrap()))
            },
            _ => panic!("Unsupported source type"),
        };

        for generator in config.generators {
            match generator {
                Generator::Typescript => {
                    println!("{}", &schema.to_typescript());
                },
                Generator::GraphQL => {
                    println!("{}", &schema.to_graphql());
                },
                _ => panic!("Unsupported output")
            }
        }
    });
}
