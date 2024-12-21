use itertools::Itertools;
use std::fs::read_to_string;

pub fn total_distance(filename: &str) -> i32 {
    let input = read_to_string(filename).unwrap_or(String::from(""));

    let left = input
        .lines()
        .map(|l| l.split_whitespace().next().unwrap().parse::<i32>().unwrap())
        .into_iter()
        .sorted()
        .collect::<Vec<_>>();

    let right = input
        .lines()
        .map(|l| l.split_whitespace().last().unwrap().parse::<i32>().unwrap())
        .into_iter()
        .sorted()
        .collect::<Vec<_>>();

    distances(&left, &right).iter().sum()
}

pub fn similarity_score(filename: &str) -> i32 {
    let input = read_to_string(filename).unwrap_or(String::from(""));

    let left = input
        .lines()
        .map(|l| l.split_whitespace().next().unwrap().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let right = input
        .lines()
        .map(|l| l.split_whitespace().last().unwrap().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    left.iter().map(|e| similarity(e, &right)).sum()
}

fn distances(left: &Vec<i32>, right: &Vec<i32>) -> Vec<i32> {
    left.iter()
        .zip(right.iter())
        .map(|e| (e.0 - e.1).abs())
        .collect()
}

fn similarity(value: &i32, list: &Vec<i32>) -> i32 {
    let mut count = 0;
    for e in list {
        if e == value {
            count += 1;
        }
    }
    count * value
}
