
use std::collections::HashMap;

fn main() {
    let input_a = include_str!("../input/a.txt");

    part_a(input_a);
    part_b(input_a);
}

fn part_a(input: &str) {
    let mut pos = (0, 0);
    let mut houses_visited = HashMap::new();

    for direction in input.chars() {
        houses_visited.insert(pos, 1);

        match direction {
            '^' => pos.1 += 1,
            '>' => pos.0 += 1,
            'v' => pos.1 -= 1,
            '<' => pos.0 -= 1,
            _ => panic!("Invalid direction"),
        }
    }

    println!("The number of houses visited is {}", houses_visited.len());
}

fn part_b(input: &str) {
    let mut human_pos = (0, 0);
    let mut robot_pos = (0, 0);
    let mut houses_visited = HashMap::new();

    for (i, direction) in input.chars().enumerate() {
        if i % 2 == 0 {
            houses_visited.insert(human_pos, 1);

            match direction {
                '^' => human_pos.1 += 1,
                '>' => human_pos.0 += 1,
                'v' => human_pos.1 -= 1,
                '<' => human_pos.0 -= 1,
                _ => panic!("Invalid direction"),
            }
        } else {
            houses_visited.insert(robot_pos, 1);

            match direction {
                '^' => robot_pos.1 += 1,
                '>' => robot_pos.0 += 1,
                'v' => robot_pos.1 -= 1,
                '<' => robot_pos.0 -= 1,
                _ => panic!("Invalid direction"),
            }
        }
    }

    println!("The number of houses visited is {}", houses_visited.len());
}
