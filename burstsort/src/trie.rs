use std::borrow::Borrow;
use std::marker::PhantomData;

/// Tuning configuration for burstsort.
pub struct BurstConfig {
    /// Threshold after which nodes are burst.
    pub burst_limit: usize,
    /// Initial allocation capacity of storage vectors.
    pub initial_capacity: usize,
    /// Number of radix buckets.
    pub classes: usize,
    /// Hints to the algorithm that items may be long.
    /// Of all the config parameters, this is the one you should look at tuning.
    pub hint_long: bool,
}

#[derive(Clone)]
pub struct TrieNode<C, T, I> {
    level: usize,
    config: C,
    matches: Vec<T>,
    inner: TrieNodeKind<C, T, I>,
    _phantom: PhantomData<I>,
}

#[derive(Clone)]
pub enum TrieNodeKind<C, T, I> {
    List(Vec<T>),
    Burst(Vec<TrieNode<C, T, I>>),
}

impl<C, T, I> TrieNode<C, T, I>
    where C: Borrow<BurstConfig> + Clone,
          T: PartialEq + AsRef<[I]> + Clone + Ord,
          I: Into<usize> + Clone + Ord
{
    pub fn root(config: C) -> Self {
        Self {
            level: 0,
            matches: Vec::with_capacity(config.borrow().initial_capacity),
            inner: TrieNodeKind::List(Vec::with_capacity(config.borrow().initial_capacity)),
            config,
            _phantom: Default::default(),
        }
    }

    pub fn insert(&mut self, item: T) {
        if let Some(radix) = item.as_ref().get(self.level).cloned() {
            let radix = radix.into();

            match &mut self.inner {
                TrieNodeKind::List(list) => {
                    let cap = self.config.borrow().initial_capacity;

                    // pre-allocate if this is a "fresh" list node
                    if cap > 0 && list.is_empty() {
                        list.reserve(cap);
                    }

                    list.push(item);

                    if list.len() > self.config.borrow().burst_limit {
                        // burst
                        let mut table = vec![
                            Self {
                                level: self.level + 1,
                                config: self.config.clone(),
                                matches: Vec::with_capacity(cap),
                                inner: TrieNodeKind::List(Vec::new()),
                                _phantom: PhantomData::default(),
                            };
                            self.config.borrow().classes];

                        for x in list.drain(..) {
                            let radix = x.as_ref()[self.level].clone().into();
                            table[radix].insert(x);
                        }

                        self.inner = TrieNodeKind::Burst(table)
                    }
                }
                TrieNodeKind::Burst(table) => {
                    table[radix].insert(item)
                }
            }
        } else {
            self.matches.push(item)
        }
    }

    pub fn merge(&mut self, target: &mut Vec<T>) {
        // append exact matches for node first
        target.append(&mut self.matches);

        match &mut self.inner {
            TrieNodeKind::List(list) => {
                // now sort internal collection and append

                // if arrays may be long, best to only sort the remaining elements
                if self.config.borrow().hint_long {
                    let level = self.level;

                    list.sort_unstable_by(|lhs, rhs| {
                        let lhs_remaining = &lhs.as_ref()[level..];
                        let rhs_remaining = &rhs.as_ref()[level..];
                        lhs_remaining.cmp(rhs_remaining)
                    });
                } else {
                    list.sort_unstable();
                }

                target.append(list);
            }
            TrieNodeKind::Burst(table) => {
                // sequentially merge each table entry
                for x in table.iter_mut() {
                    x.merge(target)
                }
            }
        }
    }
}