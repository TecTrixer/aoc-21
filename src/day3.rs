type Line = Vec<bool>;
#[aoc_generator(day3)]
pub fn generator_day3(input: &str) -> Vec<Line> {
    let mut res: Vec<Line> = vec![];
    for raw_line in input.lines() {
        let mut line: Line = vec![];
        for num in raw_line.trim().chars() {
            match num {
                '0' => line.push(false),
                '1' => line.push(true),
                _ => (),
            }
        }
        res.push(line);
    }
    res
}

#[aoc(day3, part1)]
pub fn solve_day3_part1(input: &Vec<Line>) -> u64 {
    let mut gamma: Vec<bool> = vec![];
    for y in 0..input[0].len() {
        let mut count0: u64 = 0;
        let mut count1: u64 = 0;
        for x in 0..input.len() {
            if input[x][y] {
                count1 += 1;
            } else {
                count0 += 1;
            }
        }
        if count1 > count0 {
            gamma.push(true);
        } else {
            gamma.push(false);
        }
    }
    let mut a: u64 = 0;
    let mut b: u64 = 0;
    for (i, x) in gamma.iter().rev().enumerate() {
        if *x {
            a += (2 as u64).pow(i as u32);
        } else {
            b += (2 as u64).pow(i as u32);
        }
    }
    a * b
}

#[aoc(day3, part2)]
pub fn solve_day3_part2(input: &Vec<Line>) -> u64 {
    let mut gamma: Vec<bool> = vec![];
    let mut epsilon: Vec<bool> = vec![];
    for y in 0..input[0].len() {
        let mut count0_gamma: u64 = 0;
        let mut count0_epsilon: u64 = 0;
        let mut count1_gamma: u64 = 0;
        let mut count1_epsilon: u64 = 0;
        let mut nums_epsilon: u64 = 0;
        for x in 0..input.len() {
            if check(&gamma, &input[x]) {
                if input[x][y] {
                    count1_gamma += 1;
                } else {
                    count0_gamma += 1;
                }
            }
            if check(&epsilon, &input[x]) {
                nums_epsilon += 1;
                if input[x][y] {
                    count1_epsilon += 1;
                } else {
                    count0_epsilon += 1;
                }
            }
        }
        if count1_gamma >= count0_gamma {
            gamma.push(true);
        } else {
            gamma.push(false);
        }
        if nums_epsilon < 2 {
            if count0_epsilon > count1_epsilon {
                epsilon.push(false);
            } else {
                epsilon.push(true);
            }
        } else if count1_epsilon < count0_epsilon {
            epsilon.push(true);
        } else {
            epsilon.push(false);
        }
    }
    let mut a: u64 = 0;
    let mut b: u64 = 0;
    for x in gamma {
        if x {
            a |= 1;
        }
        a <<= 1;
    }
    a >>= 1;
    for x in epsilon {
        if x {
            b |= 1;
        }
        b <<= 1;
    }
    b >>= 1;
    a * b
}

fn check(a: &Vec<bool>, b: &Vec<bool>) -> bool {
    for (x, y) in a.iter().zip(b.iter()) {
        if x != y {
            return false;
        }
    }
    return true;
}
