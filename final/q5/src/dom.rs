//! Basic DOM data structures.

use std::collections::{HashMap,HashSet};

pub type AttrMap = HashMap<String, String>;

#[derive(Debug)]
pub struct Node {
    // data common to all nodes:
    pub children: Vec<Node>,

    // data specific to each node type:
    pub node_type: NodeType,
}

#[derive(Debug)]
pub enum NodeType {
    Element(ElementData),
    Text(String),
}

#[derive(Debug)]
pub struct ElementData {
    pub tag_name: String,
    pub id: Option<String>,
    pub classes : HashSet<String>,
}

// Constructor functions for convenience:

pub fn text(data: String) -> Node {
    Node { children: vec![], node_type: NodeType::Text(data) }
}

pub fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    let classes = match attrs.get("class") {
        Some(classlist) => classlist.split(' ').map(|a|a.to_string()).collect(),
        None => HashSet::new()
    };
    let id = match attrs.get("id") {
        Some(id) => {Some(id.to_owned())}
        None => {None}
    };
    Node {
        children: children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            id: id,
            classes: classes
        })
    }
}

