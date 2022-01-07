use std::collections::HashSet;

fn evaluate_guess(guess: &Vec<char>, correct: &Vec<char>) -> Vec<char>
{
    let mut greens = vec!['\0'; 5];

    let it = correct.iter().zip(guess.iter());

    for (i, (c, g)) in it.enumerate() {
        if *c == *g
        {
            greens[i] = *c;
        }
    }
    return greens;
}

fn main() {
    let mut greens = vec!['\0'; 5];
    let mut yellows = vec![HashSet::with_capacity(10); 5];
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

    greens = evaluate_guess(&guess.chars().collect(), &correct.chars().collect());

    println!("Green: {:?}", greens);
    println!("Yellow: {:?}", yellows);
    println!("Grey: {:?}", greys);
}
