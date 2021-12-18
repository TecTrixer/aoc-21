// execute vim command
// :%s/18/{Number of Day}/g
// to set correct day

type Line = Vec<Elem>;
type Inp = Vec<Line>;

#[aoc_generator(day18)]
pub fn generator_day18(input: &str) -> Inp {
    let mut res: Vec<Line> = vec![];
    for line in input.lines() {
        let mut temp: Vec<Elem> = vec![];
        for c in line.trim().chars() {
            if c == '[' || c == ']' || c == ',' {
                temp.push(Elem::C(c));
            } else {
                temp.push(Elem::Num((c as u8 - '0' as u8) as u32));
            }
        }
        res.push(temp);
    }
    res
}

#[derive(Clone, PartialEq, Debug)]
pub enum Elem {
    C(char),
    Num(u32),
}

#[aoc(day18, part1)]
pub fn solve_day18_part1(input: &Inp) -> u64 {
    let mut res: Vec<Elem> = input[0].clone();
    for i in 1..input.len() {
        res = add(&mut res, &mut input[i].clone());
    }
    magnitute(&res[..])
}

fn add(a: &mut Vec<Elem>, b: &mut Vec<Elem>) -> Vec<Elem> {
    let mut new: Vec<Elem> = vec![];
    new.push(Elem::C('['));
    new.append(a);
    new.push(Elem::C(','));
    new.append(b);
    new.push(Elem::C(']'));
    reduce(&mut new);
    new
}

fn reduce(input: &mut Vec<Elem>) {
    // Find depth bigger 4
    let mut diff: usize = 0;
    let mut j: usize = 0;
    for i in 0..input.len() {
        let elem = &input[i];
        if elem == &Elem::C('[') {
            diff += 1;
        } else if elem == &Elem::C(']') {
            diff -= 1;
            if diff > 3 {
                j = i;
                break;
            }
        }
    }
    // if we need to explode:
    if j != 0 {
        // Find possibly existing values
        let left: u32 = match input[j - 3] {
            Elem::Num(x) => x,
            _ => 0,
        };
        let right: u32 = match input[j - 1] {
            Elem::Num(x) => x,
            _ => 0,
        };
        let mut left_idx = j;
        let mut right_idx = j - 4;
        for i in (0..j - 4).rev() {
            let elem = &input[i];
            if let &Elem::Num(_) = elem {
                left_idx = i;
                break;
            }
        }
        for i in j + 1..input.len() {
            let elem = &input[i];
            if let &Elem::Num(_) = elem {
                right_idx = i - 4;
                break;
            }
        }
        // replace exploded pair with 0
        input.splice(j - 4..=j, [Elem::Num(0)]);

        if left_idx != j {
            // left value needs split
            let left_neighbour: u32 = match input[left_idx] {
                Elem::Num(x) => x,
                _ => panic!(),
            };
            let new_val: u32 = left_neighbour + left;
            input[left_idx] = Elem::Num(new_val);
        }
        if right_idx != j - 4 {
            // left value needs split
            let right_neighbour: u32 = match input[right_idx] {
                Elem::Num(x) => x,
                _ => panic!(),
            };
            let new_val: u32 = right_neighbour + right;
            input[right_idx] = Elem::Num(new_val);
        }
        reduce(input);
    } else {
        // find to be values for splitting
        j = 0;
        for i in 0..input.len() {
            let elem = &input[i];
            if let &Elem::Num(x) = elem {
                if x > 9 {
                    j = i;
                    break;
                }
            }
        }
        if j != 0 {
            let old = match input[j] {
                Elem::Num(x) => x,
                _ => panic!(),
            };
            let new_left = old / 2;
            let new_right = old - new_left;
            input.splice(
                j..=j,
                [
                    Elem::C('['),
                    Elem::Num(new_left),
                    Elem::C(','),
                    Elem::Num(new_right),
                    Elem::C(']'),
                ],
            );
            reduce(input);
        }
    }
}

fn magnitute(input: &[Elem]) -> u64 {
    if input.len() < 2 {
        let val = match input[0] {
            Elem::Num(x) => x,
            _ => panic!(),
        };
        return val as u64;
    }
    // find comma
    let mut diff: u64 = 0;
    let mut j = 0;
    for i in 0..input.len() {
        let elem = &input[i];
        if elem == &Elem::C('[') {
            diff += 1;
        } else if elem == &Elem::C(']') {
            diff -= 1;
        } else if elem == &Elem::C(',') && diff < 2 {
            j = i;
            break;
        }
    }
    let left: u64 = magnitute(&input[1..j]);
    let right: u64 = magnitute(&input[j + 1..input.len() - 1]);
    return left * 3 + 2 * right;
}

#[aoc(day18, part2)]
pub fn solve_day18_part2(input: &Inp) -> u64 {
    let mut max = 0;
    for i in 0..input.len() {
        for j in 0..input.len() {
            let a = add(&mut input[i].clone(), &mut input[j].clone());
            let res = magnitute(&a[..]);
            if res > max {
                max = res;
            }
        }
    }
    max
}

fn _print(input: &Vec<Elem>) {
    let new = input.clone();
    let string: String = new
        .iter()
        .map(|x| match x {
            Elem::Num(y) => format!("{}", y),
            Elem::C(c) => format!("{}", c),
        })
        .collect();
    println!("{}", string);
}
