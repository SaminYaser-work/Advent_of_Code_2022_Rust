use std::collections::HashMap;
use std::fs;

// Rock: A, X
// Paper: B, Y
// Scissor: C, Z
//
// X: Lose
// Y: Draw
// Z: Win

fn main() {
    let points = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);
    // let losing_combs = HashMap::from([("X", "B"), ("Y", "C"), ("Z", "A")]);
    let losing_combs = HashMap::from([("B", "X"), ("C", "Y"), ("A", "Z")]);
    // let winning_combs = HashMap::from([("X", "C"), ("Y", "A"), ("Z", "B")]);
    let winning_combs = HashMap::from([("C", "X"), ("A", "Y"), ("B", "Z")]);
    // let draw_combs = HashMap::from([("X", "A"), ("Y", "B"), ("Z", "C")]);
    let draw_combs = HashMap::from([("A", "X"), ("B", "Y"), ("C", "Z")]);
    let def_points = HashMap::from([("X", 0), ("Y", 3), ("Z", 6)]);

    let content = fs::read_to_string("input.txt").unwrap();

    let _lines = content.lines();

    let mut score = 0;

    for line in _lines {
        let x: Vec<&str> = line.split_whitespace().collect();

        print!("{} _ {} --> ", x[0], x[1]);

        // if x[0] == draw_combs[x[1]] {
        //     score += 3 + points[x[1]];
        //     println!("Draw. Score: {}", score)
        // } else if x[0] == losing_combs[x[1]] {
        //     score += 0 + points[x[1]];
        //     println!("Lost. Score: {}", score)
        // } else {
        //     score += 6 + points[x[1]];
        //     println!("Win. Score: {}", score)
        // }

        if x[1] == "X" {
            score += def_points[x[1]] + points[losing_combs[x[0]]];
            println!("Lost. Score: {}", score)
        } else if x[1] == "Y" {
            score += def_points[x[1]] + points[draw_combs[x[0]]];
            println!("Draw. Score: {}", score)
        } else {
            score += def_points[x[1]] + points[winning_combs[x[0]]];
            println!("Win. Score: {}", score)
        }
    }

    println!("{}", score);
}
