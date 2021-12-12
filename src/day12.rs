// execute vim command
// :%s/12/{Number of Day}/g
// to set correct day
use std::collections::{HashMap, HashSet};

type Line = HashSet<String>;
type Inp = HashMap<String, Line>;

#[aoc_generator(day12)]
pub fn generator_day12(input: &str) -> Inp {
    let mut list_of_nodes: HashMap<String, Line> = HashMap::new();
    for line in input.lines() {
        let mut iter = line.trim().split('-');
        let a: String = iter.next().unwrap().to_string();
        let b: String = iter.next().unwrap().to_string();
        if !list_of_nodes.contains_key(&a) {
            list_of_nodes.insert(a.clone(), HashSet::new());
        }
        if !list_of_nodes.contains_key(&b) {
            list_of_nodes.insert(b.clone(), HashSet::new());
        }
        if let Some(x) = list_of_nodes.get_mut(&a) {
            x.insert(b.clone());
        }
        if let Some(x) = list_of_nodes.get_mut(&b) {
            x.insert(a);
        }
    }
    list_of_nodes
}

#[aoc(day12, part1)]
pub fn solve_day12_part1(input: &Inp) -> u64 {
    let mut count: u64 = 0;
    dfs("start".to_string(), input, &vec![], &mut count);
    count
}

fn dfs(current: String, list_of_nodes: &Inp, visited_old: &Vec<String>, count: &mut u64) {
    let mut visited = visited_old.clone();
    visited.push(current.clone());
    if &current == "end" {
        *count += 1;
        visited.pop();
        return;
    }
    for next in list_of_nodes.get(&current).unwrap() {
        if next == "start" {
            continue;
        }
        if &next.to_lowercase() == next {
            if visited.contains(next) {
                continue;
            }
        }
        dfs(next.clone(), list_of_nodes, &visited, count);
    }
    visited.pop();
}

#[aoc(day12, part2)]
pub fn solve_day12_part2(input: &Inp) -> u64 {
    let mut count: u64 = 0;
    dfs2("start".to_string(), input, &vec![], &mut count, false);
    count
}

fn dfs2(
    current: String,
    list_of_nodes: &Inp,
    visited_old: &Vec<String>,
    count: &mut u64,
    twice: bool,
) {
    let mut visited = visited_old.clone();
    visited.push(current.clone());
    if &current == "end" {
        *count += 1;
        visited.pop();
        return;
    }
    for next in list_of_nodes.get(&current).unwrap() {
        if next == "start" {
            continue;
        }
        if &next.to_lowercase() == next {
            if visited.contains(next) {
                if !twice {
                    dfs2(next.clone(), list_of_nodes, &visited, count, true);
                }
                continue;
            }
        }
        dfs2(next.clone(), list_of_nodes, &visited, count, twice);
    }
    visited.pop();
}
