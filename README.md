# Wordle solver

A short Rust program to suggest words to play in [Wordle](https://www.powerlanguage.co.uk/wordle/) (hard mode), a fantastic word game.

<img src="https://user-images.githubusercontent.com/3873851/149624092-61312053-af80-4b19-8e31-d715536d0f88.png" width="200">

Example usage:

```
$ cargo run --release
There are 2315 possible words.
Play the word: raise
For each position 1-5, indicate whether there was:
    an exact match [e],
    a match but in the wrong position [w],
    or no match [n]
For example, if there were an exact match in the first position and no remaining matches, enter 'ennnn'.
newnn
There are 14 possible words.
Possible words: maxim panic tacit valid patio mania magic cavil cabin mafia vapid manic cacti habit
Play the word: panic
For each position 1-5, indicate whether there was:
    an exact match [e],
    a match but in the wrong position [w],
    or no match [n]
For example, if there were an exact match in the first position and no remaining matches, enter 'ennnn'.
eeeee
You win!
```

It chooses words so as to minimize the expected logarithm of the number of remaining words (see the information-theoretic justification [here](https://langproc.substack.com/p/information-theoretic-analysis-of)). This is just a heuristic -- for optimality, we would need to [consider the game tree](http://sonorouschocolate.com/notes/index.php?title=The_best_strategies_for_Wordle). It typically solves in three or four guesses.

I wrote this because I was wondering which word to start with. It suggests "raise" (and considers the worst starting word to be "mamma").
