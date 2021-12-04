type Line = Vec<Vec<u64>>;
type Inp = (Vec<u64>, Vec<Line>);
#[aoc_generator(day4)]
pub fn generator_day4(input: &str) -> Inp {
    let mut nums: Vec<u64> = vec![];
    let mut iter = input.lines();
    for i in iter.next().unwrap().trim().split(',') {
        nums.push(i.parse().unwrap());
    }
    iter.next().unwrap();
    let mut bingo_player: Vec<Vec<u64>> = vec![];
    let mut bingos: Vec<Vec<Vec<u64>>> = vec![];
    for line in iter {
        if line.trim() == "" {
            bingos.push(bingo_player);
            bingo_player = vec![];
        } else {
            let mut row: Vec<u64> = vec![];
            for letter in line.trim().split_whitespace() {
                row.push(letter.parse().unwrap());
            }
            bingo_player.push(row);
        }
    }
    bingos.push(bingo_player);
    (nums, bingos)
}

#[aoc(day4, part1)]
pub fn solve_day4_part1(input: &Inp) -> u64 {
    let mut nums: Vec<u64> = vec![];
    for num in &input.0 {
        nums.push(*num);
        for player in &input.1 {
            if check(&nums, &player) {
                return sum(&nums, &player) * nums[nums.len() - 1];
            }
        }
    }
    return 0;
}

fn sum(nums: &Vec<u64>, player: &Vec<Vec<u64>>) -> u64 {
    let mut res: u64 = 0;
    for x in 0..player.len() {
        for y in 0..player[0].len() {
            if !nums.contains(&player[x as usize][y as usize]) {
                res += player[x as usize][y as usize];
            }
        }
    }
    res
}

fn check(nums: &Vec<u64>, bingo_sheet: &Vec<Vec<u64>>) -> bool {
    for x in 0..bingo_sheet.len() {
        let mut worked: bool = true;
        for y in 0..bingo_sheet[0].len() {
            if !nums.contains(&bingo_sheet[x as usize][y as usize]) {
                worked = false;
                break;
            }
        }
        if worked {
            return true;
        }
    }
    for y in 0..bingo_sheet[0].len() {
        let mut worked: bool = true;
        for x in 0..bingo_sheet.len() {
            if !nums.contains(&bingo_sheet[x as usize][y as usize]) {
                worked = false;
                break;
            }
        }
        if worked {
            return true;
        }
    }
    return false;
}

#[aoc(day4, part2)]
pub fn solve_day4_part2(input: &Inp) -> u64 {
    let mut nums: Vec<u64> = vec![];
    let mut won: Vec<bool> = vec![];
    for _ in &input.1 {
        won.push(false);
    }

    for num in &input.0 {
        nums.push(*num);
        for (i, player) in (&input.1).iter().enumerate() {
            if !won[i] && check(&nums, &player) {
                won[i] = true;
                if !won.contains(&false) {
                    return sum(&nums, &player) * nums[nums.len() - 1];
                }
            }
        }
    }
    return 0;
}
