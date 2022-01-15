# Wordle solver

A short Rust program to suggest words to play in [Wordle](https://www.powerlanguage.co.uk/wordle/) (hard mode), a fantastic word game.

<img src="https://user-images.githubusercontent.com/3873851/149624092-61312053-af80-4b19-8e31-d715536d0f88.png" width="200">

To run it: `cargo run --release`.

It chooses words so as to minimize the expected logarithm of the number of remaining words. It typically solves in three guesses.

I wrote this because I was wondering which word to start with. It suggests "raise" (and considers the worst starting word to be "mamma").
