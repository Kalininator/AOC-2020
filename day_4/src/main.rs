use std::env;
use std::fs;
use std::io::{BufRead, BufReader};

fn print_lines(lines: &Vec<String>) {
    for line in lines.iter() {
        println!("{}", line);
    }
}

fn file_lines_to_passports(file_lines: Vec<String>) -> Vec<String> {
    // return file_lines.into_iter().filter(|l| l != "").collect();
    let mut result = Vec::<String>::new();
    let mut buffer = Vec::<String>::new();
    for line in file_lines.into_iter() {
        if line.len() != 0 {
            buffer.push(line);
            continue;
        }
        result.push(buffer.join(" "));
        buffer.clear();
    }
    result.push(buffer.join(" "));

    return result;
}

fn get_passport_line_fields(line: String) -> Vec<String> {
    let result = line
        .split(" ")
        .map(|i| return String::from(i.split(":").collect::<Vec<&str>>()[0]))
        .collect();

    return result;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = fs::File::open(filename).expect("Failed to open file.");
    let br = BufReader::new(file);

    let lines: Vec<String> = br
        .lines()
        .map(|l| l.expect("Could not read line."))
        .collect();

    let passport_lines = file_lines_to_passports(lines);
    let count = passport_lines.len();
    let passport_fields: Vec<Vec<String>> = passport_lines
        .into_iter()
        .map(|line| get_passport_line_fields(line))
        .collect();

    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let mut invalid_count = 0;

    'top: for passport in passport_fields {
        for field in &required_fields {
            if !passport.iter().any(|f| f == field) {
                invalid_count += 1;
                continue 'top;
            }
        }
    }

    println!("Valid: {}", count - invalid_count);
}
