use std::fs;
use std::io;
use std::ops::Index;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Unable to read file");
    let numbers = contents
        .split_whitespace()
        .map(|number| number.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut column1: Vec<i32> = Vec::new();
    let mut colunmn2: Vec<i32> = Vec::new();
    for i in 0..numbers.len() {
        if (i % 2) == 0 {
            colunmn2.push(numbers[i]);
        } else {
            column1.push(numbers[i]);
        }
    }
    column1.sort();
    colunmn2.sort();
    part_1(&column1, &colunmn2);
    part_2(&column1, &colunmn2);
}

// part1
fn part_1(column1: &Vec<i32>, colunmn2: &Vec<i32>) {
    let mut pairs = Vec::new();
    for i in 0..column1.len() {
        pairs.push(vec![column1[i], colunmn2[i]]);
    }
    let mut total_distance = 0;
    for i in 0..pairs.len() {
        total_distance += (pairs[i][0] - pairs[i][1]).abs();
    }
    println!("Total distance: {}", total_distance);
}

fn part_2(column1: &Vec<i32>, colunmn2: &Vec<i32>) {
    let mut simmilarities: Vec<Vec<i32>> = Vec::new();
    for i in 0..column1.len() {
        for j in 0..colunmn2.len() {
            if column1[i] == colunmn2[j] {
                println!("Common number: {}", column1[i]);
                if simmilarities.iter().any(|v| v[0] == column1[i]) {
                    let index = simmilarities
                        .iter()
                        .position(|v| v[0] == column1[i])
                        .unwrap();
                    simmilarities[index][1] += 1;
                } else {
                    simmilarities.push(vec![column1[i], 1]);
                }
            }
        }
    }
    println!("Simmilarities: {:?}", simmilarities);
    let mut similiarites_index = 0;
    for i in 0..simmilarities.len() {
        similiarites_index += simmilarities[i][0] * simmilarities[i][1];
    }
    println!("Similiarites index: {}", similiarites_index);
}
