use regex::Regex;
use std::env;
use std::fs;
use std::io::{BufRead, BufReader};

fn file_lines_to_passports(file_lines: Vec<String>) -> Vec<String> {
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

fn check_field(field: &str, value: String) -> bool {
    return match field {
        "cid" => true,
        "byr" => {
            let val = value.parse::<i32>().expect("failed parse number");
            val <= 2002 && val >= 1920
        }
        "iyr" => {
            let val = value.parse::<i32>().expect("failed parse number");
            val <= 2020 && val >= 2010
        }
        "eyr" => {
            let val = value.parse::<i32>().expect("failed parse number");
            val <= 2030 && val >= 2020
        }
        "hgt" => {
            if value.ends_with("cm") {
                let num = value[..value.len() - 2]
                    .parse::<i32>()
                    .expect("failed to get height number");
                if num >= 150 && num <= 193 {
                    return true;
                }
            }

            if value.ends_with("in") {
                let num = value[..value.len() - 2]
                    .parse::<i32>()
                    .expect("failed to get height number");
                if num >= 59 && num <= 76 {
                    return true;
                }
            }
            false
        }
        "hcl" => {
            let regex = Regex::new(r"^#([0-9a-f]){6}$").unwrap();
            regex.is_match(&*value)
        }
        "ecl" => {
            let regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
            regex.is_match(&*value)
        }
        "pid" => {
            let regex = Regex::new(r"^\d{9}$").unwrap();
            regex.is_match(&*value)
        }
        _ => false,
    };
}

fn get_passport_field_keys(line: &String) -> Vec<String> {
    let result = line
        .split(" ")
        .map(|i| return String::from(i.split(":").collect::<Vec<&str>>()[0]))
        .collect();

    return result;
}

fn is_passport_valid_basic(passport: &String) -> bool {
    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let passport_fields = get_passport_field_keys(passport);
    for field in &required_fields {
        if !passport_fields.iter().any(|f| f == field) {
            return false;
        }
    }
    return true;
}

fn is_passport_valid_advanced(passport: &String) -> bool {
    if !is_passport_valid_basic(passport) {
        return false;
    }

    let is_valid = passport
        .split(" ")
        .map(|f| {
            let split: Vec<&str> = f.split(":").collect::<Vec<&str>>();
            let res = check_field(split[0], split[1].to_string());
            // println!("{}:{} - {}", split[0], split[1], res);
            return res;
        })
        .collect::<Vec<bool>>();

    return !is_valid.iter().any(|f| f == &false);
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

    let checked_passports: Vec<bool> = passport_lines
        .into_iter()
        .map(|p| is_passport_valid_advanced(&p))
        .collect();

    let valid_passports = checked_passports.into_iter().filter(|&p| p == true).count();

    println!("Valid: {}", valid_passports);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_field_works() {
        assert_eq!(check_field("byr", String::from("1900")), false);
        assert_eq!(check_field("byr", String::from("1930")), true);
        assert_eq!(check_field("byr", String::from("2020")), false);

        assert_eq!(check_field("hcl", String::from("#afd123")), true);
    }

    #[test]
    fn invalid_passports() {
        assert_eq!(
            is_passport_valid_advanced(&String::from(
                "hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926 eyr:1972 cid:100"
            )),
            false
        );
    }
}
