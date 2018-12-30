use super::dialect::typescript::Typescript;

#[derive(Debug)]
pub enum ParamLocation {
  Uri,
  QueryString,
}

#[derive(Debug)]
pub struct Param {
  pub description: String,
  pub name: String,
  pub nullable: bool,
  pub location: ParamLocation,
}

impl Typescript for Param {
  fn to_typescript(&self) -> String {
    let nullable = if self.nullable { "?" } else { "" };
    format!("{}: {}{};", self.name, nullable, "type")
  }
}
