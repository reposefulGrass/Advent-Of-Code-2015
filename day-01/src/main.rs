
fn main() {
    let input_a = include_str!("../input/a.txt");
    part_a(input_a);
    part_b(input_a);
}

fn part_a(input: &str) {
    let mut floor = 0;

    for c in input.chars() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        } else {
            panic!("Invalid character in input: {c}");
        }
    }

    println!("The floor is {floor}!");
}

fn part_b(input: &str) {
    let mut floor = 0;

    for (i, c) in input.chars().enumerate() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        } else {
            panic!("Invalid character in input: {c}");
        }

        if floor == -1 {
            let position = i + 1;
            println!("The character in position {position} refers to the basement!");
            break;
        }
    }
}