#[aoc_generator(day1)]
pub fn generator_day1(input: &str) -> Vec<u32> {
    let mut res: Vec<u32> = vec![];
    for line in input.lines() {
        res.push(line.parse().unwrap());
    }
    res
}

#[aoc(day1, part1)]
pub fn solve_day1_part1(input: &Vec<u32>) -> u32 {
    let mut counter = 0;
    for i in 1..input.len() {
        if input[i] > input[i - 1] {
            counter += 1;
        }
    }
    counter
}
#[aoc(day1, part2)]
pub fn solve_day1_part2(input: &Vec<u32>) -> u32 {
    let mut counter = 0;
    for i in 3..input.len() {
        if input[i] > input[i - 3] {
            counter += 1;
        }
    }
    counter
}
