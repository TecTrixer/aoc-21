// execute vim command
// :%s/X/{Number of Day}/g
// to set correct day

type Line = Vec<u64>;
type Inp = (Line);

#[aoc_generator(dayX)]
pub fn generator_dayX(input: &str) -> Inp {
    let mut res: Vec<u64> = vec![];
    for line in input.lines() {
        res.push(line.parse().unwrap());
    }
    (res)
}

#[aoc(dayX, part1)]
pub fn solve_dayX_part1(input: &Inp) -> u64 {
    0
}

#[aoc(dayX, part2)]
pub fn solve_dayX_part2(input: &Inp) -> u64 {
    0
}
