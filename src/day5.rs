use std::{collections::HashMap, fs::read_to_string};

use itertools::Itertools;

pub fn sum_of_correct_update_middle_pages(filename_rules: &str, filename_updates: &str) -> i32 {
    let rules = parse_rules(filename_rules);
    let updates = parse_updates(filename_updates);

    updates
        .iter()
        .filter(|update| is_correctly_ordered(update, &rules))
        .map(|update| update[update.len() / 2])
        .sum()
}

pub fn sum_of_reordered_middle_pages(filename_rules: &str, filename_updates: &str) -> i32 {
    let rules = parse_rules(filename_rules);
    let updates = parse_updates(filename_updates);

    updates
        .iter()
        .filter(|update| !is_correctly_ordered(update, &rules))
        .map(|update| reordered_update(update, &rules))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn parse_rules(filename: &str) -> HashMap<i32, Vec<i32>> {
    read_to_string(filename)
        .unwrap_or(String::from(""))
        .lines()
        .fold(HashMap::<i32, Vec<i32>>::new(), |mut map, line| {
            let mut parts = line.split("|").map(|e| e.parse::<i32>().unwrap());
            match parts.next_tuple() {
                Some((l, r)) => map.entry(l).or_insert(vec![]).push(r),
                None => (),
            }
            map
        })
}

fn parse_updates(filename: &str) -> Vec<Vec<i32>> {
    read_to_string(filename)
        .unwrap_or(String::from(""))
        .lines()
        .map(|line| {
            line.split(",")
                .map(|e| e.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn reordered_update(update: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut new_update = update.clone();

    while !is_correctly_ordered(&new_update, rules) {
        reorder_single(&mut new_update, rules);
    }

    new_update
}

fn reorder_single(update: &mut Vec<i32>, rules: &HashMap<i32, Vec<i32>>) {
    for i in 0..update.len() {
        let page = update[i];
        for j in i + 1..update.len() {
            let following_page = update[j];
            match rules.get(&following_page) {
                Some(deps) => {
                    if deps.contains(&page) {
                        update.swap(i, j);
                        return;
                    }
                }
                None => (),
            }
        }
    }
}

fn is_correctly_ordered(update: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> bool {
    for (i, page) in update.iter().enumerate() {
        for following_page in update.iter().skip(i + 1) {
            match rules.get(following_page) {
                Some(deps) => {
                    if deps.contains(page) {
                        return false;
                    }
                }
                None => (),
            }
        }
    }

    true
}
