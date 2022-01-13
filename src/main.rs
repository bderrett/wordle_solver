use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Match {
    Exact,
    WrongPosition,
    Missing,
}

type Matches = [Match; 5];

fn get_match(played_word: &str, actual_word: &str) -> Matches {
    let mut matches = [Match::Missing; 5];
    for (pos, char) in played_word.chars().enumerate() {
        if char == actual_word.chars().nth(pos).unwrap() {
            matches[pos] = Match::Exact;
        } else if actual_word.contains(char) {
            matches[pos] = Match::WrongPosition;
        } else {
            matches[pos] = Match::Missing;
        }
    }
    matches
}

fn get_word_to_play(words: &[String]) -> String {
    let mut max_value = -f32::INFINITY;
    let mut best_word = None;
    for played_word in words {
        let mut match_counts = HashMap::new();
        for actual_word in words {
            *match_counts
                .entry(get_match(played_word, actual_word))
                .or_insert(0) += 1;
        }
        let value: f32 = match_counts.values().map(|&v| (v as f32).log2()).sum();
        if value > max_value {
            max_value = value;
            best_word = Some(played_word);
        }
    }
    best_word.unwrap().to_string()
}

fn read_matches() -> Matches {
    let mut matches = [Match::Missing; 5];
    for (pos, pos_match) in matches.iter_mut().enumerate() {
        loop {
            let mut line = String::new();
            println!("In position {}, was there an exact match [e], a match but in the wrong position [w], or no match [n]?", pos + 1);
            std::io::stdin().read_line(&mut line).unwrap();
            if let Some(char) = line.chars().next() {
                if char == 'e' {
                    *pos_match = Match::Exact;
                    break;
                } else if char == 'w' {
                    *pos_match = Match::WrongPosition;
                    break;
                } else if char == 'n' {
                    *pos_match = Match::Missing;
                    break;
                }
            }
        }
    }
    matches
}
fn main() {
    let file = File::open("/usr/share/dict/american-english").expect("could not read dictionary");
    let re = Regex::new(r"^[a-z]{5}$").unwrap();
    let mut words = std::io::BufReader::new(file)
        .lines()
        .flatten()
        .into_iter()
        .filter(|word| re.is_match(word))
        .collect::<Vec<_>>();
    loop {
        println!("There are {} possible words.", words.len());
        if words.len() < 50 {
            println!("Possible words: {}", words.join(" "));
        }
        let word_to_play = get_word_to_play(&words);
        println!("Play the word: {}", word_to_play);
        let matches = read_matches();
        words.retain(|w| get_match(&word_to_play, w) == matches);
    }
}
