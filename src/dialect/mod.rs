pub mod graphql;
pub mod reasonml;
pub mod typescript;

pub trait Translate {
  fn to_dialect(&self) {}
}
