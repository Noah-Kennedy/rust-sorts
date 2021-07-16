use crate::alphanumeric::AlphaNumericTrieNode;

/// Sorts a vector of strings using a simple burstsort implementation.
/// Data is sorted as if the string were simply an array of bytes.
///
/// This performs best on ASCII, but can perform alright on other unicode.
/// Just don't expect anything resembling a lexicographic ordering on non-ascii text.
///
/// # Arguments
///
/// * `data`: Mutable vector reference of data to be sorted.
///
/// returns: ()
///
/// # Examples
///
/// ```
/// use burstsort::byte_sort;
///
/// let mut d = vec![
///     "cat".to_string(),
///     "12345".to_string(),
///     "Bat".to_string(),
///     "apple".to_string(),
///     "π".to_string(),
/// ];
///
/// byte_sort(&mut d);
/// assert_eq!(
///     vec![
///         "12345".to_string(),
///         "Bat".to_string(),
///         "apple".to_string(),
///         "cat".to_string(),
///         "π".to_string(),
///     ],
///     d
/// );
/// ```
pub fn byte_sort(data: &mut Vec<String>) {
    let mut trie = crate::tabular::TrieNode::new(0);

    while let Some(s) = data.pop() {
        trie.insert(s);
    }

    trie.merge(data);
}


///
///
/// # Arguments
///
/// * `data`:
///
/// returns: ()
///
/// # Examples
///
/// ```
///
/// ```
pub fn unicode_sort(data: &mut Vec<String>) {
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

pub fn alphanumeric_unchecked_sort(data: &mut Vec<String>) {
    let mut trie = crate::alphanumeric::make_trie();

    while let Some(s) = data.pop() {
        trie.insert_unchecked(s);
    }

    trie.merge(data);
}