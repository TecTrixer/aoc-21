use std::collections::HashMap;
// execute vim command
// :%s/14/{Number of Day}/g
// to set correct day

type Line = HashMap<[char; 2], char>;
type Inp = (Vec<char>, Line);

#[aoc_generator(day14)]
pub fn generator_day14(input: &str) -> Inp {
    let mut res: HashMap<[char; 2], char> = HashMap::new();
    let mut first_part = true;
    let mut polymer = vec![];
    for line in input.lines() {
        if line == "" {
            first_part = false;
            continue;
        }
        if first_part {
            polymer = line.trim().to_string().chars().collect();
        } else {
            let mut iter = line.trim().split(" -> ");
            let mut pattern_iter = iter.next().unwrap().chars();
            let pattern: [char; 2] = [pattern_iter.next().unwrap(), pattern_iter.next().unwrap()];
            let letter: char = iter.next().unwrap().parse().unwrap();
            res.insert(pattern, letter);
        }
    }
    (polymer, res)
}

#[aoc(day14, part1)]
pub fn solve_day14_part1(input: &Inp) -> u64 {
    let string = input.0.clone();
    let mut polymer: HashMap<[char; 2], u64> = HashMap::with_capacity(10);
    for i in 0..string.len() - 1 {
        if let Some(x) = polymer.get(&[string[i], string[i + 1]]) {
            polymer.insert([string[i], string[i + 1]], x + 1);
        } else {
            polymer.insert([string[i], string[i + 1]], 1);
        }
    }
    let mut new_polymer: HashMap<[char; 2], u64> = HashMap::with_capacity(10);
    for _ in 0..10 {
        for ([a, b], x) in polymer.iter() {
            if let Some(c) = input.1.get(&[*a, *b]) {
                if let Some(num) = new_polymer.get(&[*a, *c]) {
                    new_polymer.insert([*a, *c], *num + *x);
                } else {
                    new_polymer.insert([*a, *c], *x);
                }
                if let Some(num) = new_polymer.get(&[*c, *b]) {
                    new_polymer.insert([*c, *b], *num + *x);
                } else {
                    new_polymer.insert([*c, *b], *x);
                }
            }
        }
        polymer = new_polymer;
        new_polymer = HashMap::with_capacity(polymer.len());
    }

    // count
    let mut count_db: HashMap<char, u64> = HashMap::with_capacity(10);
    for ([a, b], x) in polymer.iter() {
        if let Some(num) = count_db.get(a) {
            count_db.insert(*a, *num + *x);
        } else {
            count_db.insert(*a, *x);
        }
    }
    let a = string[string.len() - 1];
    if let Some(num) = count_db.get(&a) {
        count_db.insert(a, *num + 1);
    } else {
        count_db.insert(a, 1);
    }

    let mut count_vec: Vec<u64> = count_db.into_values().collect();
    count_vec.sort();
    count_vec[count_vec.len() - 1] - count_vec[0]
}

#[aoc(day14, part2)]
pub fn solve_day14_part2(input: &Inp) -> u64 {
    let string = input.0.clone();
    let mut polymer: HashMap<[char; 2], u64> = HashMap::with_capacity(10);
    for i in 0..string.len() - 1 {
        if let Some(x) = polymer.get(&[string[i], string[i + 1]]) {
            polymer.insert([string[i], string[i + 1]], x + 1);
        } else {
            polymer.insert([string[i], string[i + 1]], 1);
        }
    }
    let mut new_polymer: HashMap<[char; 2], u64> = HashMap::with_capacity(10);
    for _ in 0..40 {
        for ([a, b], x) in polymer.iter() {
            if let Some(c) = input.1.get(&[*a, *b]) {
                if let Some(num) = new_polymer.get(&[*a, *c]) {
                    new_polymer.insert([*a, *c], *num + *x);
                } else {
                    new_polymer.insert([*a, *c], *x);
                }
                if let Some(num) = new_polymer.get(&[*c, *b]) {
                    new_polymer.insert([*c, *b], *num + *x);
                } else {
                    new_polymer.insert([*c, *b], *x);
                }
            }
        }
        polymer = new_polymer;
        new_polymer = HashMap::with_capacity(polymer.len());
    }

    // count
    let mut count_db: HashMap<char, u64> = HashMap::with_capacity(10);
    for ([a, b], x) in polymer.iter() {
        if let Some(num) = count_db.get(a) {
            count_db.insert(*a, *num + *x);
        } else {
            count_db.insert(*a, *x);
        }
    }
    let a = string[string.len() - 1];
    if let Some(num) = count_db.get(&a) {
        count_db.insert(a, *num + 1);
    } else {
        count_db.insert(a, 1);
    }

    let mut count_vec: Vec<u64> = count_db.into_values().collect();
    count_vec.sort();
    count_vec[count_vec.len() - 1] - count_vec[0]
}
