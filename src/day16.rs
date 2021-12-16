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
        let hex: String = line.trim().trim_end_matches("0").to_string();
        res.push(convert_to_binary_from_hex(&hex));
    }
    res
}

#[aoc(day16, part1)]
pub fn solve_day16_part1(input: &Inp) -> u64 {
    return parse_version(&input[0], 0, input[0].len());
}

#[aoc(day16, part2)]
pub fn solve_day16_part2(input: &Inp) -> u64 {
    0
}

fn parse_version(packet: &str, idx: usize, len: usize) -> u64 {
    println!("parse_version, idx: {}", idx);
    let version = u64::from_str_radix(&packet[idx..idx + 3], 2).unwrap();
    return version + parse_id(packet, idx + 3, len);
}

fn parse_id(packet: &str, idx: usize, len: usize) -> u64 {
    println!("parse_id, idx: {}", idx);
    let id = u64::from_str_radix(&packet[idx..idx + 3], 2).unwrap();
    match id {
        4 => parse_literal(packet, idx + 3, len),
        _ => parse_operator(packet, idx + 3, len),
    }
}

fn parse_literal(packet: &str, idx: usize, len: usize) -> u64 {
    println!("parse_literal, idx: {}", idx);
    return 0;
}

fn parse_operator(packet: &str, idx: usize, len: usize) -> u64 {
    println!("parse_operator, idx: {}", idx);
    let length_id = u64::from_str_radix(&packet[idx..idx + 1], 2).unwrap();
    println!("length_id: {}", length_id);
    match length_id {
        0 => {
            let length = usize::from_str_radix(&packet[(idx + 1)..idx + 16], 2).unwrap();
            let num = (len - idx - 16) / length;
            let mut sum: u64 = 0;
            println!("num: {}, length: {}", num, length);
            for i in 0..num {
                sum += parse_version(packet, i * length + idx + 16, length);
            }
            sum
        }
        1 => {
            let num = usize::from_str_radix(&packet[idx + 1..idx + 12], 2).unwrap();
            let length = (len - idx - 12) / num;
            println!("num: {}, length: {}, len: {}", num, length, len);
            let mut sum: u64 = 0;
            for i in 0..num {
                sum += parse_version(packet, i * length + idx + 12, length);
            }
            sum
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
