use serde::*;
use serde_json::Value;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Edge {
    pub id: Uuid,
    pub label: String,
    pub in_v_label: String,
    pub out_v_label: String,
    pub in_v: Uuid,
    pub out_v: Uuid,
    pub properties: Value,
}

#[derive(Deserialize, Debug)]
pub struct Vertex {
    pub id: Uuid,
    pub label: String,
    #[serde(default)]
    pub properties: Value,
}

#[derive(Deserialize, Debug)]
pub struct VertexProperty<T> {
    pub id: i64,
    pub label: String,
    pub value: T,
    pub vertex: Vertex,
}

#[derive(Deserialize, Debug)]
pub struct Property<T, E> {
    pub key: String,
    pub value: T,
    pub element: E,
}
