const ALPHANUMERIC_BURST_LIMIT: usize = 8192;

pub fn make_trie() -> DynamicAlphaNumericNode {
    DynamicAlphaNumericNode::new(0)
}

pub trait AlphaNumericTrieNode {
    fn insert(&mut self, s: String) -> Result<(), u8>;
    fn insert_silent(&mut self, s: String);
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

    fn insert_silent(&mut self, s: String) {
        match &mut self.kind {
            AlphaNumericNodeKind::Burst(inner) => {
                inner.insert_silent(s)
            }
            AlphaNumericNodeKind::Collapsed(inner) => {
                inner.insert_silent(s);

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

    fn insert_silent(&mut self, s: String) {
        if let Some(&c) = s.as_bytes().get(self.offset) {
            if c.is_ascii_alphanumeric() {
                self.bucket.push(s);
            }
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
                let index = to_alphanumeric_index(c) as usize;

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

    fn insert_silent(&mut self, s: String) {
        if let Some(&c) = s.as_bytes().get(self.offset) {
            if c.is_ascii_alphanumeric() {
                let index = to_alphanumeric_index(c) as usize;

                self.children[index].insert_silent(s);
            }
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

const fn make_const_table() -> [u8; 62] {
    let mut out = [0; 62];

    let mut i = 0;

    while i < out.len() as u8 {
        if i.is_ascii_alphanumeric() {
            out[i as usize] = compute_index(i);
        }

        i += 1;
    }

    out
}

#[inline(always)]
const fn compute_index(c: u8) -> u8 {
    if c < 65 {
        // if digit, shift to 0-9
        c - 48
    } else if c < 97 {
        // if uppercase, shift to 10-35
        c - 65 + 9
    } else {
        // if lowercase, shift to 36-61
        c - 97 + 9 + 26
    }
}

#[inline(always)]
const fn to_alphanumeric_index(c: u8) -> u8 {
    const TABLE: [u8; 62] = make_const_table();
    TABLE[c as usize]
}