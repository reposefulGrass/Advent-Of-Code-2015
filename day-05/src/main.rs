
fn main() {
    let input = include_str!("../input/a.txt");

    part_a(input);
    part_b(input);
}

fn part_a(input: &str) {
    let mut nice_strings = 0;

    for line in input.lines() {
        if check_vowels(line) && check_double_letter(line) && check_blacklist(line) {
            nice_strings += 1;
        }
    }

    println!("[Part A] There are {} nice strings", nice_strings);
}

fn part_b(input: &str) {
    let mut nice_strings = 0;

    for line in input.lines() {
        if check_pair_twice(line) && check_double_letter_between(line) {
            nice_strings += 1;
        }
    }

    println!("[Part B] There are {} nice strings", nice_strings);
}

fn check_vowels(input: &str) -> bool {
    let mut vowels = String::from("");

    for c in input.chars() {
        if is_vowel(c) {
            vowels.push_str(&String::from(c));
        }
    }

    if vowels.len() >= 3 {
        return true;
    } 

    false
}

fn is_vowel(c: char) -> bool {
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}

fn check_double_letter(input: &str) -> bool {
    for i in 0..input.len() - 1 {
        let first = input.chars().nth(i).unwrap();
        let second = input.chars().nth(i + 1).unwrap();

        if first == second { 
            return true;
        }
    }

    false
}

fn check_blacklist(input: &str) -> bool {
    let blacklist = vec!["ab", "cd", "pq", "xy"];

    for ss in blacklist {
        if input.contains(ss) {
            return false;
        }
    }

    true
}

fn check_pair_twice(input: &str) -> bool {
    for i in 0..input.len()-2 {
        let ss = input[i..i+2].to_string();
        if input[i+2..].contains(&ss) {
            return true;
        }
    }

    false
}

fn check_double_letter_between(input: &str) -> bool {
    for i in 0..input.len() - 2 {
        let first = input.chars().nth(i).unwrap();
        let third = input.chars().nth(i + 2).unwrap();

        if first == third { 
            return true;
        }
    }

    false
}