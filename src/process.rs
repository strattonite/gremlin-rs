use serde::*;
mod anonymous;
mod traversal;
use lazy_static::*;
use std::time;
use uuid::Uuid;

use anonymous::AnonymousTraversal;
pub use traversal::*;

lazy_static! {
    pub static ref __: AnonymousTraversal = AnonymousTraversal::new();
}

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "@type", content = "@value")]
pub enum GValue {
    #[serde(rename = "g:Date")]
    Date(u64),
    #[serde(rename = "g:Double")]
    Double(f64),
    #[serde(rename = "g:Float")]
    Float(f32),
    #[serde(rename = "g:Int32")]
    Integer(i32),
    #[serde(rename = "g:Int64")]
    Long(i64),
    #[serde(rename = "g:Timestamp")]
    Timestamp(u64),
    #[serde(rename = "g:UUID")]
    UUID(Uuid),
}

impl From<time::Duration> for GValue {
    fn from(v: std::time::Duration) -> Self {
        Self::Timestamp(v.as_secs())
    }
}

impl From<time::SystemTime> for GValue {
    fn from(v: time::SystemTime) -> Self {
        Self::Date(v.duration_since(time::UNIX_EPOCH).unwrap().as_secs())
    }
}

impl From<f64> for GValue {
    fn from(v: f64) -> Self {
        Self::Double(v)
    }
}

impl From<f32> for GValue {
    fn from(v: f32) -> Self {
        Self::Float(v)
    }
}

impl From<i32> for GValue {
    fn from(v: i32) -> Self {
        Self::Integer(v)
    }
}

impl From<i64> for GValue {
    fn from(v: i64) -> Self {
        Self::Long(v)
    }
}

impl From<Uuid> for GValue {
    fn from(v: Uuid) -> Self {
        Self::UUID(v)
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Cardinality {
    List,
    Set,
    Single,
}

#[derive(Serialize, Debug, Clone)]
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

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "predicate", content = "value")]
#[serde(rename_all = "lowercase")]
pub enum Predicate {
    Eq(GValue),
    Neq(GValue),
    Lt(GValue),
    Lte(GValue),
    Gt(GValue),
    Gte(GValue),
    Inside(GValue),
    Outside(GValue),
    Between(GValue),
    Within(GValue),
    Without(GValue),
}

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "predicate", content = "value")]
#[serde(rename_all = "camelCase")]
pub enum TextPredicate {
    StartingWith(String),
    EndingWith(String),
    Containing(String),
    NotStartingWith(String),
    NotEndingWith(String),
    NotContaining(String),
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Order {
    Asc,
    Desc,
    Shuffle,
}

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "@type", content = "@value")]
pub(crate) enum Process {
    #[serde(rename = "g:Cardinality")]
    Cardinality(Cardinality),
    #[serde(rename = "g:Operator")]
    Operator(Operator),
    #[serde(rename = "g:P")]
    Predicate(Predicate),
    #[serde(rename = "g:TextP")]
    TextPredicate(TextPredicate),
    #[serde(rename = "g:Order")]
    Order(Order),
    #[serde(rename = "g:Bytecode")]
    Bytecode(Bytecode),
}

impl From<Cardinality> for Process {
    fn from(v: Cardinality) -> Self {
        Self::Cardinality(v)
    }
}

impl From<Operator> for Process {
    fn from(v: Operator) -> Self {
        Self::Operator(v)
    }
}

impl From<Predicate> for Process {
    fn from(v: Predicate) -> Self {
        Self::Predicate(v)
    }
}

impl From<TextPredicate> for Process {
    fn from(v: TextPredicate) -> Self {
        Self::TextPredicate(v)
    }
}

impl From<Order> for Process {
    fn from(v: Order) -> Self {
        Self::Order(v)
    }
}

impl From<Bytecode> for Process {
    fn from(v: Bytecode) -> Self {
        Self::Bytecode(v)
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub(crate) enum StepValue {
    String(String),
    Process(Process),
    Literal(GValue),
}

impl From<String> for StepValue {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}

impl From<&str> for StepValue {
    fn from(v: &str) -> Self {
        Self::String(v.into())
    }
}

impl From<Uuid> for StepValue {
    fn from(v: Uuid) -> Self {
        Self::String(v.to_string())
    }
}

impl From<Process> for StepValue {
    fn from(v: Process) -> Self {
        Self::Process(v)
    }
}

impl From<GValue> for StepValue {
    fn from(v: GValue) -> Self {
        Self::Literal(v)
    }
}

impl From<f32> for StepValue {
    fn from(v: f32) -> Self {
        Into::<GValue>::into(v).into()
    }
}

impl From<f64> for StepValue {
    fn from(v: f64) -> Self {
        Into::<GValue>::into(v).into()
    }
}

impl From<i32> for StepValue {
    fn from(v: i32) -> Self {
        Into::<GValue>::into(v).into()
    }
}

impl From<i64> for StepValue {
    fn from(v: i64) -> Self {
        Into::<GValue>::into(v).into()
    }
}

impl From<Traversal> for StepValue {
    fn from(v: Traversal) -> Self {
        let b: Bytecode = v.into();
        let p: Process = b.into();
        p.into()
    }
}

impl<T: Into<StepValue> + Clone> From<&T> for StepValue {
    fn from(v: &T) -> Self {
        Into::<StepValue>::into(v.clone())
    }
}

pub struct Step(Vec<StepValue>);

impl Step {
    pub(crate) fn operator(mut self, op: &str) -> Vec<StepValue> {
        self.0.insert(0, op.into());
        self.0
    }
    pub(crate) fn no_arg(op: &str) -> Vec<StepValue> {
        vec![op.into()]
    }
}

impl From<()> for Step {
    fn from(_: ()) -> Self {
        Step(Vec::new())
    }
}

impl From<&str> for Step {
    fn from(s: &str) -> Self {
        Step(vec![s.into()])
    }
}

impl From<Traversal> for Step {
    fn from(t: Traversal) -> Self {
        Step(vec![t.into()])
    }
}

macro_rules! test_macro {
    ($($T:ident),+) => {
        impl <$($T: Into<StepValue>),+> Into<Step> for ($($T,)+) {
            fn into(self) -> Step {
                let mut v = Vec::new();
                let (
                    $($T,)+
                ) = self;
                $(
                    v.push($T.into());
                )+
                Step(v)
            }
        }
    };
}
test_macro![T0];
test_macro![T0, T1];
test_macro![T0, T1, T2];
test_macro![T0, T1, T2, T3];
test_macro![T0, T1, T2, T3, T4];
test_macro![T0, T1, T2, T3, T4, T5];
test_macro![T0, T1, T2, T3, T4, T5, T6];
test_macro![T0, T1, T2, T3, T4, T5, T6, T7];
test_macro![T0, T1, T2, T3, T4, T5, T6, T7, T8];
test_macro![T0, T1, T2, T3, T4, T5, T6, T7, T8, T9];
test_macro![T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10];
test_macro![T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11];
