use std::collections::HashSet;
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
    let correct = String::from("slump");

    let guess1 = String::from("raise");
    let mut guess_eval = evaluate_guess(guess1.chars().collect(), correct.chars().collect());
    println!("Green: {:?}", guess_eval.green);
    println!("Yellow: {:?}", guess_eval.yellow);
    println!("Grey: {:?}", guess_eval.grey);

    let guess2 = String::from("snout");
    assert!(possible_word(guess2.chars().collect(), &guess_eval));
    guess_eval = guess_eval + evaluate_guess(guess2.chars().collect(), correct.chars().collect());
    println!("Green: {:?}", guess_eval.green);
    println!("Yellow: {:?}", guess_eval.yellow);
    println!("Grey: {:?}", guess_eval.grey);

    let guess3 = String::from("sully");
    assert!(possible_word(guess3.chars().collect(), &guess_eval));
    guess_eval = guess_eval + evaluate_guess(guess3.chars().collect(), correct.chars().collect());
    println!("Green: {:?}", guess_eval.green);
    println!("Yellow: {:?}", guess_eval.yellow);
    println!("Grey: {:?}", guess_eval.grey);

    let guess4 = String::from("slump");
    assert!(possible_word(guess4.chars().collect(), &guess_eval));
    guess_eval = guess_eval + evaluate_guess(guess4.chars().collect(), correct.chars().collect());
    println!("Green: {:?}", guess_eval.green);
    println!("Yellow: {:?}", guess_eval.yellow);
    println!("Grey: {:?}", guess_eval.grey);
}
