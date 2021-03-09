pub mod graph {
    pub struct Graph {
        nodes: Vec<graph_items::Node>,
        edges: Vec<graph_items::Edge>

    pub mod graph_items {
        pub struct Node;
        pub struct Edge;

        impl Node {
            pub fn new() -> Self {
            }
        }
    }

    impl Graph {
        pub fn new() -> Self {
            unimplemented!("Construct a new Graph struct.");
        }
        pub fn is_empty(&self) -> bool {
        }
        pub fn with_nodes(self, nodes: Vec<Node>) -> Self  {
        }
    }
}
