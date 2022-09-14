# gremlin-rs

## Usage
- gremlin remote driver/client for rust
- allows for both strongly and weakly typed parsing of query responses, types implementing serde::Deserialize can be parsed, for untyped results parse into the GsonV2 enum
- ClientPool struct provided to allow for cluster logic and more scaleable concurrency, if querying a cluster (e.g. Neptune) it is recommended to use one read client per read-only server to ensure equal distribution of queries. 

## "features"
- only supports GsonV2
- only GLV queries supported (no string based)
- toList works slightly differently to most GLV clients where traversal is initiated with "withRemote()",
instead client passed to traversal at execution stage, allows many traversals to use same client reference
- driver::Client recieves queries to execute and handles reponses on same (tokio) thread so for very large throughput use ClientPool

## Possible future features
3. move to GraphSON V3 from V2?
5. implement LocalClient?
6. implement authentication for server communication?
10. macro/function for parsing string query into bytecode for submission?
11. string query submission for client?

feature #3 would allow for List/Set distinction and parse maps with non-string keys