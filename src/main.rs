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
        .map(|(&c, &g)| if c != g && correct_set.contains(&g) { HashSet::from([g]) } else { HashSet::new() })
        .collect();

    let guess_set: HashSet<char> = HashSet::from_iter(guess.iter().cloned());
    let grey = &guess_set - &correct_set;

    return Guess { green, yellow, grey };
}

fn possible_word(word: Vec<char>, guess: Guess) -> bool {
    return false;
}

impl ops::Add<Guess> for Guess {
    type Output = Guess;

    fn add(self, _rhs: Guess) -> Guess {
        let green = self.green
        .iter()
        .zip(_rhs.green.iter())
        .map(|(&g1, &g2)| if g1 == '\0' { g2 } else { g1 })
        .collect();

        let yellow = self.yellow
        .into_iter()
        .zip(_rhs.yellow.into_iter())
        .map(|(y1, y2)| y1.into_iter().chain(y2).collect())
        .collect();

        let grey = self.grey.into_iter().chain(_rhs.grey).collect();

        return Guess{green, yellow, grey};
    }
}

fn main() {
    let correct = String::from("slump");
    let guess1 = String::from("raise");
    let guess2 = String::from("snout");
    let guess3 = String::from("sully");
    let guess4 = String::from("slump");

    let mut guess_eval = evaluate_guess(guess1.chars().collect(), correct.chars().collect());
    println!("Green: {:?}", guess_eval.green);
    println!("Yellow: {:?}", guess_eval.yellow);
    println!("Grey: {:?}", guess_eval.grey);

    guess_eval = guess_eval + evaluate_guess(guess2.chars().collect(), correct.chars().collect());
    println!("Green: {:?}", guess_eval.green);
    println!("Yellow: {:?}", guess_eval.yellow);
    println!("Grey: {:?}", guess_eval.grey);

    guess_eval = guess_eval + evaluate_guess(guess3.chars().collect(), correct.chars().collect());
    println!("Green: {:?}", guess_eval.green);
    println!("Yellow: {:?}", guess_eval.yellow);
    println!("Grey: {:?}", guess_eval.grey);

    guess_eval = guess_eval + evaluate_guess(guess4.chars().collect(), correct.chars().collect());
    println!("Green: {:?}", guess_eval.green);
    println!("Yellow: {:?}", guess_eval.yellow);
    println!("Grey: {:?}", guess_eval.grey);
}
