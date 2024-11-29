// This function returns the index of the first space in given string
fn get_word(s: String) -> usize {
    for (i, &b) in s.as_bytes().iter().enumerate() {
        if b == b' ' {
            return i;
        }
    }
    return s.len()
}

// Rust can cut slice from a String
// By using &str as parameter, this function can receive String and &str variable
// A little trick
fn get_word_by_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}