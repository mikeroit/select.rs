use node::Node;

pub trait Predicate: Sized {
    fn matches(&self, node: &Node) -> bool;
    fn or<T: Predicate>(self, other: T) -> Or<Self, T> {
        Or(self, other)
    }
}

impl Predicate for () {
    fn matches(&self, _: &Node) -> bool {
        true
    }
}

pub struct Name<T>(pub T);

impl<'a> Predicate for Name<&'a str> {
    fn matches(&self, node: &Node) -> bool {
        node.name() == Some(self.0)
    }
}

pub struct Class<T>(pub T);

impl<'a> Predicate for Class<&'a str> {
    fn matches(&self, node: &Node) -> bool {
        node.attr("class").map(|classes| {
            classes.split_whitespace().any(|class| class == self.0)
        }).unwrap_or(false)
    }
}

pub struct Not<T>(pub T);

impl<T: Predicate> Predicate for Not<T> {
    fn matches(&self, node: &Node) -> bool {
        !self.0.matches(node)
    }
}

pub struct Attr<N, V>(pub N, pub V);

impl<'a> Predicate for Attr<&'a str, &'a str> {
    fn matches(&self, node: &Node) -> bool {
        node.attr(self.0) == Some(self.1)
    }
}

impl<F: Fn(&Node) -> bool> Predicate for F {
    fn matches(&self, node: &Node) -> bool {
        self(node)
    }
}

pub struct Element;

impl Predicate for Element {
    fn matches(&self, node: &Node) -> bool {
        node.name().is_some()
    }
}

pub struct Text;

impl Predicate for Text {
    fn matches(&self, node: &Node) -> bool {
        node.name().is_none()
    }
}

pub struct Or<A, B>(pub A, pub B);

impl<A: Predicate, B: Predicate> Predicate for Or<A, B> {
    fn matches(&self, node: &Node) -> bool {
        self.0.matches(node) || self.1.matches(node)
    }
}
