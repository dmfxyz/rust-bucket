// Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.
// A slice is a kind of reference, so it does not have ownership.

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    
    let first_word = first_word_1(&input);
    println!("{}", first_word);

    let first_word = first_word_2(&input);
    println!("{}", first_word);
    
}

fn first_word_1(s: &String) -> String {
    let first_word = match s.split(" ").next() {
        Some(word) => word,
        None => "",
    };
    first_word.to_string()

}

// This function is not that useful. it returns a usize that only
// applies to a borrowed string S, which could fall out of scope
// at any time in the future
#[allow(dead_code)]
fn first_word_book(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_2(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

