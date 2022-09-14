use crate::structure::gson::GsonV2;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bytecode {
    step: Vec<Vec<GsonV2>>,
}

impl Bytecode {
    pub fn new() -> Self {
        Self { step: Vec::new() }
    }

    pub fn add_step<T: Into<BytecodeStep>>(&mut self, op: &str, args: T) {
        let mut step: BytecodeStep = args.into();
        step.0.insert(0, op.into());
        self.step.push(step.0)
    }

    pub fn no_arg_step(&mut self, op: &str) {
        self.step.push(vec![op.into()])
    }

    pub fn is_mutating(&self) -> bool {
        todo!()
    }
}

pub struct BytecodeStep(pub Vec<GsonV2>);

impl<T: Into<GsonV2> + Debug + Clone> From<T> for BytecodeStep {
    fn from(a: T) -> Self {
        Self(vec![a.into()])
    }
}

impl<T: Into<GsonV2> + Debug + Clone> From<Vec<T>> for BytecodeStep {
    fn from(a: Vec<T>) -> Self {
        Self(a.into_iter().map(|e| e.into()).collect())
    }
}

impl From<()> for BytecodeStep {
    fn from(_: ()) -> Self {
        Self(vec![])
    }
}

macro_rules! tuple_impl {
    ($($T:ident),+) => {
        impl <$($T: Into<GsonV2>),+> Into<BytecodeStep> for ($($T,)+) {
            fn into(self) -> BytecodeStep {
                let mut v = Vec::new();
                let (
                    $($T,)+
                ) = self;
                $(
                    v.push($T.into());
                )+
                BytecodeStep(v)
            }
        }
    };
}
tuple_impl![T0];
tuple_impl![T0, T1];
tuple_impl![T0, T1, T2];
tuple_impl![T0, T1, T2, T3];
tuple_impl![T0, T1, T2, T3, T4];
tuple_impl![T0, T1, T2, T3, T4, T5];
tuple_impl![T0, T1, T2, T3, T4, T5, T6];
tuple_impl![T0, T1, T2, T3, T4, T5, T6, T7];
tuple_impl![T0, T1, T2, T3, T4, T5, T6, T7, T8];
tuple_impl![T0, T1, T2, T3, T4, T5, T6, T7, T8, T9];
tuple_impl![T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10];
tuple_impl![T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11];
