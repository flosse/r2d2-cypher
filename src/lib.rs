// Copyright (c) 2015 Markus Kohlhase <mail@markus-kohlhase.de>

extern crate r2d2;
extern crate rusted_cypher;

use rusted_cypher::GraphClient;
use rusted_cypher::error::GraphError;

pub struct CypherConnectionManager { pub url: String }

impl r2d2::ManageConnection for CypherConnectionManager{
  type Connection = GraphClient;
  type Error = GraphError;

  fn connect(&self) -> Result<GraphClient, GraphError> {
    GraphClient::connect(&self.url)
  }

  fn is_valid(&self, conn: &mut GraphClient) -> Result<(), GraphError> {
    Ok(())
  }

  fn has_broken(&self, conn: &mut GraphClient) -> bool {
    false
  }
}
