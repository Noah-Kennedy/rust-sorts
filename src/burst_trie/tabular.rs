const BURST_LIMIT: usize = 8192;

pub struct TrieNode {
    pub offset: usize,
    pub data: NodeKind,
}

pub enum NodeKind {
    Bucket(BucketNode),
    Table(TableNode),
}

pub struct BucketNode {
    pub bucket: Vec<String>,
}

pub struct TableNode {
    pub matches: Vec<String>,
    pub table: Vec<TrieNode>,
}

impl TrieNode {
    pub fn new(offset: usize) -> Self {
        Self {
            offset,
            data: NodeKind::Bucket(BucketNode {
                bucket: Vec::new()
            }),
        }
    }

    pub fn insert(&mut self, s: String) {
        match &mut self.data {
            NodeKind::Table(data) => data.insert(s, self.offset),
            NodeKind::Bucket(data) => {
                data.insert(s);

                if data.should_burst() {
                    let table = data.burst(self.offset);
                    self.data = NodeKind::Table(table);
                }
            }
        }
    }

    pub fn merge(self, target: &mut Vec<String>) {
        match self.data {
            NodeKind::Bucket(bucket) => bucket.merge(target, self.offset),
            NodeKind::Table(table) => table.merge(target),
        }
    }
}

impl BucketNode {
    pub fn insert(&mut self, s: String) {
        self.bucket.push(s);
    }

    pub fn should_burst(&self) -> bool {
        self.bucket.len() >= BURST_LIMIT
    }

    pub fn burst(&mut self, off: usize) -> TableNode {
        let mut table = TableNode::new(off);

        while let Some(item) = self.bucket.pop() {
            table.insert(item, off);
        }

        table
    }

    pub fn merge(mut self, target: &mut Vec<String>, offset: usize) {
        self.bucket.sort_by(|l, r| {
            l.as_bytes()[offset..].cmp(&r.as_bytes()[offset..])
        });
        target.append(&mut self.bucket)
    }
}

impl TableNode {
    pub fn new(offset: usize) -> Self {
        let mut table = Vec::new();

        for _ in 0..255 {
            table.push(TrieNode::new(offset + 1));
        }

        Self { matches: Vec::new(), table }
    }

    pub fn insert(&mut self, s: String, off: usize) {
        let raw_form = s.as_bytes();
        if off == raw_form.len() {
            self.matches.push(s)
        } else {
            let index = raw_form[off] as usize;
            self.table[index].insert(s);
        }
    }

    pub fn merge(mut self, target: &mut Vec<String>) {
        target.append(&mut self.matches);

        for entry in self.table {
            entry.merge(target)
        }
    }
}