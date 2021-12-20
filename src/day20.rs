// execute vim command
// :%s/20/{Number of Day}/g
// to set correct day

type Inp = (Vec<bool>, Vec<Vec<bool>>);

#[aoc_generator(day20)]
pub fn generator_day20(input: &str) -> Inp {
    let mut inst: Vec<bool> = vec![];
    let mut first_part = true;
    let mut field: Vec<Vec<bool>> = vec![];
    for line in input.lines() {
        if line.trim() == "" {
            first_part = false;
            continue;
        }
        if first_part {
            inst.append(
                &mut line
                    .trim()
                    .chars()
                    .filter_map(|x| match x {
                        '.' => Some(false),
                        _ => Some(true),
                    })
                    .collect::<Vec<bool>>(),
            );
        } else {
            field.push(
                line.trim()
                    .chars()
                    .filter_map(|x| match x {
                        '.' => Some(false),
                        _ => Some(true),
                    })
                    .collect::<Vec<bool>>(),
            );
        }
    }
    (inst, field)
}

#[aoc(day20, part1)]
pub fn solve_day20_part1(input: &Inp) -> u64 {
    let field = input.1.clone();
    let inst = &input.0;
    let mut iter: Vec<Vec<bool>> = sim_step(inst, field, 0);
    for i in 1..2 {
        iter = sim_step(inst, iter, i);
    }
    let mut count: u64 = 0;
    for row in iter {
        for elem in row {
            if elem {
                count += 1;
            }
        }
    }
    count
}

fn sim_step(inst: &Vec<bool>, mut field: Vec<Vec<bool>>, step: usize) -> Vec<Vec<bool>> {
    let border: bool;
    if step % 2 == 0 {
        border = false;
    } else {
        border = inst[0];
    }
    let mut bigger_field: Vec<Vec<bool>> = vec![];
    let row_size = field[0].len();
    let col_size = field.len();
    bigger_field.push([border].repeat(field[0].len() + 4));
    bigger_field.push([border].repeat(field[0].len() + 4));
    for row in field.iter_mut() {
        row.insert(0, border);
        row.insert(0, border);
        row.push(border);
        row.push(border);
        bigger_field.push(row.clone());
    }
    bigger_field.push([border].repeat(field[0].len() + 4));
    bigger_field.push([border].repeat(field[0].len() + 4));

    let mut res: Vec<Vec<bool>> = vec![];
    for i in 0..row_size + 2 {
        let mut row: Vec<bool> = vec![];
        for j in 0..col_size + 2 {
            let mut idx = 0;
            if bigger_field[i][j] {
                idx += 256;
            }
            if bigger_field[i][j + 1] {
                idx += 128;
            }
            if bigger_field[i][j + 2] {
                idx += 64;
            }
            if bigger_field[i + 1][j] {
                idx += 32;
            }
            if bigger_field[i + 1][j + 1] {
                idx += 16;
            }
            if bigger_field[i + 1][j + 2] {
                idx += 8;
            }
            if bigger_field[i + 2][j] {
                idx += 4;
            }
            if bigger_field[i + 2][j + 1] {
                idx += 2;
            }
            if bigger_field[i + 2][j + 2] {
                idx += 1;
            }
            row.push(inst[idx]);
        }
        res.push(row);
    }
    res
}

#[aoc(day20, part2)]
pub fn solve_day20_part2(input: &Inp) -> u64 {
    let field = input.1.clone();
    let inst = &input.0;
    let mut iter: Vec<Vec<bool>> = sim_step(inst, field, 0);
    for i in 1..50 {
        iter = sim_step(inst, iter, i);
    }
    let mut count: u64 = 0;
    for row in iter {
        for elem in row {
            if elem {
                count += 1;
            }
        }
    }
    count
}
