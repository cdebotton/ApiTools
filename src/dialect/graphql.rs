use crate::schema::Schema;

pub trait GraphQLDialect {
  fn to_graphql(&self) -> &'static str;
}

impl GraphQLDialect for Schema {
  fn to_graphql(&self) -> &'static str {
    "GraphQL!"
  }
}
