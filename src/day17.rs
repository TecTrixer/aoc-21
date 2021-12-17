// execute vim command
// :%s/17/{Number of Day}/g
// to set correct day

pub struct Line {
    x1: i64,
    x2: i64,
    y1: i64,
    y2: i64,
}
type Inp = Line;

#[aoc_generator(day17)]
pub fn generator_day17(_input: &str) -> Inp {
    Line {
        x1: 235,
        x2: 259,
        y1: -62,
        y2: -118,
    }
}

#[aoc(day17, part1)]
pub fn solve_day17_part1(input: &Inp) -> u64 {
    let y2 = input.y2;
    let res = -1 * y2 * (-1 * y2 - 1) / 2;
    res as u64
}

#[aoc(day17, part2)]
pub fn solve_day17_part2(input: &Inp) -> u64 {
    let mut count: u64 = 0;
    for x_start_vel in 1..input.x2 + 1 {
        for y_start_vel in input.y2..solve_day17_part1(input) as i64 {
            let mut x: i64 = 0;
            let mut y: i64 = 0;
            let mut y_vel: i64 = y_start_vel;
            let mut x_vel: i64 = x_start_vel;
            while x < input.x2 + 1 && y + 1 > input.y2 {
                if x + 1 > input.x1 && y < input.y1 + 1 {
                    count += 1;
                    break;
                }
                x += x_vel;
                y += y_vel;
                if x_vel > 0 {
                    x_vel -= 1;
                }
                y_vel -= 1;
            }
        }
    }
    count
}
