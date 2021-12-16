// execute vim command
// :%s/15/{Number of Day}/g
// to set correct day

type Line = Vec<u64>;
type Inp = Vec<Line>;

#[aoc_generator(day15)]
pub fn generator_day15(input: &str) -> Inp {
    let mut res: Vec<Line> = vec![];
    for line in input.lines() {
        let mut x: Line = vec![];
        for c in line.trim().chars() {
            x.push((c as u8 - '0' as u8) as u64);
        }
        res.push(x);
    }
    res
}

#[aoc(day15, part1)]
pub fn solve_day15_part1(input: &Inp) -> u64 {
    let mut dp: Vec<Vec<u64>> = vec![];
    for row in 0..input.len() {
        let mut temp: Vec<u64> = vec![];
        for col in 0..input[0].len() {
            temp.push(input[row][col]);
        }
        dp.push(temp);
    }
    for i in 0..dp.len() {
        for j in 0..dp[0].len() {
            if i == 0 && j == 0 {
                continue;
            } else if i == 0 {
                dp[i][j] += dp[i][j - 1];
            } else if j == 0 {
                dp[i][j] += dp[i - 1][j];
            } else {
                dp[i][j] += min(dp[i - 1][j], dp[i][j - 1]);
            }
        }
    }
    dp[input.len() - 1][input[0].len() - 1] - input[0][0]
}

fn _print(input: &Vec<Vec<u64>>) {
    for row in input {
        print!("[");
        for elem in row {
            print!("{:?}, ", elem);
        }
        println!("]");
    }
}

#[aoc(day15, part2)]
pub fn solve_day15_part2(input: &Inp) -> u64 {
    let mut dp: Vec<Vec<u64>> = vec![];
    for row in 0..input.len() * 5 {
        let mut temp: Vec<u64> = vec![];
        for col in 0..input[0].len() * 5 {
            let x = col / input[0].len();
            let y = row / input.len();
            let mut val = (input[row % input.len()][col % input.len()] + x as u64 + y as u64) % 9;
            if val == 0 {
                val = 9;
            }
            temp.push(val);
        }
        dp.push(temp);
    }
    for i in 0..dp.len() {
        for j in 0..dp[0].len() {
            if i == 0 && j == 0 {
                continue;
            } else if i == 0 {
                dp[i][j] += dp[i][j - 1];
            } else if j == 0 {
                dp[i][j] += dp[i - 1][j];
            } else {
                dp[i][j] += min(dp[i - 1][j], dp[i][j - 1]);
            }
        }
    }
    dp[dp.len() - 1][dp[0].len() - 1] - input[0][0]
}

fn min(a: u64, b: u64) -> u64 {
    if a < b {
        return a;
    } else {
        return b;
    }
}
