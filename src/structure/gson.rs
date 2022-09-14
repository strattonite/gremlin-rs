use super::*;
use crate::process::*;
use serde::{de::*, ser::*};
use std::{collections::HashMap, fmt::Debug, marker::PhantomData, time};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "@type", content = "@value")]
pub enum GsonNumber {
    #[serde(rename = "g:Double")]
    Double(f64),
    #[serde(rename = "g:Float")]
    Float(f32),
    #[serde(rename = "g:Int32")]
    Integer(i32),
    #[serde(rename = "g:Int64")]
    Long(i64),
}

impl From<f64> for GsonNumber {
    fn from(f: f64) -> Self {
        Self::Double(f)
    }
}

impl From<f32> for GsonNumber {
    fn from(f: f32) -> Self {
        Self::Float(f)
    }
}

impl From<i64> for GsonNumber {
    fn from(f: i64) -> Self {
        Self::Long(f)
    }
}

impl From<i32> for GsonNumber {
    fn from(f: i32) -> Self {
        Self::Integer(f)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "@type", content = "@value")]
pub enum GsonProcess {
    #[serde(rename = "g:Cardinality")]
    Cardinality(Cardinality),
    #[serde(rename = "g:Operator")]
    Operator(Operator),
    #[serde(rename = "g:P")]
    Predicate(P),
    #[serde(rename = "g:TextP")]
    TextPredicate(TextP),
    #[serde(rename = "g:Order")]
    Order(Order),
    #[serde(rename = "g:Bytecode")]
    Bytecode(bytecode::Bytecode),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "@type", content = "@value")]
