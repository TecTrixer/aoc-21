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
    getBigger(&mut input.clone()) * getSmaller(&mut input.clone())
}

fn getBigger(rows: &mut Vec<Vec<bool>>) -> u64 {
    for col in 0..rows[0].len() {
        let mut count0: u64 = 0;
        let mut count1: u64 = 0;
        for row in 0..rows.len() {
            if rows[row][col] {
                count1 += 1;
            } else {
                count0 += 1;
            }
        }
        if count1 >= count0 {
            rows.retain(|x| x[col]);
        } else {
            rows.retain(|x| !x[col]);
        }
        if rows.len() < 2 {
            break;
        }
    }
    let mut res: u64 = 0;
    for (i, x) in rows[0].iter().rev().enumerate() {
        if *x {
            res += (2 as u64).pow(i as u32);
        }
    }
    res
}
fn getSmaller(rows: &mut Vec<Vec<bool>>) -> u64 {
    for col in 0..rows[0].len() {
        let mut count0: u64 = 0;
        let mut count1: u64 = 0;
        for row in 0..rows.len() {
            if rows[row][col] {
                count1 += 1;
            } else {
                count0 += 1;
            }
        }
        if count1 < count0 {
            rows.retain(|x| x[col]);
        } else {
            rows.retain(|x| !x[col]);
        }
        if rows.len() < 2 {
            break;
        }
    }
    let mut res: u64 = 0;
    for (i, x) in rows[0].iter().rev().enumerate() {
        if *x {
            res += (2 as u64).pow(i as u32);
        }
    }
    res
}
