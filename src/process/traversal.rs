use crate::driver::*;

use super::*;

#[derive(Serialize, Debug, Clone)]
pub struct Bytecode {
    steps: Vec<Vec<StepValue>>,
}

impl Bytecode {
    pub fn new() -> Self {
        Bytecode { steps: Vec::new() }
    }
    pub fn add_step(&mut self, s: Vec<StepValue>) {
        self.steps.push(s)
    }
}

#[derive(Clone)]
pub struct Traversal {
    bytecode: Bytecode,
}

impl Into<Bytecode> for Traversal {
    fn into(self) -> Bytecode {
        self.bytecode
    }
}

impl Into<Bytecode> for &mut Traversal {
    fn into(self) -> Bytecode {
        self.to_owned().bytecode
    }
}

#[allow(non_snake_case)]
impl Traversal {
    pub fn new() -> Self {
        Traversal {
            bytecode: Bytecode::new(),
        }
    }

    pub async fn execute(&mut self, client: &Client) -> Result<ClientResponse, ClientError> {
        client.execute(self.to_owned()).await
    }

    pub fn V<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "V".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn addE<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "addE".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn addV<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "addV".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn aggregate<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "aggregate".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn and<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "and".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn as_<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "as".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn barrier<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "barrier".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn both<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "both".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn bothE<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "bothE".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn bothV<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "bothV".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn branch(&mut self) -> &mut Self {
        self.bytecode.add_step(vec!["branch".into()]);
        self
    }

    pub fn by<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "by".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn call<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "call".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn cap<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "cap".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn choose<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "choose".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn coalesce<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "coalesce".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn coin<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "coin".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn connectedComponent<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "connectedComponent".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn constant<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "constant".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn count(&mut self) -> &mut Self {
        self.bytecode.add_step(vec!["count".into()]);
        self
    }

    pub fn cyclicPath<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "cyclicPath".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn dedup<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "dedup".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn drop(&mut self) -> &mut Self {
        self.bytecode.add_step(vec!["drop".into()]);
        self
    }

    pub fn element(&mut self) -> &mut Self {
        self.bytecode.add_step(vec!["element".into()]);
        self
    }

    pub fn elementMap<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "elementMap".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn emit<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "emit".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn fail(&mut self) -> &mut Self {
        self.bytecode.add_step(vec!["fail".into()]);
        self
    }

    pub fn filter<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "filter".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn flatMap(&mut self) -> &mut Self {
        self.bytecode.add_step(vec!["flatMap".into()]);
        self
    }

    pub fn fold(&mut self) -> &mut Self {
        self.bytecode.add_step(vec!["fold".into()]);
        self
    }

    pub fn from<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "from".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn group<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "group".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn groupCount<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "groupCount".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn has<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "has".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn hasId<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "hasId".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn hasKey<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "hasKey".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn hasLabel<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "hasLabel".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn hasNot<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "hasNot".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn hasValue<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "hasValue".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn id(&mut self) -> &mut Self {
        self.bytecode.add_step(vec!["id".into()]);
        self
    }

    pub fn identity(&mut self) -> &mut Self {
        self.bytecode.add_step(vec!["identity".into()]);
        self
    }

    pub fn in_<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "in".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn inE<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "inE".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn inV<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "inV".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn index(&mut self) -> &mut Self {
        self.bytecode.add_step(vec!["index".into()]);
        self
    }

    pub fn inject<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "inject".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn is<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "is".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn key(&mut self) -> &mut Self {
        self.bytecode.add_step(vec!["key".into()]);
        self
    }

    pub fn label(&mut self) -> &mut Self {
        self.bytecode.add_step(vec!["label".into()]);
        self
    }

    pub fn limit<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "limit".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn local<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "local".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn loops(&mut self) -> &mut Self {
        self.bytecode.add_step(vec!["loops".into()]);
        self
    }

