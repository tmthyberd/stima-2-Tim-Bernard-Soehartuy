use serde::Serialize;
use std::collections::HashMap;
#[derive(Debug, Serialize)]
pub enum NodeType {
    Element,
    Text,
    Document,
}
#[derive(Debug, Serialize)]
pub struct DomNode {
    pub node_type: NodeType,
    pub tag_name: Option<String>,
    pub attributes: HashMap<String, String>,
    pub children: Vec<usize>,
    pub parent: Option<usize>,
    pub text_content: Option<String>,
}
#[derive(Debug, Serialize)]
pub struct DomTree {
    pub nodes: Vec<DomNode>,
}

impl DomTree {
    pub fn new() -> Self {
        let root = DomNode {
            node_type: NodeType::Document,
            tag_name: None,
            attributes: HashMap::new(),
            children: Vec::new(),
            parent: None,
            text_content: None,
        };

        DomTree { nodes: vec![root] }
    }

    pub fn add_node(&mut self, node: DomNode) -> usize {
        self.nodes.push(node);
        self.nodes.len() - 1
    }
}

impl Default for DomTree {
    fn default() -> Self {
        Self::new()
    }
}
