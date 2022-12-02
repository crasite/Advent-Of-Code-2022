use chrono::prelude::*;

fn main() {
    let _sample = include_str!("input_sample.txt");
    let input = include_str!("input.txt");
    let t1 = Utc::now();
    let current_max = calculate_max(input);
    println!("{} in {}", current_max, Utc::now() - t1);
    let t1 = Utc::now();
    let current_max = calculate_top_three(input);
    println!("{} in {}", current_max, Utc::now() - t1);
}

fn calculate_max(input: &str) -> u64 {
    let mut current_max: u64 = 0;
    let mut current: u64 = 0;
    for line in input.lines() {
        if !line.is_empty() {
            current += line.parse::<u64>().unwrap();
        } else {
            compare_current(&mut current, &mut current_max);
        }
    }
    compare_current(&mut current, &mut current_max);
    current_max
}

fn compare_current(current: &mut u64, current_max: &mut u64) {
    if *current > *current_max {
        *current_max = *current;
    }
    *current = 0;
}

fn calculate_top_three(input: &str) -> u64 {
    let mut current_top = vec![0, 0, 0];
    let mut current: u64 = 0;
    for line in input.lines() {
        if !line.is_empty() {
            current += line.parse::<u64>().unwrap();
        } else {
            compare_top(&mut current_top, &mut current);
        }
    }
    compare_top(&mut current_top, &mut current);
    current_top.iter().sum()
}

fn compare_top(current_top: &mut Vec<u64>, current: &mut u64) {
    current_top.push(*current);
    current_top.sort_by(|a, b| b.cmp(a));
    current_top.pop();
    *current = 0;
}
