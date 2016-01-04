# r2d2_cypher

[r2d2-cypher](https://github.com/flosse/r2d2-cypher) is a
[r2d2](https://github.com/sfackler/r2d2) connection pool for
[rusted-cypher](https://github.com/livioribeiro/rusted-cypher).

## Example

```rust
extern crate r2d2;
extern crate r2d2_cypher;

use r2d2::{Config, Pool};
use r2d2_cypher::CypherConnectionManager;

pub fn main() {
  let db_url  = "http://neo4j:neo4j@127.0.0.1:7474/db/data";
  let manager = CypherConnectionManager{url:db_url.to_owned()};
  let config  = Config::builder().pool_size(5).build();
  let pool    = Pool::new(config, manager).unwrap();
  let client  = pool.clone().get().unwrap();
  let result  = client.cypher().exec("MATCH (n)-[r]->() RETURN n");
}
```

## License

This project is licensed under the MIT license.
