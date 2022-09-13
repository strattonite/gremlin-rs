use super::{
    de::{from_vec, GsonError},
    *,
};
use crate::process::*;
use serde::{de::*, ser::*};
use std::{fmt::Debug, marker::PhantomData, time};

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
pub enum GsonGraph<T: Debug + Clone> {
    #[serde(rename = "g:Edge")]
    Edge(Edge),
    #[serde(rename = "g:Vertex")]
    Vertex(Vertex),
    #[serde(rename = "g:VertexProperty")]
    VertexProperty(VertexProperty<T>),
    #[serde(rename = "g:Property")]
    Property(Property<T>),
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
pub enum GsonV2<T: Debug + Clone = String> {
    Date(i64),
    Double(f64),
    Float(f32),
    Integer(i32),
    Long(i64),
    Timestamp(i64),
    UUID(Uuid),
    Edge(Edge),
    Vertex(Vertex),
    VertexProperty(VertexProperty<T>),
    Property(Property<T>),
    String(String),
    Bool(bool),
    Null,
    Cardinality(Cardinality),
    Operator(Operator),
    Predicate(P),
    TextPredicate(TextP),
    Order(Order),
    Bytecode(bytecode::Bytecode),
}

impl<T: Debug + Clone + Serialize> TryInto<Vec<u8>> for GsonV2<T> {
    type Error = serde_json::Error;
    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        serde_json::to_vec(&self)
    }
}

impl<T: Debug + Clone + DeserializeOwned> TryFrom<Vec<u8>> for GsonV2<T> {
    type Error = GsonError;
    fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
        from_vec::<Self>(&v)
    }
}

impl<'de, T: Debug + Clone + Deserialize<'de>> TryFrom<&'de Vec<u8>> for GsonV2<T> {
    type Error = GsonError;
    fn try_from(v: &'de Vec<u8>) -> Result<Self, Self::Error> {
        from_vec::<Self>(&v)
    }
}

impl<T: Serialize + Debug + Clone> Serialize for GsonV2<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::String(s) => serializer.serialize_str(s),
            Self::Null => serializer.serialize_none(),
            Self::Bool(b) => serializer.serialize_bool(*b),

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
                }
                map.end()
            }
        }
    }
}

impl<'de, T: Deserialize<'de> + Debug + Clone> Deserialize<'de> for GsonV2<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(GsonVisitor::<T>::new())
    }
}

struct GsonVisitor<T: Debug + Clone> {
    marker: PhantomData<fn() -> GsonV2<T>>,
}

impl<T: Debug + Clone> GsonVisitor<T> {
    fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

impl<'de, T: Deserialize<'de> + Debug + Clone> Visitor<'de> for GsonVisitor<T> {
    type Value = GsonV2<T>;

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

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        if let Some((t, v)) = map.next_entry()? {
            match t {
                "@type" => match v {
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
                        if let Some(("@value", v)) = map.next_entry::<&str, VertexProperty<T>>()? {
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
                        if let Some(("@value", v)) = map.next_entry::<&str, Property<T>>()? {
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
                x => return Err(serde::de::Error::unknown_field(x, &["@type", "@value"])),
            }
        }
        Err(serde::de::Error::missing_field("@type"))
    }
}

impl<T: Debug + Clone> From<time::Duration> for GsonV2<T> {
    fn from(v: std::time::Duration) -> Self {
        Self::Timestamp(v.as_millis() as i64)
    }
}

impl<T: Debug + Clone> From<time::SystemTime> for GsonV2<T> {
    fn from(v: time::SystemTime) -> Self {
        Self::Date(v.duration_since(time::UNIX_EPOCH).unwrap().as_millis() as i64)
    }
}

impl<T: Debug + Clone> From<f64> for GsonV2<T> {
    fn from(v: f64) -> Self {
        Self::Double(v)
    }
}

impl<T: Debug + Clone> From<f32> for GsonV2<T> {
    fn from(v: f32) -> Self {
        Self::Float(v)
    }
}

impl<T: Debug + Clone> From<i32> for GsonV2<T> {
    fn from(v: i32) -> Self {
        Self::Integer(v)
    }
}

impl<T: Debug + Clone> From<i64> for GsonV2<T> {
    fn from(v: i64) -> Self {
        Self::Long(v)
    }
}

impl<T: Debug + Clone> From<Uuid> for GsonV2<T> {
    fn from(v: Uuid) -> Self {
        Self::UUID(v)
    }
}

impl<T: Debug + Clone> From<Vertex> for GsonV2<T> {
    fn from(v: Vertex) -> Self {
        Self::Vertex(v)
    }
}

impl<T: Debug + Clone> From<VertexProperty<T>> for GsonV2<T> {
    fn from(v: VertexProperty<T>) -> Self {
        Self::VertexProperty(v)
    }
}

impl<T: Debug + Clone> From<Property<T>> for GsonV2<T> {
    fn from(v: Property<T>) -> Self {
        Self::Property(v)
    }
}

impl<T: Debug + Clone> From<String> for GsonV2<T> {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl<T: Debug + Clone> From<&str> for GsonV2<T> {
    fn from(s: &str) -> Self {
        Self::String(s.to_string())
    }
}

impl<T: Debug + Clone> From<bool> for GsonV2<T> {
    fn from(s: bool) -> Self {
        Self::Bool(s)
    }
}

impl<T: Debug + Clone> From<Option<()>> for GsonV2<T> {
    fn from(_: Option<()>) -> Self {
        Self::Null
    }
}

impl<T: Debug + Clone> From<Cardinality> for GsonV2<T> {
    fn from(c: Cardinality) -> Self {
        Self::Cardinality(c)
    }
}

impl<T: Debug + Clone> From<Operator> for GsonV2<T> {
    fn from(c: Operator) -> Self {
        Self::Operator(c)
    }
}

impl<T: Debug + Clone> From<Order> for GsonV2<T> {
    fn from(c: Order) -> Self {
        Self::Order(c)
    }
}

impl<T: Debug + Clone> From<P> for GsonV2<T> {
    fn from(p: P) -> Self {
        Self::Predicate(p)
    }
}

impl<T: Debug + Clone> From<TextP> for GsonV2<T> {
    fn from(p: TextP) -> Self {
        Self::TextPredicate(p)
    }
}

impl<T: Debug + Clone> From<bytecode::Bytecode> for GsonV2<T> {
    fn from(b: bytecode::Bytecode) -> Self {
        Self::Bytecode(b)
    }
}

impl<T: Debug + Clone> From<Traversal> for GsonV2<T> {
    fn from(t: Traversal) -> Self {
        let b: bytecode::Bytecode = t.into();
        b.into()
    }
}
