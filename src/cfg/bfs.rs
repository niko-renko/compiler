use super::*;

use std::collections::{HashMap, HashSet};

pub struct BFSIter {
    edges_out: HashMap<Label, Vec<Label>>,
    queue: Vec<Label>,
    visited: HashSet<Label>,
}

impl BFSIter {
    pub fn from(edges_out: HashMap<Label, Vec<Label>>, start: Label) -> Self {
        BFSIter {
            edges_out,
            queue: vec![start],
            visited: HashSet::new(),
        }
    }
}

impl Iterator for BFSIter {
    type Item = Label;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(label) = self.queue.pop() {
            let empty = vec![];
            for edge_out in self.edges_out.get(&label).unwrap_or(&empty) {
                if self.queue.contains(edge_out) || self.visited.contains(edge_out) {
                    continue;
                }
                self.queue.push(*edge_out);
            }
            self.visited.insert(label);
            Some(label)
        } else {
            None
        }
    }
}
