
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
    
    for group in group(input) {
        output.push_str(&group.len().to_string());
        output.push(group.chars().nth(0).unwrap());
    }

    output
}

fn group<'a> (input: &'a str) -> Vec<&'a str> {
    let mut groups = vec![];
    let mut index = 0;

    while index < input.len() {
        let len = get_char_length(&input[index..]);
        groups.push(&input[index..index+len]);
        index += len;
    }

    groups
}

fn get_char_length (input: &str) -> usize {
    let head = input.chars().nth(0).unwrap();
    let mut len = 1;

    loop {
        if let Some(character) = input.chars().nth(len) {
            if character == head {
                len += 1;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    len
}

#[test]
fn test_group_1 () {
    let empty_vec: Vec<String> = vec![];
    assert_eq!(group(""), empty_vec);
}

#[test]
fn test_group_2 () {
    assert_eq!(group("1"), vec!["1"]);
}

#[test]
fn test_group_3 () {
    assert_eq!(group("111221"), vec!["111", "22", "1"]);
}

#[test]
fn test_look_and_say () {
    assert_eq!(look_and_say("111221"), String::from("312211"));
}