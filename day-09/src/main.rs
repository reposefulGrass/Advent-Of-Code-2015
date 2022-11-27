
use regex::Regex;
use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let input_a = include_str!("../input/test.txt");

    part_a(input_a);
}

fn part_a(input: &str) {
    let city_set = determine_city_set(input);
    let city_map = determine_city_map(input);
    //println!("{} {:?}", city_set.len(), city_set);
    //for (k, v) in city_map.iter() {
    //    println!("{:?} -> {}", k, v);
    //}

    let mut shortest_path: Vec<&String> = vec![];
    let mut shortest_distance = 0;

'permutations:
    for comb in city_set.iter().permutations(city_set.len()) {
        //println!("{:?}", comb);

        let mut distance = 0;
        for i in 0 .. comb.len()-1 {
            let path = (comb[i].to_string(), comb[i+1].to_string());
            if let Some(d) = city_map.get(&path) {
                distance += d;
            } else {
                continue 'permutations;
            }
        }

        if  shortest_distance == 0 || distance < shortest_distance {
            shortest_distance = distance;
            shortest_path = comb.clone();
        }
    }

    println!("The shortest distance was {} using path {:?}.", shortest_distance, shortest_path);
}

fn determine_city_set(input: &str) -> Vec<String> {
    let mut city_set: Vec<String> = vec![];
    let re = Regex::new(r"(\w+) to (\w+) = \d+").unwrap();

    for line in input.lines() {
        if let Some(caps) = re.captures(line) {
            let from_city = caps.get(1).unwrap().as_str().to_string();
            let to_city = caps.get(2).unwrap().as_str().to_string();

            if ! city_set.contains(&from_city) {
                city_set.push(from_city);
            }

            if ! city_set.contains(&to_city) {
                city_set.push(to_city);
            }
        }
    }

    return city_set;
}

fn determine_city_map(input: &str) -> HashMap<(String, String), u32> {
    let mut city_map = HashMap::new();
    let re = Regex::new(r"(\w+) to (\w+) = (\d+)").unwrap();

    for line in input.lines() {
        if let Some(caps) = re.captures(line) {
            let from_city = caps.get(1).unwrap().as_str().to_string();
            let to_city = caps.get(2).unwrap().as_str().to_string();
            let distance: u32 = caps.get(3).unwrap().as_str().parse().unwrap();

            city_map.insert((from_city, to_city), distance);
        }
    }

    return city_map;
}
