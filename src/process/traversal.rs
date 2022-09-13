use crate::{driver::*, process::bytecode::*};

#[derive(Debug)]
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

    pub fn is_mutating(&self) -> bool {
        self.bytecode.is_mutating()
    }

    pub async fn to_list(self, client: &Client) -> Result<ClientResponse, ClientError> {
        client.execute(self).await
    }

    pub fn V<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("V", args);
        self
    }

    pub fn E<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("E", args);
        self
    }

    pub fn addE<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("addE", args);
        self
    }

    pub fn addV<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("addV", args);
        self
    }

    pub fn aggregate<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("aggregate", args);
        self
    }

    pub fn and<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("and", args);
        self
    }

    pub fn as_<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("as", args);
        self
    }

    pub fn barrier<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("barrier", args);
        self
    }

    pub fn both<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("both", args);
        self
    }

    pub fn bothE<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("bothE", args);
        self
    }

    pub fn bothV<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("bothV", args);
        self
    }

    pub fn branch(mut self) -> Self {
        self.bytecode.no_arg_step("branch");
        self
    }

    pub fn by<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("by", args);
        self
    }

    pub fn call<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("call", args);
        self
    }

    pub fn cap<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("cap", args);
        self
    }

    pub fn choose<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("choose", args);
        self
    }

    pub fn coalesce<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("coalesce", args);
        self
    }

    pub fn coin<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("coin", args);
        self
    }

    pub fn connectedComponent<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("connectedComponent", args);
        self
    }

    pub fn constant<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("constant", args);
        self
    }

    pub fn count(mut self) -> Self {
        self.bytecode.no_arg_step("count");
        self
    }

    pub fn cyclicPath<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("cyclicPath", args);
        self
    }

    pub fn dedup<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("dedup", args);
        self
    }

    pub fn drop(mut self) -> Self {
        self.bytecode.no_arg_step("drop");
        self
    }

    pub fn element(mut self) -> Self {
        self.bytecode.no_arg_step("element");
        self
    }

    pub fn elementMap<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("elementMap", args);
        self
    }

    pub fn emit<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("emit", args);
        self
    }

    pub fn fail(mut self) -> Self {
        self.bytecode.no_arg_step("fail");
        self
    }

    pub fn filter<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("filter", args);
        self
    }

    pub fn flatMap(mut self) -> Self {
        self.bytecode.no_arg_step("flatMap");
        self
    }

    pub fn fold(mut self) -> Self {
        self.bytecode.no_arg_step("fold");
        self
    }

    pub fn from<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("from", args);
        self
    }

    pub fn group<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("group", args);
        self
    }

    pub fn groupCount<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("groupCount", args);
        self
    }

    pub fn has<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("has", args);
        self
    }

    pub fn hasId<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("hasId", args);
        self
    }

    pub fn hasKey<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("hasKey", args);
        self
    }

    pub fn hasLabel<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("hasLabel", args);
        self
    }

    pub fn hasNot<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("hasNot", args);
        self
    }

    pub fn hasValue<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("hasValue", args);
        self
    }

    pub fn id(mut self) -> Self {
        self.bytecode.no_arg_step("id");
        self
    }

    pub fn identity(mut self) -> Self {
        self.bytecode.no_arg_step("identity");
        self
    }

    pub fn in_<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("in", args);
        self
    }

    pub fn inE<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("inE", args);
        self
    }

    pub fn inV<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("inV", args);
        self
    }

    pub fn index(mut self) -> Self {
        self.bytecode.no_arg_step("index");
        self
    }

    pub fn inject<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("inject", args);
        self
    }

    pub fn is<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("is", args);
        self
    }

    pub fn key(mut self) -> Self {
        self.bytecode.no_arg_step("key");
        self
    }

    pub fn label(mut self) -> Self {
        self.bytecode.no_arg_step("label");
        self
    }

    pub fn limit<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("limit", args);
        self
    }

    pub fn local<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("local", args);
        self
    }

    pub fn loops(mut self) -> Self {
        self.bytecode.no_arg_step("loops");
        self
    }

    pub fn map<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("map", args);
        self
    }

    pub fn match_<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("match", args);
        self
    }

    pub fn math<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("math", args);
        self
    }

    pub fn max(mut self) -> Self {
        self.bytecode.no_arg_step("max");
        self
    }
    pub fn mean(mut self) -> Self {
        self.bytecode.no_arg_step("mean");
        self
    }
    pub fn mergeE<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("mergeE", args);
        self
    }
    pub fn mergeV<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("mergeV", args);
        self
    }
    pub fn min(mut self) -> Self {
        self.bytecode.no_arg_step("min");
        self
    }
    pub fn none(mut self) -> Self {
        self.bytecode.no_arg_step("none");
        self
    }
    pub fn not<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("not", args);
        self
    }

    pub fn option<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("option", args);
        self
    }
    pub fn or<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("or", args);
        self
    }
    pub fn order(mut self) -> Self {
        self.bytecode.no_arg_step("order");
        self
    }

    pub fn otherV<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("otherV", args);
        self
    }
    pub fn out<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("out", args);
        self
    }
    pub fn outE<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("outE", args);
        self
    }

    pub fn outV<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("outV", args);
        self
    }
    pub fn pageRank(mut self) -> Self {
        self.bytecode.no_arg_step("pageRank");
        self
    }
    pub fn path(mut self) -> Self {
        self.bytecode.no_arg_step("path");
        self
    }
    pub fn peerPressure(mut self) -> Self {
        self.bytecode.no_arg_step("peerPressure");
        self
    }
    pub fn profile(mut self) -> Self {
        self.bytecode.no_arg_step("profile");
        self
    }
    pub fn program<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("program", args);
        self
    }
    pub fn project<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("project", args);
        self
    }
    pub fn properties<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("properties", args);
        self
    }
    pub fn property<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("property", args);
        self
    }
    pub fn propertyMap<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("propertyMap", args);
        self
    }
    pub fn range<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("range", args);
        self
    }
    pub fn read<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("read", args);
        self
    }
    pub fn repeat<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("repeat", args);
        self
    }

    pub fn sack<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("sack", args);
        self
    }
    pub fn sample<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("sample", args);
        self
    }
    pub fn select<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("select", args);
        self
    }
    pub fn shortestPath<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("shortestPath", args);
        self
    }
    pub fn sideEffect<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("sideEffect", args);
        self
    }
    pub fn simplePath<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("simplePath", args);
        self
    }
    pub fn skip<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("skip", args);
        self
    }
    pub fn store<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("store", args);
        self
    }
    pub fn subgraph<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("subgraph", args);
        self
    }
    pub fn sum(mut self) -> Self {
        self.bytecode.no_arg_step("sum");
        self
    }
    pub fn tail<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("tail", args);
        self
    }
    pub fn timeLimit<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("timeLimit", args);
        self
    }
    pub fn times<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("times", args);
        self
    }
    pub fn to<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("to", args);
        self
    }
    pub fn toE<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("toE", args);
        self
    }
    pub fn toV<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("toV", args);
        self
    }
    pub fn tree<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("tree", args);
        self
    }
    pub fn unfold(mut self) -> Self {
        self.bytecode.no_arg_step("unfold");
        self
    }
    pub fn union<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("union", args);
        self
    }
    pub fn until<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("until", args);
        self
    }
    pub fn value(mut self) -> Self {
        self.bytecode.no_arg_step("value");
        self
    }
    pub fn valueMap<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("valueMap", args);
        self
    }

    pub fn values<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("values", args);
        self
    }
    pub fn where_<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("where", args);
        self
    }
    pub fn with<T: Into<BytecodeStep> + Clone>(mut self, args: T) -> Self {
        self.bytecode.add_step("with", args);
        self
    }
    pub fn write(mut self) -> Self {
        self.bytecode.no_arg_step("write");
        self
    }
}
