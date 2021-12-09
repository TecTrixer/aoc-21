// execute vim command
// :%s/9/{Number of Day}/g
// to set correct day

type Line = Vec<u64>;
type Inp = Vec<Line>;

#[aoc_generator(day9)]
pub fn generator_day9(input: &str) -> Inp {
    let mut res: Vec<Line> = vec![];
    for line in input.lines() {
        let mut line_parsed: Line = vec![];
        for c in line.trim().chars() {
            line_parsed.push((c as u8 - '0' as u8) as u64);
        }
        res.push(line_parsed);
    }
    res
}

#[aoc(day9, part1)]
pub fn solve_day9_part1(input: &Inp) -> u64 {
    let mut counter: u64 = 0;
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            let elem = input[y as usize][x as usize];
            let mut lower = true;
            'outer: for y_diff in -1 as isize..=1 as isize {
                for x_diff in -1 as isize..=1 as isize {
                    if (y_diff != 0 || x_diff != 0) && (y_diff == 0 || x_diff == 0) {
                        if y_diff + (y as isize) < input.len() as isize
                            && y_diff + (y as isize) >= 0
                            && x_diff + (x as isize) < input[0].len() as isize
                            && x_diff + (x as isize) >= 0
                        {
                            if elem
                                >= input[(y as isize + y_diff) as usize]
                                    [(x as isize + x_diff) as usize]
                            {
                                lower = false;
                                break 'outer;
                            }
                        }
                    }
                }
            }
            if lower {
                counter += 1 + elem;
            }
        }
    }
    counter
}

#[aoc(day9, part2)]
pub fn solve_day9_part2(input: &Inp) -> u64 {
    let mut basins: Vec<u64> = vec![];
    let mut field: Vec<Vec<(u64, bool)>> = vec![];
    for y in 0..input.len() {
        let mut row: Vec<(u64, bool)> = vec![];
        for x in 0..input[0].len() {
            row.push((input[y][x], false));
        }
        field.push(row);
    }
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            let elem = input[y as usize][x as usize];
            let mut lower = true;
            'outer: for y_diff in -1 as isize..=1 as isize {
                for x_diff in -1 as isize..=1 as isize {
                    if (y_diff != 0 || x_diff != 0) && (y_diff == 0 || x_diff == 0) {
                        if y_diff + (y as isize) < input.len() as isize
                            && y_diff + (y as isize) >= 0
                            && x_diff + (x as isize) < input[0].len() as isize
                            && x_diff + (x as isize) >= 0
                        {
                            if elem
                                >= input[(y as isize + y_diff) as usize]
                                    [(x as isize + x_diff) as usize]
                            {
                                lower = false;
                                break 'outer;
                            }
                        }
                    }
                }
            }
            if lower {
                field[y][x].1 = true;
                basins.push(rec_basins(&mut field, x, y));
            }
        }
    }
    basins.sort();
    let mut iter = basins.iter().rev();
    iter.next().unwrap() * iter.next().unwrap() * iter.next().unwrap()
}

fn rec_basins(input: &mut Vec<Vec<(u64, bool)>>, x: usize, y: usize) -> u64 {
    let mut count: u64 = 0;
    let elem: u64 = input[y][x].0;
    if elem == 9 {
        return 0;
    };
    for y_diff in -1 as isize..=1 as isize {
        for x_diff in -1 as isize..=1 as isize {
            if (y_diff != 0 || x_diff != 0) && (y_diff == 0 || x_diff == 0) {
                if y_diff + (y as isize) < input.len() as isize
                    && y_diff + (y as isize) >= 0
                    && x_diff + (x as isize) < input[0].len() as isize
                    && x_diff + (x as isize) >= 0
                    && !input[(y as isize + y_diff) as usize][(x as isize + x_diff) as usize].1
                {
                    if elem
                        < input[(y as isize + y_diff) as usize][(x as isize + x_diff) as usize].0
                    {
                        input[(y as isize + y_diff) as usize][(x as isize + x_diff) as usize].1 =
                            true;
                        count += rec_basins(
                            input,
                            (x as isize + x_diff) as usize,
                            (y as isize + y_diff) as usize,
                        );
                    }
                }
            }
        }
    }
    return count + 1;
}
