pub fn tabular_burst_sort(data: &mut Vec<String>) {
    let mut trie = crate::tabular::TrieNode::new(0);

    while let Some(s) = data.pop() {
        trie.insert(s);
    }

    trie.merge(data);
}

pub fn dynamic_burst_sort(data: &mut Vec<String>) {
    let mut trie = crate::dynamic::TrieNode::new(0);

    while let Some(s) = data.pop() {
        trie.insert(s);
    }

    trie.merge(data);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[quickcheck]
    fn check_tabular(data: Vec<String>) {
        let mut expected = data.clone();
        let mut actual = data;

        expected.sort_unstable();
        tabular_burst_sort(&mut actual);

        assert_eq!(expected, actual)
    }

    #[quickcheck]
    fn check_dynamic(data: Vec<String>) {
        let mut expected = data.clone();
        let mut actual = data;

        expected.sort_unstable();
        dynamic_burst_sort(&mut actual);

        assert_eq!(expected, actual)
    }

    #[test]
    fn simple_dynamic() {
        let data = vec![
            "cat".to_string(),
            "apple".to_string(),
            "jackal".to_string(),
            "silver".to_string(),
            "bat".to_string(),
            "applesauce".to_string(),
            "ostritch".to_string(),
        ];

        let mut expected = data.clone();
        let mut actual = data;

        expected.sort_unstable();
        dynamic_burst_sort(&mut actual);

        assert_eq!(expected, actual)
    }
}