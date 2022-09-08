use crate::driver::*;

use super::*;

#[derive(Serialize, Debug, Clone)]
pub(crate) struct Bytecode {
    #[serde(rename = "step")]
    steps: Vec<Vec<StepValue>>,
}

impl Bytecode {
    pub(crate) fn new() -> Self {
        Bytecode { steps: Vec::new() }
    }
    pub(crate) fn add_step(&mut self, s: Vec<StepValue>) {
        self.steps.push(s)
    }
}

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

impl Clone for Traversal {
    fn clone(&self) -> Self {
        Traversal::new()
    }
}

#[allow(non_snake_case)]
impl Traversal {
    pub fn new() -> Self {
        Traversal {
            bytecode: Bytecode::new(),
        }
    }

    pub async fn to_list(self, client: &Client) -> Result<ClientResponse, ClientError> {
        client.execute(self).await
    }

    pub fn V<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("V"));
        self
    }

    pub fn E<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("E"));
        self
    }

    pub fn addE<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("addE"));

        self
    }

    pub fn addV<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("addV"));

        self
    }

    pub fn aggregate<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("aggregate"));

        self
    }

    pub fn and<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("and"));

        self
    }

    pub fn as_<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("as"));

        self
    }

    pub fn barrier<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("barrier"));

        self
    }

    pub fn both<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("both"));

        self
    }

    pub fn bothE<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("bothE"));

        self
    }

    pub fn bothV<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("bothV"));

        self
    }

    pub fn branch(mut self) -> Self {
        self.bytecode.add_step(Step::no_arg("branch"));

        self
    }

    pub fn by<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("by"));

        self
    }

    pub fn call<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("call"));

        self
    }

    pub fn cap<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("cap"));

        self
    }

    pub fn choose<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("choose"));

        self
    }

    pub fn coalesce<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("coalesce"));

        self
    }

    pub fn coin<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("coin"));

        self
    }

    pub fn connectedComponent<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("connectedComponent"));

        self
    }

    pub fn constant<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("constant"));

        self
    }

    pub fn count(mut self) -> Self {
        self.bytecode.add_step(Step::no_arg("count"));

        self
    }

    pub fn cyclicPath<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("cyclicPath"));

        self
    }

    pub fn dedup<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("dedup"));

        self
    }

    pub fn drop(mut self) -> Self {
        self.bytecode.add_step(Step::no_arg("drop"));

        self
    }

    pub fn element(mut self) -> Self {
        self.bytecode.add_step(Step::no_arg("element"));

        self
    }

    pub fn elementMap<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("elementMap"));

        self
    }

    pub fn emit<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("emit"));

        self
    }

    pub fn fail(mut self) -> Self {
        self.bytecode.add_step(Step::no_arg("fail"));

        self
    }

    pub fn filter<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("filter"));

        self
    }

    pub fn flatMap(mut self) -> Self {
        self.bytecode.add_step(Step::no_arg("flatMap"));

        self
    }

    pub fn fold(mut self) -> Self {
        self.bytecode.add_step(Step::no_arg("fold"));

        self
    }

    pub fn from<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("from"));

        self
    }

    pub fn group<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("group"));

        self
    }

    pub fn groupCount<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("groupCount"));

        self
    }

    pub fn has<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("has"));

        self
    }

    pub fn hasId<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("hasId"));

        self
    }

    pub fn hasKey<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("hasKey"));

        self
    }

    pub fn hasLabel<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("hasLabel"));

        self
    }

    pub fn hasNot<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("hasNot"));

        self
    }

    pub fn hasValue<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("hasValue"));

        self
    }

    pub fn id(mut self) -> Self {
        self.bytecode.add_step(Step::no_arg("id"));

        self
    }

    pub fn identity(mut self) -> Self {
        self.bytecode.add_step(Step::no_arg("identity"));

        self
    }

    pub fn in_<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("in"));

        self
    }

    pub fn inE<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("inE"));

        self
    }

    pub fn inV<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("inV"));

        self
    }

    pub fn index(mut self) -> Self {
        self.bytecode.add_step(Step::no_arg("index"));

        self
    }

    pub fn inject<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("inject"));

        self
    }

    pub fn is<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("is"));

        self
    }

    pub fn key(mut self) -> Self {
        self.bytecode.add_step(Step::no_arg("key"));

        self
    }

    pub fn label(mut self) -> Self {
        self.bytecode.add_step(Step::no_arg("label"));

        self
    }

    pub fn limit<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("limit"));

        self
    }

    pub fn local<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("local"));

        self
    }

    pub fn loops(mut self) -> Self {
        self.bytecode.add_step(Step::no_arg("loops"));

        self
    }

    pub fn map<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("map"));

        self
    }

    pub fn match_<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("match"));

        self
    }

    pub fn math<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("math"));

        self
    }

    pub fn max(mut self) -> Self {
        self.bytecode.add_step(Step::no_arg("max"));

        self
    }
    pub fn mean(mut self) -> Self {
        self.bytecode.add_step(Step::no_arg("mean"));

        self
    }
    pub fn mergeE<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("mergeE"));

        self
    }
    pub fn mergeV<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("mergeV"));

        self
    }
    pub fn min(mut self) -> Self {
        self.bytecode.add_step(Step::no_arg("min"));

        self
    }
    pub fn none(mut self) -> Self {
        self.bytecode.add_step(Step::no_arg("none"));

        self
    }
    pub fn not<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("not"));

        self
    }

    pub fn option<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("option"));

        self
    }
    pub fn or<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("or"));

        self
    }
    pub fn order(mut self) -> Self {
        self.bytecode.add_step(Step::no_arg("order"));

        self
    }

    pub fn otherV<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("otherV"));

        self
    }
    pub fn out<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("out"));

        self
    }
    pub fn outE<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("outE"));

        self
    }

    pub fn outV<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("outV"));

        self
    }
    pub fn pageRank(mut self) -> Self {
        self.bytecode.add_step(Step::no_arg("pageRank"));

        self
    }
    pub fn path(mut self) -> Self {
        self.bytecode.add_step(Step::no_arg("path"));

        self
    }
    pub fn peerPressure(mut self) -> Self {
        self.bytecode.add_step(Step::no_arg("peerPressure"));

        self
    }
    pub fn profile(mut self) -> Self {
        self.bytecode.add_step(Step::no_arg("profile"));

        self
    }
    pub fn program<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("program"));

        self
    }
    pub fn project<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("project"));

        self
    }
    pub fn properties<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("properties"));

        self
    }
    pub fn property<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("property"));

        self
    }
    pub fn propertyMap<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("propertyMap"));

        self
    }
    pub fn range<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("range"));

        self
    }
    pub fn read<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("read"));

        self
    }
    pub fn repeat<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("repeat"));

        self
    }

    pub fn sack<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("sack"));

        self
    }
    pub fn sample<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("sample"));

        self
    }
    pub fn select<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("select"));

        self
    }
    pub fn shortestPath<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("shortestPath"));

        self
    }
    pub fn sideEffect<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("sideEffect"));

        self
    }
    pub fn simplePath<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("simplePath"));

        self
    }
    pub fn skip<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("skip"));

        self
    }
    pub fn store<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("store"));

        self
    }
    pub fn subgraph<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("subgraph"));

        self
    }
    pub fn sum(mut self) -> Self {
        self.bytecode.add_step(Step::no_arg("sum"));

        self
    }
    pub fn tail<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("tail"));

        self
    }
    pub fn timeLimit<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("timeLimit"));

        self
    }
    pub fn times<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("times"));

        self
    }
    pub fn to<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("to"));

        self
    }
    pub fn toE<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("toE"));

        self
    }
    pub fn toV<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("toV"));

        self
    }
    pub fn tree<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("tree"));

        self
    }
    pub fn unfold(mut self) -> Self {
        self.bytecode.add_step(Step::no_arg("unfold"));

        self
    }
    pub fn union<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("union"));

        self
    }
    pub fn until<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("until"));

        self
    }
    pub fn value(mut self) -> Self {
        self.bytecode.add_step(Step::no_arg("value"));

        self
    }
    pub fn valueMap<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("valueMap"));

        self
    }

    pub fn values<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("values"));

        self
    }
    pub fn where_<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("where"));

        self
    }
    pub fn with<T: Into<Step> + Clone>(mut self, args: T) -> Self {
        let step: Step = args.into();
        self.bytecode.add_step(step.operator("with"));

        self
    }
    pub fn write(mut self) -> Self {
        self.bytecode.add_step(Step::no_arg("write"));

        self
    }
}
