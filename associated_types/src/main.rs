fn main() {
    use std::fmt;

    println!("Traits");

    // Graph trait has two generic types: node & edge
    // Graph<N, E>

    trait Graph<N, E> {
      type N: fmt::Display;
      type E;

      fn has_edge(&self, &Self::N, &Self::N) -> bool;
      fn edges(&self, &Self::N) -> Vec<Self::E>;
    }
    // associated types use the type keyword, an go inside the body of
    // the trait, with the functions. These type declarations can have
    // all the same thing as functions do.

    struct Node;

    struct Edge;

    struct MyGraph;

    impl Graph for MyGraph {
      type N = Node;
      type E = Edge;

      fn has_edge(&self, n1: &Node, n2: &Node) -> bool {
        true
      }

      fn edges(&self, n: &Node) -> Vec<Edge> {
        Vec::new()
      }
    }

    // we use = to define associated types and concrete types in our function
    // declarations

    // trait objects with associated types
    let graph = MyGraph;
    let obj = Box::new(graph) as Box<Graph<N=Node, E=Edge>>;

    // The N=Node syntax allows us to provide a concrete type, Node,
    // for the N type parameter. Same with E=Edge.
    // If we didn’t proide this constraint, we couldn’t be sure
    // which impl to match this trait object to.
}
