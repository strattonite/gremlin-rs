pub mod de;
pub mod gson;

use serde::*;
use uuid::Uuid;

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Edge {
    pub id: Uuid,
    pub label: String,
    pub in_v_label: String,
    pub out_v_label: String,
    pub in_v: Uuid,
    pub out_v: Uuid,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Vertex {
    pub id: Uuid,
    pub label: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct VertexProperty {
    pub id: i64,
    pub label: String,
    pub value: Box<gson::GsonV2>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Property {
    pub key: String,
    pub value: Box<gson::GsonV2>,
}
