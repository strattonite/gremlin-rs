use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::*;
use crate::{process::bytecode, structure::de::*};

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GremlinResponse<T> {
    pub request_id: Option<Uuid>,
    pub status: ResponseStatus,
    pub result: ResponseResult<T>,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseHeader {
    pub request_id: Option<Uuid>,
    pub status: ResponseStatus,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ResponseResult<T> {
    pub data: Option<Vec<T>>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ResponseStatus {
    pub message: String,
    pub code: usize,
}

impl ClientResponse {
    pub fn parse<'de, T: Deserialize<'de>>(&'de self) -> GResult<Vec<T>> {
        let mut v = Vec::new();
        for r in self.0.iter() {
            let rp = from_vec::<GremlinResponse<T>>(r)?;
            if let Some(mut d) = rp.result.data {
                v.append(&mut d);
            }
        }
        Ok(v)
    }
}

pub fn parse_response_header(data: &Vec<u8>) -> GResult<ResponseHeader> {
    let mut de = Deserializer::from_vec(data);

    ResponseHeader::deserialize(&mut de)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "@type", content = "@value")]
pub(crate) enum ReqEnum {
    #[serde(rename = "g:UUID")]
    Uuid(Uuid),
    #[serde(rename = "g:UUID")]
    Bytecode(bytecode::Bytecode),
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GremlinRequest {
    pub(crate) request_id: ReqEnum,
    pub(crate) op: &'static str,
    pub(crate) processor: &'static str,
    pub(crate) args: RequestArgs,
}

impl GremlinRequest {
    pub fn new(b: bytecode::Bytecode) -> (Uuid, Self) {
        let u = Uuid::new_v4();
        (
            u.clone(),
            Self {
                request_id: ReqEnum::Uuid(u),
                op: "bytecode",
                processor: "traversal",
                args: RequestArgs {
                    gremlin: ReqEnum::Bytecode(b),
                    aliases: RequestAliases { g: "g" },
                },
            },
        )
    }
}

#[derive(Serialize, Debug)]
pub(crate) struct RequestArgs {
    pub(crate) gremlin: ReqEnum,
    pub(crate) aliases: RequestAliases,
}

#[derive(Serialize, Debug)]
pub(crate) struct RequestAliases {
    pub(crate) g: &'static str,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structure::gson::*;
    use serde_json::to_string_pretty;

    #[test]
    fn request_serialization() {
        let bytecode = bytecode::Bytecode::new();
        let (_, req) = GremlinRequest::new(bytecode);
        println!("{}", to_string_pretty(&req).unwrap());
    }

    #[test]
    fn response_deserialization() {
        let null_data = r#"{"requestId":"b65e6f64-a839-4c3f-a33b-047d9798f94a","status":{"message":"","code":204,"attributes":{"host":"/172.31.14.18:55854"}},"result":{"data":null,"meta":{}}}"#;
        let map_data = r#"{"requestId":"f6180536-dcd2-460f-ba04-e59549a466cd","status":{"message":"","code":200,"attributes":{"host":"/172.31.14.18:57596"}},"result":{"data":[{"@type":"g:Traverser","@value":{"bulk":{"@type":"g:Int64","@value":1},"value":{"id":"1ac193d0-ff14-cfe0-404d-2b0e38ec8619","label":"user","cred":"a3c78b86c2680abf","meta":"{}","last_name":"butcher","first_name":"billy","email":"billy-butcher@gmail.com","timestamp":{"@type":"g:Double","@value":1.6628172714206238E9},"profile_image":"some-s3-url-260abb2b","secondary_image":"some-s3-url-c5c93ff1"}}},{"@type":"g:Traverser","@value":{"bulk":{"@type":"g:Int64","@value":1},"value":{"id":"88c193d0-ff26-8435-65fe-893619454acf","label":"user","cred":"8aa4ac62f7f769fb","meta":"{}","last_name":"edgar","first_name":"stan","email":"stan-edgar@gmail.com","timestamp":{"@type":"g:Double","@value":1.6628172714867363E9},"profile_image":"some-s3-url-3deae8ca","secondary_image":"some-s3-url-ef131cc2"}}}],"meta":{}}}"#;
        let vtx_data = r#"{"requestId":"bb0fcb1e-f51e-47a1-8139-d35f5fbe44ef","status":{"message":"","code":200,"attributes":{"host":"/172.31.14.18:37326"}},"result":{"data":[{"@type":"g:Traverser","@value":{"bulk":{"@type":"g:Int64","@value":1},"value":{"@type":"g:Vertex","@value":{"id":"1ac193d0-ff14-cfe0-404d-2b0e38ec8619","label":"user"}}}},{"@type":"g:Traverser","@value":{"bulk":{"@type":"g:Int64","@value":1},"value":{"@type":"g:Vertex","@value":{"id":"88c193d0-ff26-8435-65fe-893619454acf","label":"user"}}}}],"meta":{}}}"#;

        let header = parse_response_header(&null_data.as_bytes().to_vec()).unwrap();

        println!("parsed header:\n{}", to_string_pretty(&header).unwrap());

        #[derive(Deserialize, Debug, Serialize)]
        struct MapData {
            email: String,
            first_name: String,
            last_name: String,
            cred: String,
            timestamp: f64,
            profile_image: String,
            secondary_image: String,
        }

        let null_parsed = from_str::<GremlinResponse<()>>(null_data).unwrap();

        println!(
            "null data parsed:\n{}",
            to_string_pretty(&null_parsed).unwrap()
        );

        let map_parsed = from_str::<GremlinResponse<MapData>>(map_data).unwrap();

        println!(
            "map data parsed:\n{}",
            to_string_pretty(&map_parsed).unwrap()
        );

        let vtx_parsed = from_str::<GremlinResponse<GsonV2>>(vtx_data).unwrap();

        println!(
            "vtx data parsed:\n{}",
            to_string_pretty(&vtx_parsed).unwrap()
        );
    }
}
