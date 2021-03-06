pub mod graph {
    use std::collections::HashMap;

    use attrs_derive::Attrs;

    use graph_items::{edge::Edge, node::Node};

    #[derive(Default, Attrs)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph::default()
        }

        pub fn with_nodes(self, nodes: &[Node]) -> Self {
            Graph {
                nodes: nodes.to_vec(),
                ..self
            }
        }

        pub fn with_edges(self, edges: &[Edge]) -> Self {
            Graph {
                edges: edges.to_vec(),
                ..self
            }
        }

        pub fn get_node(self, name: &str) -> Option<Node> {
            self.nodes.iter().find(|n| n.name == name).cloned()
        }
    }

    pub mod graph_items {
        pub mod edge {
            use attrs_derive::Attrs;

            use std::collections::HashMap;

            #[derive(Attrs, Debug, Clone, Eq, PartialEq)]
            pub struct Edge {
                a: &'static str,
                b: &'static str,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(a: &'static str, b: &'static str) -> Self {
                    Edge {
                        a,
                        b,
                        attrs: HashMap::new(),
                    }
                }
            }
        }
        pub mod node {
            use attrs_derive::Attrs;

            use std::collections::HashMap;

            #[derive(Attrs, Debug, Clone, Eq, PartialEq)]
            pub struct Node {
                pub name: &'static str,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &'static str) -> Self {
                    Node {
                        name,
                        attrs: HashMap::new(),
                    }
                }

                pub fn get_attr(&self, name: &str) -> Option<&str> {
                    self.attrs.get(name).map(|s| s.as_str())
                }
            }
        }
    }
}
