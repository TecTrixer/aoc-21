pub struct Line {
    x1: u64,
    y1: u64,
    x2: u64,
    y2: u64,
}
#[aoc_generator(day5)]
pub fn generator_day5(input: &str) -> Vec<Line> {
    let mut res: Vec<Line> = vec![];
    for line in input.lines() {
        let mut coords = line.trim().split("->");
        let mut coord1 = coords.next().unwrap().trim().split(',');
        let mut coord2 = coords.next().unwrap().trim().split(',');
        let mut x1: u64 = coord1.next().unwrap().parse().unwrap();
        let mut y1: u64 = coord1.next().unwrap().parse().unwrap();
        let mut x2: u64 = coord2.next().unwrap().parse().unwrap();
        let mut y2: u64 = coord2.next().unwrap().parse().unwrap();
        if x1 > x2 {
            let temp: u64 = x2;
            let temp2: u64 = y2;
            x2 = x1;
            y2 = y1;
            x1 = temp;
            y1 = temp2;
        }
        res.push(Line { x1, y1, x2, y2 });
    }
    res
}

#[aoc(day5, part1)]
pub fn solve_day5_part1(input: &Vec<Line>) -> u64 {
    // find max x and max y
    let mut max_x: u64 = 0;
    let mut max_y: u64 = 0;
    for line in input {
        if line.x1 > max_x {
            max_x = line.x1;
        }
        if line.x2 > max_x {
            max_x = line.x2;
        }
        if line.y1 > max_y {
            max_y = line.y1;
        }
        if line.y2 > max_y {
            max_y = line.y2;
        }
    }

    // generate array
    let mut field: Vec<Vec<u64>> = vec![];
    for _ in 0..max_y + 1 {
        let mut row: Vec<u64> = vec![];
        for _ in 0..max_x + 1 {
            row.push(0);
        }
        field.push(row);
    }

    // draw lines
    for line in input {
        if line.x1 == line.x2 || line.y1 == line.y2 {
            let mut y_start = line.y1;
            let mut y_end = line.y2;
            if line.y1 > line.y2 {
                y_start = line.y2;
                y_end = line.y1;
            }
            for y in y_start..y_end + 1 {
                for x in line.x1..line.x2 + 1 {
                    field[y as usize][x as usize] += 1;
                }
            }
        }
    }

    // find res
    let mut res: u64 = 0;
    for y in 0..max_y + 1 {
        for x in 0..max_x + 1 {
            if field[y as usize][x as usize] > 1 {
                res += 1;
            }
        }
    }
    res
}

#[aoc(day5, part2)]
pub fn solve_day5_part2(input: &Vec<Line>) -> u64 {
    // find max x and max y
    let mut max_x: u64 = 0;
    let mut max_y: u64 = 0;
    for line in input {
        if line.x1 > max_x {
            max_x = line.x1;
        }
        if line.x2 > max_x {
            max_x = line.x2;
        }
        if line.y1 > max_y {
            max_y = line.y1;
        }
        if line.y2 > max_y {
            max_y = line.y2;
        }
    }

    // generate array
    let mut field: Vec<Vec<u64>> = vec![];
    for _ in 0..max_y + 1 {
        let mut row: Vec<u64> = vec![];
        for _ in 0..max_x + 1 {
            row.push(0);
        }
        field.push(row);
    }

    // draw lines
    for line in input {
        if line.x1 == line.x2 || line.y1 == line.y2 {
            let mut y_start = line.y1;
            let mut y_end = line.y2;
            if line.y1 > line.y2 {
                y_start = line.y2;
                y_end = line.y1;
            }
            for y in y_start..y_end + 1 {
                for x in line.x1..line.x2 + 1 {
                    field[y as usize][x as usize] += 1;
                }
            }
        } else if line.y1 <= line.y2 {
            for x in 0..line.x2 - line.x1 + 1 {
                field[(line.y1 + x) as usize][(line.x1 + x) as usize] += 1;
            }
        } else {
            for x in 0..line.x2 - line.x1 + 1 {
                field[(line.y1 - x) as usize][(line.x1 + x) as usize] += 1;
            }
        }
    }

    // find res
    let mut res: u64 = 0;
    for y in 0..max_y + 1 {
        for x in 0..max_x + 1 {
            if field[y as usize][x as usize] > 1 {
                res += 1;
            }
        }
    }
    res
}
