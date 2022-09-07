# gremlin-rs

### Not really a full gremlin driver/GLV yet, created to use as a client for AWS Neptune

## Usage
- allows for strongly typed parsing of gremlin responses (by serde_json) as opposed to enum format (gremlin_client), for weakly typed data response can be kept as serde_json::Value enum
- traversal steps with optional args can be passed &[()] to avoid type specification when not using any args
- currently no query timeout implemented, use tokio::time::timeout if desired

## Caveats
- if using a step with single u32/u64/f32/f64... argument, use a tuple e.g. g.V().sample((1u32,))

- toList works slightly differently to most GLV clients where traversal is initiated with "withRemote()",
instead client passed to traversal at execution stage, allows many traversals to use same client reference e.g. should be more performant

- all vertex and edge ID's must be uuid::Uuid parsable (could allow arbitrary strings in future...)

- g:Path data structure not yet implemented

- driver::Client recieves queries to execute and handles reponses on same (tokio::spawn) thread so for very large throughput create multiple clients for multi-core parallelism (may implement client pool in future...)

- parsing method means gremlin types are lost on deserialization and converted to serde_json::Value instead of GValue, could change in future if I implement a parser for GValue's but not currently needed


## Possible future features
1. implement serde::{Serialize, Deserialize} for GValue, allowing responses to be parsed without serde_json
2. implement g:Path
3. move to GraphSON V3 from V2
4. implement Traversal::with_remote(remote &RemoteClient)?
5. implement LocalClient?
6. implement authentication for server communication
7. create ClientPool to use multiple cores
8. implement cluster logic for Client/ClientPool e.g. all read queries to read-only endpoints
9. static query macro? (e.g. lazy_static for query serialization but uses format! to sub args in before query submission, may result in better performance)
10. macro/function for parsing string query into bytecode for submission

feature #1 would allow for non uuid::Uuid vertex/edge ID's 
feature #3 would allow for List/Set distinction and parse maps with non-string keys -> HashMap
features #7 & #8 needed for production use