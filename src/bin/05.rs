use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io;

const DAY: &str = "05"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: io::BufRead>(reader: R) -> Result<usize> {
        let mut lines = reader.lines().flatten();
        let mut ranges = Vec::<(u64, u64)>::new();

        for line in lines.by_ref() {
            if line.trim().is_empty() {
                break; // blank line ends the range section
            }

            // Parse "a-b"
            let mut parts = line.split('-');
            let start: u64 = parts.next().unwrap().parse().unwrap();
            let end: u64 = parts.next().unwrap().parse().unwrap();
            ranges.push((start, end));
        }

        // --- Read ingredient IDs ---
        let mut ids = Vec::<u64>::new();
        for line in lines {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            ids.push(trimmed.parse().unwrap());
        }

        println!("Ranges: {:?}", ranges);
        println!("IDs: {:?}", ids);

        let mut count = 0;
        for id in ids {
            let mut is_fresh = false;
            for range in &ranges {
                if id >= range.0 && id <= range.1 {
                    is_fresh = true;
                    break;
                }
            }
            if is_fresh {
                count += 1;
            }
        }

        Ok(count)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(3, part1(io::BufReader::new(TEST.as_bytes()))?);

    let input_file = io::BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: io::BufRead>(reader: R) -> Result<u64> {
        let mut lines = reader.lines().flatten();
        let mut ranges = Vec::<(u64, u64)>::new();

        for line in lines.by_ref() {
            if line.trim().is_empty() {
                break; // blank line ends the range section
            }

            // Parse "a-b"
            let mut parts = line.split('-');
            let start: u64 = parts.next().unwrap().parse().unwrap();
            let end: u64 = parts.next().unwrap().parse().unwrap();
            ranges.push((start, end));
        }

        ranges.sort_by_key(|r| r.0);

        let mut count = 0;
        let mut upto = 0;
        let mut first = true;
        for range in ranges {
            if first {
                count += range.1 - range.0 + 1;
                upto = range.1;
                first = false;
                continue;
            }
            if range.1 <= upto {
                // included in previous
                continue;
            }
            if range.0 <= upto {
                // overlap
                count += range.1 - upto;
                upto = range.1;
                continue;
            }
            // gap
            count += range.1 - range.0 + 1;
            upto = range.1;
            first = false
        }

        Ok(count)
    }

    // TODO: Set the expected answer for the 2nd part test input
    assert_eq!(14, part2(io::BufReader::new(TEST.as_bytes()))?);

    let input_file = io::BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
