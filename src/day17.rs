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
pub fn generator_day17(input: &str) -> Inp {
    for line in input.lines() {
        let mut space_iter = line.trim().split(" ");
        space_iter.next().unwrap();
        space_iter.next().unwrap();
        let mut x_iter = space_iter
            .next()
            .unwrap()
            .trim_start_matches("x=")
            .trim_end_matches(",")
            .split("..");
        let mut y_iter = space_iter
            .next()
            .unwrap()
            .trim_start_matches("y=")
            .split("..");
        let x1: i64 = x_iter.next().unwrap().parse().unwrap();
        let x2: i64 = x_iter.next().unwrap().parse().unwrap();
        let y2: i64 = y_iter.next().unwrap().parse().unwrap();
        let y1: i64 = y_iter.next().unwrap().parse().unwrap();
        return Line { x1, x2, y1, y2 };
    }
    Line {
        x1: 235,
        x2: 259,
        y1: -62,
        y2: -118,
    }
}

#[aoc(day17, part1)]
pub fn solve_day17_part1(input: &Inp) -> u64 {
    let y2 = input.y2 * -1;
    let res = y2 * (y2 - 1) / 2;
    res as u64
}

#[aoc(day17, part2)]
pub fn solve_day17_part2(input: &Inp) -> u64 {
    let mut count: u64 = 0;
    let mut x_pos: Vec<(i64, bool)> = vec![];
    for x_start_vel in 1..input.x2 + 1 {
        if x_start_vel * (x_start_vel + 1) / 2 < input.x1 {
            continue;
        }
        let mut x: i64 = x_start_vel;
        let mut x_vel: i64 = x_start_vel - 1;
        while x < input.x2 + 1 {
            if x + 1 > input.x1 {
                if x_vel * (x_vel + 1) / 2 + x <= input.x2 {
                    x_pos.push((x_start_vel, true));
                } else {
                    x_pos.push((x_start_vel, false));
                }
                break;
            } else if x == 0 {
                break;
            }
            x += x_vel;
            x_vel -= 1;
        }
    }
    let upper_y = solve_day17_part1(input) as i64;
    for (x_start_vel, stopping) in x_pos {
        for y_start_vel in input.y2..upper_y {
            if (y_start_vel > 0 && x_start_vel >= input.x1)
                || (!stopping && y_start_vel + 1 > (input.x2 / x_start_vel + 1) / 2)
            {
                break;
            }
            // find x_step_lower and x_step_upper
            let x_step_lower = compute_lower_bound(x_start_vel as f64, input.x1 as f64);
            let x_step_upper = compute_upper_bound(x_start_vel as f64, input.x2 as f64);

            let y_step_lower =
                -1 * compute_upper_bound(-1f64 * y_start_vel as f64, input.y1 as f64);
            let y_step_upper =
                -1 * compute_lower_bound(-1f64 * y_start_vel as f64, input.y2 as f64);

            if y_step_lower > y_step_upper {
                continue;
            }

            //println!(
            //    "x_vel: {}, y_vel: {}  ||   x_lower: {}, y_lower: {}   ||   x_upper: {}, y_upper: {}",
            //    x_start_vel, y_start_vel, x_step_lower, y_step_lower, x_step_upper, y_step_upper
            //);

            if y_step_lower <= x_step_lower && y_step_upper >= x_step_lower {
                count += 1;
                //println!("used x: {}, y: {}", x_start_vel, y_start_vel);
            } else if x_step_lower <= y_step_lower && x_step_upper >= y_step_lower {
                count += 1;
                //println!("used x: {}; y: {}", x_start_vel, y_start_vel);
            } else if x_step_upper == 0 && y_step_upper >= x_step_lower {
                count += 1;
                //println!("used x: {}; y: {}", x_start_vel, y_start_vel);
            }
        }
    }
    count
}

fn compute_lower_bound(vel: f64, goal: f64) -> i64 {
    ((2f64 * vel + 1f64) / 2f64
        - ((2f64 * vel + 1f64) * (2f64 * vel + 1f64) / 4f64 - 2f64 * goal).sqrt())
    .ceil() as i64
}
fn compute_upper_bound(vel: f64, goal: f64) -> i64 {
    ((2f64 * vel + 1f64) / 2f64
        - ((2f64 * vel + 1f64) * (2f64 * vel + 1f64) / 4f64 - 2f64 * goal).sqrt())
    .floor() as i64
}
