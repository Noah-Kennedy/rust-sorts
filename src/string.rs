use crate::burst_trie::TrieNode;

pub fn burst_sort(data: &mut Vec<String>) {
    let mut trie = TrieNode::new(0);

    while let Some(s) = data.pop() {
        trie.insert(s);
    }

    trie.merge(data);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[quickcheck]
    fn unstable_sort_equivalence(data: Vec<String>) {
        let mut expected = data.clone();
        let mut actual = data;

        expected.sort_unstable();
        burst_sort(&mut actual);

        assert_eq!(expected, actual)
    }
}