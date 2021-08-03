const BURST_LIMIT: usize = 8192;

pub struct TrieNode<T> {
    pub offset: usize,
    pub data: NodeKind<T>,
}

pub enum NodeKind<T> {
    Bucket(BucketNode<T>),
    Table(TableNode<T>),
}

pub struct BucketNode<T> {
    pub bucket: Vec<T>,
}

pub struct TableNode<T> {
    pub matches: Vec<T>,
    pub table: Vec<TrieNode<T>>,
}

impl<T> TrieNode<T> where T: AsRef<[u8]> {
    pub fn new(offset: usize) -> Self {
        Self {
            offset,
            data: NodeKind::Bucket(BucketNode {
                bucket: Vec::new()
            }),
        }
    }

    pub fn insert(&mut self, s: T) {
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

    pub fn merge(&mut self, target: &mut Vec<T>) {
        match &mut self.data {
            NodeKind::Bucket(bucket) => bucket.merge(target, self.offset),
            NodeKind::Table(table) => table.merge(target),
        }
    }

    pub fn merge_unstable(&mut self, target: &mut Vec<T>) {
        match &mut self.data {
            NodeKind::Bucket(bucket) => bucket.merge_unstable(target, self.offset),
            NodeKind::Table(table) => table.merge_unstable(target),
        }
    }
}

impl<T> BucketNode<T> where T: AsRef<[u8]> {
    pub fn insert(&mut self, s: T) {
        self.bucket.push(s);
    }

    pub fn should_burst(&self) -> bool {
        self.bucket.len() >= BURST_LIMIT
    }

    pub fn burst(&mut self, off: usize) -> TableNode<T> {
        let mut table = TableNode::new(off);

        while let Some(item) = self.bucket.pop() {
            table.insert(item, off);
        }

        table
    }

    pub fn merge(&mut self, target: &mut Vec<T>, offset: usize) {
        self.bucket.sort_by(|l, r| {
            l.as_ref()[offset..].cmp(&r.as_ref()[offset..])
        });
        target.append(&mut self.bucket)
    }

    pub fn merge_unstable(&mut self, target: &mut Vec<T>, offset: usize) {
        self.bucket.sort_unstable_by(|l, r| {
            l.as_ref()[offset..].cmp(&r.as_ref()[offset..])
        });
        target.append(&mut self.bucket)
    }
}

impl<T> TableNode<T> where T: AsRef<[u8]> {
    pub fn new(offset: usize) -> Self {
        let mut table = Vec::new();

        for _ in 0..255 {
            table.push(TrieNode::new(offset + 1));
        }

        Self { matches: Vec::new(), table }
    }

    pub fn insert(&mut self, s: T, off: usize) {
        let raw_form = s.as_ref();
        if off == raw_form.len() {
            self.matches.push(s)
        } else {
            let index = raw_form[off] as usize;
            self.table[index].insert(s);
        }
    }

    pub fn merge(&mut self, target: &mut Vec<T>) {
        target.append(&mut self.matches);

        for entry in self.table.iter_mut() {
            entry.merge(target)
        }
    }

    pub fn merge_unstable(&mut self, target: &mut Vec<T>) {
        target.append(&mut self.matches);

        for entry in self.table.iter_mut() {
            entry.merge_unstable(target)
        }
    }
}