use std::borrow::Borrow;
use std::marker::PhantomData;

#[cfg(feature = "parallelization")]
use rayon::prelude::ParallelSliceMut;

/// Tuning configuration for burstsort.
pub struct BurstConfig {
    /// Threshold after which nodes are burst.
    pub burst_limit: usize,
    /// Initial allocation capacity of storage vectors.
    pub initial_capacity: usize,
    /// Number of radix buckets.
    pub classes: usize,
    /// Hints to the algorithm that items may be long.
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
    where C: Borrow<BurstConfig> + Clone + Send + Sync,
          T: PartialEq + AsRef<[I]> + Clone + Ord + Send + Sync,
          I: Into<usize> + Clone + Ord + Send + Sync
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
        let cap = self.config.borrow().initial_capacity;

        if let Some(radix) = item.as_ref().get(self.level).cloned() {
            let radix = radix.into();

            match &mut self.inner {
                TrieNodeKind::List(list) => {
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
                                matches: Vec::new(),
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
            // pre-allocate if this is a "fresh" matches list
            if cap > 0 && self.matches.is_empty() {
                self.matches.reserve(cap);
            }

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

    #[cfg(feature = "parallelization")]
    pub fn par_merge(&mut self, target: &mut Vec<T>) {
        rayon::scope(|s| {
            self.par_sort(s);
        });

        self.merge_sorted(target);
    }

    #[cfg(feature = "parallelization")]
    fn merge_sorted(&mut self, target: &mut Vec<T>) {
        target.append(&mut self.matches);

        match &mut self.inner {
            TrieNodeKind::List(list) => {
                target.append(list);
            }
            TrieNodeKind::Burst(table) => {
                for x in table.iter_mut() {
                    x.merge_sorted(target)
                }
            }
        }
    }

    #[cfg(feature = "parallelization")]
    fn par_sort<'scope>(&'scope mut self, scope: &rayon::Scope<'scope>) {
        let long = self.config.borrow().hint_long;
        let level = self.level;
        match &mut self.inner {
            TrieNodeKind::List(list) => {
                if !list.is_empty() {
                    scope.spawn(move |_| {
                        if long {
                            list.par_sort_unstable_by(|lhs, rhs| {
                                let lhs_remaining = &lhs.as_ref()[level..];
                                let rhs_remaining = &rhs.as_ref()[level..];
                                lhs_remaining.cmp(rhs_remaining)
                            });
                        } else {
                            list.par_sort_unstable();
                        }
                    })
                }
            }
            TrieNodeKind::Burst(table) => {
                scope.spawn(move |s| {
                    for x in table.iter_mut() {
                        x.par_sort(s);
                    }
                });
            }
        }
    }
}