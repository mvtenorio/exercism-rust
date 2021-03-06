use std::collections::HashMap;

use attrs_derive::Attrs;

use graph_items::{edge::Edge, node::Node};

pub mod graph_items;

pub trait Attrs {
    fn with_attrs(self, attrs: &[(&'static str, &'static str)]) -> Self;
    fn get_attr(&self, name: &str) -> Option<&str>;
}

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
