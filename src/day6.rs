// execute vim command
// :%s/6/{Number of Day}/g
// to set correct day

type Inp = [u64; 9];

#[aoc_generator(day6)]
pub fn generator_day6(input: &str) -> Inp {
    let mut res: [u64; 9] = [0; 9];
    for line in input.lines() {
        for num in input.trim().split(',') {
            res[num.parse::<usize>().unwrap()] += 1;
        }
    }
    res
}

#[aoc(day6, part1)]
pub fn solve_day6_part1(input: &Inp) -> u64 {
    let mut oldArr: [u64; 9] = *input;
    for _ in 0..80 {
        let mut newArr: [u64; 9] = [0; 9];
        newArr[6] = oldArr[0];
        newArr[8] = oldArr[0];
        for i in 1..9 {
            newArr[i - 1] += oldArr[i];
        }
        oldArr = newArr;
    }
    let mut count: u64 = 0;
    for i in oldArr {
        count += i;
    }
    count
}

#[aoc(day6, part2)]
pub fn solve_day6_part2(input: &Inp) -> u64 {
    let mut oldArr: [u64; 9] = *input;
    for _ in 0..256 {
        let mut newArr: [u64; 9] = [0; 9];
        newArr[6] = oldArr[0];
        newArr[8] = oldArr[0];
        for i in 1..9 {
            newArr[i - 1] += oldArr[i];
        }
        oldArr = newArr;
    }
    let mut count: u64 = 0;
    for i in oldArr {
        count += i;
    }
    count
}