    pub fn map<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "map".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn match_<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "match".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn math<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "math".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn max(&mut self) -> &mut Self {
        self.bytecode.add_step(vec!["max".into()]);
        self
    }
    pub fn mean(&mut self) -> &mut Self {
        self.bytecode.add_step(vec!["mean".into()]);
        self
    }
    pub fn mergeE<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "mergeE".into());
        self.bytecode.add_step(step);
        self
    }
    pub fn mergeV<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "mergeV".into());
        self.bytecode.add_step(step);
        self
    }
    pub fn min(&mut self) -> &mut Self {
        self.bytecode.add_step(vec!["min".into()]);
        self
    }
    pub fn none(&mut self) -> &mut Self {
        self.bytecode.add_step(vec!["none".into()]);
        self
    }
    pub fn not<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "not".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn option<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "option".into());
        self.bytecode.add_step(step);
        self
    }
    pub fn or<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "or".into());
        self.bytecode.add_step(step);
        self
    }
    pub fn order(&mut self) -> &mut Self {
        self.bytecode.add_step(vec!["order".into()]);
        self
    }

    pub fn otherV<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "otherV".into());
        self.bytecode.add_step(step);
        self
    }
    pub fn out<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "out".into());
        self.bytecode.add_step(step);
        self
    }
    pub fn outE<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "outE".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn outV<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "outV".into());
        self.bytecode.add_step(step);
        self
    }
    pub fn pageRank(&mut self) -> &mut Self {
        self.bytecode.add_step(vec!["pageRank".into()]);
        self
    }
    pub fn path(&mut self) -> &mut Self {
        self.bytecode.add_step(vec!["path".into()]);
        self
    }
    pub fn peerPressure(&mut self) -> &mut Self {
        self.bytecode.add_step(vec!["peerPressure".into()]);
        self
    }
    pub fn profile(&mut self) -> &mut Self {
        self.bytecode.add_step(vec!["profile".into()]);
        self
    }
    pub fn program<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "program".into());
        self.bytecode.add_step(step);
        self
    }
    pub fn project<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "project".into());
        self.bytecode.add_step(step);
        self
    }
    pub fn properties<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "properties".into());
        self.bytecode.add_step(step);
        self
    }
    pub fn property<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "property".into());
        self.bytecode.add_step(step);
        self
    }
    pub fn propertyMap<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "propertyMap".into());
        self.bytecode.add_step(step);
        self
    }
    pub fn range<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "range".into());
        self.bytecode.add_step(step);
        self
    }
    pub fn read<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "read".into());
        self.bytecode.add_step(step);
        self
    }
    pub fn repeat<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "repeat".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn sack<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "sack".into());
        self.bytecode.add_step(step);
        self
    }
    pub fn sample<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "sample".into());
        self.bytecode.add_step(step);
        self
    }
    pub fn select<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "select".into());
        self.bytecode.add_step(step);
        self
    }
    pub fn shortestPath<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "shortestPath".into());
        self.bytecode.add_step(step);
        self
    }
    pub fn sideEffect<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "sideEffect".into());
        self.bytecode.add_step(step);
        self
    }
    pub fn simplePath<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "simplePath".into());
        self.bytecode.add_step(step);
        self
    }
    pub fn skip<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "skip".into());
        self.bytecode.add_step(step);
        self
    }
    pub fn store<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "store".into());
        self.bytecode.add_step(step);
        self
    }
    pub fn subgraph<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "subgraph".into());
        self.bytecode.add_step(step);
        self
    }
    pub fn sum(&mut self) -> &mut Self {
        self.bytecode.add_step(vec!["sum".into()]);
        self
    }
    pub fn tail<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "tail".into());
        self.bytecode.add_step(step);
        self
    }
    pub fn timeLimit<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "timeLimit".into());
        self.bytecode.add_step(step);
        self
    }
    pub fn times<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "times".into());
        self.bytecode.add_step(step);
        self
    }
    pub fn to<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "to".into());
        self.bytecode.add_step(step);
        self
    }
    pub fn toE<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "toE".into());
        self.bytecode.add_step(step);
        self
    }
    pub fn toV<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "toV".into());
        self.bytecode.add_step(step);
        self
    }
    pub fn tree<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "tree".into());
        self.bytecode.add_step(step);
        self
    }
    pub fn unfold(&mut self) -> &mut Self {
        self.bytecode.add_step(vec!["unfold".into()]);
        self
    }
    pub fn union<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "union".into());
        self.bytecode.add_step(step);
        self
    }
    pub fn until<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "until".into());
        self.bytecode.add_step(step);
        self
    }
    pub fn value(&mut self) -> &mut Self {
        self.bytecode.add_step(vec!["value".into()]);
        self
    }
    pub fn valueMap<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "valueMap".into());
        self.bytecode.add_step(step);
        self
    }

    pub fn values<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "values".into());
        self.bytecode.add_step(step);
        self
    }
    pub fn where_<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "where".into());
        self.bytecode.add_step(step);
        self
    }
    pub fn with<T: Into<StepValue> + Clone>(&mut self, args: &[T]) -> &mut Self {
        let mut step: Vec<StepValue> = args.to_vec().into_iter().map(|v| v.into()).collect();
        step.insert(0, "with".into());
        self.bytecode.add_step(step);
        self
    }
    pub fn write(&mut self) -> &mut Self {
        self.bytecode.add_step(vec!["write".into()]);
        self
    }
}
