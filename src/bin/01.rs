use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut cur = 50;
        let mut count = 0;
        reader
            .lines()
            .flatten()
            .filter(|l| !l.trim().is_empty())
            .for_each(|line| {
                let (dir, rest) = line.split_at(1);
                let num = rest.parse::<i64>().unwrap();
                match dir {
                    "L" => cur = (cur - num) % 100,
                    "R" => cur = (cur + num) % 100,
                    _ => panic!("invalid direction"),
                }
                if cur == 0 {
                    count += 1;
                }
            });
        Ok(count)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 2 of the puzzle
        let mut cur = 50;
        let mut count = 0;
        reader
            .lines()
            .flatten()
            .filter(|l| !l.trim().is_empty())
            .for_each(|line| {
                let (dir, rest) = line.split_at(1);
                let num = rest.parse::<i64>().unwrap();
                if num == 0 {
                    return;
                }
                match dir {
                    "L" => {
                        let new_cur = cur - num;
                        let mut clicks = (100 - new_cur) / 100;
                        if cur == 0 {
                            clicks -= 1
                        }
                        count += clicks;
                        cur = ((new_cur % 100) + 100) % 100
                    }
                    "R" => {
                        let new_cur = cur + num;
                        let clicks = new_cur / 100;
                        count += clicks;
                        cur = new_cur % 100
                    }
                    _ => panic!("invalid direction"),
                }
            });
        Ok(count as usize)
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
