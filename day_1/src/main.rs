use std::env;
use std::fs;
use std::io::{BufRead, BufReader};

fn read_file(path: &str) -> Vec<i32> {
    println!("Reading file {}", path);

    let file = fs::File::open(path).expect("Failed to read file");
    let br = BufReader::new(file);

    let mut v = Vec::new();

    for line in br.lines() {
        let line = line.expect("Failed to read line");
        let s: i32 = line.parse().expect("Failed to parse");
        v.push(s);
    }

    return v;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // let contents = fs::read_to_string(filename).expect("Failed to read file");

    let arr = read_file(filename);
    // for val in arr {
    //     println!("{}", val)
    // }

    'outer: for i in 0..arr.len() {
        for j in (i + 1)..arr.len() {
            for k in (i + 2)..arr.len() {
                if arr[i] + arr[j] + arr[k] == 2020 {
                    println!("values: {} {} {}", arr[i], arr[j], arr[k]);
                    println!("end result: {}", arr[i] * arr[j] * arr[k]);
                    break 'outer;
                }
            }
        }
    }
}
