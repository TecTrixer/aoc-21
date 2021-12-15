use priority_queue::PriorityQueue;
use std::collections::HashSet;
// execute vim command
// :%s/15/{Number of Day}/g
// to set correct day

type Line = Vec<u64>;
type Inp = Vec<Line>;

#[aoc_generator(day15)]
pub fn generator_day15(input: &str) -> Inp {
    let mut res: Vec<Line> = vec![];
    for line in input.lines() {
        let mut x: Line = vec![];
        for c in line.trim().chars() {
            x.push((c as u8 - '0' as u8) as u64);
        }
        res.push(x);
    }
    res
}

#[aoc(day15, part1)]
pub fn solve_day15_part1(input: &Inp) -> u64 {
    let mut q: HashSet<(u8, u8)> = HashSet::with_capacity(256);
    let mut dist: Vec<Vec<u64>> = vec![];
    for row in 0..input.len() {
        let mut new_row: Vec<u64> = vec![];
        for col in 0..input[0].len() {
            new_row.push(u64::MAX);
            q.insert((row as u8, col as u8));
        }
        dist.push(new_row);
    }
    dist[0][0] = input[0][0];
    while q.len() != 0 {
        let mut u = (0, 0);
        let mut dist_u = u64::MAX;
        for row in 0..input.len() {
            for col in 0..input.len() {
                if q.contains(&(row as u8, col as u8)) && dist[row][col] <= dist_u {
                    dist_u = input[row][col];
                    u = (row, col);
                }
            }
        }
        if u.0 == input.len() - 1 && u.1 == input[0].len() - 1 {
            break;
        }
        q.remove(&(u.0 as u8, u.1 as u8));
        if q.contains(&((u.0 + 1) as u8, u.1 as u8)) {
            let alt = dist[u.0][u.1] + input[u.0 + 1][u.1];
            if alt < dist[u.0 + 1][u.1] {
                dist[u.0 + 1][u.1] = alt;
            }
        }
        if q.contains(&(u.0 as u8, (u.1 + 1) as u8)) {
            let alt = dist[u.0][u.1] + input[u.0][u.1 + 1];
            if alt < dist[u.0][u.1 + 1] {
                dist[u.0][u.1 + 1] = alt;
            }
        }
    }
    dist[input.len() - 1][input[0].len() - 1] - input[input.len() - 1][input[0].len() - 1]
}

fn print(input: &Vec<Vec<u64>>) {
    for row in input {
        print!("[");
        for elem in row {
            print!("{:?}, ", elem);
        }
        println!("]");
    }
}

#[aoc(day15, part2)]
pub fn solve_day15_part2(input: &Inp) -> u64 {
    let mut q: PriorityQueue<(u8, u8), i64> = PriorityQueue::new();
    let mut done: HashSet<(u8, u8)> = HashSet::with_capacity(256);
    let mut dist: Vec<Vec<u64>> = vec![];
    for row in 0..input.len() {
        let mut new_row: Vec<u64> = vec![];
        for col in 0..input[0].len() {
            new_row.push(u64::MAX);
            q.push((row as u8, col as u8), i64::MIN);
        }
        dist.push(new_row);
    }
    dist[0][0] = input[0][0];
    q.push_decrease((0, 0), -(input[0][0] as i64));
    while !q.is_empty() {
        println!("{:?}", q);
        println!("---------------------------------------------");
        let u = q.pop().unwrap();
        done.insert(u.0);

        if u.0 .0 as usize == input.len() - 1 && u.0 .1 as usize == input[0].len() - 1 {
            ()
        }
        if done.contains(&((u.0 .0 + 1) as u8, u.0 .1 as u8)) {
            let alt = dist[u.0 .0 as usize][u.0 .1 as usize]
                + input[u.0 .0 as usize + 1][u.0 .1 as usize];
            if alt < dist[u.0 .0 as usize + 1][u.0 .1 as usize] {
                dist[u.0 .0 as usize + 1][u.0 .1 as usize] = alt;
                q.change_priority(&(u.0 .0 + 1, u.0 .1), -(alt as i64));
            }
        }
        if done.contains(&(u.0 .0 as u8, (u.0 .1 + 1) as u8)) {
            let alt = dist[u.0 .0 as usize][u.0 .1 as usize]
                + input[u.0 .0 as usize][u.0 .1 as usize + 1];
            if alt < dist[u.0 .0 as usize][u.0 .1 as usize + 1] {
                dist[u.0 .0 as usize][u.0 .1 as usize + 1] = alt;
                q.change_priority(&(u.0 .0, u.0 .1 + 1), -(alt as i64));
            }
        }
    }
    dist[input.len() - 1][input[0].len() - 1] - input[input.len() - 1][input[0].len() - 1]
}
