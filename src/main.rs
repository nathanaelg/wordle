use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops;

#[derive(Copy, Clone)]
enum GuessColour {
    Green,
    Yellow,
    Grey,
}

struct Guess {
    green: Vec<char>,
    yellow: Vec<HashSet<char>>,
    grey: HashSet<char>,
}

fn evaluate_guess(guess: Vec<char>, correct: Vec<char>) -> Vec<GuessColour> {
    let correct_set: HashSet<char> = HashSet::from_iter(correct.iter().cloned());

    return correct
        .iter()
        .zip(guess.iter())
        .map(|(&c, &g)| {
            if c == g {
                GuessColour::Green
            } else if correct_set.contains(&g) {
                GuessColour::Yellow
            } else {
                GuessColour::Grey
            }
        })
        .collect();
}

fn build_guess(guess: Vec<char>, eval: Vec<GuessColour>) -> Guess {
    let green = guess
        .iter()
        .zip(eval.iter())
        .map(|(&g, &e)| {
            if matches!(e, GuessColour::Green) {
                g
            } else {
                '\0'
            }
        })
        .collect();

    let yellow = guess
        .iter()
        .zip(eval.iter())
        .map(|(&g, &e)| {
            if matches!(e, GuessColour::Yellow) {
                HashSet::from([g])
            } else {
                HashSet::new()
            }
        })
        .collect();

    let grey = guess
        .iter()
        .zip(eval.iter())
        .filter(|(_, &e)| matches!(e, GuessColour::Grey))
        .map(|(&g, _)| g)
        .collect();

    return Guess {
        green,
        yellow,
        grey,
    };
}

fn print_guess_colour(colour: GuessColour) -> char {
    match colour {
        GuessColour::Green => '🟩',
        GuessColour::Yellow => '🟨',
        GuessColour::Grey => '⬜',
    }
}

fn print_evaluated_guess(eval: Vec<GuessColour>) -> String {
    return eval.iter().map(|&e| print_guess_colour(e)).collect();
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
    let args: Vec<String> = env::args().collect();

    let file = File::open("words.txt").unwrap();
    let words: Vec<String> = BufReader::new(file)
        .lines()
        .collect::<Result<_, _>>()
        .unwrap();
    let correct_words_list: Vec<_> = if args.len() == 2 {
        words.iter().filter(|w| **w == args[1]).collect()
    } else {
        words.iter().collect()
    };
    let mut correct_words = 1;
    let mut guess_statistics: HashMap<usize, usize> = HashMap::with_capacity(10);

    for correct in correct_words_list {
        println!("Word {:?}: {}", correct_words, correct);
        correct_words += 1;

        let mut guesses = 1;
        let mut guess = words.iter().find(|&w| w == "raise").unwrap();
        let mut guess_eval = build_guess(
            guess.chars().collect(),
            evaluate_guess(guess.chars().collect(), correct.chars().collect()),
        );

        let mut remaining: Vec<&String> = words
            .iter()
            .filter(|w| possible_word(w.chars().collect(), &guess_eval))
            .collect();
        println!(
            "Guess {:?}: {} {} [{:?}]",
            guesses,
            guess,
            print_evaluated_guess(evaluate_guess(
                guess.chars().collect(),
                correct.chars().collect()
            )),
            remaining.len()
        );

        while guess != correct {
            let mut counts: HashMap<&String, usize> = HashMap::with_capacity(remaining.len());

            for possible_guess in &remaining {
                for possible_correct in &remaining {
                    let possible_guess_eval = build_guess(
                        possible_guess.chars().collect(),
                        evaluate_guess(
                            possible_guess.chars().collect(),
                            possible_correct.chars().collect(),
                        ),
                    );
                    let possible_remaining: Vec<_> = remaining
                        .iter()
                        .filter(|&w| possible_word(w.chars().collect(), &possible_guess_eval))
                        .collect();
                    let count = counts.entry(possible_guess).or_insert(0);
                    *count += possible_remaining.len();
                }
            }

            guess = counts.iter().min_by_key(|c| c.1).unwrap().0;
            guesses += 1;

            guess_eval = guess_eval
                + build_guess(
                    guess.chars().collect(),
                    evaluate_guess(guess.chars().collect(), correct.chars().collect()),
                );

            remaining = words
                .iter()
                .filter(|w| possible_word(w.chars().collect(), &guess_eval))
                .collect();
            println!(
                "Guess {:?}: {} {} [{:?}]",
                guesses,
                guess,
                print_evaluated_guess(evaluate_guess(
                    guess.chars().collect(),
                    correct.chars().collect()
                )),
                remaining.len()
            );
        }

        let guess_statistic = guess_statistics.entry(guesses).or_insert(0);
        *guess_statistic += 1;

        println!("Guess statistics: {:?}", guess_statistics);
    }
}