pub enum GsonGraph {
    #[serde(rename = "g:Edge")]
    Edge(Edge),
    #[serde(rename = "g:Vertex")]
    Vertex(Vertex),
    #[serde(rename = "g:VertexProperty")]
    VertexProperty(VertexProperty),
    #[serde(rename = "g:Property")]
    Property(Property),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GsonLiteral {
    String(String),
    Bool(bool),
    Null(()),
}

impl From<String> for GsonLiteral {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<&str> for GsonLiteral {
    fn from(s: &str) -> Self {
        Self::String(s.to_string())
    }
}

impl From<bool> for GsonLiteral {
    fn from(s: bool) -> Self {
        Self::Bool(s)
    }
}

impl From<()> for GsonLiteral {
    fn from(_: ()) -> Self {
        Self::Null(())
    }
}

#[derive(Debug, Clone)]
pub enum GsonV2 {
    Date(i64),
    Double(f64),
    Float(f32),
    Integer(i32),
    Long(i64),
    Timestamp(i64),
    UUID(Uuid),
    Edge(Edge),
    Vertex(Vertex),
    VertexProperty(VertexProperty),
    Property(Property),
    String(String),
    Bool(bool),
    Null,
    Cardinality(Cardinality),
    Operator(Operator),
    Predicate(P),
    TextPredicate(TextP),
    Order(Order),
    Bytecode(bytecode::Bytecode),
    List(Vec<GsonV2>),
    Map(HashMap<String, GsonV2>),
}

impl Serialize for GsonV2 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::String(s) => serializer.serialize_str(s),
            Self::Null => serializer.serialize_none(),
            Self::Bool(b) => serializer.serialize_bool(*b),
            Self::List(l) => {
                let mut seq = serializer.serialize_seq(Some(l.len()))?;
                for e in l.iter() {
                    seq.serialize_element(e)?;
                }
                seq.end()
            }
            Self::Map(hm) => {
                let mut map = serializer.serialize_map(Some(hm.len()))?;
                for (k, v) in hm.iter() {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            }

            _ => {
                let mut map = serializer.serialize_map(Some(2))?;
                match self {
                    Self::Date(d) => {
                        map.serialize_entry("@type", "g:Date")?;
                        map.serialize_entry("@value", d)?;
                    }
                    Self::Double(d) => {
                        map.serialize_entry("@type", "g:Double")?;
                        map.serialize_entry("@value", d)?;
                    }
                    Self::Float(d) => {
                        map.serialize_entry("@type", "g:Float")?;
                        map.serialize_entry("@value", d)?;
                    }
                    Self::Integer(d) => {
                        map.serialize_entry("@type", "g:Int32")?;
                        map.serialize_entry("@value", d)?;
                    }
                    Self::Long(d) => {
                        map.serialize_entry("@type", "g:Int64")?;
                        map.serialize_entry("@value", d)?;
                    }
                    Self::Timestamp(d) => {
                        map.serialize_entry("@type", "g:Timestamp")?;
                        map.serialize_entry("@value", d)?;
                    }
                    Self::UUID(d) => {
                        map.serialize_entry("@type", "g:UUID")?;
                        map.serialize_entry("@value", d)?;
                    }
                    Self::Edge(d) => {
                        map.serialize_entry("@type", "g:Edge")?;
                        map.serialize_entry("@value", d)?;
                    }
                    Self::Vertex(d) => {
                        map.serialize_entry("@type", "g:Vertex")?;
                        map.serialize_entry("@value", d)?;
                    }
                    Self::Property(d) => {
                        map.serialize_entry("@type", "g:Property")?;
                        map.serialize_entry("@value", d)?;
                    }
                    Self::VertexProperty(d) => {
                        map.serialize_entry("@type", "g:VertexProperty")?;
                        map.serialize_entry("@value", d)?;
                    }
                    Self::Cardinality(d) => {
                        map.serialize_entry("@type", "g:Cardinality")?;
                        map.serialize_entry("@value", d)?;
                    }
                    Self::Bytecode(d) => {
                        map.serialize_entry("@type", "g:Bytecode")?;
                        map.serialize_entry("@value", d)?;
                    }
                    Self::Predicate(d) => {
                        map.serialize_entry("@type", "g:P")?;
                        map.serialize_entry("@value", d)?;
                    }
                    Self::Order(d) => {
                        map.serialize_entry("@type", "g:Order")?;
                        map.serialize_entry("@value", d)?;
                    }
                    Self::TextPredicate(d) => {
                        map.serialize_entry("@type", "g:TextP")?;
                        map.serialize_entry("@value", d)?;
                    }
                    Self::Operator(d) => {
                        map.serialize_entry("@type", "g:Operator")?;
                        map.serialize_entry("@value", d)?;
                    }
                    Self::String(_) => panic!(),
                    Self::Bool(_) => panic!(),
                    Self::Null => panic!(),
                    Self::List(_) => panic!(),
                    Self::Map(_) => panic!(),
                }
                map.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for GsonV2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(GsonVisitor::new())
    }
}

struct GsonVisitor {
    marker: PhantomData<fn() -> GsonV2>,
}

impl GsonVisitor {
    fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

impl<'de> Visitor<'de> for GsonVisitor {
    type Value = GsonV2;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid GsonV2 value")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: std::error::Error,
    {
        Ok(GsonV2::Bool(v))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: std::error::Error,
    {
        Ok(GsonV2::String(v))
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: std::error::Error,
    {
        Ok(GsonV2::Float(v))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: std::error::Error,
    {
        Ok(GsonV2::Double(v))
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: std::error::Error,
    {
        Ok(GsonV2::Integer(v))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: std::error::Error,
    {
        Ok(GsonV2::Long(v))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: std::error::Error,
    {
        Ok(GsonV2::Null)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: std::error::Error,
    {
        Ok(GsonV2::Null)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut list = Vec::new();
        while let Some(v) = seq.next_element()? {
            list.push(v)
        }
        return Ok(GsonV2::List(list));
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        if let Some(k) = map.next_key()? {
            match k {
                "@type" => match map.next_value()? {
                    "g:Date" => {
                        if let Some(("@value", v)) = map.next_entry::<&str, i64>()? {
                            map.next_entry::<(), ()>()?;
                            return Ok(GsonV2::Date(v));
                        }
                        return Err(serde::de::Error::missing_field("@value"));
                    }
                    "g:Timestamp" => {
                        if let Some(("@value", v)) = map.next_entry::<&str, i64>()? {
                            map.next_entry::<(), ()>()?;
                            return Ok(GsonV2::Timestamp(v));
                        }
                        return Err(serde::de::Error::missing_field("@value"));
                    }
                    "g:Edge" => {
                        if let Some(("@value", v)) = map.next_entry::<&str, Edge>()? {
                            map.next_entry::<(), ()>()?;
                            return Ok(GsonV2::Edge(v));
                        }
                        return Err(serde::de::Error::missing_field("@value"));
                    }
                    "g:Vertex" => {
                        if let Some(("@value", v)) = map.next_entry::<&str, Vertex>()? {
                            map.next_entry::<(), ()>()?;
                            return Ok(GsonV2::Vertex(v));
                        }
                        return Err(serde::de::Error::missing_field("@value"));
                    }
                    "g:VertexProperty" => {
                        if let Some(("@value", v)) = map.next_entry::<&str, VertexProperty>()? {
                            map.next_entry::<(), ()>()?;
                            return Ok(GsonV2::VertexProperty(v));
                        }
                        return Err(serde::de::Error::missing_field("@value"));
                    }
                    "g:Cardinality" => {
                        if let Some(("@value", v)) = map.next_entry::<&str, Cardinality>()? {
                            map.next_entry::<(), ()>()?;
                            return Ok(GsonV2::Cardinality(v));
                        }
                        return Err(serde::de::Error::missing_field("@value"));
                    }
                    "g:Operator" => {
                        if let Some(("@value", v)) = map.next_entry::<&str, Operator>()? {
                            map.next_entry::<(), ()>()?;
                            return Ok(GsonV2::Operator(v));
                        }
                        return Err(serde::de::Error::missing_field("@value"));
                    }
                    "g:P" => {
                        if let Some(("@value", v)) = map.next_entry::<&str, P>()? {
                            map.next_entry::<(), ()>()?;
                            return Ok(GsonV2::Predicate(v));
                        }
                        return Err(serde::de::Error::missing_field("@value"));
                    }
                    "g:TextP" => {
                        if let Some(("@value", v)) = map.next_entry::<&str, TextP>()? {
                            map.next_entry::<(), ()>()?;
                            return Ok(GsonV2::TextPredicate(v));
                        }
                        return Err(serde::de::Error::missing_field("@value"));
                    }
                    "g:Order" => {
                        if let Some(("@value", v)) = map.next_entry::<&str, Order>()? {
                            map.next_entry::<(), ()>()?;
                            return Ok(GsonV2::Order(v));
                        }
                        return Err(serde::de::Error::missing_field("@value"));
                    }
                    "g:Bytecode" => {
                        if let Some(("@value", v)) = map.next_entry::<&str, bytecode::Bytecode>()? {
                            map.next_entry::<(), ()>()?;
                            return Ok(GsonV2::Bytecode(v));
                        }
                        return Err(serde::de::Error::missing_field("@value"));
                    }
                    "g:UUID" => {
                        if let Some(("@value", v)) = map.next_entry::<&str, Uuid>()? {
                            map.next_entry::<(), ()>()?;
                            return Ok(GsonV2::UUID(v));
                        }
                        return Err(serde::de::Error::missing_field("@value"));
                    }
                    "g:Property" => {
                        if let Some(("@value", v)) = map.next_entry::<&str, Property>()? {
                            map.next_entry::<(), ()>()?;
                            return Ok(GsonV2::Property(v));
                        }
                        return Err(serde::de::Error::missing_field("@value"));
                    }
                    x => {
                        return Err(serde::de::Error::invalid_value(
                            Unexpected::Str(x),
                            &"g:Identifier",
                        ))
                    }
                },
                x => {
                    let mut hm = HashMap::new();
                    let v: GsonV2 = map.next_value()?;
                    hm.insert(x.to_string(), v);
                    while let Some((k, v)) = map.next_entry()? {
                        hm.insert(k, v);
                    }
                    return Ok(GsonV2::Map(hm));
                }
            }
        } else {
            return Ok(GsonV2::Map(HashMap::new()));
        }
    }
}

impl From<time::Duration> for GsonV2 {
    fn from(v: std::time::Duration) -> Self {
        Self::Timestamp(v.as_millis() as i64)
    }
}

impl From<time::SystemTime> for GsonV2 {
    fn from(v: time::SystemTime) -> Self {
        Self::Date(v.duration_since(time::UNIX_EPOCH).unwrap().as_millis() as i64)
    }
}

impl From<f64> for GsonV2 {
    fn from(v: f64) -> Self {
        Self::Double(v)
    }
}

impl From<f32> for GsonV2 {
    fn from(v: f32) -> Self {
        Self::Float(v)
    }
}

impl From<i32> for GsonV2 {
    fn from(v: i32) -> Self {
        Self::Integer(v)
    }
}

impl From<i64> for GsonV2 {
    fn from(v: i64) -> Self {
        Self::Long(v)
    }
}

impl From<Uuid> for GsonV2 {
    fn from(v: Uuid) -> Self {
        Self::UUID(v)
    }
}

impl From<Vertex> for GsonV2 {
    fn from(v: Vertex) -> Self {
        Self::Vertex(v)
    }
}

impl From<VertexProperty> for GsonV2 {
    fn from(v: VertexProperty) -> Self {
        Self::VertexProperty(v)
    }
}

impl From<Property> for GsonV2 {
    fn from(v: Property) -> Self {
        Self::Property(v)
    }
}

impl From<String> for GsonV2 {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<&str> for GsonV2 {
    fn from(s: &str) -> Self {
        Self::String(s.to_string())
    }
}

impl From<bool> for GsonV2 {
    fn from(s: bool) -> Self {
        Self::Bool(s)
    }
}

impl From<Option<()>> for GsonV2 {
    fn from(_: Option<()>) -> Self {
        Self::Null
    }
}

impl From<Cardinality> for GsonV2 {
    fn from(c: Cardinality) -> Self {
        Self::Cardinality(c)
    }
}

impl From<Operator> for GsonV2 {
    fn from(c: Operator) -> Self {
        Self::Operator(c)
    }
}

impl From<Order> for GsonV2 {
    fn from(c: Order) -> Self {
        Self::Order(c)
    }
}

impl From<P> for GsonV2 {
    fn from(p: P) -> Self {
        Self::Predicate(p)
    }
}

impl From<TextP> for GsonV2 {
    fn from(p: TextP) -> Self {
        Self::TextPredicate(p)
    }
}

impl From<bytecode::Bytecode> for GsonV2 {
    fn from(b: bytecode::Bytecode) -> Self {
        Self::Bytecode(b)
    }
}

impl From<Traversal> for GsonV2 {
    fn from(t: Traversal) -> Self {
        let b: bytecode::Bytecode = t.into();
        b.into()
    }
}
