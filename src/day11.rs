// execute vim command
// :%s/11/{Number of Day}/g
// to set correct day

type Line = Vec<(u64, bool)>;
type Inp = Vec<Line>;

#[aoc_generator(day11)]
pub fn generator_day11(input: &str) -> Inp {
    let mut res: Vec<Line> = vec![];
    for line in input.lines() {
        let mut x: Line = vec![];
        for c in line.trim().chars() {
            x.push(((c as u8 - '0' as u8) as u64, false));
        }
        res.push(x);
    }
    res
}

#[aoc(day11, part1)]
pub fn solve_day11_part1(inp: &Inp) -> u64 {
    let mut input = inp.clone();
    let mut count: u64 = 0;
    for _ in 0..100 {
        // increase everything by 1
        for y in 0..input.len() {
            for x in 0..input[0].len() {
                input[y][x].0 += 1;
            }
        }

        // for all bigger than 9s flash around them
        for y in 0..input.len() {
            for x in 0..input[0].len() {
                if input[y][x].0 > 9 {
                    rec_flash(&mut input, x, y);
                }
            }
        }

        // count bigger than 9s and reset them to 0s
        for y in 0..input.len() {
            for x in 0..input[0].len() {
                input[y][x].1 = false;
                if input[y][x].0 > 9 {
                    count += 1;
                    input[y][x].0 = 0;
                }
            }
        }
    }
    count
}

fn rec_flash(input: &mut Vec<Vec<(u64, bool)>>, x: usize, y: usize) {
    if input[y][x].0 > 9 && input[y][x].1 == false {
        input[y][x].1 = true;

        // iterate over every neighbor
        for y_diff in 0..=2 {
            for x_diff in 0..=2 {
                if x + x_diff - 1 < input[0].len()
                    && x + x_diff >= 1
                    && y + y_diff - 1 < input.len()
                    && y + y_diff >= 1
                    && !input[y + y_diff - 1][x + x_diff - 1].1
                {
                    // if the neighbor has not been visited do so
                    input[y + y_diff - 1][x + x_diff - 1].0 += 1;
                    rec_flash(input, x + x_diff - 1, y + y_diff - 1);
                }
            }
        }
    }
}

#[aoc(day11, part2)]
pub fn solve_day11_part2(inp: &Inp) -> u64 {
    let mut input = inp.clone();
    let mut step: u64 = 0;
    loop {
        step += 1;

        // increase everything by 1
        for y in 0..input.len() {
            for x in 0..input[0].len() {
                input[y][x].0 += 1;
            }
        }

        // for all bigger than 9s flash around them
        for y in 0..input.len() {
            for x in 0..input[0].len() {
                if input[y][x].0 > 9 {
                    rec_flash(&mut input, x, y);
                }
            }
        }

        // check if all flashed otherwise reset all flashed ones
        let mut all_flashed: bool = true;
        for y in 0..input.len() {
            for x in 0..input[0].len() {
                input[y][x].1 = false;
                if input[y][x].0 > 9 {
                    input[y][x].0 = 0;
                } else {
                    all_flashed = false;
                }
            }
        }
        if all_flashed {
            break;
        }
    }
    step
}
