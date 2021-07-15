use crate::alphanumeric::AlphaNumericTrieNode;

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

pub fn alphanumeric_sort(data: &mut Vec<String>) -> Result<(), u8> {
    let mut trie = crate::alphanumeric::make_trie();

    while let Some(s) = data.pop() {
        trie.insert(s)?;
    }

    trie.merge(data);

    Ok(())
}

pub fn alphanumeric_silent_sort(data: &mut Vec<String>) {
    let mut trie = crate::alphanumeric::make_trie();

    while let Some(s) = data.pop() {
        trie.insert_unchecked(s);
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

    #[test]
    fn simple_alpha() {
        let data = vec![
            "cat".to_string(),
            "apple".to_string(),
            "jackal".to_string(),
            "si7568ver".to_string(),
            "b4t".to_string(),
            "appls4uce".to_string(),
            "ostr1tch".to_string(),
            "sdghdfg567gdfh5".to_string(),
        ];

        let mut expected = data.clone();
        let mut actual = data;

        expected.sort_unstable();
        alphanumeric_sort(&mut actual).unwrap();

        assert_eq!(expected, actual)
    }
}