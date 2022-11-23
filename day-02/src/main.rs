
pub mod present;

use crate::present::Present;

fn main() {
    let input_a = include_str!("../input/a.txt");

    part_a(input_a);
    part_b(input_a);
}

fn part_a(input: &str) {
    let mut total_wrapping_paper = 0;

    for line in input.lines() {
        let present = Present::try_from(line).unwrap();
        total_wrapping_paper += present.calculate_amount_of_wrapping_paper();
    }

    println!("The total amount of wrapping paper needed is {} square feet!", total_wrapping_paper);
}

fn part_b(input: &str) {
    let mut total_ribbon = 0;

    for line in input.lines() {
        let present = Present::try_from(line).unwrap();
        total_ribbon += present.calculate_amount_of_ribbon();
    }

    println!("The total amount of ribbon needed is {} feet!", total_ribbon);
}