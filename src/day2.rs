pub enum Inst {
    Down(u64),
    Up(u64),
    Forward(u64),
}
#[aoc_generator(day2)]
pub fn generator_day2(input: &str) -> Vec<Inst> {
    let mut res: Vec<Inst> = vec![];
    for line in input.lines() {
        let mut iter = line.trim().split_whitespace();
        match iter.next().unwrap() {
            "forward" => res.push(Inst::Forward(iter.next().unwrap().parse().unwrap())),
            "down" => res.push(Inst::Down(iter.next().unwrap().parse().unwrap())),
            "up" => res.push(Inst::Up(iter.next().unwrap().parse().unwrap())),
            _ => (),
        }
    }
    res
}

#[aoc(day2, part1)]
pub fn solve_day1_part1(input: &Vec<Inst>) -> u64 {
    let mut depth: u64 = 0;
    let mut hor: u64 = 0;
    for line in input {
        match line {
            Inst::Down(amount) => depth += amount,
            Inst::Up(amount) => depth -= amount,
            Inst::Forward(amount) => hor += amount,
        }
    }
    depth * hor
}
#[aoc(day2, part2)]
pub fn solve_day2_part2(input: &Vec<Inst>) -> u64 {
    let mut depth: u64 = 0;
    let mut hor: u64 = 0;
    let mut aim: u64 = 0;
    for line in input {
        match line {
            Inst::Down(amount) => aim += amount,
            Inst::Up(amount) => aim -= amount,
            Inst::Forward(amount) => {
                hor += amount;
                depth += aim * amount;
            }
        }
    }
    depth * hor
}
