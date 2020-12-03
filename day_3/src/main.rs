use std::env;
use std::fs;
use std::io::{BufRead, BufReader};

fn lets_crash_into_some_trees(map: &Vec<Vec<char>>, x_vel: usize, y_vel: usize) -> usize {
    let width = map[0].len();
    let height = map.len();

    let mut tings: Vec<char> = Vec::new();

    for y in (y_vel..(height + (y_vel - 1))).step_by(y_vel) {
        let x = (y / y_vel) * x_vel;
        tings.push(map[y][x % width]);
    }

    let tree_count = tings
        .iter()
        .filter(|ting| ting == &&'#')
        .collect::<Vec<&char>>()
        .len();

    return tree_count;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = fs::File::open(filename).expect("Failed to open file");
    let br = BufReader::new(file);

    let lines: Vec<String> = br
        .lines()
        .map(|l| l.expect("Failed to read line"))
        .collect();

    let rows: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();

    let tree_count_1 = lets_crash_into_some_trees(&rows, 1, 1);
    let tree_count_2 = lets_crash_into_some_trees(&rows, 3, 1);
    let tree_count_3 = lets_crash_into_some_trees(&rows, 5, 1);
    let tree_count_4 = lets_crash_into_some_trees(&rows, 7, 1);
    let tree_count_5 = lets_crash_into_some_trees(&rows, 1, 2);

    println!(
        "{}",
        tree_count_1 * tree_count_2 * tree_count_3 * tree_count_4 * tree_count_5
    );
}
