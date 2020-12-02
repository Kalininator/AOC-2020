use std::env;
use std::fs;
use std::io::{BufRead, BufReader};

fn check_line(line: &str) -> bool {
    let without_colon = line.replace(":", "");
    let split: Vec<&str> = without_colon.split(' ').collect();

    let range = split[0];
    let range_values: Vec<&str> = range.split('-').collect();
    let min_count: usize = range_values[0].parse().expect("Invalid min range");
    let max_count: usize = range_values[1].parse().expect("asdfasf");
    let char_to_check = split[1];
    let password = split[2];
    let password_char_count = password
        .chars()
        .filter(|c| c.to_string() == char_to_check)
        .count();

    return password_char_count >= min_count && password_char_count <= max_count;
}

fn read_file(path: &str) -> Vec<String> {
    let file = fs::File::open(path).expect("Failed to read file");
    let br = BufReader::new(file);

    return br.lines().map(|l| l.expect("Failed to get line")).collect();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let lines = read_file(&filename);

    let mut count: i32 = 0;

    for line in lines {
        if check_line(&line) {
            count = count + 1;
        }
    }
    println!("Correct passwords: {}", count);
}
