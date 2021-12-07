// execute vim command
// :%s/7/{Number of Day}/g
// to set correct day

type Line = Vec<u64>;
type Inp = (u64, u64, Line);

#[aoc_generator(day7)]
pub fn generator_day7(input: &str) -> Inp {
    let mut res: Vec<u64> = vec![];
    for line in input.lines().next().unwrap().trim().split(",") {
        res.push(line.parse().unwrap());
    }
    let mut min: u64 = u64::MAX;
    let mut max: u64 = 0;
    for num in &res {
        if num < &min {
            min = *num;
        }
        if num > &max {
            max = *num;
        }
    }
    (min, max, res)
}

#[aoc(day7, part1)]
pub fn solve_day7_part1(input: &Inp) -> u64 {
    let min: u64 = input.0;
    let max: u64 = input.1;
    let nums: &Vec<u64> = &input.2;
    let mut best_x = min;
    let mut best_num = u64::MAX;
    for i in min..=max {
        let mut counter: u64 = 0;
        for num in nums {
            counter += (*num as i64 - i as i64).abs() as u64;
        }
        if counter < best_num {
            best_num = counter;
            best_x = i;
        }
    }
    best_num
}

#[aoc(day7, part2)]
pub fn solve_day7_part2(input: &Inp) -> u64 {
    let min: u64 = input.0;
    let max: u64 = input.1;
    let nums: &Vec<u64> = &input.2;
    let mut best_x = min;
    let mut best_num = u64::MAX;
    for i in min..=max {
        let mut counter: u64 = 0;
        for num in nums {
            let dist: u64 = (*num as i64 - i as i64).abs() as u64;
            counter += dist * (dist + 1) / 2 + 1;
        }
        if counter < best_num {
            best_num = counter;
            best_x = i;
        }
    }
    best_num
}
