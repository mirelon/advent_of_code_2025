use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
"; // TODO: Add the test input

fn is_invalid_1(x: usize) -> bool {
    let str = x.to_string();
    let len = str.len();
    if len % 2 == 1 {
        return false;
    }
    let mid = len / 2;
    let prefix = &str[..mid];
    let suffix = &str[mid..];
    return prefix == suffix;
}

fn is_invalid_2(x: usize) -> bool {
    let str = x.to_string();
    let len = str.len();
    for part_len in 1..=len / 2 {
        if len % part_len != 0 {
            continue;
        }
        if &str == &str[..part_len].repeat(len / part_len) {
            return true;
        }
    }
    false
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut sum: usize = 0;
        reader.lines().flatten().for_each(|line| {
            line.split(",").for_each(|x| {
                let (from_str, to_str) = x.split_once("-").unwrap();
                let from = from_str.parse().unwrap();
                let to = to_str.parse().unwrap();
                for x in from..=to {
                    if is_invalid_1(x) {
                        sum += x
                    }
                }
            })
        });
        Ok(sum)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(1227775554, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 2 of the puzzle
        let mut sum: usize = 0;
        reader.lines().flatten().for_each(|line| {
            line.split(",").for_each(|x| {
                let (from_str, to_str) = x.split_once("-").unwrap();
                let from = from_str.parse().unwrap();
                let to = to_str.parse().unwrap();
                for x in from..=to {
                    if is_invalid_2(x) {
                        sum += x
                    }
                }
            })
        });
        Ok(sum)
    }

    // TODO: Set the expected answer for the 2nd part test input
    assert_eq!(4174379265, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
