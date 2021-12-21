// execute vim command
// :%s/21/{Number of Day}/g
// to set correct day
use cached::proc_macro::cached;

type Inp = [u64; 2];

#[aoc_generator(day21)]
pub fn generator_day21(input: &str) -> Inp {
    let mut res: Vec<u64> = vec![];
    for line in input.lines() {
        let mut iter = line.trim().split_whitespace();
        iter.next().unwrap();
        iter.next().unwrap();
        iter.next().unwrap();
        iter.next().unwrap();
        res.push(iter.next().unwrap().parse().unwrap());
    }
    [res[0], res[1]]
}

#[aoc(day21, part1)]
pub fn solve_day21_part1(input: &Inp) -> u64 {
    let mut a_score = 0;
    let mut a_pos = input[0];
    let mut b_score = 0;
    let mut b_pos = input[1];
    let mut dice_score = 1;
    let mut roll: (u64, u64);
    let mut roll_count = 0;
    while a_score < 1000 && b_score < 1000 {
        roll = roll_dice(dice_score);
        roll_count += 3;
        dice_score = roll.1;
        a_pos = mod10(a_pos + roll.0);
        a_score += a_pos;
        if a_score >= 1000 {
            break;
        }
        roll = roll_dice(dice_score);
        dice_score = roll.1;
        roll_count += 3;
        b_pos = mod10(b_pos + roll.0);
        b_score += b_pos;
    }
    if a_score >= 1000 {
        return b_score * roll_count;
    } else {
        return a_score * roll_count;
    }
}

fn roll_dice(start: u64) -> (u64, u64) {
    let first = mod100(start);
    let sec = mod100(start + 1);
    let third = mod100(start + 2);
    return (first + sec + third, mod100(start + 3));
}
fn mod10(x: u64) -> u64 {
    if x > 10 {
        let y = x % 10;
        if y == 0 {
            return 10;
        } else {
            return y;
        }
    } else {
        return x;
    }
}
fn mod100(x: u64) -> u64 {
    if x > 100 {
        let y = x % 100;
        if y == 0 {
            return 100;
        } else {
            return y;
        }
    } else {
        return x;
    }
}

#[aoc(day21, part2)]
pub fn solve_day21_part2(input: &Inp) -> u64 {
    let res = wins(0, input[0], 0, input[1], true, 3);
    println!("a: {}, b: {}", res.0, res.1);
    if res.0 > res.1 {
        return res.0;
    } else {
        return res.1;
    }
}

#[cached]
fn wins(
    a_score: u64,
    a_pos: u64,
    b_score: u64,
    b_pos: u64,
    as_turn: bool,
    turns_left: u64,
) -> (u64, u64) {
    if as_turn {
        if turns_left == 1 {
            let a_score1 = a_score + mod10(a_pos + 1);
            let a_score2 = a_score + mod10(a_pos + 2);
            let a_score3 = a_score + mod10(a_pos + 3);
            let mut wins1 = 0;
            let mut wins1b = 0;
            let mut wins2 = 0;
            let mut wins2b = 0;
            let mut wins3 = 0;
            let mut wins3b = 0;
            if a_score1 >= 21 {
                wins1 = 1;
            };
            if a_score2 >= 21 {
                wins2 = 1;
            };
            if a_score3 >= 21 {
                wins3 = 1;
            };
            if wins1 == 0 {
                let first = wins(a_score1, mod10(a_pos + 1), b_score, b_pos, false, 3);
                wins1 = first.0;
                wins1b = first.1;
            }
            if wins2 == 0 {
                let sec = wins(a_score2, mod10(a_pos + 2), b_score, b_pos, false, 3);
                wins2 = sec.0;
                wins2b = sec.1;
            }
            if wins3 == 0 {
                let third = wins(a_score3, mod10(a_pos + 3), b_score, b_pos, false, 3);
                wins3 = third.0;
                wins3b = third.1;
            }
            return (wins1 + wins2 + wins3, wins1b + wins2b + wins3b);
        } else {
            let first = wins(
                a_score,
                mod10(a_pos + 1),
                b_score,
                b_pos,
                as_turn,
                turns_left - 1,
            );
            let sec = wins(
                a_score,
                mod10(a_pos + 2),
                b_score,
                b_pos,
                as_turn,
                turns_left - 1,
            );
            let third = wins(
                a_score,
                mod10(a_pos + 3),
                b_score,
                b_pos,
                as_turn,
                turns_left - 1,
            );
            return (first.0 + sec.0 + third.0, first.1 + sec.1 + third.1);
        }
    } else {
        if turns_left == 1 {
            let b_score1 = b_score + mod10(b_pos + 1);
            let b_score2 = b_score + mod10(b_pos + 2);
            let b_score3 = b_score + mod10(b_pos + 3);
            let mut wins1 = 0;
            let mut wins1b = 0;
            let mut wins2 = 0;
            let mut wins2b = 0;
            let mut wins3 = 0;
            let mut wins3b = 0;
            if b_score1 >= 21 {
                wins1b = 1;
            };
            if b_score2 >= 21 {
                wins2b = 1;
            };
            if b_score3 >= 21 {
                wins3b = 1;
            };
            if wins1b == 0 {
                let first = wins(a_score, a_pos, b_score1, mod10(b_pos + 1), true, 3);
                wins1 = first.0;
                wins1b = first.1;
            }
            if wins2b == 0 {
                let sec = wins(a_score, a_pos, b_score2, mod10(b_pos + 2), true, 3);
                wins2 = sec.0;
                wins2b = sec.1;
            }
            if wins3b == 0 {
                let third = wins(a_score, a_pos, b_score3, mod10(b_pos + 3), true, 3);
                wins3 = third.0;
                wins3b = third.1;
            }
            return (wins1 + wins2 + wins3, wins1b + wins2b + wins3b);
        } else {
            let first = wins(
                a_score,
                a_pos,
                b_score,
                mod10(b_pos + 1),
                as_turn,
                turns_left - 1,
            );
            let sec = wins(
                a_score,
                a_pos,
                b_score,
                mod10(b_pos + 2),
                as_turn,
                turns_left - 1,
            );
            let third = wins(
                a_score,
                a_pos,
                b_score,
                mod10(b_pos + 3),
                as_turn,
                turns_left - 1,
            );
            return (first.0 + sec.0 + third.0, first.1 + sec.1 + third.1);
        }
    }
}
