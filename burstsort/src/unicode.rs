const BURST_LIMIT: usize = 8192;

pub struct TrieNode<T> {
    pub offset: usize,
    pub matches: Vec<T>,
    pub kind: NodeKind<T>,
}

pub enum NodeKind<T> {
    Collapsed(Vec<T>),
    Burst(Vec<(char, TrieNode<T>)>),
}

impl<T> TrieNode<T> where T: AsRef<str> {
    pub fn new(offset: usize) -> Self {
        Self {
            offset,
            matches: Vec::new(),
            kind: NodeKind::Collapsed(Vec::new()),
        }
    }

    pub fn insert(&mut self, s: T) {
        if let Some(c) = s.as_ref().chars().nth(self.offset) {
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
            let mut new_children: Vec<(char, TrieNode<T>)> = Vec::new();

            while let Some(child) = children.pop() {
                let k = child.as_ref().chars().nth(self.offset).unwrap();
                add_to_burst_node(&mut new_children, child, k, self.offset)
            }

            self.kind = NodeKind::Burst(new_children);
        } else {
            unreachable!()
        }
    }

    pub fn merge(&mut self, target: &mut Vec<T>) {
        target.append(&mut self.matches);

        match &mut self.kind {
            NodeKind::Collapsed(children) => {
                children.sort_by(|l, r| l.as_ref().cmp(r.as_ref()));
                target.append(children);
            }
            NodeKind::Burst(children) => {
                for (_, child) in children.iter_mut() {
                    child.merge(target)
                }
            }
        }
    }

    pub fn merge_unstable(&mut self, target: &mut Vec<T>) {
        target.append(&mut self.matches);

        match &mut self.kind {
            NodeKind::Collapsed(children) => {
                children.sort_unstable_by(|l, r| l.as_ref().cmp(r.as_ref()));
                target.append(children);
            }
            NodeKind::Burst(children) => {
                for (_, child) in children.iter_mut() {
                    child.merge_unstable(target)
                }
            }
        }
    }
}

#[inline(always)]
fn add_to_burst_node<T>(children: &mut Vec<(char, TrieNode<T>)>, s: T, c: char, offset: usize)
    where T: AsRef<str>
{
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