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
        if let Some(c) = s.chars().nth(self.offset) {
            match &mut self.kind {
                NodeKind::Burst(children) => {
                    add_to_burst_node(children, s, c, self.offset);
                }
                NodeKind::Collapsed(children) => {
                    children.push(s);

                    if children.len() >= BURST_LIMIT {
                        self.burst()
                    }
                }
            }
        } else {
            self.matches.push(s);
        }
    }

    fn burst(&mut self) {
        if let NodeKind::Collapsed(children) = &mut self.kind {
            let mut new_children: Vec<(char, TrieNode)> = Vec::new();

            while let Some(child) = children.pop() {
                let k = child.chars().nth(self.offset).unwrap();
                add_to_burst_node(&mut new_children, child, k, self.offset)
            }

            self.kind = NodeKind::Burst(new_children);
        } else {
            unreachable!()
        }
    }

    pub fn merge(&mut self, target: &mut Vec<String>) {
        target.append(&mut self.matches);

        match &mut self.kind {
            NodeKind::Collapsed(children) => {
                children.sort_unstable();
                target.append(children);
            }
            NodeKind::Burst(children) => {
                for (_, child) in children.iter_mut() {
                    child.merge(target)
                }
            }
        }
    }
}

#[inline(always)]
fn add_to_burst_node(children: &mut Vec<(char, TrieNode)>, s: String, c: char, offset: usize) {
    match children.binary_search_by_key(&c, |x| x.0) {
        Ok(idx) => {
            children[idx].1.insert(s);
        }
        Err(idx) => {
            let mut new_node = TrieNode::new(offset + 1);
            new_node.insert(s);
            children.insert(idx, (c, new_node));
        }
    }
}