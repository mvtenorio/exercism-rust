use std::collections::HashMap;

use attrs_derive::Attrs;

use crate::Attrs;

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
}
