
use regex::Regex;

fn main() {
    let input_a = include_str!("../input/a.txt");

    part_a(input_a);
    part_b(input_a);
}

struct Lights(Vec<u64>);

impl Lights {
    fn new() -> Self {
        Lights(vec![0u64; 1_000_000])
    }

    fn flip_on(&mut self, from: (u32, u32), to: (u32, u32), on: bool) {
        for y in from.1 ..= to.1 {
            for x in from.0 ..= to.0 {
                if let Some(light) = self.0.get_mut((y * 1000 + x) as usize) {
                    if on {
                        *light = 1;
                    } else {
                        *light = 0;
                    }
                }
            }
        }
    }

    fn flip_brightness(&mut self, from: (u32, u32), to: (u32, u32), on: bool) {
        for y in from.1 ..= to.1 {
            for x in from.0 ..= to.0 {
                if let Some(light) = self.0.get_mut((y * 1000 + x) as usize) {
                    if on {
                        *light += 1
                    } else {
                        if (*light > 0) {
                            *light -= 1
                        }
                    }
                }
            }
        }
    }

    fn toggle(&mut self, from: (u32, u32), to: (u32, u32)) {
        for y in from.1 ..= to.1 {
            for x in from.0 ..= to.0 {
                if let Some(light) = self.0.get_mut((y * 1000 + x) as usize) {
                    *light = *light ^ 1;
                }
            }
        }
    }

    fn toggle_brightness(&mut self, from: (u32, u32), to: (u32, u32)) {
        for y in from.1 ..= to.1 {
            for x in from.0 ..= to.0 {
                if let Some(light) = self.0.get_mut((y * 1000 + x) as usize) {
                    *light += 2;
                }
            }
        }
    }
}

fn part_a(input: &str) {
    let mut lights = Lights::new();

    let re = Regex::new(r".* (\d+),(\d+).through.(\d+),(\d+)").unwrap();

    for line in input.lines() {
        if let Some(caps) = re.captures(line) {
            let from_x = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let from_y = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
            let to_x   = caps.get(3).unwrap().as_str().parse::<u32>().unwrap();
            let to_y   = caps.get(4).unwrap().as_str().parse::<u32>().unwrap();

            //println!("({}, {}) to ({}, {}).", from_x, from_y, to_x, to_y);

            if line.starts_with("turn on") {
                lights.flip_on((from_x, from_y), (to_x, to_y), true);
            } else if line.starts_with("turn off") {
                lights.flip_on((from_x, from_y), (to_x, to_y), false);
            } else if line.starts_with("toggle") {
                lights.toggle((from_x, from_y), (to_x, to_y));
            } else {
                panic!("{}", format!("Invalid line `{}`", line));
            }
        } else {
            panic!("{}", format!("Invalid line: `{}`", line));
        }
    }

    let mut total = 0;

    for light in lights.0.iter() {
        if *light == 1 {
            total += 1
        }
    }

    println!("There are {} lights lit!", total);
}

fn part_b(input: &str) {
    let mut lights = Lights::new();

    let re = Regex::new(r".* (\d+),(\d+).through.(\d+),(\d+)").unwrap();

    for line in input.lines() {
        if let Some(caps) = re.captures(line) {
            let from_x = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let from_y = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
            let to_x   = caps.get(3).unwrap().as_str().parse::<u32>().unwrap();
            let to_y   = caps.get(4).unwrap().as_str().parse::<u32>().unwrap();

            //println!("({}, {}) to ({}, {}).", from_x, from_y, to_x, to_y);

            if line.starts_with("turn on") {
                lights.flip_brightness((from_x, from_y), (to_x, to_y), true);
            } else if line.starts_with("turn off") {
                lights.flip_brightness((from_x, from_y), (to_x, to_y), false);
            } else if line.starts_with("toggle") {
                lights.toggle_brightness((from_x, from_y), (to_x, to_y));
            } else {
                panic!("{}", format!("Invalid line `{}`", line));
            }
        } else {
            panic!("{}", format!("Invalid line: `{}`", line));
        }
    }

    let mut total = 0;

    for light in lights.0.iter() {
        total += *light;
    }

    println!("There is a total brightness of {}!", total);
}