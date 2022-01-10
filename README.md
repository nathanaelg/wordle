## Wordle bot
https://powerlanguage.co.uk/wordle/

### Strategy
From the set of remaining words, select the guess which will result in the smallest (on average) remaining guesses. Because the average remaining guesses can be the same for a number of guesses and `HashMap.iter()` has an arbitrary order, the guesses will not always be the same between test runs.

### Running
```bash
# Build binary
$ cargo build --release

# Run with single word
$ target/debug/wordle-rust gorge
Word 1: gorge
Guess 1: raise 🟨⬜⬜⬜🟩 [40]
Guess 2: prove ⬜🟨🟨⬜🟩 [7]
Guess 3: forge ⬜🟩🟩🟩🟩 [1]
Guess 4: gorge 🟩🟩🟩🟩🟩 [1]
Guess statistics: {4: 1}

# Run through all 2315 words
$ target/debug/wordle-rust
Word 1: aback
Guess 1: raise ⬜🟨⬜⬜⬜ [92]
Guess 2: cloak 🟨⬜⬜🟨🟩 [4]
Guess 3: quack ⬜⬜🟩🟩🟩 [3]
Guess 4: knack 🟨⬜🟩🟩🟩 [2]
Guess 5: aback 🟩🟩🟩🟩🟩 [1]
Guess statistics: {5: 1}

...

Word 2315: zonal
Guess 1: raise ⬜🟨⬜⬜⬜ [92]
Guess 2: cloak ⬜🟨🟨🟩⬜ [6]
Guess 3: tonal ⬜🟩🟩🟩🟩 [1]
Guess 4: zonal 🟩🟩🟩🟩🟩 [1]
Guess statistics: {3: 923, 1: 1, 8: 3, 4: 974, 5: 221, 6: 49, 7: 14, 2: 130}
```
