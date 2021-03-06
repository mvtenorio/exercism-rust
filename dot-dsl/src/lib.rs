pub mod graph {
    use std::collections::HashMap;
    use std::iter::FromIterator;

    pub struct Graph {
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &[graph_items::node::Node]) -> Self {
            self.nodes = nodes.to_owned();
            self
        }

        pub fn with_edges(mut self, edges: &[graph_items::edge::Edge]) -> Self {
            self.edges = edges.to_owned();
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&'static str, &'static str)]) -> Self {
            self.attrs =
                HashMap::from_iter(attrs.iter().map(|(a, b)| (a.to_string(), b.to_string())));
            self
        }

        pub fn get_node(self, name: &str) -> Option<graph_items::node::Node> {
            self.nodes.iter().cloned().find(|n| n.name == name)
        }
    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;
            use std::iter::FromIterator;

            #[derive(Debug, Clone, Eq, PartialEq)]
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

                pub fn with_attrs(mut self, attrs: &[(&'static str, &'static str)]) -> Self {
                    self.attrs = HashMap::from_iter(
                        attrs.iter().map(|(a, b)| (a.to_string(), b.to_string())),
                    );
                    self
                }
            }
        }
        pub mod node {
            use std::collections::HashMap;
            use std::iter::FromIterator;

            #[derive(Debug, Clone, Eq, PartialEq)]
            pub struct Node {
                pub name: &'static str,
                attrs: HashMap<&'static str, &'static str>,
            }

            impl Node {
                pub fn new(name: &'static str) -> Self {
                    Node {
                        name,
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&'static str, &'static str)]) -> Self {
                    self.attrs = HashMap::from_iter(attrs.to_owned());
                    self
                }

                pub fn get_attr(self, name: &str) -> Option<&str> {
                    self.attrs.get(name).cloned()
                }
            }
        }
    }
}
