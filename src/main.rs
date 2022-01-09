use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops;

struct Guess {
    green: Vec<char>,
    yellow: Vec<HashSet<char>>,
    grey: HashSet<char>,
}

fn evaluate_guess(guess: Vec<char>, correct: Vec<char>) -> Guess {
    let green = correct
        .iter()
        .zip(guess.iter())
        .map(|(&c, &g)| if c == g { c } else { '\0' })
        .collect();

    let correct_set: HashSet<char> = HashSet::from_iter(correct.iter().cloned());

    let yellow = correct
        .iter()
        .zip(guess.iter())
        .map(|(&c, &g)| {
            if c != g && correct_set.contains(&g) {
                HashSet::from([g])
            } else {
                HashSet::new()
            }
        })
        .collect();

    let guess_set: HashSet<char> = HashSet::from_iter(guess.iter().cloned());
    let grey = &guess_set - &correct_set;

    return Guess {
        green,
        yellow,
        grey,
    };
}

fn possible_word(word: Vec<char>, guess: &Guess) -> bool {
    let green = word
        .iter()
        .zip(guess.green.iter())
        .map(|(&w, &g)| g == '\0' || w == g)
        .reduce(|a, b| a && b)
        .unwrap();
    if !green {
        return false;
    }

    let word_set: HashSet<char> = HashSet::from_iter(word.iter().cloned());
    if !word_set.is_disjoint(&guess.grey) {
        return false;
    }

    let yellow_position = word
        .iter()
        .zip(guess.yellow.iter())
        .map(|(w, y)| !y.contains(w))
        .reduce(|a, b| a && b)
        .unwrap();
    if !yellow_position {
        return false;
    }

    let yellow_presence = guess
        .yellow
        .iter()
        .map(|y| word_set.is_superset(y))
        .reduce(|a, b| a && b)
        .unwrap();
    if !yellow_presence {
        return false;
    }
    return true;
}

impl ops::Add<Guess> for Guess {
    type Output = Guess;

    fn add(self, _rhs: Guess) -> Guess {
        let green = self
            .green
            .iter()
            .zip(_rhs.green.iter())
            .map(|(&g1, &g2)| if g1 == '\0' { g2 } else { g1 })
            .collect();

        let yellow = self
            .yellow
            .into_iter()
            .zip(_rhs.yellow.into_iter())
            .map(|(y1, y2)| y1.into_iter().chain(y2).collect())
            .collect();

        let grey = self.grey.into_iter().chain(_rhs.grey).collect();

        return Guess {
            green,
            yellow,
            grey,
        };
    }
}

fn main() {
    let file = File::open("words.txt").unwrap();
    let words: Vec<String> = BufReader::new(file)
        .lines()
        .collect::<Result<_, _>>()
        .unwrap();
    // println!("{:?}", words);

    // let correct = words.iter().find(|&w| w == "crank").unwrap();
    let mut correct_words = 1;
    let mut guess_statistics: HashMap<usize, usize> = HashMap::with_capacity(10);

    for correct in &words {
        println!("Word {:?}: {:?}", correct_words, correct);
        correct_words += 1;

        let mut guesses = 1;
        let mut guess = words.iter().find(|&w| w == "raise").unwrap();
        let mut guess_eval = evaluate_guess(guess.chars().collect(), correct.chars().collect());
        // println!("Green: {:?}", guess_eval.green);
        // println!("Yellow: {:?}", guess_eval.yellow);
        // println!("Grey: {:?}", guess_eval.grey);

        let mut remaining: Vec<&String> = words
            .iter()
            .filter(|w| possible_word(w.chars().collect(), &guess_eval))
            .collect();
        println!("Guess {:?}: {:?} [{:?}]", guesses, guess, remaining.len());
        // println!("{:?}", remaining);

        while guess != correct {
            let mut counts: HashMap<&String, usize> = HashMap::with_capacity(remaining.len());

            for possible_guess in &remaining {
                for possible_correct in &remaining {
                    let possible_guess_eval = evaluate_guess(
                        possible_guess.chars().collect(),
                        possible_correct.chars().collect(),
                    );
                    let possible_remaining: Vec<_> = remaining
                        .iter()
                        .filter(|&w| possible_word(w.chars().collect(), &possible_guess_eval))
                        .collect();
                    let count = counts.entry(possible_guess).or_insert(0);
                    *count += possible_remaining.len();
                }
            }
            // println!("{:?}", counts);

            guess = counts.iter().min_by_key(|c| c.1).unwrap().0;
            guesses += 1;

            guess_eval =
                guess_eval + evaluate_guess(guess.chars().collect(), correct.chars().collect());
            // println!("Green: {:?}", guess_eval.green);
            // println!("Yellow: {:?}", guess_eval.yellow);
            // println!("Grey: {:?}", guess_eval.grey);

            remaining = words
                .iter()
                .filter(|w| possible_word(w.chars().collect(), &guess_eval))
                .collect();
            println!("Guess {:?}: {:?} [{:?}]", guesses, guess, remaining.len());
        }

        let guess_statistic = guess_statistics.entry(guesses).or_insert(0);
        *guess_statistic += 1;

        println!("{:?}", guess_statistics);
    }
}

// let guess2 = String::from("snout");
// assert!(possible_word(guess2.chars().collect(), &guess_eval));
// guess_eval = guess_eval + evaluate_guess(guess2.chars().collect(), correct.chars().collect());
// println!("Green: {:?}", guess_eval.green);
// println!("Yellow: {:?}", guess_eval.yellow);
// println!("Grey: {:?}", guess_eval.grey);

// let guess3 = String::from("sully");
// assert!(possible_word(guess3.chars().collect(), &guess_eval));
// guess_eval = guess_eval + evaluate_guess(guess3.chars().collect(), correct.chars().collect());
// println!("Green: {:?}", guess_eval.green);
// println!("Yellow: {:?}", guess_eval.yellow);
// println!("Grey: {:?}", guess_eval.grey);

// let guess4 = String::from("slump");
// assert!(possible_word(guess4.chars().collect(), &guess_eval));
// guess_eval = guess_eval + evaluate_guess(guess4.chars().collect(), correct.chars().collect());
// println!("Green: {:?}", guess_eval.green);
// println!("Yellow: {:?}", guess_eval.yellow);
// println!("Grey: {:?}", guess_eval.grey);

// Wordle 203 3/6

// 🟨🟨⬜⬜⬜
// 🟨🟨⬜🟨🟨
// 🟩🟩🟩🟩🟩
