use super::*;

pub struct TraversalSource {
    traversal: Traversal,
}

impl TraversalSource {
    pub fn new() -> Self {
        Self {
            traversal: Traversal::new(),
        }
    }

    pub fn V<T: Into<Step> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().V(args)
    }

    pub fn addE<T: Into<Step> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().addE(args)
    }

    pub fn addV<T: Into<Step> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().addV(args)
    }

    pub fn E<T: Into<Step> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().E(args)
    }

    pub fn inject<T: Into<Step> + Clone>(&self, args: T) -> Traversal {
        self.traversal.clone().inject(args)
    }
}
