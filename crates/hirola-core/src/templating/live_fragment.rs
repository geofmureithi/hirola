use web_sys::{Document, Node};

use crate::generic_node::GenericNode;

pub struct LiveFragment<G> {
    parent: G,
    children: Vec<G>,
    prev_sibling: Option<G>,
    next_sibling: Option<G>,
    owner_document: Option<Document>,
    is_document_fragment: bool,
}

impl<G: GenericNode> LiveFragment<G> {
    pub fn new(parent: G) -> Self {
        Self {
            parent,
            children: Vec::new(),
            prev_sibling: None,
            next_sibling: None,
            owner_document: None,
            is_document_fragment: false,
        }
    }
    pub fn append_child(&mut self, child: G) {
        if let Some(parent) = child.parent_node() {
            parent.remove_child(&child);
        }

        if let Some(next) = &self.next_sibling {
            self.parent.insert_child_before(&child, Some(&next));
        } else {
            self.parent.append_child(&child);
        }
        self.children.push(child);
    }

    pub fn insert_before(&mut self, child: G, next: G) {
        if let Some(parent) = child.parent_node() {
            parent.remove_child(&child);
        }
        self.parent.insert_child_before(&child, Some(&next));
        self.children.push(child);
    }

    pub fn remove_child(&mut self, child: &G) {
        if let Some(pos) = self.children.iter().position(|x| x == child) {
            self.children.remove(pos);
        }
        self.parent.remove_child(child);
    }

    pub fn replace_child(&mut self, old: &G, new: &G) {
        self.parent.replace_child(old, new);
        if let Some(pos) = self.children.iter().position(|x: &G| x == old) {
            self.children.remove(pos);
        }
    }

    pub fn empty(&mut self) {
        for child in &mut self.children {
            self.parent.remove_child(child);
        }
        self.children = Vec::new();
    }

    pub fn extend(&mut self, node: &Node) {}

    pub fn shrink(&mut self, node: &Node) {}

    pub fn prepend(&mut self, node: &Node) {}

    pub fn append(&mut self, node: &Node) {}

    pub fn get_document_fragment(&mut self) {}
    pub fn before(&mut self, node: &Node) {}

    pub fn after(&mut self, node: &Node) {}

    pub fn replace(&mut self, node: &Node) {}

    pub fn contains(&mut self, node: &Node) {}
}
