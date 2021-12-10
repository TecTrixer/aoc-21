// execute vim command
// :%s/10/{Number of Day}/g
// to set correct day

#[derive(Debug, PartialEq)]
pub enum Sign {
    LeftSquare,
    RightSquare,
    LeftArrow,
    RightArrow,
    LeftBracket,
    RightBracket,
    LeftCurly,
    RightCurly,
}
type Line = Vec<Sign>;
type Inp = Vec<Line>;

#[aoc_generator(day10)]
pub fn generator_day10(input: &str) -> Inp {
    let mut res: Vec<Line> = vec![];
    for line in input.lines() {
        let mut a: Line = vec![];
        for c in line.trim().chars() {
            a.push(match c {
                '[' => Sign::LeftSquare,
                ']' => Sign::RightSquare,
                '<' => Sign::LeftArrow,
                '>' => Sign::RightArrow,
                '(' => Sign::LeftBracket,
                ')' => Sign::RightBracket,
                '{' => Sign::LeftCurly,
                '}' => Sign::RightCurly,
                _ => {
                    panic!();
                }
            });
        }
        res.push(a);
    }
    res
}

#[aoc(day10, part1)]
pub fn solve_day10_part1(input: &Inp) -> u64 {
    let mut count: u64 = 0;
    for line in input {
        let mut stack: Vec<Sign> = vec![];
        for sign in line {
            match sign {
                Sign::LeftSquare => {
                    stack.push(Sign::LeftSquare);
                }
                Sign::RightSquare => {
                    if stack[stack.len() - 1] != Sign::LeftSquare {
                        count += 57;
                        break;
                    } else {
                        stack.pop();
                    }
                }
                Sign::LeftArrow => {
                    stack.push(Sign::LeftArrow);
                }
                Sign::RightArrow => {
                    if stack[stack.len() - 1] != Sign::LeftArrow {
                        count += 25137;
                        break;
                    } else {
                        stack.pop();
                    }
                }
                Sign::LeftCurly => {
                    stack.push(Sign::LeftCurly);
                }
                Sign::RightCurly => {
                    if stack[stack.len() - 1] != Sign::LeftCurly {
                        count += 1197;
                        break;
                    } else {
                        stack.pop();
                    }
                }
                Sign::LeftBracket => {
                    stack.push(Sign::LeftBracket);
                }
                Sign::RightBracket => {
                    if stack[stack.len() - 1] != Sign::LeftBracket {
                        count += 3;
                        break;
                    } else {
                        stack.pop();
                    }
                }
            }
        }
    }
    count
}

#[aoc(day10, part2)]
pub fn solve_day10_part2(input: &Inp) -> u64 {
    let mut res: Vec<u64> = vec![];
    for line in input {
        let mut count: u64 = 0;
        let mut stack: Vec<Sign> = vec![];
        for sign in line {
            match sign {
                Sign::LeftSquare => {
                    stack.push(Sign::LeftSquare);
                }
                Sign::RightSquare => {
                    if stack[stack.len() - 1] != Sign::LeftSquare {
                        stack.clear();
                        break;
                    } else {
                        stack.pop();
                    }
                }
                Sign::LeftArrow => {
                    stack.push(Sign::LeftArrow);
                }
                Sign::RightArrow => {
                    if stack[stack.len() - 1] != Sign::LeftArrow {
                        stack.clear();
                        break;
                    } else {
                        stack.pop();
                    }
                }
                Sign::LeftCurly => {
                    stack.push(Sign::LeftCurly);
                }
                Sign::RightCurly => {
                    if stack[stack.len() - 1] != Sign::LeftCurly {
                        stack.clear();
                        break;
                    } else {
                        stack.pop();
                    }
                }
                Sign::LeftBracket => {
                    stack.push(Sign::LeftBracket);
                }
                Sign::RightBracket => {
                    if stack[stack.len() - 1] != Sign::LeftBracket {
                        stack.clear();
                        break;
                    } else {
                        stack.pop();
                    }
                }
            }
        }
        for i in (0..stack.len()).rev() {
            match stack[i] {
                Sign::LeftArrow => {
                    count *= 5;
                    count += 4;
                }
                Sign::LeftBracket => {
                    count *= 5;
                    count += 1;
                }
                Sign::LeftCurly => {
                    count *= 5;
                    count += 3;
                }
                Sign::LeftSquare => {
                    count *= 5;
                    count += 2;
                }
                _ => (),
            }
        }
        if count != 0 {
            res.push(count);
            stack.clear();
        }
    }
    res.sort();
    let idx = res.len() / 2;
    res[idx as usize]
}
