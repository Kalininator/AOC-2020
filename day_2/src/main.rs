use std::env;
use std::fs;
use std::io::{BufRead, BufReader};

struct Input {
    first_number: i32,
    second_number: i32,
    char_to_check: char,
    password: String,
}

fn extract_input(line: &str) -> Input {
    let without_colon = line.replace(":", "");
    let split: Vec<&str> = without_colon.split(' ').collect();

    let range = split[0];
    let range_values: Vec<&str> = range.split('-').collect();
    let first_number = range_values[0].parse().expect("Invalid min range");
    let second_number = range_values[1].parse().expect("asdfasf");
    let char_to_check = split[1].chars().collect::<Vec<char>>()[0];
    let password = split[2];
    return Input {
        first_number: first_number,
        second_number: second_number,
        char_to_check: char_to_check,
        password: String::from(password),
    };
}

fn count_chars(string: String, character: char) -> usize {
    return string.chars().filter(|c| c == &character).count();
}

fn check_line_task_1(line: &str) -> bool {
    let input = extract_input(line);
    let password_char_count = count_chars(input.password, input.char_to_check);

    return password_char_count >= input.first_number as usize
        && password_char_count <= input.second_number as usize;
}

fn has_char_at_index(text: &String, character: char, position: usize) -> bool {
    return text.len() > position && text.chars().collect::<Vec<char>>()[position] == character;
}

fn check_line_task_2(line: &str) -> bool {
    let input = extract_input(line);

    let a_valid = has_char_at_index(
        &input.password,
        input.char_to_check,
        input.first_number as usize - 1,
    );
    let b_valid = has_char_at_index(
        &input.password,
        input.char_to_check,
        input.second_number as usize - 1,
    );

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
        if check_line_task_1(&line) {
            count_part_1 = count_part_1 + 1;
        }
        if check_line_task_2(&line) {
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
    fn extract_input_works() {
        let line = "4-7 z: zzzfzlzzz";
        let parsed_input = extract_input(line);
        assert_eq!(parsed_input.first_number, 4);
        assert_eq!(parsed_input.second_number, 7);
        assert_eq!(parsed_input.char_to_check, 'z');
        assert_eq!(parsed_input.password, "zzzfzlzzz");
    }

    #[test]
    fn task_1_works() {
        let correct_line = "1-3 a: abcde";
        assert_eq!(check_line_task_1(correct_line), true);

        let incorrect_line = "1-3 b: cdefg";
        assert_eq!(check_line_task_1(incorrect_line), false);
    }

    #[test]
    fn count_chars_works() {
        let string = String::from("ababa");
        assert_eq!(count_chars(string, 'a'), 3);
    }

    #[test]
    fn task_2_works() {
        let correct_string = "1-3 a: abcde";
        assert_eq!(check_line_task_2(correct_string), true);

        let incorrect_string = "1-3 b: cdefg";
        assert_eq!(check_line_task_2(incorrect_string), false);

        let incorrect_string_xor = "2-9 c: ccccccccc";
        assert_eq!(check_line_task_2(incorrect_string_xor), false);
    }

    #[test]
    fn check_char_works() {
        assert_eq!(has_char_at_index(&String::from("abc"), 'b', 1), true);
        assert_eq!(has_char_at_index(&String::from("abc"), 'b', 4), false);
    }
}
