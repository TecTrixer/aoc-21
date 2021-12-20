// execute vim command
// :%s/19/{Number of Day}/g
// to set correct day
use std::collections::HashMap;
use std::collections::HashSet;

type Line = HashSet<Row>;
type Inp = Vec<Line>;
type Row = [i32; 3];

#[aoc_generator(day19)]
pub fn generator_day19(input: &str) -> Inp {
    let mut res: Vec<Line> = vec![];
    let mut needs_skip: bool = true;
    let mut block: Line = HashSet::with_capacity(26);
    for line in input.lines() {
        if needs_skip {
            if block.len() > 0 {
                res.push(block);
                block = HashSet::with_capacity(26);
            }
            needs_skip = false;
            continue;
        } else if line.trim() == "" {
            needs_skip = true;
            continue;
        }
        let nums = line
            .trim()
            .split(",")
            .filter_map(|x| x.parse::<i32>().ok())
            .collect::<Vec<i32>>();
        block.insert([nums[0], nums[1], nums[2]]);
    }
    res.push(block);
    res
}

#[aoc(day19, part1)]
pub fn solve_day19_part1(input: &Inp) -> u64 {
    let mut known: HashSet<Row> = HashSet::with_capacity(100);
    for x in input[0].iter() {
        known.insert(*x);
    }
    let mut known_index: HashSet<usize> = HashSet::new();
    known_index.insert(0);
    while known_index.len() < input.len() {
        for j in 0..input.len() {
            if known_index.contains(&j) {
                continue;
            }
            match matching(&known, &input[j]) {
                Some((x, _)) => {
                    for row in x {
                        known.insert(row);
                    }
                    known_index.insert(j);
                    break;
                }
                None => {
                    continue;
                }
            }
        }
    }
    known.len() as u64
}

fn matching(a: &HashSet<Row>, b: &HashSet<Row>) -> Option<(HashSet<Row>, Row)> {
    let mut b_rot: HashSet<Row>;
    for i in 0..24 {
        b_rot = rotate(b, i);
        let mut found: bool = false;
        let mut offset: Row = [0, 0, 0];
        let mut map: HashMap<Row, u8> = HashMap::with_capacity(26);
        'outer: for b_row in b_rot.iter() {
            for point in a {
                let x = map
                    .entry(RowVec { row: point } - RowVec { row: b_row })
                    .or_insert(0);
                *x += 1;
                if x >= &mut 12 {
                    offset = RowVec { row: point } - RowVec { row: b_row };
                    found = true;
                    break 'outer;
                }
            }
        }
        if found {
            let mut res: HashSet<Row> = HashSet::with_capacity(26);
            for row in b_rot {
                res.insert(Rowvec { row } + Rowvec { row: offset });
            }
            return Some((res, offset));
        }
    }
    None
}

pub struct RowVec<'a> {
    row: &'a Row,
}
pub struct Rowvec {
    row: Row,
}
impl<'a> std::ops::Sub for RowVec<'a> {
    type Output = Row;
    fn sub(self, other: Self) -> Row {
        [
            self.row[0] - other.row[0],
            self.row[1] - other.row[1],
            self.row[2] - other.row[2],
        ]
    }
}
impl std::ops::Add for Rowvec {
    type Output = Row;
    fn add(self, other: Self) -> Row {
        [
            self.row[0] + other.row[0],
            self.row[1] + other.row[1],
            self.row[2] + other.row[2],
        ]
    }
}

fn rotations() -> [[i8; 3]; 24] {
    [
        [0, 1, 2],
        [0, 2, -2],
        [0, -2, -3],
        [0, -3, 1],
        [1, 0, -3],
        [1, 2, 0],
        [1, -1, 2],
        [1, -3, -1],
        [2, 0, 1],
        [2, 1, -1],
        [2, -1, -2],
        [2, -2, 0],
        [-1, 1, -3],
        [-1, 2, 1],
        [-1, -2, 2],
        [-1, -3, -2],
        [-2, 0, 2],
        [-2, 2, -1],
        [-2, -1, -3],
        [-2, -3, 0],
        [-3, 0, -2],
        [-3, 1, 0],
        [-3, -1, 1],
        [-3, -2, -1],
    ]
}

fn rotate(input: &HashSet<[i32; 3]>, idx: usize) -> HashSet<[i32; 3]> {
    let rotation = rotations()[idx];
    let mut res: HashSet<[i32; 3]> = HashSet::with_capacity(26);
    for row in input {
        let mut new_row: [i32; 3] = [0, 0, 0];
        if rotation[0] < 0 {
            new_row[0] = -1 * row[(-1 * (rotation[0] + 1)) as usize];
        } else {
            new_row[0] = row[rotation[0] as usize];
        }
        if rotation[1] < 0 {
            new_row[1] = -1 * row[(-1 * (rotation[1] + 1)) as usize];
        } else {
            new_row[1] = row[rotation[1] as usize];
        }
        if rotation[2] < 0 {
            new_row[2] = -1 * row[(-1 * (rotation[2] + 1)) as usize];
        } else {
            new_row[2] = row[rotation[2] as usize];
        }
        res.insert(new_row);
    }
    res
}

fn manhatten_dist(a: &Row, b: &Row) -> u64 {
    let mut x = a[0] - b[0];
    let mut y = a[1] - b[1];
    let mut z = a[2] - b[2];
    if x < 0 {
        x *= -1;
    }
    if y < 0 {
        y *= -1;
    }
    if z < 0 {
        z *= -1;
    }
    return (x + y + z) as u64;
}

#[aoc(day19, part2)]
pub fn solve_day19_part2(input: &Inp) -> u64 {
    let mut known: HashSet<Row> = HashSet::with_capacity(100);
    let mut scanners: HashSet<Row> = HashSet::with_capacity(100);
    for x in input[0].iter() {
        known.insert(*x);
    }
    let mut known_index: HashSet<usize> = HashSet::new();
    known_index.insert(0);
    while known_index.len() < input.len() {
        for j in 0..input.len() {
            if known_index.contains(&j) {
                continue;
            }
            match matching(&known, &input[j]) {
                Some((x, offset)) => {
                    for row in x {
                        known.insert(row);
                    }
                    known_index.insert(j);
                    scanners.insert(offset);
                    break;
                }
                None => {
                    continue;
                }
            }
        }
    }
    let mut dist = 0;
    for a in scanners.iter() {
        for b in scanners.iter() {
            let diff = manhatten_dist(a, b);
            if diff > dist {
                dist = diff;
            }
        }
    }
    dist
}
