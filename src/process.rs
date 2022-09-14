use serde::*;
mod anonymous;
mod traversal;
use lazy_static::*;
pub mod bytecode;
mod source;

use anonymous::AnonymousTraversal;
use source::TraversalSource;
pub use traversal::*;

use crate::structure::gson::*;

lazy_static! {
    pub static ref __: AnonymousTraversal = AnonymousTraversal::new();
    pub static ref g: TraversalSource = TraversalSource::new();
}

#[derive(Serialize, Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Cardinality {
    List,
    Set,
    Single,
}

#[derive(Serialize, Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Operator {
    Abs,
    Acos,
    Asin,
    Atan,
    Cbrt,
    Ceil,
    Cos,
    Cosh,
    Exp,
    Floor,
    Log,
    Log10,
    Log2,
    Sin,
    Sinh,
    Sqrt,
    Tan,
    Tanh,
    Signum,
}

#[derive(Serialize, Debug, Clone, Deserialize)]
#[serde(tag = "predicate", content = "value")]
#[serde(rename_all = "lowercase")]
pub enum P {
    Eq(GsonNumber),
    Neq(GsonNumber),
    Lt(GsonNumber),
    Lte(GsonNumber),
    Gt(GsonNumber),
    Gte(GsonNumber),
    Inside(RangeInput),
    Outside(RangeInput),
    Between(RangeInput),
    // Within(Vec<GsonV2>),
    // Without(Vec<GsonV2>),
}

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct RangeInput((GsonNumber, GsonNumber));

impl From<(f64, f64)> for RangeInput {
    fn from(f: (f64, f64)) -> Self {
        Self((f.0.into(), f.1.into()))
    }
}

impl From<(f32, f32)> for RangeInput {
    fn from(f: (f32, f32)) -> Self {
        Self((f.0.into(), f.1.into()))
    }
}

impl From<(i64, i64)> for RangeInput {
    fn from(f: (i64, i64)) -> Self {
        Self((f.0.into(), f.1.into()))
    }
}

impl From<(i32, i32)> for RangeInput {
    fn from(f: (i32, i32)) -> Self {
        Self((f.0.into(), f.1.into()))
    }
}

#[derive(Serialize, Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
#[serde(tag = "predicate", content = "value")]
#[serde(rename_all = "camelCase")]
pub enum TextP {
    StartingWith(String),
    EndingWith(String),
    Containing(String),
    NotStartingWith(String),
    NotEndingWith(String),
    NotContaining(String),
}

#[derive(Serialize, Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Order {
    Asc,
    Desc,
    Shuffle,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::to_string_pretty;

    #[test]
    fn build_traversal() {
        let qry = g
            .V("USER_ID")
            .hasLabel("user")
            .addE("edge_label")
            .from(__.V(()).has(("timestamp", P::Gt(1000_000.into()))))
            .property(("hello", 1.05));

        let b: bytecode::Bytecode = qry.into();
        println!("{}", to_string_pretty(&b).unwrap());
    }
}
