use serde::*;
use serde_json::{Map, Value};
use uuid::Uuid;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GremlinResponse {
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

#[cfg(test)]
mod tests {
    use super::GremlinResponse;
    use serde_json::from_slice;

    #[test]
    fn test_response_deserialization() {
        let response_str = r#"{"requestId":"9a4cfad9-7f66-49f9-8935-7ff82bb4e5ca","status":{"message":"","code":200,"attributes":{"host":"/172.31.14.18:60944"}},"result":{"data":[{"@type":"g:Traverser","@value":{"bulk":{"@type":"g:Int64","@value":1},"value":{"id":"dac146b6-ddb4-5292-674b-0a4754e29ed1","label":"user","cred":"dc976ceebeda590aa3d38eb34473662388c236d80cbd841b573a1ec3ac66168a","meta":"{}","last_name":"coetzee","first_name":"tristan","email":"coetzeetrist@gmail.com"}}},{"@type":"g:Traverser","@value":{"bulk":{"@type":"g:Int64","@value":1},"value":{"id":"82c17f17-5785-cbf0-bd24-6a81100511d0","label":"user","cred":"dc956ceebeda590aa3d38eb34473662388c236d80cbd8461573b1ec3ac66168a","meta":"{}","last_name":"train","first_name":"a","email":"a-train@icloud.com"}}},{"@type":"g:Traverser","@value":{"bulk":{"@type":"g:Int64","@value":1},"value":{"id":"66c146d2-9582-b09e-f273-924c773c6776","label":"user","cred":"dc976ceebeda590aa3d38eb34473616398c236d80cbd465b573a1ec3ac66168a","meta":"{}","last_name":"campbell","first_name":"hughie","email":"hughiecampbell@outlook.com"}}},{"@type":"g:Traverser","@value":{"bulk":{"@type":"g:Int64","@value":1},"value":{"id":"28c146d2-9584-690d-0f86-a4b27893424a","label":"user","cred":"dc976ceebeda590aa3d38eb34473662388c236d80cbd1207573a1ec3ac66168a","meta":"{}","last_name":"neuman","first_name":"victoria","email":"vneuman@outlook.com"}}},{"@type":"g:Traverser","@value":{"bulk":{"@type":"g:Int64","@value":1},"value":{"id":"aec146d2-9585-6716-5a3e-e0cb1688a31f","label":"user","cred":"dc976ceebeda590aa3d38eb34473662388c236d80cbd8461573a1ec3ac66168a","meta":"{}","last_name":"edgar","first_name":"stan","email":"stan.edgar@gmail.com"}}},{"@type":"g:Traverser","@value":{"bulk":{"@type":"g:Int64","@value":1},"value":{"id":"72c146d2-9586-25c6-4667-4f9db3d9b4cc","label":"user","cred":"dc976ceebeda590aa3d38eb34473662388c236d80cb9368573a1ec3ac66168a","meta":"{}","last_name":"barrett","first_name":"ashley","email":"ashleyb@outlook.com"}}},{"@type":"g:Traverser","@value":{"bulk":{"@type":"g:Int64","@value":1},"value":{"id":"6cc153e2-6815-156d-8460-36640b67d4c3","label":"user","cred":"dc976ceebeda590aa3d38eb34473662388c236d80cbd8461573b1ec3ac66168a","meta":"{}","last_name":"butcher","first_name":"becca","email":"b.butcher@icloud.com"}}},{"@type":"g:Traverser","@value":{"bulk":{"@type":"g:Int64","@value":1},"value":{"id":"94c181af-3756-e195-8d47-a864324fa6d6","label":"user","cred":"dc976ceebeda590aa3d38eb34473662388c236d80cbd8461573a1ec3ac68168a","meta":"{}","last_name":"deep","first_name":"the","email":"thedeep@gmail.com","timestamp":"1662208863.9086125","profile_image":"some_s3_id","secondary_image":"another_s3_id"}}},{"@type":"g:Traverser","@value":{"bulk":{"@type":"g:Int64","@value":1},"value":{"id":"a8c18695-9b0a-f404-0e7b-d9bab44b121b","label":"user","cred":"dc976ceebeda590aa3d38eb34473662388c236d80cbd465b573a1ec3ac66168a","meta":"{}","last_name":"butcher","first_name":"william","email":"billybutcher@outlook.com"}}},{"@type":"g:Traverser","@value":{"bulk":{"@type":"g:Int64","@value":1},"value":{"id":"30c18695-9b0d-4fee-5d54-dd194dfd5d65","label":"user","cred":"dc976ceebeda590aa3d38eb37483662388c236d80cbd465b573a1ec3ac66168a","meta":"{}","last_name":"january","first_name":"annie","email":"ajanuary.gmail.com"}}}],"meta":{}}}"#;
        let response: GremlinResponse = from_slice(response_str.as_bytes()).unwrap();
        println!("{:?}", response);
    }
}
