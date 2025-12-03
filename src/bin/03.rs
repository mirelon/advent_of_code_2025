use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "03"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111
"; // TODO: Add the test input

fn highest_joltage(line: &str, n: usize) -> u64 {
    let digits: Vec<u64> = line
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect();
    let mut max1 = None;
    let mut max1i = 0;
    for i in 0..digits.len() - n + 1 {
        if max1 == None || digits[i] > max1.unwrap() {
            max1 = Some(digits[i]);
            max1i = i;
        }
    }
    if n == 1 {
        return max1.unwrap();
    }
    let d = max1.unwrap();
    let multiplier = 10_u64.pow(n as u32 - 1);
    d * multiplier + highest_joltage(&line[max1i + 1..], n - 1)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let answer = reader
            .lines()
            .map(|line| highest_joltage(&line.unwrap(), 2) as usize)
            .sum();
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(357, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 2 of the puzzle
        let answer = reader
            .lines()
            .map(|line| highest_joltage(&line.unwrap(), 12) as usize)
            .sum();
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(3121910778619, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
