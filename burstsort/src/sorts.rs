use crate::alphanumeric::AlphaNumericTrieNode;

/// Sorts a vector of [AsRef<\[u8\]>] using a simple burstsort implementation.
///
/// When working with strings, keep in mind that this performs best on ASCII, but can perform
/// alright on other unicode.
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
pub fn byte_sort<T>(data: &mut Vec<T>) where T: AsRef<[u8]> {
    let mut trie = crate::byte::TrieNode::new(0);

    while let Some(s) = data.pop() {
        trie.insert(s);
    }

    trie.merge(data);
}

/// Like [byte_sort], but stable.
///
/// See [byte_sort] for more information.
pub fn byte_sort_unstable<T>(data: &mut Vec<T>) where T: AsRef<[u8]> {
    let mut trie = crate::byte::TrieNode::new(0);

    while let Some(s) = data.pop() {
        trie.insert(s);
    }

    trie.merge_unstable(data);
}

pub fn unicode_sort_unstable<T>(data: &mut Vec<T>) where T: AsRef<str> {
    let mut trie = crate::unicode::TrieNode::new(0);

    while let Some(s) = data.pop() {
        trie.insert(s);
    }

    trie.merge_unstable(data);
}

#[cfg(feature = "unstable")]
pub fn alphanumeric_sort(data: &mut Vec<String>) -> Result<(), u8> {
    let mut trie = crate::alphanumeric::make_trie();

    while let Some(s) = data.pop() {
        trie.insert(s)?;
    }

    trie.merge(data);

    Ok(())
}

#[cfg(feature = "unstable")]
pub fn alphanumeric_unchecked_sort(data: &mut Vec<String>) {
    let mut trie = crate::alphanumeric::make_trie();

    while let Some(s) = data.pop() {
        trie.insert_unchecked(s);
    }

    trie.merge(data);
}