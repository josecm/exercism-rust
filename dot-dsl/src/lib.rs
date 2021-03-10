pub mod graph {
    use std::collections::HashMap;
    pub type Attrs = HashMap<String, String>;
    use crate::graph::graph_items::{edge::Edge, node::Node};

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: Attrs,
    }

    pub mod graph_items {

        pub mod node {
            use super::super::Attrs;
            use std::collections::HashMap;

            #[derive(Debug, PartialEq, Clone)]
            pub struct Node {
                pub name: String,
                pub attrs: Attrs,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: String::from(name),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(self, attrs: &[(&str,&str)]) -> Self{
                    Self {
                        attrs: attrs
                            .iter()
                            .map(|(a,n)| (a.to_string(), n.to_string()))
                            .collect(),
                        ..self
                    }
                }
                
                pub fn get_attr(&self, attr: &str) -> Option<&str> {
                    self.attrs.get(attr).map(|s| &s[..])
                }
            }
        }

        pub mod edge {
            use super::super::Attrs;
            use std::collections::HashMap;

            #[derive(Debug, PartialEq, Clone)]
            pub struct Edge {
                pub attrs: Attrs,
            }

            impl Edge {
                pub fn new(_a: &str, _b: &str) -> Self {
                    Edge {
                        attrs: HashMap::new(),
                    }
                }
                pub fn with_attrs(self, attrs: &[(&str,&str)]) -> Self{
                    Self {
                        attrs: attrs
                            .iter()
                            .map(|(a,n)| (a.to_string(), n.to_string()))
                            .collect(),
                        ..self
                    }
                }
 
            }
        }
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }
        pub fn with_nodes(self, nodes: &[Node]) -> Self {
            Graph {
                nodes: Vec::from(nodes), 
                ..self
            }
        }
        pub fn with_edges(self, edges: &[Edge]) -> Self {
            Graph {
                edges: Vec::from(edges),
                ..self
            }
        }
        pub fn with_attrs(self, attrs: &[(&str,&str)]) -> Self{
            Self {
                attrs: attrs
                    .iter()
                    .map(|(a,n)| (a.to_string(), n.to_string()))
                    .collect(),
                ..self
            }
        }
 
        pub fn get_node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|n| n.name == name)
        }
    }
}
