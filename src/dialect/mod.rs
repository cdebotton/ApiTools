pub mod graphql;
pub mod reasonml;
pub mod typescript;

pub trait Dialect {
  fn to_dialect<D>(&self) {}
}
