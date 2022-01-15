# Wordle solver

A short Rust program to suggest words to play in [Wordle](https://www.powerlanguage.co.uk/wordle/) (hard mode), a fantastic word game.

You may need to point it to a different dictionary file if you don't have a dictionary at `/usr/share/dict/american-english`.

To run it: `cargo run --release`.

It chooses words so as to minimize the expected logarithm of the number of remaining words. It typically solves in three guesses.

I wrote this because I was wondering which word to start with. It suggests "tares".
