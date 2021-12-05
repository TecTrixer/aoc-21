// execute vim command
// :%s/6/{Number of Day}/g
// to set correct day

type Line = Vec<u64>;
type Inp = (Line);

#[aoc_generator(day6)]
pub fn generator_day6(input: &str) -> Inp {
    let mut res: Vec<u64> = vec![];
    for line in input.lines() {
        res.push(line.parse().unwrap());
    }
    (res)
}

#[aoc(day6, part1)]
pub fn solve_day6_part1(input: &Inp) -> u64 {
    0
}

#[aoc(day6, part2)]
pub fn solve_day6_part2(input: &Inp) -> u64 {
    0
}
