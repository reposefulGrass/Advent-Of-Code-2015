
use std::collections::HashMap;

fn main() {
    let input_a = include_str!("../input/a.txt");

    part_a(input_a);
}

enum Operation {
    VALUE,
    AND,
    OR,
    LSHIFT,
    RSHIFT,
    NOT,
}

impl Operation {
    fn determine (line: &str) -> Self {
        if line.contains("AND") {
            Operation::AND   
        } else if line.contains("OR") {
            Operation::OR 
        } else if line.contains("LSHIFT") {
            Operation::LSHIFT
        } else if line.contains("RSHIFT") {
            Operation::RSHIFT  
        } else if line.contains("NOT") {
            Operation::NOT 
        } else {
            Operation::VALUE
        }
    }

    fn compute (line: &str, wires: &mut HashMap<String, u16>) {
        let splits: Vec<&str> = line.split(" -> ").collect();
        let lhand = splits[0].trim();
        let rhand = splits[1].trim().to_string();

        match Operation::determine(line) {
            Operation::AND => {
                let lhand_splits: Vec<&str> = lhand.split(" AND ").collect();
                let lhand_1 = lhand_splits[0].trim();
                let lhand_2 = lhand_splits[1].trim();

                println!("{} {}", lhand_1, lhand_2);
                wires.insert(rhand, wires.get(lhand_1).unwrap() & wires.get(lhand_2).unwrap());
            },
            Operation::OR => {
                let lhand_splits: Vec<&str> = lhand.split(" OR ").collect();
                let lhand_1 = lhand_splits[0].trim();
                let lhand_2 = lhand_splits[1].trim();

                wires.insert(rhand, wires.get(lhand_1).unwrap() | wires.get(lhand_2).unwrap());
            }
            Operation::LSHIFT => {
                let lhand_splits: Vec<&str> = lhand.split(" LSHIFT ").collect();
                let lhand_1 = lhand_splits[0].trim();
                let lhand_2: u8 = lhand_splits[1].trim().parse().unwrap();

                wires.insert(rhand, wires.get(lhand_1).unwrap() << lhand_2);
            },
            Operation::RSHIFT => {
                let lhand_splits: Vec<&str> = lhand.split(" RSHIFT ").collect();
                let lhand_1 = lhand_splits[0].trim();
                let lhand_2: u8 = lhand_splits[1].trim().parse().unwrap();

                wires.insert(rhand, wires.get(lhand_1).unwrap() >> lhand_2);
            }
            Operation::NOT => {
                let lhand_1: Vec<&str> = lhand.split(" ").collect();
                let lhand_1 = lhand_1[1];

                wires.insert(rhand, !wires.get(lhand_1).unwrap());
            },
            Operation::VALUE => {
                let lhand: u16 = lhand.parse().unwrap();

                wires.insert(rhand, lhand);
            },
        }
    }
}

fn part_a(input: &str) {
    let mut wires = HashMap::new();

    for line in input.lines() {
        Operation::compute(line, &mut wires);
    }

    println!("The wire `a` contains {}", wires.get("a").unwrap());
}

