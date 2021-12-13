// execute vim command
// :%s/13/{Number of Day}/g
// to set correct day

pub enum Fold {
    X(u64),
    Y(u64),
}

type Line = Vec<(u64, u64)>;
type Inp = (Line, Vec<Fold>);

#[aoc_generator(day13)]
pub fn generator_day13(input: &str) -> Inp {
    let mut res: Vec<(u64, u64)> = vec![];
    let mut folds: Vec<Fold> = vec![];
    let mut first_part = true;
    for line in input.lines() {
        if line == "" {
            first_part = false;
            continue;
        }
        if first_part {
            let mut iter = line.split(',');
            res.push((
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
            ));
        } else {
            let mut iter = line.split_whitespace();
            iter.next().unwrap();
            iter.next().unwrap();
            let mut fold = iter.next().unwrap().split('=');
            match fold.next().unwrap() {
                "y" => folds.push(Fold::Y(fold.next().unwrap().parse().unwrap())),
                "x" => folds.push(Fold::X(fold.next().unwrap().parse().unwrap())),
                _ => panic!(),
            }
        }
    }
    (res, folds)
}

#[aoc(day13, part1)]
pub fn solve_day13_part1(input: &Inp) -> u64 {
    let points: Vec<(u64, u64)> = input.0.clone();
    let mut new_points: Vec<(u64, u64)> = vec![];

    // fold
    match input.1[0] {
        Fold::X(col) => {
            for (x, y) in points.iter() {
                if *x > col {
                    let new_x: u64 = col - (x - col);
                    if !points.contains(&(new_x, *y)) {
                        new_points.push((new_x, *y));
                    }
                } else {
                    new_points.push((*x, *y));
                }
            }
        }
        Fold::Y(row) => {
            for (x, y) in points.iter() {
                if *y > row {
                    let new_y: u64 = row - (y - row);
                    if !points.contains(&(*x, new_y)) {
                        new_points.push((*x, new_y));
                    }
                } else {
                    new_points.push((*x, *y));
                }
            }
        }
    }
    new_points.len() as u64
}

#[aoc(day13, part2)]
pub fn solve_day13_part2(input: &Inp) -> u64 {
    let mut points: Vec<(u64, u64)> = input.0.clone();
    let mut new_points: Vec<(u64, u64)> = vec![];

    // fold
    for fold in input.1.iter() {
        match fold {
            &Fold::X(col) => {
                for (x, y) in points.iter() {
                    if *x > col {
                        let new_x: u64 = col - (x - col);
                        if !points.contains(&(new_x, *y)) {
                            new_points.push((new_x, *y));
                        }
                    } else {
                        new_points.push((*x, *y));
                    }
                }
            }
            &Fold::Y(row) => {
                for (x, y) in points.iter() {
                    if *y > row {
                        let new_y: u64 = row - (y - row);
                        if !points.contains(&(*x, new_y)) {
                            new_points.push((*x, new_y));
                        }
                    } else {
                        new_points.push((*x, *y));
                    }
                }
            }
        }
        points = new_points;
        new_points = vec![];
    }

    // finding max

    let mut max_y = 0;
    let mut max_x = 0;
    for (x, y) in points.iter() {
        if *x > max_x {
            max_x = *x;
        }
        if *y > max_y {
            max_y = *y;
        }
    }

    // outputting
    for y in 0..max_y + 1 {
        for x in 0..max_x + 1 {
            if points.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    0
}
