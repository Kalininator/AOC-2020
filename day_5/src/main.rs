use std::fs;
use std::io::{BufRead, BufReader};

fn parse_string(string: &String, truthy_char: char, _falsy_char: char) -> Vec<bool> {
    return string
        .chars()
        .map(|c| {
            if c == truthy_char {
                return true;
            }
            return false;
        })
        .collect();
}

fn binary_find(max_num: usize, instructions: Vec<bool>) -> usize {
    let mut lower: usize = 0;
    let mut upper: usize = max_num;

    for instruction in instructions.iter() {
        let middle = lower + ((upper + 1) - lower) / 2;
        if instruction == &true {
            lower = middle;
        } else {
            upper = middle;
        }
    }

    return lower;
}

fn get_seat_id(input: &String) -> usize {
    let rows = &input[..7];
    let columns = &input[7..10];
    let row = binary_find(127, parse_string(&String::from(rows), 'B', 'F'));
    let column = binary_find(7, parse_string(&String::from(columns), 'R', 'L'));
    return (row * 8) + column;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let file = fs::File::open(filename).expect("failed to read file");
    let br = BufReader::new(file);
    let lines: Vec<String> = br.lines().map(|l| l.expect("no line")).collect();
    // let input = String::from("BFFFBBFRRR");
    // println!("seat ID: {}", get_seat_id(&input));
    let ids: Vec<usize> = lines.iter().map(|l| get_seat_id(&l)).collect();
    let max_id = ids.iter().max().expect("Failed to get max");
    println!("Part 1 result: {}", max_id);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_string_works() {
        assert_eq!(
            parse_string(&String::from("BFFF"), 'F', 'B'),
            vec![false, true, true, true]
        );
    }

    #[test]
    fn binary_find_works() {
        assert_eq!(binary_find(7, vec![true, false, true]), 5,);
        assert_eq!(
            binary_find(127, vec![false, true, false, true, true, false, false]),
            44,
        );
    }

    #[test]
    fn get_seat_id_works() {
        assert_eq!(get_seat_id(&String::from("BFFFBBFRRR")), 567);
        assert_eq!(get_seat_id(&String::from("FFFBBBFRRR")), 119);
        assert_eq!(get_seat_id(&String::from("BBFFBBFRLL")), 820);
    }
}
