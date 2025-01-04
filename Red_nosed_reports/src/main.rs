use core::panic;
use std::fs;

enum Level {
    Safe,
    Unsafe,
}

fn is_safe(level: &Vec<i32>) -> Level {
    let is_increasing = level.windows(2).all(|w| w[1] > w[0]);
    let is_decreasing = level.windows(2).all(|w| w[1] < w[0]);

    if (is_increasing || is_decreasing)
        && level
            .windows(2)
            .all(|w| (w[1] - w[0]).abs() >= 1 && (w[1] - w[0]).abs() <= 3)
    {
        Level::Safe
    } else {
        Level::Unsafe
    }
}

fn is_safe_with_dampener(level: &Vec<i32>) -> Level {
    for i in 0..level.len() {
        let mut temp_level = level.clone();
        temp_level.remove(i);
        if let Level::Safe = is_safe(&temp_level) {
            return Level::Safe;
        }
    }
    Level::Unsafe
}

fn main() {
    let contents = match fs::read_to_string("input.txt") {
        Ok(contents) => contents,
        Err(err) => panic!("Error reading file: {}", err),
    };
    let lines: Vec<String> = contents.lines().map(|s| String::from(s)).collect();
    let lines_as_ints: Vec<Vec<i32>> = lines
        .iter()
        .map(|s| {
            s.split_whitespace()
                .map(|x| x.parse::<i32>().expect("Error parsing to int"))
                .collect()
        })
        .collect();
    let mut safe_levels = 0;
    for level in lines_as_ints.iter() {
        match is_safe(&level) {
            Level::Safe => safe_levels += 1,
            Level::Unsafe => continue,
        }
    }
    let mut safe_levels_with_dampener = 0;
    for level in lines_as_ints.iter() {
        match is_safe_with_dampener(&level) {
            Level::Safe => safe_levels_with_dampener += 1,
            Level::Unsafe => continue,
        }
    }
    println!("Safe levels: {}", safe_levels);
    println!("Safe levels with dampener: {}", safe_levels_with_dampener);
}
