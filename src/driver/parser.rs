use serde::*;
use serde_json::{Map, Value};
use uuid::Uuid;

#[derive(Deserialize, Debug)]

pub struct GremlinResponse {
    #[serde(rename(deserialize = "camelCase"))]
    pub request_id: Uuid,
    pub status: ResponseStatus,
    pub result: ResponseResult,
}

#[derive(Deserialize, Debug)]
pub struct ResponseResult {
    pub data: Option<Vec<Value>>,
    pub meta: Value,
}

#[derive(Deserialize, Debug)]
pub struct ResponseStatus {
    pub message: String,
    pub code: usize,
}

pub fn unroll(v: Value) -> Value {
    match v {
        Value::Object(mut obj) => {
            if let Some(Value::String(s)) = obj.get("@type") {
                match s.as_str() {
                    "g:Traverser" => {
                        if let Some(Value::Object(mut o)) = obj.remove("@value") {
                            return unroll(o.remove("value").unwrap_or(Value::Null));
                        }
                        return Value::Null;
                    }
                    _ => unroll(obj.remove("@value").unwrap_or(Value::Null)),
                }
            } else {
                let mut map = Map::new();
                obj.into_iter().for_each(|(k, v)| {
                    map.insert(k, unroll(v));
                });
                Value::Object(map)
            }
        }
        Value::Array(arr) => Value::Array(arr.into_iter().map(unroll).collect()),
        _ => v,
    }
}
