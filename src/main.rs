use std::collections::HashMap;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to open stdin!");
    let split: Vec<&str> = input.split(' ').collect();

    let word_map = split
        .iter()
        .filter_map(|c| strip_punctuation(c))
        .fold(HashMap::new(), |map, word| map.increment(word));

    println!("{:?}", word_map);
}

fn strip_punctuation(word: &str) -> Option<String> {
    let s: String = word
        .chars()
        .filter_map(|c| match c {
            ',' | ';' | '{' | '}' | '[' | ']' | '(' | ')' | '$' | '#' | '\'' | '\n' | '\0' => None,
            c => Some(c),
        })
        .collect();
    if s.len() == 0 {
        None
    } else {
        Some(s)
    }
}

trait Incrementable {
    fn increment(self, word: String) -> Self;
}

impl Incrementable for HashMap<String, usize> {
    fn increment(mut self, word: String) -> Self {
        if let Some(v) = self.get_mut(&word) {
            *v += 1;
        } else {
            self.insert(word, 1);
        }
        self
    }
}
