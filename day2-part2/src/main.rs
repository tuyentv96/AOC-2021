use std::{
    fs::File,
    io::{BufRead, BufReader},
};

enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

fn parse_command(str: &str) -> Option<Command> {
    let mut parts = str.split_whitespace();

    let cmd = match parts.next() {
        Some(v) => v,
        None => return None,
    };

    let value: i32 = match parts.next() {
        Some(v) => match v.parse() {
            Ok(i) => i,
            Err(_) => return None,
        },
        None => return None,
    };

    match cmd {
        "forward" => Some(Command::Forward(value)),
        "up" => Some(Command::Up(value)),
        "down" => Some(Command::Down(value)),
        _ => None,
    }
}

fn main() {
    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);
    let commands: Vec<Option<Command>> = reader
        .lines()
        .map(|s| s.unwrap())
        .map(|s| parse_command(&s))
        .collect();

    let mut depth: i32 = 0;
    let mut horizontal: i32 = 0;
    let mut aim: i32 = 0;

    for command in commands {
        match command.unwrap() {
            Command::Forward(v) => {
                horizontal += v;
                depth += aim * v;
            }

            Command::Up(v) => {
                aim -= v;
            }
            Command::Down(v) => {
                aim += v;
            }
        }
    }

    println!("result: {}", depth * horizontal);
}
