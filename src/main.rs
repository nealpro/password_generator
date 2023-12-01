use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let path: &str = "/usr/share/dict/words";
    let words: Vec<String> = read_and_filter_words(path).expect("Failed to read words");

    let password: String = generate_password(&words, 3);
    println!("Generated password: {}", password);
}

fn read_and_filter_words<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let words: Vec<String> = reader
        .lines()
        .filter_map(Result::ok)
        .filter(|word| word.len() >= 5)
        .collect();

    Ok(words)
}

fn generate_password(word_list: &[String], word_count: usize) -> String {
    let mut rng = thread_rng();
    let mut password = String::new();

    for i in 0..word_count {
        let word: &String = word_list.choose(&mut rng).unwrap();
        password.push_str(word);
        if i < word_count - 1 {
            password.push_str(" ");
            // password.push_str("-");
        } else {
            password.push_str("#");
            // password.push_str("!");
        }
    }

    password
}
