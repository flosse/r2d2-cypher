// Copyright (c) 2015 - 2017 Markus Kohlhase <mail@markus-kohlhase.de>

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
//! [![](http://meritbadge.herokuapp.com/r2d2_cypher)](https://crates.io/crates/r2d2_cypher)
//! [![Build Status](https://travis-ci.org/flosse/r2d2-cypher.svg?branch=master)](https://travis-ci.org/flosse/r2d2-cypher)
//!
//! # Example
//!
//! ```
//! extern crate r2d2;
//! extern crate r2d2_cypher;
//!
//! use r2d2::Pool;
//! use r2d2_cypher::CypherConnectionManager;
//!
//! pub fn main() {
//!   let db_url  = "http://neo4j:neo4j@127.0.0.1:7474/db/data";
//!   let manager = CypherConnectionManager{url:db_url.to_owned()};
//!   let pool    = Pool::builder().max_size(5).build(manager).unwrap();
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
    conn.exec("RETURN 1").map(|_| ())
  }

  fn has_broken(&self, _: &mut GraphClient) -> bool {
    false
  }
}
