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
            if stopping {
                let mut y: i64 = 0;
                let mut y_vel = y_start_vel;
                let mut i = 0;
                if y_start_vel > 0 {
                    y_vel = -1 * (y_start_vel + 1);
                    i = 2 * y_start_vel;
                }
                while y + 1 > input.y2 {
                    if y < input.y1 + 1 {
                        if x_start_vel * (x_start_vel + 1) / 2
                            - (x_start_vel - i) * (x_start_vel - i + 1) / 2
                            >= input.x1
                        {
                            count += 1;
                        } else {
                            println!("x: {}, i: {}, y: {}", x_start_vel, i, y_start_vel);
                        }
                        break;
                    }
                    y += y_vel;
                    y_vel -= 1;
                    i += 1;
                }
                continue;
            }
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
