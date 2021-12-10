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
        let mut c_square: u64 = 0;
        let mut c_arrow: u64 = 0;
        let mut c_bracket: u64 = 0;
        let mut c_curly: u64 = 0;
        let mut stack: Vec<Sign> = vec![];
        for sign in line {
            match sign {
                Sign::LeftSquare => {
                    c_square += 1;
                    stack.push(Sign::LeftSquare);
                }
                Sign::RightSquare => {
                    if c_square == 0 || stack[stack.len() - 1] != Sign::LeftSquare {
                        count += 57;
                        break;
                    } else {
                        stack.pop();
                        c_square -= 1;
                    }
                }
                Sign::LeftArrow => {
                    c_arrow += 1;
                    stack.push(Sign::LeftArrow);
                }
                Sign::RightArrow => {
                    if c_arrow == 0 || stack[stack.len() - 1] != Sign::LeftArrow {
                        count += 25137;
                        break;
                    } else {
                        stack.pop();
                        c_arrow -= 1;
                    }
                }
                Sign::LeftCurly => {
                    c_curly += 1;
                    stack.push(Sign::LeftCurly);
                }
                Sign::RightCurly => {
                    if c_curly == 0 || stack[stack.len() - 1] != Sign::LeftCurly {
                        count += 1197;
                        break;
                    } else {
                        stack.pop();
                        c_curly -= 1;
                    }
                }
                Sign::LeftBracket => {
                    c_bracket += 1;
                    stack.push(Sign::LeftBracket);
                }
                Sign::RightBracket => {
                    if c_bracket == 0 || stack[stack.len() - 1] != Sign::LeftBracket {
                        count += 3;
                        break;
                    } else {
                        stack.pop();
                        c_bracket -= 1;
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
        let mut c_square: u64 = 0;
        let mut c_arrow: u64 = 0;
        let mut c_bracket: u64 = 0;
        let mut c_curly: u64 = 0;
        let mut stack: Vec<Sign> = vec![];
        for sign in line {
            match sign {
                Sign::LeftSquare => {
                    c_square += 1;
                    stack.push(Sign::LeftSquare);
                }
                Sign::RightSquare => {
                    if c_square == 0 || stack[stack.len() - 1] != Sign::LeftSquare {
                        stack.retain(|_| false);
                        break;
                    } else {
                        stack.pop();
                        c_square -= 1;
                    }
                }
                Sign::LeftArrow => {
                    c_arrow += 1;
                    stack.push(Sign::LeftArrow);
                }
                Sign::RightArrow => {
                    if c_arrow == 0 || stack[stack.len() - 1] != Sign::LeftArrow {
                        stack.retain(|_| false);
                        break;
                    } else {
                        stack.pop();
                        c_arrow -= 1;
                    }
                }
                Sign::LeftCurly => {
                    c_curly += 1;
                    stack.push(Sign::LeftCurly);
                }
                Sign::RightCurly => {
                    if c_curly == 0 || stack[stack.len() - 1] != Sign::LeftCurly {
                        stack.retain(|_| false);
                        break;
                    } else {
                        stack.pop();
                        c_curly -= 1;
                    }
                }
                Sign::LeftBracket => {
                    c_bracket += 1;
                    stack.push(Sign::LeftBracket);
                }
                Sign::RightBracket => {
                    if c_bracket == 0 || stack[stack.len() - 1] != Sign::LeftBracket {
                        stack.retain(|_| false);
                        break;
                    } else {
                        stack.pop();
                        c_bracket -= 1;
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
        }
    }
    res.sort();
    let idx = res.len() / 2;
    res[idx as usize]
}
