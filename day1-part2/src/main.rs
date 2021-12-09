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

    let mut count: i32 = 0;
    let mut prev: i32 = num_lines.iter().take(2).sum();

    for (i, x) in num_lines.iter().enumerate() {
        if i == num_lines.len() - 2 {
            break;
        }

        if i == 0 {
            continue;
        }

        let sum = prev + *num_lines.get(i + 2).unwrap() - *num_lines.get(i - 1).unwrap();

        if sum > prev {
            count += 1;
        }

        prev = sum;
    }

    println!("result: {}", count);
}
