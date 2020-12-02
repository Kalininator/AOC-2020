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

fn check_line_2(line: &str) -> bool {
    let without_colon = line.replace(":", "");
    let split: Vec<&str> = without_colon.split(' ').collect();

    let range = split[0];
    let range_values: Vec<&str> = range.split('-').collect();
    let position_1: usize = range_values[0].parse().expect("Invalid min range");
    let position_2: usize = range_values[1].parse().expect("asdfasf");
    let char_to_check = split[1];
    let password_chars: Vec<char> = split[2].chars().collect();

    let a_valid: bool = password_chars.len() > position_1 - 1
        && password_chars[position_1 - 1].to_string() == char_to_check;

    let b_valid: bool = password_chars.len() > position_2 - 1
        && password_chars[position_2 - 1].to_string() == char_to_check;

    return a_valid ^ b_valid;
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

    let mut count_part_1: i32 = 0;
    let mut count_part_2: i32 = 0;

    for line in lines {
        if check_line(&line) {
            count_part_1 = count_part_1 + 1;
        }
        if check_line_2(&line) {
            count_part_2 = count_part_2 + 1;
        }
    }
    println!("[PART 1] Correct passwords: {}", count_part_1);
    println!("[PART 2] Correct passwords: {}", count_part_2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(check_line_2("1-3 a: abcde"), true);
    }
}
