use regex::Regex;
use std::{fmt::Error, fs};

fn main() {
    part_1();
}

fn part_1() {
    let contents = match fs::read_to_string("./input.txt") {
        Ok(contents) => contents,
        Err(_) => panic!("Failed to read file"),
    };

    // Find all control instructions and multiplications
    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let do_regex = Regex::new(r"do\(\)").unwrap();
    let dont_regex = Regex::new(r"don't\(\)").unwrap();

    // Collect all operations with their positions
    let mut operations = Vec::new();

    // Add multiplications
    for cap in mul_regex.captures_iter(&contents) {
        let x: i32 = cap[1].parse().unwrap();
        let y: i32 = cap[2].parse().unwrap();
        let pos = cap.get(0).unwrap().start();
        operations.push((pos, Operation::Mul(x, y)));
    }

    // Add do() operations
    for m in do_regex.find_iter(&contents) {
        operations.push((m.start(), Operation::Do));
    }

    // Add don't() operations
    for m in dont_regex.find_iter(&contents) {
        operations.push((m.start(), Operation::Dont));
    }

    // Sort by position
    operations.sort_by_key(|&(pos, _)| pos);

    // Process operations in order
    let mut enabled = true;
    let mut sum = 0;

    for (_, op) in operations {
        match op {
            Operation::Do => enabled = true,
            Operation::Dont => enabled = false,
            Operation::Mul(x, y) if enabled => sum += x * y,
            _ => {}
        }
    }

    println!("Sum of enabled multiplications: {}", sum);
}

#[derive(Debug)]
enum Operation {
    Mul(i32, i32),
    Do,
    Dont,
}
