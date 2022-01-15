//! A solver for Wordle puzzles.
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;

/// Represents whether a particular position of a played word
/// matches the hidden word.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Match {
    /// The letter is an exact match. Shown in green on Wordle.
    Exact,
    /// The letter is in the wrong position. Shown in ochre on Wordle.
    WrongPosition,
    /// The letter is not present in the hidden word. Shown in grey on Wordle.
    Missing,
}

/// The matches for a particular played word.
type Matches = [Match; 5];

/// Determines the matches for a particular played word.
fn get_match(played_word: &str, hidden_word: &str) -> Matches {
    let mut matches = [Match::Missing; 5];
    for (pos, char) in played_word.chars().enumerate() {
        matches[pos] = if char == hidden_word.chars().nth(pos).unwrap() {
            Match::Exact
        } else if hidden_word.contains(char) {
            Match::WrongPosition
        } else {
            Match::Missing
        };
    }
    matches
}

/// Chooses a word to play according to the entropy objective.
fn get_word_to_play(words: &[String]) -> Option<String> {
    words
        .iter()
        .map(|word| {
            let mut match_counts = HashMap::new();
            for hidden_word in words {
                *match_counts
                    .entry(get_match(word, hidden_word))
                    .or_insert(0) += 1;
            }
            (
                match_counts
                    .values()
                    .map(|&v| (v as f32) * (v as f32).log2())
                    .sum::<f32>(),
                word.clone(),
            )
        })
        .reduce(|x, y| if x.0 < y.0 { x } else { y })
        .map(|x| x.1)
}

/// Asks the user which letters of the played word matched.
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
        if let Some(word_to_play) = get_word_to_play(&words) {
            println!("There are {} possible words.", words.len());
            if words.len() < 50 {
                println!("Possible words: {}", words.join(" "));
            }
            println!("Play the word: {}", word_to_play);
            let matches = read_matches();
            words.retain(|w| get_match(&word_to_play, w) == matches);
        } else {
            println!("There are no matching words.");
            break;
        }
    }
}
