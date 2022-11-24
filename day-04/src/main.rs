
use md5;

fn main() {
    let input = "yzbqklnj";

    part_a(input.to_string());
    part_b(input.to_string());
}

fn part_a(input: String) {
    for answer in 0..1_000_000_000 {
        let hash_input = input.clone() + &answer.to_string();
        let digest = md5::compute(hash_input);
        let hash_output = format!("{:?}", digest);

        if hash_output.starts_with("00000") {
            println!("The answer is {}", answer);
            break;
        }
    }
}

fn part_b(input: String) {
    for answer in 0..1_000_000_000 {
        let hash_input = input.clone() + &answer.to_string();
        let digest = md5::compute(hash_input);
        let hash_output = format!("{:?}", digest);

        if hash_output.starts_with("000000") {
            println!("The answer is {}", answer);
            break;
        }
    }
}
