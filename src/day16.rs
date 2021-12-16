// execute vim command
// :%s/16/{Number of Day}/g
// to set correct day

use std::u64;
type Line = String;
type Inp = Vec<Line>;

#[aoc_generator(day16)]
pub fn generator_day16(input: &str) -> Inp {
    let mut res: Vec<String> = vec![];
    for line in input.lines() {
        let hex: String = line.trim().to_string();
        res.push(convert_to_binary_from_hex(&hex));
    }
    res
}

#[aoc(day16, part1)]
pub fn solve_day16_part1(input: &Inp) -> u64 {
    return parse_version(&input[0], &mut 0, input[0].len(), false);
}

#[aoc(day16, part2)]
pub fn solve_day16_part2(input: &Inp) -> u64 {
    return parse_version(&input[0], &mut 0, input[0].len(), true);
}

fn parse_version(packet: &str, idx: &mut usize, len: usize, part2: bool) -> u64 {
    //println!("parse_version, idx: {}", idx);
    let version = u64::from_str_radix(&packet[*idx..*idx + 3], 2).unwrap();
    *idx += 3;
    if part2 {
        return parse_id(packet, idx, len, part2);
    } else {
        return version + parse_id(packet, idx, len, part2);
    }
}

fn parse_id(packet: &str, idx: &mut usize, len: usize, part2: bool) -> u64 {
    //println!("parse_id, idx: {}", idx);
    let id = u64::from_str_radix(&packet[*idx..*idx + 3], 2).unwrap();
    *idx += 3;
    if part2 {
        match id {
            0 => parse_sum(packet, idx, len),
            1 => parse_product(packet, idx, len),
            2 => parse_min(packet, idx, len),
            3 => parse_max(packet, idx, len),
            4 => parse_literal(packet, idx, part2),
            5 => parse_greater(packet, idx, len),
            6 => parse_smaller(packet, idx, len),
            7 => parse_equal(packet, idx, len),
            _ => parse_operator(packet, idx, len, part2),
        }
    } else {
        match id {
            4 => parse_literal(packet, idx, part2),
            _ => parse_operator(packet, idx, len, part2),
        }
    }
}

fn parse_literal(packet: &str, idx: &mut usize, part2: bool) -> u64 {
    //println!("parse_literal, idx: {}", idx);
    let mut value: u64 = 0;
    while usize::from_str_radix(&packet[*idx..*idx + 1], 2).unwrap() == 1 {
        *idx += 1;
        let bits = u64::from_str_radix(&packet[*idx..*idx + 4], 2).unwrap();
        value *= 16;
        value += bits;
        *idx += 4;
    }
    *idx += 1;
    let bits = u64::from_str_radix(&packet[*idx..*idx + 4], 2).unwrap();
    value *= 16;
    value += bits;
    *idx += 4;
    if part2 {
        return value;
    } else {
        return 0;
    }
}

fn parse_operator(packet: &str, idx: &mut usize, len: usize, part2: bool) -> u64 {
    //println!("parse_operator, idx: {}", idx);
    let length_id = u64::from_str_radix(&packet[*idx..*idx + 1], 2).unwrap();
    *idx += 1;
    //println!("length_id: {}", length_id);
    match length_id {
        0 => {
            let length = usize::from_str_radix(&packet[*idx..*idx + 15], 2).unwrap();
            *idx += 15;
            let mut sum: u64 = 0;
            let stop = length + *idx;
            while *idx < stop {
                sum += parse_version(packet, idx, len, part2);
            }
            sum
        }
        1 => {
            let num = usize::from_str_radix(&packet[*idx..*idx + 11], 2).unwrap();
            *idx += 11;
            let mut sum: u64 = 0;
            for _ in 0..num {
                sum += parse_version(packet, idx, len, part2);
            }
            sum
        }
        _ => panic!("ERRORORO"),
    }
}

fn parse_sum(packet: &str, idx: &mut usize, len: usize) -> u64 {
    //println!("parse_sum, idx: {}", idx);
    let length_id = u64::from_str_radix(&packet[*idx..*idx + 1], 2).unwrap();
    *idx += 1;
    //println!("length_id: {}", length_id);
    match length_id {
        0 => {
            let length = usize::from_str_radix(&packet[*idx..*idx + 15], 2).unwrap();
            *idx += 15;
            let mut sum: u64 = 0;
            let stop = length + *idx;
            while *idx < stop {
                sum += parse_version(packet, idx, len, true);
            }
            sum
        }
        1 => {
            let num = usize::from_str_radix(&packet[*idx..*idx + 11], 2).unwrap();
            *idx += 11;
            let mut sum: u64 = 0;
            for _ in 0..num {
                sum += parse_version(packet, idx, len, true);
            }
            sum
        }
        _ => panic!("ERRORORO"),
    }
}

fn parse_product(packet: &str, idx: &mut usize, len: usize) -> u64 {
    //println!("parse_product, idx: {}", idx);
    let length_id = u64::from_str_radix(&packet[*idx..*idx + 1], 2).unwrap();
    *idx += 1;
    //println!("length_id: {}", length_id);
    match length_id {
        0 => {
            let length = usize::from_str_radix(&packet[*idx..*idx + 15], 2).unwrap();
            *idx += 15;
            let stop = length + *idx;
            let mut product: u64 = parse_version(packet, idx, len, true);
            while *idx < stop {
                product *= parse_version(packet, idx, len, true);
            }
            product
        }
        1 => {
            let num = usize::from_str_radix(&packet[*idx..*idx + 11], 2).unwrap();
            *idx += 11;
            let mut product: u64 = parse_version(packet, idx, len, true);
            for _ in 1..num {
                product *= parse_version(packet, idx, len, true);
            }
            product
        }
        _ => panic!("ERRORORO"),
    }
}

