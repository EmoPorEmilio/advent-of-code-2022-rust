//https://adventofcode.com/2022/day/1

use std::fs::read_to_string;

fn first() {
    let mut max = 0;
    let strings: Vec<String> = read_to_string("src/input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    let mut i = 0;
    while i < strings.len() {
        let mut current_sum = 0;
        while i < strings.len() && !strings[i].is_empty() {
            let current_number = strings[i].parse::<i32>().unwrap();
            current_sum += current_number;
            i += 1;
        }
        if current_sum > max {
            max = current_sum;
        }
        i += 1;
    }
    println!("The max is {}", max);
}

fn second() {
    /*
    let mut max = [0, 0, 0];
    let strings: Vec<String> = read_to_string("src/input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    let mut i = 0;
    while i < strings.len() {
        let mut current_sum = 0;
        while i < strings.len() && !strings[i].is_empty() {
            let current_number = strings[i].parse::<i32>().unwrap();
            current_sum += current_number;
            i += 1;
        }
        if current_sum > max {
            max = current_sum;
        }
        i += 1;
    }
    println!("The max is {}", max); */
}
fn main() {
    first();
    second();
}
