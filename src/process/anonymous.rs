use super::*;
use crate::process::bytecode::BytecodeStep;

pub struct AnonymousTraversal {
    traversal: Traversal,
}

impl AnonymousTraversal {
    pub fn new() -> Self {
        Self {
            traversal: Traversal::new(),
        }
    }

    pub fn V<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().V(args)
    }

    pub fn addE<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().addE(args)
    }

    pub fn addV<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().addV(args)
    }

    pub fn aggregate<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().aggregate(args)
    }

    pub fn and<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().and(args)
    }

    pub fn as_<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().as_(args)
    }

    pub fn barrier<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().barrier(args)
    }

    pub fn both<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().both(args)
    }

    pub fn bothE<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().bothE(args)
    }

    pub fn bothV<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().bothV(args)
    }

    pub fn branch(&self) -> Traversal {
        self.traversal.clone().branch()
    }

    pub fn by<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().by(args)
    }

    pub fn call<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().call(args)
    }

    pub fn cap<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().cap(args)
    }

    pub fn choose<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().choose(args)
    }

    pub fn coalesce<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().coalesce(args)
    }

    pub fn coin<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().coin(args)
    }

    pub fn connectedComponent<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().connectedComponent(args)
    }

    pub fn constant<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().constant(args)
    }

    pub fn count(&self) -> Traversal {
        self.traversal.clone().count()
    }

    pub fn cyclicPath<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().cyclicPath(args)
    }

    pub fn dedup<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().dedup(args)
    }

    pub fn drop(&self) -> Traversal {
        self.traversal.clone().drop()
    }

    pub fn element(&self) -> Traversal {
        self.traversal.clone().element()
    }

    pub fn elementMap<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().elementMap(args)
    }

    pub fn emit<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().emit(args)
    }

    pub fn fail(&self) -> Traversal {
        self.traversal.clone().fail()
    }

    pub fn filter<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().filter(args)
    }

    pub fn flatMap(&self) -> Traversal {
        self.traversal.clone().flatMap()
    }

    pub fn fold(&self) -> Traversal {
        self.traversal.clone().fold()
    }

    pub fn from<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().from(args)
    }

    pub fn group<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().group(args)
    }

    pub fn groupCount<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().groupCount(args)
    }

    pub fn has<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().has(args)
    }

    pub fn hasId<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().hasId(args)
    }

    pub fn hasKey<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().hasKey(args)
    }

    pub fn hasLabel<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().hasLabel(args)
    }

    pub fn hasNot<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().hasNot(args)
    }

    pub fn hasValue<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().hasValue(args)
    }

    pub fn id(&self) -> Traversal {
        self.traversal.clone().id()
    }

    pub fn identity(&self) -> Traversal {
        self.traversal.clone().identity()
    }

    pub fn in_<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().in_(args)
    }

    pub fn inE<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().inE(args)
    }

    pub fn inV<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().inV(args)
    }

    pub fn index(&self) -> Traversal {
        self.traversal.clone().index()
    }

    pub fn inject<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().inject(args)
    }

    pub fn is<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().is(args)
    }

    pub fn key(&self) -> Traversal {
        self.traversal.clone().key()
    }

    pub fn label(&self) -> Traversal {
        self.traversal.clone().label()
    }

    pub fn limit<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().limit(args)
    }

    pub fn local<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().local(args)
    }

    pub fn loops(&self) -> Traversal {
        self.traversal.clone().loops()
    }

    pub fn map<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().map(args)
    }

    pub fn match_<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().match_(args)
    }

    pub fn math<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().math(args)
    }

    pub fn max(&self) -> Traversal {
        self.traversal.clone().max()
    }
    pub fn mean(&self) -> Traversal {
        self.traversal.clone().mean()
    }
    pub fn mergeE<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().mergeE(args)
    }
    pub fn mergeV<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().mergeV(args)
    }
    pub fn min(&self) -> Traversal {
        self.traversal.clone().min()
    }
    pub fn none(&self) -> Traversal {
        self.traversal.clone().none()
    }
    pub fn not<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().not(args)
    }

    pub fn option<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().option(args)
    }
    pub fn or<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().or(args)
    }
    pub fn order(&self) -> Traversal {
        self.traversal.clone().order()
    }

    pub fn otherV<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().otherV(args)
    }
    pub fn out<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().out(args)
    }
    pub fn outE<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().outE(args)
    }

    pub fn outV<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().outV(args)
    }
    pub fn pageRank(&self) -> Traversal {
        self.traversal.clone().pageRank()
    }
    pub fn path(&self) -> Traversal {
        self.traversal.clone().path()
    }
    pub fn peerPressure(&self) -> Traversal {
        self.traversal.clone().peerPressure()
    }
    pub fn profile(&self) -> Traversal {
        self.traversal.clone().profile()
    }
    pub fn program<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().program(args)
    }
    pub fn project<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().project(args)
    }
    pub fn properties<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().properties(args)
    }
    pub fn property<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().property(args)
    }
    pub fn propertyMap<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().propertyMap(args)
    }
    pub fn range<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().range(args)
    }
    pub fn read<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().read(args)
    }
    pub fn repeat<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().repeat(args)
    }

    pub fn sack<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().sack(args)
    }
    pub fn sample<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().sample(args)
    }
    pub fn select<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().select(args)
    }
    pub fn shortestPath<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().shortestPath(args)
    }
    pub fn sideEffect<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().sideEffect(args)
    }
    pub fn simplePath<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().simplePath(args)
    }
    pub fn skip<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().skip(args)
    }
    pub fn store<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().store(args)
    }
    pub fn subgraph<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().subgraph(args)
    }
    pub fn sum(&self) -> Traversal {
        self.traversal.clone().sum()
    }
    pub fn tail<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().tail(args)
    }
    pub fn timeLimit<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().timeLimit(args)
    }
    pub fn times<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().times(args)
    }
    pub fn to<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().to(args)
    }
    pub fn toE<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().toE(args)
    }
    pub fn toV<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().toV(args)
    }
    pub fn tree<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().tree(args)
    }
    pub fn unfold(&self) -> Traversal {
        self.traversal.clone().unfold()
    }
    pub fn union<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().union(args)
    }
    pub fn until<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().until(args)
    }
    pub fn value(&self) -> Traversal {
        self.traversal.clone().value()
    }
    pub fn valueMap<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().valueMap(args)
    }

    pub fn values<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().values(args)
    }
    pub fn where_<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().where_(args)
    }
    pub fn with<T: Into<BytecodeStep> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().with(args)
    }
    pub fn write(&self) -> Traversal {
        self.traversal.clone().write()
    }
}
