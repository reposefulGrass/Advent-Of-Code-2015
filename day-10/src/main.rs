
fn main() {
    let input = "1113122113";

    part_a(input);
    part_b(input);
}

fn part_a (input: &str) {
    let mut pass = String::from(input);
    for _ in 0..40 {
        pass = look_and_say(&pass);
    }
    println!("{}", pass.len());
}

fn part_b (input: &str) {
    let mut pass = String::from(input);
    for _ in 0..50 {
        pass = look_and_say(&pass);
    }
    println!("{}", pass.len());
}

fn look_and_say (input: &str) -> String {
    let mut output = String::from("");
    let mut index = 0;
    while index < input.len() {
        let (character, length) = get_char_length(&input[index..]);
        output.push_str(&length.to_string());
        output.push_str(&character.to_string());

        index += length;
    }

    output
}

fn get_char_length (input: &str) -> (char, usize) {
    let head = input.chars().nth(0).unwrap();
    let mut len = 1;

    loop {
        match input.chars().nth(len) {
            Some(character) => {
                if character == head {
                    len += 1;
                } else {
                    break;
                }
            },
            None => break,
        }
    }

    (head, len)
}