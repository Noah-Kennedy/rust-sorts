use alpha_table::lookup_alpha_index;

mod alpha_table;

const ALPHANUMERIC_BURST_LIMIT: usize = 8192;

pub fn make_trie() -> DynamicAlphaNumericNode {
    DynamicAlphaNumericNode::new(0)
}

pub trait AlphaNumericTrieNode {
    fn insert(&mut self, s: String) -> Result<(), u8>;
    fn insert_unchecked(&mut self, s: String);
    fn merge(&mut self, target: &mut Vec<String>);
}

pub struct DynamicAlphaNumericNode {
    kind: AlphaNumericNodeKind,
}

impl DynamicAlphaNumericNode {
    fn new(offset: usize) -> Self {
        Self {
            kind: AlphaNumericNodeKind::Collapsed(CollapsedAlphaNumericNode::new(offset))
        }
    }
}

impl AlphaNumericTrieNode for DynamicAlphaNumericNode {
    fn insert(&mut self, s: String) -> Result<(), u8> {
        match &mut self.kind {
            AlphaNumericNodeKind::Burst(inner) => {
                inner.insert(s)
            }
            AlphaNumericNodeKind::Collapsed(inner) => {
                inner.insert(s)?;

                if inner.should_burst() {
                    self.kind = AlphaNumericNodeKind::Burst(inner.burst())
                }

                Ok(())
            }
        }
    }

    fn insert_unchecked(&mut self, s: String) {
        match &mut self.kind {
            AlphaNumericNodeKind::Burst(inner) => {
                inner.insert_unchecked(s)
            }
            AlphaNumericNodeKind::Collapsed(inner) => {
                inner.insert_unchecked(s);

                if inner.should_burst() {
                    self.kind = AlphaNumericNodeKind::Burst(inner.burst())
                }
            }
        }
    }

    fn merge(&mut self, target: &mut Vec<String>) {
        match &mut self.kind {
            AlphaNumericNodeKind::Burst(inner) => {
                inner.merge(target)
            }
            AlphaNumericNodeKind::Collapsed(inner) => {
                inner.merge(target)
            }
        }
    }
}

enum AlphaNumericNodeKind {
    Burst(BurstAlphaNumericNode),
    Collapsed(CollapsedAlphaNumericNode),
}

struct CollapsedAlphaNumericNode {
    offset: usize,
    matches: Vec<String>,
    bucket: Vec<String>,
}

impl AlphaNumericTrieNode for CollapsedAlphaNumericNode {
    fn insert(&mut self, s: String) -> Result<(), u8> {
        if let Some(&c) = s.as_bytes().get(self.offset) {
            if c.is_ascii_alphanumeric() {
                self.bucket.push(s);
                Ok(())
            } else {
                Err(c)
            }
        } else {
            self.matches.push(s);
            Ok(())
        }
    }

    fn insert_unchecked(&mut self, s: String) {
        if s.len() > self.offset {
            self.bucket.push(s);
        } else {
            self.matches.push(s);
        }
    }

    fn merge(&mut self, target: &mut Vec<String>) {
        target.append(&mut self.matches);
        self.bucket.sort_unstable();
        target.append(&mut self.bucket);
    }
}

impl CollapsedAlphaNumericNode {
    fn new(offset: usize) -> Self {
        Self {
            offset,
            matches: Vec::new(),
            bucket: Vec::new(),
        }
    }

    fn should_burst(&self) -> bool {
        self.bucket.len() >= ALPHANUMERIC_BURST_LIMIT
    }

    fn burst(&mut self) -> BurstAlphaNumericNode {
        let mut new_node = BurstAlphaNumericNode::new(self.offset);

        std::mem::swap(&mut self.matches, &mut new_node.matches);

        while let Some(item) = self.bucket.pop() {
            new_node.insert(item).unwrap();
        }

        new_node
    }
}

struct BurstAlphaNumericNode {
    offset: usize,
    matches: Vec<String>,
    children: [Box<DynamicAlphaNumericNode>; 62],
}

impl AlphaNumericTrieNode for BurstAlphaNumericNode {
    fn insert(&mut self, s: String) -> Result<(), u8> {
        if let Some(&c) = s.as_bytes().get(self.offset) {
            if c.is_ascii_alphanumeric() {
                let index = lookup_alpha_index(c) as usize;

                self.children[index].insert(s)?;

                Ok(())
            } else {
                Err(c)
            }
        } else {
            self.matches.push(s);

            Ok(())
        }
    }

    fn insert_unchecked(&mut self, s: String) {
        if let Some(&c) = s.as_bytes().get(self.offset) {
            let index = lookup_alpha_index(c) as usize;

            self.children[index].insert_unchecked(s);
        } else {
            self.matches.push(s);
        }
    }

    fn merge(&mut self, target: &mut Vec<String>) {
        target.append(&mut self.matches);

        for c in self.children.iter_mut() {
            c.merge(target);
        }
    }
}

impl BurstAlphaNumericNode {
    fn new(offset: usize) -> Self {
        let children = array_init::array_init(
            |_| Box::new(DynamicAlphaNumericNode::new(offset + 1))
        );

        Self {
            offset,
            matches: Vec::new(),
            children,
        }
    }
}