//! A solver for Wordle puzzles.
use colored::Colorize;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
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
        .par_iter()
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
        .reduce_with(|x, y| if x.0 < y.0 { x } else { y })
        .map(|x| x.1)
}

/// Asks the user which letters of the played word matched.
fn read_matches() -> Matches {
    let mut matches = [Match::Missing; 5];
    let e = "e".white().on_truecolor(106, 170, 100);
    let w = "w".white().on_truecolor(201, 180, 88);
    let n = "n".white().on_truecolor(120, 124, 126);
    'a: loop {
        let mut line = String::new();
        println!("For each position 1-5, indicate whether there was:
    an exact match [{e}],
    a match but in the wrong position [{w}],
    or no match [{n}]
For example, if there were an exact match in the first position and no remaining matches, enter '{e}{n}{n}{n}{n}'.", e=e, n=n, w=w);
        std::io::stdin().read_line(&mut line).unwrap();
        line.pop(); // remove \n
        if line.len() != 5 {
            continue;
        }
        for (pos_match, char) in matches.iter_mut().zip(line.chars()) {
            if char == 'e' {
                *pos_match = Match::Exact;
            } else if char == 'w' {
                *pos_match = Match::WrongPosition;
            } else if char == 'n' {
                *pos_match = Match::Missing;
            } else {
                continue 'a;
            }
        }
        break;
    }
    matches
}

/// Loads a dictionary.
fn read_words(path: &str) -> Vec<String> {
    let file = File::open(path).expect("could not read dictionary");
    std::io::BufReader::new(file)
        .lines()
        .flatten()
        .filter(|word| word.len() == 5 && word.bytes().all(|b| b.is_ascii_lowercase()))
        .collect::<Vec<_>>()
}
fn main() {
    let mut words = read_words("wordle_hidden_words.txt");
    loop {
        if let Some(word_to_play) = get_word_to_play(&words) {
            println!("There are {} possible words.", words.len());
            if words.len() < 50 {
                println!("Possible words: {}", words.join(" "));
            }
            println!("Play the word: {}", word_to_play.bold());
            let matches = read_matches();
            if matches == [Match::Exact; 5] {
                println!("You win!");
                break;
            }
            words.retain(|w| get_match(&word_to_play, w) == matches);
        } else {
            println!("There are no matching words.");
            break;
        }
    }
}
