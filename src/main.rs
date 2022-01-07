use std::collections::HashSet;

fn evaluate_guess(guess: &str, correct: &str) -> [char; 5]
{
    let mut greens = ['\0', '\0', '\0', '\0', '\0'];
    for (i, c) in correct.chars().enumerate() {
        if c == guess.chars().nth(i).unwrap()
        {
            greens[i] = c;
        }
    }
    return greens;
}

fn main() {
    let mut greens = ['\0', '\0', '\0', '\0', '\0'];
    let mut yellows = [HashSet::with_capacity(10), HashSet::with_capacity(10), HashSet::with_capacity(10), HashSet::with_capacity(10), HashSet::with_capacity(10)];
    let mut greys = HashSet::with_capacity(26);

    yellows[0].insert('i');
    yellows[1].insert('r');
    greys.insert('a');
    yellows[3].insert('t');
    yellows[4].insert('e');

    println!("Green: {:?}", greens);
    println!("Yellow: {:?}", yellows);
    println!("Grey: {:?}", greys);

    greens[0] = 't';
    greens[1] = 'i';
    yellows[1].insert('r');
    greens[3] = 'e';
    greys.insert('d');

    println!("Green: {:?}", greens);
    println!("Yellow: {:?}", yellows);
    println!("Grey: {:?}", greys);

    greens[2] = 'g';
    greens[4] = 'r';

    println!("Green: {:?}", greens);
    println!("Yellow: {:?}", yellows);
    println!("Grey: {:?}", greys);

    let correct = String::from("banal");
    let guess = String::from("candy");

    greens = evaluate_guess(&guess, &correct);

    println!("Green: {:?}", greens);
    println!("Yellow: {:?}", yellows);
    println!("Grey: {:?}", greys);
}
