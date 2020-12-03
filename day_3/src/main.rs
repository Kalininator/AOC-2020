use std::env;
use std::fs;
use std::io::{BufRead, BufReader};

fn count_trees_on_slope(map: &Vec<Vec<char>>, x_vel: usize, y_vel: usize) -> usize {
    let width = map[0].len();
    let height = map.len();

    let mut sections_encountered: Vec<char> = Vec::new();

    for y in (y_vel..(height + y_vel - 1)).step_by(y_vel) {
        let x = (y / y_vel) * x_vel;
        sections_encountered.push(map[y][x % width]);
    }

    let tree_count = sections_encountered
        .iter()
        .filter(|section| section == &&'#')
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

    let tree_count_1 = count_trees_on_slope(&rows, 1, 1);
    let tree_count_2 = count_trees_on_slope(&rows, 3, 1);
    let tree_count_3 = count_trees_on_slope(&rows, 5, 1);
    let tree_count_4 = count_trees_on_slope(&rows, 7, 1);
    let tree_count_5 = count_trees_on_slope(&rows, 1, 2);

    println!(
        "{}",
        tree_count_1 * tree_count_2 * tree_count_3 * tree_count_4 * tree_count_5
    );
}
