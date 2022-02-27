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
///
/// See `tests::test_get_match`.
fn get_match(played_word: &str, hidden_word: &str) -> Matches {
    let mut matches = [Match::Missing; 5];
    for (pos, char) in played_word.chars().enumerate() {
        if char == hidden_word.chars().nth(pos).unwrap() {
            matches[pos] = Match::Exact;
        }
    }
    for (pos, char) in played_word.chars().enumerate() {
        if matches[pos] == Match::Exact {
            continue;
        }
        for (other_pos, other_char) in hidden_word.chars().enumerate() {
            if char == other_char && matches[other_pos] != Match::Exact {
                matches[pos] = Match::WrongPosition;
            }
        }
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
    'get_input: loop {
        let mut line = String::new();
        println!("For each position 1-5, indicate whether there was:
    an exact match [{e}],
    a match but in the wrong position [{w}],
    or no match [{n}]
For example, if there were an exact match in the first position and no remaining matches, enter '{e}{n}{n}{n}{n}'.", e=e, n=n, w=w);
        std::io::stdin().read_line(&mut line).unwrap();
        let characters: [u8; 5] = match line.trim().to_ascii_lowercase().as_bytes().try_into() {
            Ok(chars) => chars,
            Err(_) => continue,
        };
        for (pos_match, b) in matches.iter_mut().zip(characters) {
            *pos_match = match b as char {
                'e' => Match::Exact,
                'w' => Match::WrongPosition,
                'n' => Match::Missing,
                _ => continue 'get_input,
            }
        }
        return matches;
    }
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
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() >= 2 {
        assert!(args[1] == "--quordle");
        quordle_solver();
        return;
    }
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

/// Chooses a word to play according to the entropy objective.
fn get_word_to_play_quordle(
    quad_words: &[Option<Vec<String>>; 4],
    dict: &[String],
) -> Option<String> {
    dict.par_iter()
        .map(|word| {
            let mut match_counts = HashMap::new();
            let mut cum_ent = 0.0;
            for words in quad_words.iter().flatten() {
                for hidden_word in words {
                    *match_counts
                        .entry(get_match(word, hidden_word))
                        .or_insert(0) += 1;
                }
                let ent = match_counts
                    .values()
                    .map(|&v| (v as f32) * (v as f32).log2())
                    .sum::<f32>();
                cum_ent += ent;
            }
            (cum_ent, word.clone())
        })
        .reduce_with(|x, y| if x.0 < y.0 { x } else { y })
        .map(|x| x.1)
}
fn quordle_solver() {
    let mut dict = read_words("wordle_hidden_words.txt");
    let mut quad_words = [
        Some(dict.clone()),
        Some(dict.clone()),
        Some(dict.clone()),
        Some(dict.clone()),
    ];
    loop {
        if let Some(word_to_play) = get_word_to_play_quordle(&quad_words, &dict) {
            println!("Play the word: {}", word_to_play.bold());
            let mut correct = true;
            for (quad_num, words_maybe) in quad_words.iter_mut().enumerate() {
                if let Some(words) = words_maybe {
                    println!("input matches for quad {}:", quad_num + 1);
                    let matches = read_matches();
                    words.retain(|w| get_match(&word_to_play, w) == matches);
                    println!("{} words left in quad {}", words.len(), quad_num + 1);
                    if matches != [Match::Exact; 5] {
                        correct = false;
                    } else {
                        *words_maybe = None;
                    }
                }
            }
            if correct {
                println!("You win!");
                break;
            }
            dict.retain(|w| {
                quad_words
                    .iter()
                    .any(|words| words.as_ref().map_or(false, |words| words.contains(w)))
                    && w != &word_to_play
            });
        } else {
            println!("There are no matching words.");
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_get_match() {
        // Note that the 2nd "o" of broom is not reported as being in the wrong position.
        assert_eq!(
            get_match("broom", "proxy"),
            [
                Match::Missing,
                Match::Exact,
                Match::Exact,
                Match::Missing,
                Match::Missing,
            ]
        );
    }
}
