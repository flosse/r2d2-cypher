// Copyright (c) 2015 Markus Kohlhase <mail@markus-kohlhase.de>

#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, unused_qualifications)]

//! [r2d2-cypher](https://github.com/flosse/r2d2-cypher) is a
//! [r2d2](https://github.com/sfackler/r2d2) connection pool for
//! [rusted-cypher](https://github.com/livioribeiro/rusted-cypher).
//!
//! # Example
//!
//! ```
//! extern crate r2d2;
//! extern crate r2d2_cypher;
//!
//! use r2d2::{Config, Pool};
//! use r2d2_cypher::CypherConnectionManager;
//!
//! pub fn main() {
//!   let db_url  = "http://neo4j:neo4j@127.0.0.1:7474/db/data".to_string();
//!   let manager = CypherConnectionManager{url:db_url};
//!   let config  = Config::builder().pool_size(5).build();
//!   let pool    = Pool::new(config, manager).unwrap();
//!   let client  = pool.clone().get().unwrap();
//!   let result  = client.cypher().exec("MATCH (n)-[r]->() RETURN n");
//! }
//! ```

extern crate r2d2;
extern crate rusted_cypher;

use rusted_cypher::GraphClient;
use rusted_cypher::error::GraphError;

/// A struct that holds connection specific information.
#[derive(Debug)]
pub struct CypherConnectionManager {
  /// the URL to the database
  pub url: String,
}

impl r2d2::ManageConnection for CypherConnectionManager {
  type Connection = GraphClient;
  type Error = GraphError;

  fn connect(&self) -> Result<GraphClient, GraphError> {
    GraphClient::connect(&self.url)
  }

  fn is_valid(&self, conn: &mut GraphClient) -> Result<(), GraphError> {
    match conn.cypher().exec("RETURN 1") {
      Ok(_)     => Ok(()),
      Err(err)  => Err(err)
    }
  }

  fn has_broken(&self, _: &mut GraphClient) -> bool {
    false
  }
}
