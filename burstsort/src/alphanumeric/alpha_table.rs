const TABLE_SIZE: usize = 127;
const ALPHANUMERIC_INDEX_TABLE: [usize; TABLE_SIZE] = make_alpha_table();

const fn make_alpha_table() -> [usize; TABLE_SIZE] {
    let mut out = [255; TABLE_SIZE];

    let mut i = 0;

    while i < out.len() as u8 {
        if i.is_ascii_alphanumeric() {
            out[i as usize] = precompute_index(i);
        }

        i += 1;
    }

    out
}

#[inline(always)]
const fn precompute_index(c: u8) -> usize {
    if c < b'A' {
        // if digit, shift to 0-9
        c as usize - b'0' as usize
    } else if c < 97 {
        // if uppercase, shift to 10-35
        c as usize - b'A' as usize + 10
    } else {
        // if lowercase, shift to 36-61
        c as usize - b'a' as usize + 10 + 26
    }
}

#[inline(always)]
pub const fn lookup_alpha_index(c: u8) -> usize {
    ALPHANUMERIC_INDEX_TABLE[c as usize]
}