fn parse_min(packet: &str, idx: &mut usize, len: usize) -> u64 {
    //println!("parse_min, idx: {}", idx);
    let length_id = u64::from_str_radix(&packet[*idx..*idx + 1], 2).unwrap();
    *idx += 1;
    //println!("length_id: {}", length_id);
    match length_id {
        0 => {
            let length = usize::from_str_radix(&packet[*idx..*idx + 15], 2).unwrap();
            *idx += 15;
            let stop = length + *idx;
            let mut min: u64 = parse_version(packet, idx, len, true);
            while *idx < stop {
                let val = parse_version(packet, idx, len, true);
                if val < min {
                    min = val;
                }
            }
            min
        }
        1 => {
            let num = usize::from_str_radix(&packet[*idx..*idx + 11], 2).unwrap();
            *idx += 11;
            let mut min: u64 = parse_version(packet, idx, len, true);
            for _ in 1..num {
                let val = parse_version(packet, idx, len, true);
                if val < min {
                    min = val;
                }
            }
            min
        }
        _ => panic!("ERRORORO"),
    }
}

fn parse_max(packet: &str, idx: &mut usize, len: usize) -> u64 {
    //println!("parse_max, idx: {}", idx);
    let length_id = u64::from_str_radix(&packet[*idx..*idx + 1], 2).unwrap();
    *idx += 1;
    //println!("length_id: {}", length_id);
    match length_id {
        0 => {
            let length = usize::from_str_radix(&packet[*idx..*idx + 15], 2).unwrap();
            *idx += 15;
            let stop = length + *idx;
            let mut max: u64 = parse_version(packet, idx, len, true);
            while *idx < stop {
                let val = parse_version(packet, idx, len, true);
                if val > max {
                    max = val;
                }
            }
            max
        }
        1 => {
            let num = usize::from_str_radix(&packet[*idx..*idx + 11], 2).unwrap();
            *idx += 11;
            let mut max: u64 = parse_version(packet, idx, len, true);
            for _ in 1..num {
                let val = parse_version(packet, idx, len, true);
                if val > max {
                    max = val;
                }
            }
            max
        }
        _ => panic!("ERRORORO"),
    }
}

fn parse_greater(packet: &str, idx: &mut usize, len: usize) -> u64 {
    //println!("parse_greater, idx: {}", idx);
    let length_id = u64::from_str_radix(&packet[*idx..*idx + 1], 2).unwrap();
    *idx += 1;
    //println!("length_id: {}", length_id);
    match length_id {
        0 => {
            *idx += 15;
            let val1: u64 = parse_version(packet, idx, len, true);
            let val2: u64 = parse_version(packet, idx, len, true);
            if val1 > val2 {
                return 1;
            } else {
                return 0;
            }
        }
        1 => {
            *idx += 11;
            let val1: u64 = parse_version(packet, idx, len, true);
            let val2: u64 = parse_version(packet, idx, len, true);
            if val1 > val2 {
                return 1;
            } else {
                return 0;
            }
        }
        _ => panic!("ERRORORO"),
    }
}

fn parse_smaller(packet: &str, idx: &mut usize, len: usize) -> u64 {
    //println!("parse_smaller, idx: {}", idx);
    let length_id = u64::from_str_radix(&packet[*idx..*idx + 1], 2).unwrap();
    *idx += 1;
    //println!("length_id: {}", length_id);
    match length_id {
        0 => {
            *idx += 15;
            let val1: u64 = parse_version(packet, idx, len, true);
            let val2: u64 = parse_version(packet, idx, len, true);
            if val1 < val2 {
                return 1;
            } else {
                return 0;
            }
        }
        1 => {
            *idx += 11;
            let val1: u64 = parse_version(packet, idx, len, true);
            let val2: u64 = parse_version(packet, idx, len, true);
            if val1 < val2 {
                return 1;
            } else {
                return 0;
            }
        }
        _ => panic!("ERRORORO"),
    }
}

fn parse_equal(packet: &str, idx: &mut usize, len: usize) -> u64 {
    //println!("parse_equal, idx: {}", idx);
    let length_id = u64::from_str_radix(&packet[*idx..*idx + 1], 2).unwrap();
    *idx += 1;
    //println!("length_id: {}", length_id);
    match length_id {
        0 => {
            *idx += 15;
            let val1: u64 = parse_version(packet, idx, len, true);
            let val2: u64 = parse_version(packet, idx, len, true);
            if val1 == val2 {
                return 1;
            } else {
                return 0;
            }
        }
        1 => {
            *idx += 11;
            let val1: u64 = parse_version(packet, idx, len, true);
            let val2: u64 = parse_version(packet, idx, len, true);
            if val1 == val2 {
                return 1;
            } else {
                return 0;
            }
        }
        _ => panic!("ERRORORO"),
    }
}

fn convert_to_binary_from_hex(hex: &str) -> String {
    let to_binary = hex[..].chars().map(|c| to_binary(c)).collect();

    to_binary
}

fn to_binary(c: char) -> String {
    let b = match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    };

    b.to_string()
}
