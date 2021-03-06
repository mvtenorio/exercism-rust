use std::collections::HashMap;

use attrs_derive::Attrs;

use crate::Attrs;

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
    }}
