---
title: "libapi"
ref: "src/libapi.rs"
---
# Libapi
can be included in other projects, exports enum API
with the following members: 

| Member                     | Description                           |
| -------------------------- | ------------------------------------- |
| API::Query(query, api_key) | Queries the Gemini AI                 |
| API::Connection(url)       | Opens A Connection to a KnowledgeBase |

an example usage of this where connection is WIP.
however Query will create a Gemini AI Query and return the
response.

```rust
async fn main() {
		let kdb = API::Connect("/test".to_string());
		let res = kdb.call().await;
		let _connection = &res.connection().unwrap();
		let ai = API::Query("request".to_string(),"Api Key".to_string());
		let res = ai.call().await;
		let msg = &res.response().unwrap();
		println!("Response from Gemini {}",msg);
}
```

