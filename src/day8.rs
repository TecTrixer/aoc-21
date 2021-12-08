// execute vim command
// :%s/8/{Number of Day}/g
// to set correct day
use itertools::Itertools;

type Line = (Vec<String>, Vec<String>);
type Inp = Vec<Line>;

#[aoc_generator(day8)]
pub fn generator_day8(input: &str) -> Inp {
    let mut res: Vec<(Vec<String>, Vec<String>)> = vec![];
    for line in input.lines() {
        let mut part1vec: Vec<String> = vec![];
        let mut part = line.split('|');
        let part1 = part.next().unwrap().trim();
        for elem in part1.split_whitespace() {
            part1vec.push(elem.to_string());
        }
        let part2 = part.next().unwrap().trim();
        let mut part2vec: Vec<String> = vec![];
        for elem in part2.split_whitespace() {
            part2vec.push(elem.to_string());
        }
        res.push((part1vec, part2vec));
    }
    res
}

#[aoc(day8, part1)]
pub fn solve_day8_part1(input: &Inp) -> u64 {
    let mut counter: u64 = 0;
    for line in input {
        let mut one: String = "".to_string();
        let mut seven: String = "".to_string();
        let mut four: String = "".to_string();
        let mut eight: String = "".to_string();
        for elem in &line.0 {
            if elem.len() == 2 {
                one = elem.chars().sorted().collect::<String>();
            } else if elem.len() == 3 {
                seven = elem.chars().sorted().collect::<String>();
            } else if elem.len() == 4 {
                four = elem.chars().sorted().collect::<String>();
            } else if elem.len() == 7 {
                eight = elem.chars().sorted().collect::<String>();
            }
        }
        for disp in &line.1 {
            let elem = disp.chars().sorted().collect::<String>();
            if elem == one || elem == four || elem == seven || elem == eight {
                counter += 1;
            }
        }
    }
    counter
}

#[aoc(day8, part2)]
pub fn solve_day8_part2(input: &Inp) -> u64 {
    let mut counter: u64 = 0;
    for line in input {
        let mut loc_count: u64 = 0;
        let mut one: String = "".to_string();
        let mut seven: String = "".to_string();
        let mut four: String = "".to_string();
        let mut eight: String = "".to_string();
        let mut two: String = "".to_string();
        let mut zero: String = "".to_string();
        let mut three: String = "".to_string();
        let mut five: String = "".to_string();
        let mut six: String = "".to_string();
        let mut nine: String = "".to_string();
        for elem in &line.0 {
            if elem.len() == 2 {
                one = elem.chars().sorted().collect::<String>();
            } else if elem.len() == 3 {
                seven = elem.chars().sorted().collect::<String>();
            } else if elem.len() == 4 {
                four = elem.chars().sorted().collect::<String>();
            } else if elem.len() == 7 {
                eight = elem.chars().sorted().collect::<String>();
            }
        }
        for disp in &line.0 {
            let elem = disp.chars().sorted().collect::<String>();
            let mut fivecheck = four.clone();
            for c in one.chars() {
                fivecheck.retain(|d| c != d);
            }
            if elem == one || elem == four || elem == seven || elem == eight {
                continue;
            } else if contains(&elem, &four) {
                nine = elem;
            } else if elem.len() == 5 && contains(&elem, &one) {
                three = elem;
            } else if elem.len() == 6 && contains(&elem, &one) {
                zero = elem;
            } else if elem.len() == 6 {
                six = elem;
            } else if contains(&elem, &fivecheck) {
                five = elem;
            } else {
                two = elem;
            }
        }
        for (i, disp) in line.1.iter().rev().enumerate() {
            let elem = disp.chars().sorted().collect::<String>();
            if elem == one {
                loc_count += 10_u64.pow(i as u32);
            } else if elem == two {
                loc_count += 2 * 10_u64.pow(i as u32);
            } else if elem == three {
                loc_count += 3 * 10_u64.pow(i as u32);
            } else if elem == four {
                loc_count += 4 * 10_u64.pow(i as u32);
            } else if elem == five {
                loc_count += 5 * 10_u64.pow(i as u32);
            } else if elem == six {
                loc_count += 6 * 10_u64.pow(i as u32);
            } else if elem == seven {
                loc_count += 7 * 10_u64.pow(i as u32);
            } else if elem == eight {
                loc_count += 8 * 10_u64.pow(i as u32);
            } else if elem == nine {
                loc_count += 9 * 10_u64.pow(i as u32);
            }
        }
        counter += loc_count;
    }
    counter
}

fn contains(a: &String, b: &String) -> bool {
    for c in b.chars() {
        if !a.contains(c) {
            return false;
        }
    }
    return true;
}
