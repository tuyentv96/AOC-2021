use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);
    let num_lines: Vec<i32> = reader
        .lines()
        .map(|s| s.unwrap())
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut prev: i32 = *num_lines.get(0).unwrap();
    let mut count: i32 = 0;

    for i in num_lines {
        if i > prev {
            count += 1;
        }

        prev = i;
    }

    println!("result: {}", count);
}
