use clap::{App, Arg};
use crate::api;
use std::env;

pub enum Generator {
  Typescript,
  ReasonML,
  GraphQL,
}

pub struct HttpConfig {
  pub url: &'static str,
  pub token: Option<api::Token>,
}
pub enum Source {
  Http(HttpConfig),
  File(String),
}

pub struct Config {
  pub generators: Vec<Generator>,
  pub source: Source,
}

impl Config {
  pub fn read_env() -> Self {
    let matches = App::new("openapi tools")
      .version("1.0")
      .author("Christian de Botton <christian@vimeo.com>")
      .about("Parse an Open API schema to generate types")
      .arg(
        Arg::with_name("generators")
          .short("g")
          .long("generate")
          .value_name("GENERATORS")
          .help("Generate TypeScript typings")
          .required(true)
          .takes_value(true),
      )
      .arg(
        Arg::with_name("source")
          .short("s")
          .long("source")
          .help("OpenAPI source schema. Can be a URL or a path to a file. If left empty, reads from the .env file")
          .value_name("SOURCE")
          .takes_value(true)
      )
      .get_matches();

    let generators: Vec<Generator> = matches
      .value_of("generators")
      .unwrap()
      .split(",")
      .into_iter()
      .fold(vec![], |mut generators, generator_string| {
        let generator = match generator_string {
          "typescript" => Generator::Typescript,
          "reasonml" => Generator::ReasonML,
          "graphql" => Generator::ReasonML,
          _ => panic!("Unkown generator: {}", generator_string),
        };

        generators.push(generator);
        generators
      });

    let source = Source::Http(HttpConfig {
      url: dotenv!("API_URL"),
      token: Some(api::Token {
        id: dotenv!("API_ID").to_string(),
        secret: dotenv!("API_SECRET").to_string(),
      }),
    });

    Config { generators, source }
  }
}
