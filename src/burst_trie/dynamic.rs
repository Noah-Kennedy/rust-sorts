use std::option::Option::Some;

const BURST_LIMIT: usize = 8192;

pub struct TrieNode {
    pub offset: usize,
    pub matches: Vec<String>,
    pub kind: NodeKind,
}

pub enum NodeKind {
    Collapsed(Vec<String>),
    Burst(Vec<(char, TrieNode)>),
}

impl TrieNode {
    pub fn new(offset: usize) -> Self {
        Self {
            offset,
            matches: Vec::new(),
            kind: NodeKind::Collapsed(Vec::new()),
        }
    }

    pub fn insert(&mut self, s: String) {
        if self.offset == s.chars().count() {
            self.matches.push(s);
        } else {
            match &mut self.kind {
                NodeKind::Burst(children) => {
                    let c = s.chars().nth(self.offset).unwrap();

                    match children.binary_search_by_key(&c, |x| x.0) {
                        Ok(idx) => {
                            children[idx].1.insert(s);
                        }
                        Err(idx) => {
                            let mut new_node = TrieNode::new(self.offset + 1);
                            new_node.insert(s);
                            children.insert(idx, (c, new_node));
                        }
                    }
                }
                NodeKind::Collapsed(children) => {
                    children.push(s);

                    if children.len() > BURST_LIMIT {
                        let mut new_children: Vec<(char, TrieNode)> = Vec::new();

                        while let Some(child) = children.pop() {
                            let c = child.chars().nth(self.offset).unwrap();

                            match new_children.binary_search_by_key(&c, |x| x.0) {
                                Ok(idx) => {
                                    new_children[idx].1.insert(child);
                                }
                                Err(idx) => {
                                    let mut new_node = TrieNode::new(self.offset + 1);
                                    new_node.insert(child);
                                    new_children.insert(idx, (c, new_node));
                                }
                            }
                        }

                        self.kind = NodeKind::Burst(new_children);
                    }
                }
            }
        }
    }

    pub fn merge(mut self, target: &mut Vec<String>) {
        target.append(&mut self.matches);

        match self.kind {
            NodeKind::Collapsed(mut children) => {
                children.sort();
                target.append(&mut children);
            }
            NodeKind::Burst(children) => {
                for (_, child) in children {
                    child.merge(target)
                }
            }
        }
    }
}