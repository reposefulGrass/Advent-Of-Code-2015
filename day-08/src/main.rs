
use crate::StringLiteral::*;
use crate::EscapeType::*;

fn main() {
    let input_a = include_str!("../input/a.txt");

    part_a(input_a);
    part_b(input_a);
}

#[derive(Debug)]
enum EscapeType {
    Hex,
    Newline,
    Return,
    Tab,
    Backslash,
    Null,
    SingleQuote,
    DoubleQuote,
}

#[derive(Debug)]
enum StringLiteral {
    Character,
    EscapedCharacter(EscapeType),
}

impl StringLiteral {
    fn size(&self) -> usize {
        match self {
            Character => 1,
            EscapedCharacter(escape_type) => 1 + escape_type.size(),
        }
    }

    fn real_size(&self) -> usize {
        match self {
            Character => 1,
            EscapedCharacter(_) => 1,
        }
    }

    fn encoded_size(&self) -> usize {
        match self {
            Character => 1,
            EscapedCharacter(escape_type) => 2 + escape_type.encoded_size(),
        }
    }

    fn parse(chars: &str) -> StringLiteral {
        match chars.chars().nth(0).unwrap() {
            '\\' => EscapedCharacter(EscapeType::parse(chars.chars().nth(1).unwrap())),
            _ => Character,
        }
    }
}

impl EscapeType {
    fn size(&self) -> usize {
        match self {
            Hex => 3,
            _ => 1
        }
    }

    fn encoded_size(&self) -> usize {
        match self {
            SingleQuote | DoubleQuote | Backslash => 2,
            Hex => 3,
            _ => 1,
        }
    }

    fn parse(character: char) -> EscapeType {
        match character {
            'x' => EscapeType::Hex,
            'n' => EscapeType::Newline,
            'r' => EscapeType::Return,
            't' => EscapeType::Tab,
            '\\' => EscapeType::Backslash,
            '0' => EscapeType::Null,
            '\'' => EscapeType::SingleQuote,
            '\"' => EscapeType::DoubleQuote,
            _ => panic!("Invalid escape type!"),
        }
    }
}

fn part_a(input: &str) {
    let mut total_difference = 0;

    for line in input.lines() {
        let mut string: Vec<StringLiteral> = vec![];
        let mut index: usize = 0;

        while index < line.len() {
            let literal = StringLiteral::parse(&line[index..]);
            index += literal.size();
            string.push(literal);
        }

        let mut real_size = 0;
        let mut size = 0;
        for literal in string.iter() {
            real_size += literal.real_size();
            size += literal.size();
        }
        real_size -= 2; // to acount for 2 doublequotes

        total_difference += size - real_size;

        //println!("{:?}", string);
    }

    println!("The total size difference is {}.", total_difference);
}

fn part_b(input: &str) {
    let mut total_encoded_size = 0;

    for line in input.lines() {
        let mut string: Vec<StringLiteral> = vec![];
        let mut index: usize = 0;

        while index < line.len() {
            let literal = StringLiteral::parse(&line[index..]);
            index += literal.size();
            string.push(literal);
        }

        let mut encoded_size = 0;
        let mut size = 0;
        for literal in string.iter() {
            encoded_size += literal.encoded_size();
            size += literal.size();
        }
        encoded_size += 4; // to acount for 2 doublequotes

        total_encoded_size += encoded_size - size;
    }

    println!("The total encoded size is {}.", total_encoded_size);
}